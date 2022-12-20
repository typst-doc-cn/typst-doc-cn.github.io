use typst::model::StyledNode;

use super::{AlignNode, Spacing};
use crate::prelude::*;

/// # Stack
/// Arrange content and spacing along an axis.
///
/// ## Parameters
/// - items: StackChild (positional, variadic)
///   The items to stack along an axis.
///
/// - dir: Dir (named)
///   The direction along which the items are stacked.
///
/// - spacing: Spacing (named)
///   Spacing to insert between items where no explicit spacing was provided.
///
/// ## Category
/// layout
#[func]
#[capable(Layout)]
#[derive(Debug, Hash)]
pub struct StackNode {
    /// The stacking direction.
    pub dir: Dir,
    /// The spacing between non-spacing children.
    pub spacing: Option<Spacing>,
    /// The children to be stacked.
    pub children: Vec<StackChild>,
}

#[node]
impl StackNode {
    fn construct(_: &Vm, args: &mut Args) -> SourceResult<Content> {
        Ok(Self {
            dir: args.named("dir")?.unwrap_or(Dir::TTB),
            spacing: args.named("spacing")?,
            children: args.all()?,
        }
        .pack())
    }

    fn field(&self, name: &str) -> Option<Value> {
        match name {
            "dir" => Some(Value::dynamic(self.dir)),
            "spacing" => Some(match self.spacing {
                Some(spacing) => spacing.encode(),
                None => Value::None,
            }),
            "items" => Some(Value::Array(
                self.children
                    .iter()
                    .map(|child| match child {
                        StackChild::Spacing(spacing) => spacing.encode(),
                        StackChild::Block(content) => Value::Content(content.clone()),
                    })
                    .collect(),
            )),
            _ => None,
        }
    }
}

impl Layout for StackNode {
    fn layout(
        &self,
        vt: &mut Vt,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<Fragment> {
        let mut layouter = StackLayouter::new(self.dir, regions, styles);

        // Spacing to insert before the next block.
        let mut deferred = None;

        for child in &self.children {
            match child {
                StackChild::Spacing(kind) => {
                    layouter.layout_spacing(*kind);
                    deferred = None;
                }
                StackChild::Block(block) => {
                    if let Some(kind) = deferred {
                        layouter.layout_spacing(kind);
                    }

                    layouter.layout_block(vt, block, styles)?;
                    deferred = self.spacing;
                }
            }
        }

        Ok(layouter.finish())
    }
}

/// A child of a stack node.
#[derive(Hash)]
pub enum StackChild {
    /// Spacing between other children.
    Spacing(Spacing),
    /// Arbitrary block-level content.
    Block(Content),
}

impl Debug for StackChild {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Spacing(kind) => kind.fmt(f),
            Self::Block(block) => block.fmt(f),
        }
    }
}

castable! {
    StackChild,
    spacing: Spacing => Self::Spacing(spacing),
    content: Content => Self::Block(content),
}

/// Performs stack layout.
struct StackLayouter<'a> {
    /// The stacking direction.
    dir: Dir,
    /// The axis of the stacking direction.
    axis: Axis,
    /// The regions to layout children into.
    regions: Regions<'a>,
    /// The inherited styles.
    styles: StyleChain<'a>,
    /// Whether the stack itself should expand to fill the region.
    expand: Axes<bool>,
    /// The full size of the current region that was available at the start.
    full: Size,
    /// The generic size used by the frames for the current region.
    used: Gen<Abs>,
    /// The sum of fractions in the current region.
    fr: Fr,
    /// Already layouted items whose exact positions are not yet known due to
    /// fractional spacing.
    items: Vec<StackItem>,
    /// Finished frames for previous regions.
    finished: Vec<Frame>,
}

/// A prepared item in a stack layout.
enum StackItem {
    /// Absolute spacing between other items.
    Absolute(Abs),
    /// Fractional spacing between other items.
    Fractional(Fr),
    /// A frame for a layouted block.
    Frame(Frame, Align),
}

impl<'a> StackLayouter<'a> {
    /// Create a new stack layouter.
    fn new(dir: Dir, regions: Regions<'a>, styles: StyleChain<'a>) -> Self {
        let axis = dir.axis();
        let expand = regions.expand;
        let full = regions.first;

        // Disable expansion along the block axis for children.
        let mut regions = regions.clone();
        regions.expand.set(axis, false);

        Self {
            dir,
            axis,
            regions,
            styles,
            expand,
            full,
            used: Gen::zero(),
            fr: Fr::zero(),
            items: vec![],
            finished: vec![],
        }
    }

