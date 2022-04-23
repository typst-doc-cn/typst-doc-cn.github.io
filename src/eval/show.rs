use std::any::{Any, TypeId};
use std::fmt::{self, Debug, Formatter};
use std::hash::Hash;
use std::sync::Arc;

use super::{Content, Dict, StyleChain};
use crate::diag::TypResult;
use crate::util::Prehashed;
use crate::Context;

/// A node that can be realized given some styles.
pub trait Show: 'static {
    /// Encode this node into a dictionary.
    fn encode(&self) -> Dict;

    /// Show this node in the given styles and optionally given the realization
    /// of a show rule.
    fn show(
        &self,
        ctx: &mut Context,
        styles: StyleChain,
        realized: Option<Content>,
    ) -> TypResult<Content>;

    /// Convert to a packed show node.
    fn pack(self) -> ShowNode
    where
        Self: Debug + Hash + Sized + Sync + Send + 'static,
    {
        ShowNode::new(self)
    }
}

/// A type-erased showable node with a precomputed hash.
#[derive(Clone, Hash)]
pub struct ShowNode(Arc<Prehashed<dyn Bounds>>);

impl ShowNode {
    /// Pack any showable node.
    pub fn new<T>(node: T) -> Self
    where
        T: Show + Debug + Hash + Sync + Send + 'static,
    {
        Self(Arc::new(Prehashed::new(node)))
    }

    /// The type id of this node.
    pub fn id(&self) -> TypeId {
        self.0.as_any().type_id()
    }
}

impl Show for ShowNode {
    fn encode(&self) -> Dict {
        self.0.encode()
    }

    fn show(
        &self,
        ctx: &mut Context,
        styles: StyleChain,
        realized: Option<Content>,
    ) -> TypResult<Content> {
        self.0.show(ctx, styles, realized)
    }

    fn pack(self) -> ShowNode {
        self
    }
}

impl Debug for ShowNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq for ShowNode {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

trait Bounds: Show + Debug + Sync + Send + 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T> Bounds for T
where
    T: Show + Debug + Hash + Sync + Send + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}