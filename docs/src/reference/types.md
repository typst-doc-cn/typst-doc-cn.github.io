# None
一个表示没有值的空值。

none 类型只有一个值： `{none}`。

当插入到文档中时，它是不可见的。它也是空代码块所产生的值。
它可以与任何值[结合]($scripting/#blocks)，产生另一个值。

## Example
```example
Not visible: #none
```

# Auto
一个表示智能默认值的值。

auto 类型只有一个值：`{auto}`。

支持 `{auto}` 值的参数具有一些智能默认值或上下文行为。
一个很好的例子是[文本方向参数]($func/text.dir)。
将其设置为 `{auto}` 可以让 Typst 根据[文本语言]($func/text.lang)自动确定方向。

# Boolean
可以是 `{true}` 或 `{false}`。

boolean 类型有两个值：`{true}` 和 `{false}`。它表示某事物是否处于激活或启用状态。

## Example
```example
#false \
#true \
#(1 < 2)
```

# Integer
一个整数。

这个数可以是负数、零或正数。
由于 Typst 使用 64 位来存储整数，整数不能小于 `{-9223372036854775808}` 或大于 `{9223372036854775807}`。

这个数字也可以用十六进制、八进制或二进制表示，只需以 `0` 开头，然后跟着 `x`， `o`，或 `b`。

## Example
```example
#(1 + 2) \
#(2 - 5) \
#(3 + 4 < 8)

#0xff \
#0o10 \
#0b1001
```

# Float
一个浮点数。

一个实数的有限精度表示。Typst 使用 64 位来存储浮点数。
在需要使用浮点数的任何地方，你也可以传递一个 [integer]($type/integer)。

## Example
```example
#3.14 \
#1e4 \
#(10 / 4)
```

# Length
一个尺寸或距离，可能使用上下文单位表示。
Typst 支持以下长度单位：

- 点：`{72pt}`
- 毫米：`{254mm}`
- 厘米：`{2.54cm}`
- 英寸：`{1in}`
- 相对于字体大小：`{2.5em}`

## Example
```example
#rect(width: 20pt)
#rect(width: 2em)
#rect(width: 1in)
```

# Angle
描述旋转的角度。
Typst 支持以下角度单位

- 度数：`{180deg}`
- 弧度：`{3.14rad}`

## Example
```example
#rotate(10deg)[Hello there!]
```

# Ratio
一个整数的比率。

以数字开头，后面跟着一个百分号来表示。

## Example
```example
#set align(center)
#scale(x: 150%)[
  Scaled apart.
]
```

# Relative Length
一个相对于某个已知长度的长度。

这种类型是 [length]($type/length) 和 [ratio]($type/ratio) 的组合。
它是通过将长度和比率进行加减运算得到的。在任何需要相对长度的地方，你也可以使用纯长度或比率。

## Example
```example
#rect(width: 100% - 50pt)
```

# Fraction
定义布局中剩余空间的分配方式。

每个具有分数大小的元素更具其分数与所有分数之和的比例获得空间。

详细信息，请参阅 [h]($func/h) 和 [v]($func/v) 函数以及 [grid]($func/grid) 函数。

## Example
```example
Left #h(1fr) Left-ish #h(2fr) Right
```

# Color
一个特定颜色空间中的颜色。

Typst 支持以下颜色空间：
- 通过 [`rgb`]($func/rgb) 函数支持 sRGB
- 通过 [`cmyk`]($func/cmyk) 函数支持 CMYK
- 通过 [`luma`]($func/luma) 函数支持 D65 灰度

此外，Typst 还提供了以下内置颜色：

`black`（黑色）, `gray`（灰色）, `silver`（银色）, `white`（白色）, `navy`（深蓝色）, `blue`（蓝色）, `aqua`（浅绿色）, `teal`（蓝绿色）, `eastern`（靛青色）,`purple`（紫色）, `fuchsia`（洋红色）, `maroon`（栗色）, `red`（红色）, `orange`（橙色）, `yellow`（黄色）, `olive`（橄榄色）, `green`（绿色）, 和 `lime`（石灰色）.

## Methods
### lighten()
颜色亮度。

- amount: ratio (positional, required)
  用于调亮颜色的因子。
- returns: color

### darken()
调暗颜色。

- amount: ratio (positional, required)
  用于调暗颜色的因子。
- returns: color

### negate()
生成颜色的负片效果。

- returns: color

# Datetime
日期、时间或两者的组合的表示。可以通过使用 [`datetime`]($func/datetime) 函数来指定自定义日期时间，
或使用 [`datetime.today`]($func/datetime.today) 获取当前日期。

## Example
```example
#let date = datetime(
  year: 2020,
  month: 10,
  day: 4,
)

#date.display() \
#date.display(
  "y:[year repr:last_two]"
)

#let time = datetime(
  hour: 18,
  minute: 2,
  second: 23,
)

#time.display() \
#time.display(
  "h:[hour repr:12][period]"
)
```

## Format
你可以使用 [`display`]($type/datetime.display) 方法来指定自定义的日期时间格式化方式。
日期时间的格式化由提供具有指定数量   _修饰符_ 的 _组件_ 来指定。
组件代表你想要显示的日期时间的特定部分，而借助修饰符，你可以定义如何显示该组件。
为了显示一个组件，你需要将组件名称用方括号括起来（例如，`[[year]]` 会显示年份）。
要添加修饰符，需要在组件名称之后加上一个空格，然后是修饰符的名称、冒号和修饰符的值（例如，`[[month repr:short]]` 会显示月份的短表示）。

以下是组件及其对应修饰符的可能组合方式：

* `year`：显示日期时间的年份。
  * `padding`：可以是 `zero`, `space` 或 `none`。指定年份的填充方式。
  * `repr`：可以是 `full`，此时显示完整年份；或者是 `last_two`，此时只显示最后两位数字。
  * `sign`：可以是 `automatic` 或 `mandatory`。指定何时显示符号。
* `month`：显示日期时间的月份。
  * `padding`：可以是 `zero`, `space` 或 `none`。指定月份的填充方式。
  * `repr`：可以是 `numerical`, `long` 或 `short`。指定月份是否以数字或单词形式显示。
    不幸的是，选择单词表示时，目前仅能显示英文版本。未来计划支持本地化。
* `day`：显示日期时间的日期。
  * `padding`：可以是 `zero`, `space` 或 `none`。指定日期的填充方式。
* `week_number`：显示日期时间的周数。
  * `padding`：可以是 `zero`, `space` 或 `none`。指定周数的填充方式。
  * `repr`：可以是 `ISO`, `sunday` 或 `monday`。对于 `ISO`，周数范围为 1 至 53；
    而对于其他表示方式，周数范围为 0 至 53。
* `weekday`：显示日期时间的星期。
  * `repr`：可以是 `long`, `short`, `sunday` 或 `monday`。
    对于 `long` 和 `short`，会显示相应的英文名称（与月份类似，目前不支持其他语言）。
    对于 `sunday` 和 `monday`，将显示数字值（分别假设星期日和星期一为一周的第一天）。
  * `one_indexed`：可以是 `true` 或 `false`。定义周的数值表示从 0 还是 1 开始。
* `hour`：显示日期时间的小时。
  * `padding`：可以是 `zero`, `space` 或 `none`。指定小时的填充方式。
  * `repr`：可以是 `24` 或 `12`。决定小时以 24 小时制还是 12 小时制显示。
* `period`：小时的上午/下午部分
  * `case`：可以是 `lower` 或 `upper`。定义时间段的大小写显示。
* `minute`：显示日期时间的分钟。
  * `padding`：可以是 `zero`, `space` 或 `none`。指定分钟的填充方式。
* `second`：显示日期时间的秒数。
  * `padding`：可以是 `zero`, `space` 或 `none`。指定秒数的填充方式。

请注意，并非始终可以使用所有组件。
例如，如果你创建一个新的日期时间，如 `{datetime(year: 2023, month: 10, day: 13)}`，
它将被内部存储为纯日期，这意味着你不能使用诸如 `hour` 或 `minute` 等仅适用于特定时间的日期时间的组件。

## Methods
### display()
以特定方式显示日期时间。
根据你是否仅定义了日期、时间或两者，日期时间的默认格式将会有所不同。
如果你只指定了日期，格式为 `[[year]-[month]-[day]]`。
如果你只指定了时间，格式为 `[[hour]:[minute]:[second]]`。
如果你同时指定了日期和时间，格式为 `[[year]-[month]-[day] [hour]:[minute]:[second]]`。

- pattern: string (positional)
  用于显示日期时间的格式。
- returns: string

### year()
如果日期时间中的 `year` 存在，返回它。否则，返回 `{none}`。

- returns: integer or none

### month()
如果日期时间中的 `month` 存在，返回它。否则，返回 `{none}`。

- returns: integer or none

### weekday()
如果日期时间的 `weekday` 存在，返回它按照从星期一开始并用数字表示。否则，返回 `{none}`。

- returns: integer or none

### day()
如果日期时间中的 `day` 存在，返回它。否则，返回 `{none}`。

- returns: integer or none

### hour()
如果日期时间中的 `hour` 存在，返回它。否则，返回 `{none}`。

- returns: integer or none

### minute()
如果日期时间中的 `minute` 存在，返回它，否则，返回 `{none}`。

- returns: integer or none

### second()
如果日期时间中的 `second` 存在，返回它，否则，返回 `{none}`。

- returns: integer or none

# Symbol
Unicode 符号。

Typst 定义了常见符号，一遍能够轻松地使用标准键盘进行输入。
这些符号被定义在不同的模块中，可以使用[字段访问表示法]($scripting/#fields)来访问：

- 一般符号被定义在 [`sym`]($category/symbols/sym) 模块
- Emoji 符号被定义在 [`emoji`]($category/symbols/emoji) 模块

此外，你还可以使用 [symbol]($func/symbol) 函数定义自定义符号。

```example
#sym.arrow.r \
#sym.gt.eq.not \
$gt.eq.not$ \
#emoji.face.halo
```

许多符号有不同的变体，可以通过使用点修饰符来进行选择。修饰符的顺序无关紧要。
访问符号模块的文档页面，并单击一个符号以查看其可用的变体。

```example
$arrow.l$ \
$arrow.r$ \
$arrow.t.quad$
```

# String
一串 Unicode 代码点（codepoints）。

你可以使用 [for 循环]($scripting/#loops)遍历字符串的字符簇（grapheme clusters）。
字符簇基本上是字符，但它们将属于一起的内容保持在一起，例如多个代码点一起形成一个国旗表情符号。
字符串可以使用 `+` 运算符相加，[连接]($scripting/#blocks)在一起，并与整数相乘。

Typst 提供了用于字符串操作的实用方法。
其中许多方法（例如 `split`, `trim` 和 `replace`）操作模式：模式可以是字符串或[正则表达式]($func/regex)。
这使得这些方法非常灵活。

所有长度和索引都以 UTF-8 字符为单位。
索引从零开始，负索引会回到字符串的末尾。

### Example
```example
#"hello world!" \
#"\"hello\n  world\"!" \
#"1 2 3".split() \
#"1,2;3".split(regex("[,;]")) \
#(regex("\d+") in "ten euros") \
#(regex("\d+") in "10 euros")
```

### 转义序列
与标记语言类似，您可以在字符串中转义一些符号：
- `[\\]` 代表反斜杠
- `[\"]` 代表引号
- `[\n]` 代表换行
- `[\r]` 代表回车
- `[\t]` 代表制表符
- `[\u{1f600}]` 代表一个十六进制 Unicode 转义序列

## Methods
### len()
字符串在UTF-8编码字节中的长度。

- returns: integer

### first()
提取字符串的第一个字形簇。如果字符串为空，则会失败并显示错误。

- returns: any

### last()
提取字符串的最后一个字形簇。如果字符串为空，则会失败并显示错误。

- returns: any

### at()
提取指定索引后的第一个字形簇。
如果索引超出范围，则返回默认值；如果没有指定默认值，则会失败并显示错误。

- index: integer (positional, required)
  字节索引。
- default: any (named)
  如果索引超出范围，返回的默认值。
- returns: string

### slice()
提取字符串的子串。如果开始索引或结束索引超出范围，则会失败并显示错误。

- start: integer (positional, required)
  起始字节索引（包含在内）。
- end: integer (positional)
  结束字节索引（不包含在内）。如果省略，则提取整个字符串直到末尾。
- count: integer (named)
  要提取的字节数。这相当于将 `start + count` 作为结束位置。与 `end` 互斥。
- returns: string

### clusters()
将字符串的字形簇作为子字符串数组返回。

- returns: array

### codepoints()
将字符串的 Unicode 码点作为子字符串数组返回。

- returns: array

### contains()
判断字符串是否包含指定的模式。

该方法还有专门的语法：您可以使用 `{"bc" in "abcd"}` 代替 `{"abcd".contains("bc")}`。

- pattern: string or regex (positional, required)
  搜索的模式。
- returns: boolean

### starts-with()
判断字符串是否以指定的模式开始。

- pattern: string or regex (positional, required)
  字符串可能开始的模式。
- returns: boolean

### ends-with()
判断字符串是否以指定的模式结尾。

- pattern: string or regex (positional, required)
  字符串可能结尾的模式。
- returns: boolean

### find()
Searches for the specified pattern in the string and returns the first match
as a string or `{none}` if there is no match.

- pattern: string or regex (positional, required)
  The pattern to search for.
- returns: string or none

### position()
在字符串中搜索指定的模式，并返回第一个匹配项的索引，如果没有匹配项则返回 `{none}`。

- pattern: string or regex (positional, required)
  搜索的模式。
- returns: integer or none

### match()
在字符串中搜索指定的模式，并返回一个包含第一个匹配项详细信息的字典，如果没有匹配项则返回 `{none}`。

返回的字典具有以下键：
* `start`: 匹配项的起始偏移量
* `end`: 匹配项的结束偏移量
* `text`: 匹配的文本
* `captures`: 一个数组，其中包含每个匹配的捕获组的字符串。
  数组的第一项包含第一个匹配的捕获组，而不是整个匹配！除非 `pattern` 是带有捕获组的正则表达式，否则此项为空。

- pattern: string or regex (positional, required)
  搜索的模式。
- returns: dictionary or none

### matches()
在字符串中搜索指定的模式，并返回一个包含所有匹配项详细信息的字典数组。有关返回的字典的详细信息，请参见上文。

- pattern: string or regex (positional, required)
  搜索的模式。
- returns: array

### replace()
替换所有或指定数量的匹配项的模式为替换字符串，并返回结果字符串。

- pattern: string or regex (positional, required)
  搜索的模式。
- replacement: string or function (positional, required)
  用于替换匹配项的字符串，或者一个函数，该函数会针对每个匹配项获取一个字典，并可以返回单独的替换字符串。
- count: integer (named)
  如果指定了 `count` 参数，只会替换模式的前 `count` 个匹配项。
- returns: string

### trim()
从字符串的一侧或两侧移除匹配模式的内容，可以选择一次性或重复操作，并返回结果字符串。

- pattern: string or regex (positional, required)
  搜索的模式。
- at: alignment (named)
  可以使用 `start` 或 `end` 来仅修剪字符串的开头或结尾。如果省略，则会修剪字符串的两侧。
- repeat: boolean (named)
  决定是重复移除模式的匹配项还是只移除一次。默认为 `{true}`。
- returns: string

### split()
将字符串按指定的模式进行分割，并返回结果部分的数组。

- pattern: string or regex (positional)
  要进行分割的模式。默认为空格。
- returns: array

# Content
一段文件内容。

这种类型是 Typst 的核心。你写的所有标记和你调用的大多数[函数]($type/function)都产生内容值。
你可以通过在方括号中包围标记来创建一个内容值。
这也是你向函数传递内容的方式。

```example
Type of *Hello!* is
#type([*Hello!*])
```
内容可以用 `+` 运算符添加，[连接在一起]($scripting/#blocks)，并与整数相乘。
凡是期望有内容的地方，你也可以传递一个[字符串]($type/string)或 `{none}`。

## Representation
内容由带有字段的元素组成。
当用元素函数构造一个元素时，你提供这些字段作为参数，当你有一个内容值时，你可以用[字段访问语法]($scripting/#field-access)访问其字段。

有些字段是必须的：这些字段必须在构造元素时提供，
因此，它们总是可以通过对该类型内容的字段访问而获得。必需的字段在文档中被标明。

大多数字段是可选的：像必填字段一样，它们可以被传递给元素函数来为单个元素配置它们。
然而，这些也可以用设定的[规则]($styling/#set-rules)来配置，将它们应用于一个范围内的所有元素。
可选字段只有在它们被明确地传递给元素函数时才能使用字段访问语法，而不是由设置规则产生。

每个元素都有一个默认的外观。然而，你也可以用一个[显示规则]($styling/#show-rules)来完全定制它的外观。
显示规则被传递给元素。它可以访问元素的字段并从中产生任意的内容。

在 Web 应用程序中，你可以将鼠标悬停在一个内容变量上，准确地看到内容是由哪些元素组成的，它们有哪些字段。
或者，你可以检查 [`repr`]($func/repr) 函数的输出。

## Methods
### func()
内容的元素函数。这个函数可以用来创建这个内容中包含的元素。
它可以用于元素的设置和显示规则。
可以与全局函数相比较，以检查你是否有一个特定种类的元素。

- returns: function

### has()
该内容是否有指定的字段。

- field: string (positional, required)
  搜索的字段。
- returns: boolean

### at()
访问内容上的指定字段。如果字段不存在，则返回默认值；如果没有指定默认值，则返回错误。

- field: string (positional, required)
  要访问的字段。
- default: any (named)
  如果字段不存在，要返回的默认值。
- returns: any

### fields()
返回该内容的所有字段。

```example
#rect(
  width: 10cm,
  height: 10cm,
).fields()
```

### location()
内容的位置。这只适用于通过[查询]($func/query)返回的内容，对于其他内容，它将以错误方式失败。
得到的位置可以用于[计数器]($func/counter)、[状态]($func/state)和[查询]($func/query)。

- returns: location

# Array
一个值的序列。

你可以通过把逗号分隔的数值序列放在括号里来构造一个数组。
这些值不一定是相同的类型。

你可以用 `.at()` 方法访问和更新数组中的一项。
索引是以零为基础的，负数的索引会循环到数组的末端。
你可以用 [for 循环]($scripting/#loops)来迭代一个数组。
数组可以用 `+` 运算符相加，[连接]($scripting/#blocks)在一起，并可与整数相乘。

**注意：** 一个长度为 `1` 的数组需要一个尾部的逗号，如 `{{1,)}`。
这是为了区别于简单的括号表达式，如 `{(1 + 2) * 3}`。
空数组为 `{()}`。

## Example
```example
#let values = (1, 7, 4, -3, 2)

#values.at(0) \
#(values.at(0) = 3)
#values.at(-1) \
#values.find(calc.even) \
#values.filter(calc.odd) \
#values.map(calc.abs) \
#values.rev() \
#(1, (2, 3)).flatten() \
#(("A", "B", "C")
    .join(", ", last: " and "))
```

## Methods
### len()
数组的长度。

- returns: integer

### first()
返回数组中的第一项。
可以用在赋值运算符的左边。
如果数组是空的，则失败并出现错误。

- returns: any

### last()
返回数组中的最后一项。
可以用在赋值运算符的左边。
如果数组是空的，则失败并出现错误。

- returns: any

### at()
返回数组中指定索引位置的项。可以在赋值的左侧使用它。
如果索引超出范围，则返回默认值；如果没有指定默认值，则会报错。

- index: integer (positional, required)
  要检索项的索引位置。
- default: any (named)
  如果索引超出范围，则返回的默认值。
- returns: any

### push()
在数组末尾添加一个值。

- value: any (positional, required)
  要插入到数组末尾的值。

### pop()
从数组中移除并返回最后一个项。如果数组为空，则会报错。

- returns: any
  The removed last value.

### insert()
在数组的指定索引位置插入一个值。如果索引超出范围，则会报错。

- index: integer (positional, required)
  要插入项的索引位置。
- value: any (positional, required)
  要插入项的值。

### remove()
从数组中删除指定索引位置的值，并将其返回。

- index: integer (positional, required)
  要删除项的索引位置。
- returns: any

### slice()
提取数组的切片。如果起始索引或结束索引超出范围，则会报错。

- start: integer (positional, required)
  起始索引（包括在内）。
- end: integer (positional)
  结束索引（不包括在内）。如果省略，则提取从起始索引到数组末尾的整个片段。
- count: integer (named)
  要提取的项数。这相当于将 `start + count` 作为结束位置。与结束索引互斥。
- returns: array

### contains()
判断数组是否包含指定的值。

这个方法还有特殊的语法：你可以写成 `{2 in (1, 2, 3)}` 而不是 `{(1, 2, 3).contains(2)}`。

- value: any (positional, required)
  要搜索的值。
- returns: boolean

### find()
搜索满足给定函数返回 `{true}` 的项，并返回第一个匹配的结果，如果没有匹配则返回 `{none}`。

- searcher: function (positional, required)
  应用于每个项的函数。必须返回一个布尔值。
- returns: any or none

### position()
搜索满足给定函数返回 `{true}` 的项，并返回第一个匹配的索引，如果没有匹配则返回 `{none}`。

- searcher: function (positional, required)
  应用于每个项的函数。必须返回一个布尔值。
- returns: integer or none

### filter()
生成一个新数组，只包含原始数组中满足给定函数返回 `{true}` 的项。

- test: function (positional, required)
  应用于每个项的函数。必须返回一个布尔值。
- returns: array

### map()
使用给定函数将原始数组中的所有项进行转换，生成一个新数组。

- mapper: function (positional, required)
  应用于每个项的函数。
- returns: array

### enumerate()
返回一个新数组，其中包含值及其索引。

返回的数组由形如长度为 2 的数组的 `(index, value)` 对组成。
可以使用 let 绑定或 for 循环对其进行[分解]($scripting/#bindings)。

- returns: array

### zip()
将两个数组进行合并。
如果两个数组长度不相等，它将仅合并到较小数组的最后一个元素，并忽略剩余的元素。
返回值是一个数组，其中每个元素又是一个大小为 2 的数组。

- other: array (positional, required)
  应该与当前数组进行合并的另一个数组。
- returns: array

### fold()
使用累加器函数将所有项折叠成一个单一的值。

- init: any (positional, required)
  用于初始化的初始值。
- folder: function (positional, required)
  折叠函数。必须具有两个参数：一个用于累加值，一个用于项。
- returns: any

### sum()
对所有项进行求和（适用于任何可相加的类型）。

- default: any (named)
  如果数组为空，应该返回什么。如果数组可以为空，则必须设置该值。
- returns: any

### product()
计算所有项的乘积（适用于任何可相乘的类型）。

- default: any (named)
  如果数组为空，需要返回什么。如果数组可以为空，则必须设置该值。
- returns: any

### any()
判断给定的函数是否对数组中的任何项返回 `{true}`。

- test: function (positional, required)
  对每个项应用的函数。该函数必须返回一个布尔值。
- returns: boolean

### all()
判断给定的函数是否对数组中的所有项返回 `{true}`。

- test: function (positional, required)
  对每个项应用的函数。该函数必须返回一个布尔值。
- returns: boolean

### flatten()
将所有嵌套数组合并为一个扁平的数组。

- returns: array

### rev()
以相反的顺序返回一个新数组，但其中包含相同的项。

- returns: array

### join()
将数组中的所有项合并为一个项。

- separator: any (positional)
  在数组的每个项之间插入一个值。
- last: any (named)
  最后两个项之间的替代分隔符。
- returns: any

### sorted()
返回一个具有相同项，且排好序的新数组。

- key: function (named)
  如果给定，则将此函数应用于数组中的元素，以确定排序的键。
- returns: array

# Dictionary
这是一个从字符串键到值的映射。

您可以在括号中使用逗号分隔的键值对构建字典。值的类型不必相同。
由于空括号已经产生了一个空数组，所以您必须使用特殊的 `(:)` 语法创建一个空字典。

字典在概念上类似于数组，但其索引是字符串而不是整数。
您可以使用 `.at()` 方法访问和创建字典条目。如果你知道键，则可以使用[字段访问符号]($scripting/#fields)(`.key`)访问值。
可以使用 `+` 运算符将字典相[加并]($scripting/#fields)合并在一起。要检查字典中是否存在某个键，请使用 `in` 关键字。

您可以使用 [for 循环]($scripting/#loops)迭代字典中的键值对。这将按照插入 / 声明的顺序进行迭代。

## Example
```example
#let dict = (
  name: "Typst",
  born: 2019,
)

#dict.name \
#(dict.launch = 20)
#dict.len() \
#dict.keys() \
#dict.values() \
#dict.at("born") \
#dict.insert("city", "Berlin ")
#("name" in dict)
```

## Methods
### len()
字典中键值对的数量。

- returns: integer

### at()
返回字典中与指定键关联的值。如果字典中已存在该键，可以在赋值语句的左侧使用它。
如果键不是字典的一部分，则返回默认值，如果没有指定默认值，则返回错误。

- key: string (positional, required)
  要检索项目的键。
- default: any (named)
  如果键不是字典的一部分，则返回的默认值。
- returns: any

### insert()
将一个新的键值对插入字典，并返回该值。如果字典已经包含了这个键，则更新该值。

- key: string (positional, required)
  要插入的键值对的键。
- value: any (positional, required)
  要插入的键值对的值。

### keys()
以插入顺序将字典的键作为数组返回。

- returns: array

### values()
以插入顺序将字典的值作为数组返回。

- returns: array

### pairs()
将字典的键和值作为键值对的数组返回。每个键值对都表示为长度为 2 的数组。

- returns: array

### remove()
通过键从字典中删除一个键值对，并返回该值。

- key: string (positional, required)
  要从字典中删除的键值对的键。
- returns: any

# Function
参数值到返回值的映射。

你可以通过在函数名后直接写一个用逗号分隔的参数列表，并用括号括起来来调用函数。
此外，你还可以在普通参数列表之后将任意数量的尾部内容块参数传递给函数。
如果普通参数列表为空，则可以省略它。
Typst 支持位置参数和命名参数。前者通过位置和类型进行标识，而后者则以 `name: value` 的形式书写。

在数学模式下，函数调用具有特殊的行为。更多详情请参阅[数学文档]($category/math)。

### Example
```example
// Call a function.
#list([A], [B])

// Named arguments and trailing
// content blocks.
#enum(start: 2)[A][B]

// Version without parentheses.
#list[A][B]
```

函数是 Typst 的基本构建块。Typst 提供了许多用于各种排版任务的函数。
此外，你编写的标记是由函数支持的，所有样式化都通过函数完成。
这个参考文档列出了所有可用的函数以及如何使用它们。
请参考有关[设置规则]($styling/#set-rules)和[展示规则]($styling/#show-rules)的文档，了解在 Typst 中可以使用函数的其他方式。

### Element functions { #element-functions }
某些函数与元素（如[标题]($func/heading)或[表格]($func/table)）相关联。
当调用这些函数时，它们会创建相应种类的元素。
与普通函数不同，它们还可以在[设置规则]($styling/#set-rules)、[展示规则]($styling/#show-rules)和[选择器]($type/selector)中进一步使用。

### Function scopes { #function-scopes }
函数可以在自己的作用域中保存相关定义，类似于一个[模块]($scripting/#modules)。例如，[`assert.eq`]($func/assert.eq) 或 [`list.item`]($func/list.item)。
然而，目前此功能仅适用于内置函数。

### Defining functions { #definitions }
你可以使用 [let]($scripting/#bindings) 绑定来定义自己的函数，绑定名称后面可以有一个参数列表。
参数列表可以包含位置参数、具有默认值的命名参数和[参数接收器]($type/arguments)。
绑定的右边可以是一个块或任何其他表达式。它定义了函数的返回值，并且可以依赖于参数。

```example
#let alert(body, fill: red) = {
  set text(white)
  set align(center)
  rect(
    fill: fill,
    inset: 8pt,
    radius: 4pt,
    [*Warning:\ #body*],
  )
}

#alert[
  Danger is imminent!
]

#alert(fill: blue)[
  KEEP OFF TRACKS
]
```

### Unnamed functions { #unnamed }
你还可以通过指定一个参数列表，后跟 `=>` 和函数体，创建一个没有绑定的匿名函数。
如果你的函数只有一个参数，则参数列表周围的括号是可选的。
匿名函数主要适用于展示规则，但也适用于接受函数的可设置属性，比如页面函数的[页脚]($func/page.footer)属性。

```example
#show "once?": it => [#it #it]
once?
```

### Notable fact
在 Typst 中，所有函数都是纯函数。这意味着对于相同的参数，它们总是返回相同的结果。它们不能在第二次调用时“记住”某些内容以产生另一个值。

唯一的例外是像 [`array.push(value)`]($type/array.push) 这样的内置方法。它们可以修改调用它们的值。

## Methods
### with()
返回一个新的函数，该函数在调用时会预先应用给定的参数。

- arguments: any (variadic)
  要应用的命名和位置参数。
- returns: function

### where()
返回一个选择器，该选择器用于筛选属于此函数的元素，其字段具有给定参数的值。

- fields: any (named, variadic)
  要按字段筛选的值。
- returns: selector

# Arguments
在函数中捕获参数。

与内置函数类似，自定义函数也可以接受可变数量的参数。
您可以使用 `..sink` 指定一个参数接收器，它将收集所有多余的参数。
`sink` 值的类型与 `argument` 相同。
它提供了访问位置和命名参数的方法，并且可以通过 [for 循环]($scripting/#loops)进行迭代。
反过来，您可以使用扩展运算符将参数、数组和字典展开到函数调用中：`{func(..args)}`。

## Example
```example
#let format(title, ..authors) = [
  *#title* \
  _Written by #(authors
    .pos()
    .join(", ", last: " and "));._
]

#format("ArtosFlow", "Jane", "Joe")
```

## Methods
### pos()
将捕获的位置参数作为数组返回。

- returns: array

### named()
将捕获的命名参数作为字典返回。

- returns: dictionary

# Selector
一个用于选择文档中元素的过滤器。

你可以按以下方式构建过滤器：
- 你可以使用元素[函数]($type/function)
- 你可以过滤具有[特定字段]($type/function.where)的元素函数
- 你可以使用[字符串]($type/string)或[正则表达式]($func/regex)
- 你可以使用 [`{<label>}`]($func/label)
- 您可以使用位置 [`location`]($func/locate)
- 调用[选择器]($func/selector)函数以将上述任何类型转换为选择器值，并使用以下方法进行进一步的细化

选择器用于对元素应用[样式规则]($styling/#show-rules)。
你还可以使用选择器[查询]($func/query)文档中特定类型的元素。

此外，您可以将选择器传递给 Typst 的几个内置函数，以配置它们的行为。
其中一个示例是大纲 [outline]($func/outline)，它可以用于更改大纲中列出的元素。

可以使用下面展示的方法组合多个选择器。但是，目前并不是所有类型的选择器在所有位置都受支持。

## Example
```example
#locate(loc => query(
  heading.where(level: 1)
    .or(heading.where(level: 2)),
  loc,
))

= This will be found
== So will this
=== But this will not.
```

## Methods
### or()
允许组合一系列选择器中的任意个。这用于同时选择多个组件或具有不同属性的组件。

- other: selector (variadic, required)
  要进行匹配的选择器列表。

### and()
允许组合一系列选择器中的所有选择器。这用于同时检查组件是否满足多个选择规则。

- other: selector (variadic, required)
  要进行匹配的选择器列表。

### before()
返回一个修改后的选择器，该选择器仅匹配在选择器参数的第一个匹配之前出现的元素。

- end: selector (positional, required)
  原始选择将在第一个 `end` 选择器匹配的位置结束。
- inclusive: boolean (named)
  确定 `end` 本身是否应匹配。这仅在两个选择器都匹配相同类型的元素时才相关。默认为 `{true}`。

### after()
返回一个修改后的选择器，该选择器仅匹配在选择器参数的第一个匹配之后出现的元素。

- start: selector (positional, required)
  原始选择将从 `start` 选择器的第一个匹配开始。
- inclusive: boolean (named)
  确定 `start` 本身是否应匹配。这仅在两个选择器都匹配相同类型的元素时才相关。默认为 `{true}`。

# Module
一个评估模块，可以是内置模块，也可以是来自文件的结果。

你可以使用[字段访问符号]($scripting/#fields)来访问模块中的定义，并可以使用 [import 和 include]($scripting/#modules) 语法与其交互。 

## Example
```example
<<< #import "utils.typ"
<<< #utils.add(2, 5)

<<< #import utils: sub
<<< #sub(1, 4)
>>> #7
>>>
>>> #(-3)
```