    /// Add spacing along the spacing direction.
    fn layout_spacing(&mut self, spacing: Spacing) {
        match spacing {
            Spacing::Relative(v) => {
                // Resolve the spacing and limit it to the remaining space.
                let resolved =
                    v.resolve(self.styles).relative_to(self.regions.base.get(self.axis));
                let remaining = self.regions.first.get_mut(self.axis);
                let limited = resolved.min(*remaining);
                *remaining -= limited;
                self.used.main += limited;
                self.items.push(StackItem::Absolute(resolved));
            }
            Spacing::Fractional(v) => {
                self.fr += v;
                self.items.push(StackItem::Fractional(v));
            }
        }
    }

    /// Layout an arbitrary block.
    fn layout_block(
        &mut self,
        vt: &mut Vt,
        block: &Content,
        styles: StyleChain,
    ) -> SourceResult<()> {
        if self.regions.is_full() {
            self.finish_region();
        }

        // Block-axis alignment of the `AlignNode` is respected
        // by the stack node.
        let aligns = if let Some(styled) = block.to::<StyledNode>() {
            styles.chain(&styled.map).get(AlignNode::ALIGNS)
        } else {
            styles.get(AlignNode::ALIGNS)
        };

        let align = aligns.get(self.axis).resolve(styles);
        let fragment = block.layout(vt, styles, self.regions)?;
        let len = fragment.len();
        for (i, frame) in fragment.into_iter().enumerate() {
            // Grow our size, shrink the region and save the frame for later.
            let size = frame.size();
            let size = match self.axis {
                Axis::X => Gen::new(size.y, size.x),
                Axis::Y => Gen::new(size.x, size.y),
            };

            self.used.main += size.main;
            self.used.cross.set_max(size.cross);
            *self.regions.first.get_mut(self.axis) -= size.main;
            self.items.push(StackItem::Frame(frame, align));

            if i + 1 < len {
                self.finish_region();
            }
        }

        Ok(())
    }

    /// Advance to the next region.
    fn finish_region(&mut self) {
        // Determine the size of the stack in this region dependening on whether
        // the region expands.
        let used = self.used.to_axes(self.axis);
        let mut size = self.expand.select(self.full, used);

        // Expand fully if there are fr spacings.
        let full = self.full.get(self.axis);
        let remaining = full - self.used.main;
        if self.fr.get() > 0.0 && full.is_finite() {
            self.used.main = full;
            size.set(self.axis, full);
        }

        let mut output = Frame::new(size);
        let mut cursor = Abs::zero();
        let mut ruler: Align = self.dir.start().into();

        // Place all frames.
        for item in self.items.drain(..) {
            match item {
                StackItem::Absolute(v) => cursor += v,
                StackItem::Fractional(v) => cursor += v.share(self.fr, remaining),
                StackItem::Frame(frame, align) => {
                    if self.dir.is_positive() {
                        ruler = ruler.max(align);
                    } else {
                        ruler = ruler.min(align);
                    }

                    // Align along the block axis.
                    let parent = size.get(self.axis);
                    let child = frame.size().get(self.axis);
                    let block = ruler.position(parent - self.used.main)
                        + if self.dir.is_positive() {
                            cursor
                        } else {
                            self.used.main - child - cursor
                        };

                    let pos = Gen::new(Abs::zero(), block).to_point(self.axis);
                    cursor += child;
                    output.push_frame(pos, frame);
                }
            }
        }

        // Advance to the next region.
        self.regions.next();
        self.full = self.regions.first;
        self.used = Gen::zero();
        self.fr = Fr::zero();
        self.finished.push(output);
    }

    /// Finish layouting and return the resulting frames.
    fn finish(mut self) -> Fragment {
        self.finish_region();
        Fragment::frames(self.finished)
    }
}

/// A container with a main and cross component.
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Gen<T> {
    /// The main component.
    pub cross: T,
    /// The cross component.
    pub main: T,
}

impl<T> Gen<T> {
    /// Create a new instance from the two components.
    const fn new(cross: T, main: T) -> Self {
        Self { cross, main }
    }

    /// Convert to the specific representation, given the current main axis.
    fn to_axes(self, main: Axis) -> Axes<T> {
        match main {
            Axis::X => Axes::new(self.main, self.cross),
            Axis::Y => Axes::new(self.cross, self.main),
        }
    }
}

impl Gen<Abs> {
    /// The zero value.
    fn zero() -> Self {
        Self { cross: Abs::zero(), main: Abs::zero() }
    }

    /// Convert to a point.
    fn to_point(self, main: Axis) -> Point {
        self.to_axes(main).to_point()
    }
}
