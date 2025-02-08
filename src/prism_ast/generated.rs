use std::fmt::Write;

use super::deserialize::*;
use enumflags2::BitFlag;
use winnow::binary::length_repeat;

/* Flags for arguments nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgumentsNodeFlags {
    NEWLINE = 1 << 0,
    STATIC_LITERAL = 1 << 1,
    /* Flags for arguments nodes. */
    CONTAINS_FORWARDING = 1 << 2,
    /* Flags for arguments nodes. */
    CONTAINS_KEYWORDS = 1 << 3,
    /* Flags for arguments nodes. */
    CONTAINS_KEYWORD_SPLAT = 1 << 4,
    /* Flags for arguments nodes. */
    CONTAINS_SPLAT = 1 << 5,
    /* Flags for arguments nodes. */
    CONTAINS_MULTIPLE_SPLATS = 1 << 6,
}

/* Flags for array nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayNodeFlags {
    NEWLINE = 1 << 0,
    STATIC_LITERAL = 1 << 1,
    /* Flags for array nodes. */
    CONTAINS_SPLAT = 1 << 2,
}

/* Flags for call nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallNodeFlags {
    NEWLINE = 1 << 0,
    STATIC_LITERAL = 1 << 1,
    /* Flags for call nodes. */
    SAFE_NAVIGATION = 1 << 2,
    /* Flags for call nodes. */
    VARIABLE_CALL = 1 << 3,
    /* Flags for call nodes. */
    ATTRIBUTE_WRITE = 1 << 4,
    /* Flags for call nodes. */
    IGNORE_VISIBILITY = 1 << 5,
}

/* Flags for nodes that have unescaped content. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodingFlags {
    NEWLINE = 1 << 0,
    STATIC_LITERAL = 1 << 1,
    /* Flags for nodes that have unescaped content. */
    FORCED_UTF8_ENCODING = 1 << 2,
    /* Flags for nodes that have unescaped content. */
    FORCED_BINARY_ENCODING = 1 << 3,
}

/* Flags for integer nodes that correspond to the base of the integer. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerBaseFlags {
    NEWLINE = 1 << 0,
    STATIC_LITERAL = 1 << 1,
    /* Flags for integer nodes that correspond to the base of the integer. */
    BINARY = 1 << 2,
    /* Flags for integer nodes that correspond to the base of the integer. */
    DECIMAL = 1 << 3,
    /* Flags for integer nodes that correspond to the base of the integer. */
    OCTAL = 1 << 4,
    /* Flags for integer nodes that correspond to the base of the integer. */
    HEXADECIMAL = 1 << 5,
}

/* Flags for interpolated string nodes that indicated mutability if they are also marked as literals. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpolatedStringNodeFlags {
    NEWLINE = 1 << 0,
    STATIC_LITERAL = 1 << 1,
    /* Flags for interpolated string nodes that indicated mutability if they are also marked as literals. */
    FROZEN = 1 << 2,
    /* Flags for interpolated string nodes that indicated mutability if they are also marked as literals. */
    MUTABLE = 1 << 3,
}

/* Flags for keyword hash nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeywordHashNodeFlags {
    NEWLINE = 1 << 0,
    STATIC_LITERAL = 1 << 1,
    /* Flags for keyword hash nodes. */
    SYMBOL_KEYS = 1 << 2,
}

/* Flags for while and until loop nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopFlags {
    NEWLINE = 1 << 0,
    STATIC_LITERAL = 1 << 1,
    /* Flags for while and until loop nodes. */
    BEGIN_MODIFIER = 1 << 2,
}

/* Flags for parameter nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterFlags {
    NEWLINE = 1 << 0,
    STATIC_LITERAL = 1 << 1,
    /* Flags for parameter nodes. */
    REPEATED_PARAMETER = 1 << 2,
}

/* Flags for range and flip-flop nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeFlags {
    NEWLINE = 1 << 0,
    STATIC_LITERAL = 1 << 1,
    /* Flags for range and flip-flop nodes. */
    EXCLUDE_END = 1 << 2,
}

/* Flags for regular expression and match last line nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegularExpressionFlags {
    NEWLINE = 1 << 0,
    STATIC_LITERAL = 1 << 1,
    /* Flags for regular expression and match last line nodes. */
    IGNORE_CASE = 1 << 2,
    /* Flags for regular expression and match last line nodes. */
    EXTENDED = 1 << 3,
    /* Flags for regular expression and match last line nodes. */
    MULTI_LINE = 1 << 4,
    /* Flags for regular expression and match last line nodes. */
    ONCE = 1 << 5,
    /* Flags for regular expression and match last line nodes. */
    EUC_JP = 1 << 6,
    /* Flags for regular expression and match last line nodes. */
    ASCII_8BIT = 1 << 7,
    /* Flags for regular expression and match last line nodes. */
    WINDOWS_31J = 1 << 8,
    /* Flags for regular expression and match last line nodes. */
    UTF_8 = 1 << 9,
    /* Flags for regular expression and match last line nodes. */
    FORCED_UTF8_ENCODING = 1 << 10,
    /* Flags for regular expression and match last line nodes. */
    FORCED_BINARY_ENCODING = 1 << 11,
    /* Flags for regular expression and match last line nodes. */
    FORCED_US_ASCII_ENCODING = 1 << 12,
}

/* Flags for shareable constant nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShareableConstantNodeFlags {
    NEWLINE = 1 << 0,
    STATIC_LITERAL = 1 << 1,
    /* Flags for shareable constant nodes. */
    LITERAL = 1 << 2,
    /* Flags for shareable constant nodes. */
    EXPERIMENTAL_EVERYTHING = 1 << 3,
    /* Flags for shareable constant nodes. */
    EXPERIMENTAL_COPY = 1 << 4,
}

/* Flags for string nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringFlags {
    NEWLINE = 1 << 0,
    STATIC_LITERAL = 1 << 1,
    /* Flags for string nodes. */
    FORCED_UTF8_ENCODING = 1 << 2,
    /* Flags for string nodes. */
    FORCED_BINARY_ENCODING = 1 << 3,
    /* Flags for string nodes. */
    FROZEN = 1 << 4,
    /* Flags for string nodes. */
    MUTABLE = 1 << 5,
}

/* Flags for symbol nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolFlags {
    NEWLINE = 1 << 0,
    STATIC_LITERAL = 1 << 1,
    /* Flags for symbol nodes. */
    FORCED_UTF8_ENCODING = 1 << 2,
    /* Flags for symbol nodes. */
    FORCED_BINARY_ENCODING = 1 << 3,
    /* Flags for symbol nodes. */
    FORCED_US_ASCII_ENCODING = 1 << 4,
}

/**
Represents the use of the `alias` keyword to alias a global variable.

    alias $foo $bar
    ^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct AliasGlobalVariableNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    Represents the new name of the global variable that can be used after aliasing.

        alias $foo $bar
              ^^^^
    */
    pub new_name: NodeRef,
    /**
    Represents the old name of the global variable that can be used before aliasing.

        alias $foo $bar
                   ^^^^
    */
    pub old_name: NodeRef,
    /**
    The location of the `alias` keyword.

        alias $foo $bar
        ^^^^^
    */
    pub keyword_loc: Location,
}
impl AliasGlobalVariableNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::AliasGlobalVariableNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![AliasGlobalVariableNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "AliasGlobalVariableNode.flags"
                )),
            new_name: parse_node.context(winnow::error::StrContext::Label(
                "AliasGlobalVariableNode.new_name"
            )),
            old_name: parse_node.context(winnow::error::StrContext::Label(
                "AliasGlobalVariableNode.old_name"
            )),
            keyword_loc: parse_location.context(winnow::error::StrContext::Label(
                "AliasGlobalVariableNode.keyword_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `alias` keyword to alias a method.

    alias foo bar
    ^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct AliasMethodNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    Represents the new name of the method that will be aliased.

        alias foo bar
              ^^^

        alias :foo :bar
              ^^^^

        alias :"#{foo}" :"#{bar}"
              ^^^^^^^^^
    */
    pub new_name: NodeRef,
    /**
    Represents the old name of the method that will be aliased.

        alias foo bar
                  ^^^

        alias :foo :bar
                   ^^^^

        alias :"#{foo}" :"#{bar}"
                        ^^^^^^^^^
    */
    pub old_name: NodeRef,
    /**
    Represents the location of the `alias` keyword.

        alias foo bar
        ^^^^^
    */
    pub keyword_loc: Location,
}
impl AliasMethodNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::AliasMethodNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![AliasMethodNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("AliasMethodNode.flags")),
            new_name: parse_node
                .context(winnow::error::StrContext::Label("AliasMethodNode.new_name")),
            old_name: parse_node
                .context(winnow::error::StrContext::Label("AliasMethodNode.old_name")),
            keyword_loc: parse_location.context(winnow::error::StrContext::Label(
                "AliasMethodNode.keyword_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents an alternation pattern in pattern matching.

    foo => bar | baz
           ^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct AlternationPatternNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    Represents the left side of the expression.

        foo => bar | baz
               ^^^
    */
    pub left: NodeRef,
    /**
    Represents the right side of the expression.

        foo => bar | baz
                     ^^^
    */
    pub right: NodeRef,
    /**
    Represents the alternation operator location.

        foo => bar | baz
                   ^
    */
    pub operator_loc: Location,
}
impl AlternationPatternNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::AlternationPatternNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![AlternationPatternNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "AlternationPatternNode.flags"
                )),
            left: parse_node.context(winnow::error::StrContext::Label(
                "AlternationPatternNode.left"
            )),
            right: parse_node.context(winnow::error::StrContext::Label(
                "AlternationPatternNode.right"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "AlternationPatternNode.operator_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `&&` operator or the `and` keyword.

    left and right
    ^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct AndNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    Represents the left side of the expression. It can be any [non-void expression](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        left and right
        ^^^^

        1 && 2
        ^
    */
    pub left: NodeRef,
    /**
    Represents the right side of the expression.

        left && right
                ^^^^^

        1 and 2
              ^
    */
    pub right: NodeRef,
    /**
    The location of the `and` keyword or the `&&` operator.

        left and right
             ^^^
    */
    pub operator_loc: Location,
}
impl AndNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::AndNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![AndNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("AndNode.flags")),
            left: parse_node.context(winnow::error::StrContext::Label("AndNode.left")),
            right: parse_node.context(winnow::error::StrContext::Label("AndNode.right")),
            operator_loc: parse_location
                .context(winnow::error::StrContext::Label("AndNode.operator_loc")),
        }]
        .parse_next(input)
    }
}

/**
Represents a set of arguments to a method or a keyword.

    return foo, bar, baz
           ^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ArgumentsNode {
    pub flags: enumflags2::BitFlags<ArgumentsNodeFlags>,
    /**
    The list of arguments, if present. These can be any [non-void expressions](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        foo(bar, baz)
            ^^^^^^^^
    */
    pub arguments: Vec<NodeRef>,
}
impl ArgumentsNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ArgumentsNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ArgumentsNode {
            flags: parse_varuint
                .map(ArgumentsNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("ArgumentsNode.flags")),
            arguments: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("ArgumentsNode.arguments")),
        }]
        .parse_next(input)
    }
}

/**
Represents an array literal. This can be a regular array using brackets or a special array using % like %w or %i.

    [1, 2, 3]
    ^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayNode {
    pub flags: enumflags2::BitFlags<ArrayNodeFlags>,
    /**
    Represent the list of zero or more [non-void expressions](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression) within the array.    */
    pub elements: Vec<NodeRef>,
    /**
    Represents the optional source location for the opening token.

        [1,2,3]                 # "["
        %w[foo bar baz]         # "%w["
        %I(apple orange banana) # "%I("
        foo = 1, 2, 3           # nil
    */
    pub opening_loc: Option<Location>,
    /**
    Represents the optional source location for the closing token.

        [1,2,3]                 # "]"
        %w[foo bar baz]         # "]"
        %I(apple orange banana) # ")"
        foo = 1, 2, 3           # nil
    */
    pub closing_loc: Option<Location>,
}
impl ArrayNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ArrayNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ArrayNode {
            flags: parse_varuint
                .map(ArrayNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("ArrayNode.flags")),
            elements: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("ArrayNode.elements")),
            opening_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("ArrayNode.opening_loc")),
            closing_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("ArrayNode.closing_loc")),
        }]
        .parse_next(input)
    }
}

/**
Represents an array pattern in pattern matching.

    foo in 1, 2
    ^^^^^^^^^^^

    foo in [1, 2]
    ^^^^^^^^^^^^^

    foo in *bar
    ^^^^^^^^^^^

    foo in Bar[]
    ^^^^^^^^^^^^

    foo in Bar[1, 2, 3]
    ^^^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayPatternNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub constant: Option<NodeRef>,
    /**
    Represents the required elements of the array pattern.

        foo in [1, 2]
                ^  ^
    */
    pub requireds: Vec<NodeRef>,
    /**
    Represents the rest element of the array pattern.

        foo in *bar
               ^^^^
    */
    pub rest: Option<NodeRef>,
    /**
    Represents the elements after the rest element of the array pattern.

        foo in *bar, baz
                     ^^^
    */
    pub posts: Vec<NodeRef>,
    /**
    Represents the opening location of the array pattern.

        foo in [1, 2]
               ^
    */
    pub opening_loc: Option<Location>,
    /**
    Represents the closing location of the array pattern.

        foo in [1, 2]
                    ^
    */
    pub closing_loc: Option<Location>,
}
impl ArrayPatternNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ArrayPatternNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ArrayPatternNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("ArrayPatternNode.flags")),
            constant: parse_optional_node.context(winnow::error::StrContext::Label(
                "ArrayPatternNode.constant"
            )),
            requireds: length_repeat(parse_varuint, parse_node).context(
                winnow::error::StrContext::Label("ArrayPatternNode.requireds")
            ),
            rest: parse_optional_node
                .context(winnow::error::StrContext::Label("ArrayPatternNode.rest")),
            posts: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("ArrayPatternNode.posts")),
            opening_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "ArrayPatternNode.opening_loc"
            )),
            closing_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "ArrayPatternNode.closing_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents a hash key/value pair.

    { a => b }
      ^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct AssocNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The key of the association. This can be any [non-void expression](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        { a: b }
          ^

        { foo => bar }
          ^^^

        { def a; end => 1 }
          ^^^^^^^^^^
    */
    pub key: NodeRef,
    /**
    The value of the association, if present. This can be any [non-void expression](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        { foo => bar }
                 ^^^

        { x: 1 }
             ^
    */
    pub value: NodeRef,
    /**
    The location of the `=>` operator, if present.

        { foo => bar }
              ^^
    */
    pub operator_loc: Option<Location>,
}
impl AssocNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::AssocNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![AssocNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("AssocNode.flags")),
            key: parse_node.context(winnow::error::StrContext::Label("AssocNode.key")),
            value: parse_node.context(winnow::error::StrContext::Label("AssocNode.value")),
            operator_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("AssocNode.operator_loc")),
        }]
        .parse_next(input)
    }
}

/**
Represents a splat in a hash literal.

    { **foo }
      ^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct AssocSplatNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The value to be splatted, if present. Will be missing when keyword rest argument forwarding is used.

        { **foo }
            ^^^
    */
    pub value: Option<NodeRef>,
    /**
    The location of the `**` operator.

        { **x }
          ^^
    */
    pub operator_loc: Location,
}
impl AssocSplatNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::AssocSplatNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![AssocSplatNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("AssocSplatNode.flags")),
            value: parse_optional_node
                .context(winnow::error::StrContext::Label("AssocSplatNode.value")),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "AssocSplatNode.operator_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents reading a reference to a field in the previous match.

    $'
    ^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct BackReferenceReadNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The name of the back-reference variable, including the leading `$`.

        $& # name `:$&`

        $+ # name `:$+`
    */
    pub name: ConstantRef,
}
impl BackReferenceReadNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::BackReferenceReadNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![BackReferenceReadNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "BackReferenceReadNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "BackReferenceReadNode.name"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents a begin statement.

    begin
      foo
    end
    ^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct BeginNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    Represents the location of the `begin` keyword.

        begin x end
        ^^^^^
    */
    pub begin_keyword_loc: Option<Location>,
    /**
    Represents the statements within the begin block.

        begin x end
              ^
    */
    pub statements: Option<NodeRef>,
    /**
    Represents the rescue clause within the begin block.

        begin x; rescue y; end
                 ^^^^^^^^
    */
    pub rescue_clause: Option<NodeRef>,
    /**
    Represents the else clause within the begin block.

        begin x; rescue y; else z; end
                           ^^^^^^
    */
    pub else_clause: Option<NodeRef>,
    /**
    Represents the ensure clause within the begin block.

        begin x; ensure y; end
                 ^^^^^^^^
    */
    pub ensure_clause: Option<NodeRef>,
    /**
    Represents the location of the `end` keyword.

        begin x end
                ^^^
    */
    pub end_keyword_loc: Option<Location>,
}
impl BeginNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::BeginNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![BeginNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("BeginNode.flags")),
            begin_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "BeginNode.begin_keyword_loc"
            )),
            statements: parse_optional_node
                .context(winnow::error::StrContext::Label("BeginNode.statements")),
            rescue_clause: parse_optional_node
                .context(winnow::error::StrContext::Label("BeginNode.rescue_clause")),
            else_clause: parse_optional_node
                .context(winnow::error::StrContext::Label("BeginNode.else_clause")),
            ensure_clause: parse_optional_node
                .context(winnow::error::StrContext::Label("BeginNode.ensure_clause")),
            end_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "BeginNode.end_keyword_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents a block argument using `&`.

    bar(&args)
    ^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct BlockArgumentNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The expression that is being passed as a block argument. This can be any [non-void expression](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        foo(&args)
            ^^^^^
    */
    pub expression: Option<NodeRef>,
    /**
    Represents the location of the `&` operator.

        foo(&args)
            ^
    */
    pub operator_loc: Location,
}
impl BlockArgumentNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::BlockArgumentNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![BlockArgumentNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("BlockArgumentNode.flags")),
            expression: parse_optional_node.context(winnow::error::StrContext::Label(
                "BlockArgumentNode.expression"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "BlockArgumentNode.operator_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents a block local variable.

    a { |; b| }
           ^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct BlockLocalVariableNode {
    pub flags: enumflags2::BitFlags<ParameterFlags>,
    /**
    The name of the block local variable.

        a { |; b| } # name `:b`
               ^
    */
    pub name: ConstantRef,
}
impl BlockLocalVariableNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::BlockLocalVariableNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![BlockLocalVariableNode {
            flags: parse_varuint
                .map(ParameterFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "BlockLocalVariableNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "BlockLocalVariableNode.name"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents a block of ruby code.

    [1, 2, 3].each { |i| puts x }
                   ^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct BlockNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The local variables declared in the block.

        [1, 2, 3].each { |i| puts x } # locals: [:i]
                          ^
    */
    pub locals: Vec<ConstantRef>,
    /**
    The parameters of the block.

        [1, 2, 3].each { |i| puts x }
                         ^^^
        [1, 2, 3].each { puts _1 }
                       ^^^^^^^^^^^
        [1, 2, 3].each { puts it }
                       ^^^^^^^^^^^
    */
    pub parameters: Option<NodeRef>,
    /**
    The body of the block.

        [1, 2, 3].each { |i| puts x }
                             ^^^^^^
    */
    pub body: Option<NodeRef>,
    /**
    Represents the location of the opening `|`.

        [1, 2, 3].each { |i| puts x }
                         ^
    */
    pub opening_loc: Location,
    /**
    Represents the location of the closing `|`.

        [1, 2, 3].each { |i| puts x }
                           ^
    */
    pub closing_loc: Location,
}
impl BlockNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::BlockNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![BlockNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("BlockNode.flags")),
            locals: length_repeat(parse_varuint, parse_constant)
                .context(winnow::error::StrContext::Label("BlockNode.locals")),
            parameters: parse_optional_node
                .context(winnow::error::StrContext::Label("BlockNode.parameters")),
            body: parse_optional_node.context(winnow::error::StrContext::Label("BlockNode.body")),
            opening_loc: parse_location
                .context(winnow::error::StrContext::Label("BlockNode.opening_loc")),
            closing_loc: parse_location
                .context(winnow::error::StrContext::Label("BlockNode.closing_loc")),
        }]
        .parse_next(input)
    }
}

/**
Represents a block parameter of a method, block, or lambda definition.

    def a(&b)
          ^^
    end
*/
#[derive(Debug, Clone, PartialEq)]
pub struct BlockParameterNode {
    pub flags: enumflags2::BitFlags<ParameterFlags>,
    /**
    The name of the block parameter.

        def a(&b) # name `:b`
               ^
        end
    */
    pub name: Option<ConstantRef>,
    /**
    Represents the location of the block parameter name.

        def a(&b)
               ^
    */
    pub name_loc: Option<Location>,
    /**
    Represents the location of the `&` operator.

        def a(&b)
              ^
        end
    */
    pub operator_loc: Location,
}
impl BlockParameterNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::BlockParameterNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![BlockParameterNode {
            flags: parse_varuint
                .map(ParameterFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("BlockParameterNode.flags")),
            name: parse_optional_constant
                .context(winnow::error::StrContext::Label("BlockParameterNode.name")),
            name_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "BlockParameterNode.name_loc"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "BlockParameterNode.operator_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents a block's parameters declaration.

    -> (a, b = 1; local) { }
       ^^^^^^^^^^^^^^^^^

    foo do |a, b = 1; local|
           ^^^^^^^^^^^^^^^^^
    end
*/
#[derive(Debug, Clone, PartialEq)]
pub struct BlockParametersNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    Represents the parameters of the block.

        -> (a, b = 1; local) { }
            ^^^^^^^^

        foo do |a, b = 1; local|
                ^^^^^^^^
        end
    */
    pub parameters: Option<NodeRef>,
    /**
    Represents the local variables of the block.

        -> (a, b = 1; local) { }
                      ^^^^^

        foo do |a, b = 1; local|
                          ^^^^^
        end
    */
    pub locals: Vec<NodeRef>,
    /**
    Represents the opening location of the block parameters.

        -> (a, b = 1; local) { }
           ^

        foo do |a, b = 1; local|
               ^
        end
    */
    pub opening_loc: Option<Location>,
    /**
    Represents the closing location of the block parameters.

        -> (a, b = 1; local) { }
                           ^

        foo do |a, b = 1; local|
                               ^
        end
    */
    pub closing_loc: Option<Location>,
}
impl BlockParametersNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::BlockParametersNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![BlockParametersNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "BlockParametersNode.flags"
                )),
            parameters: parse_optional_node.context(winnow::error::StrContext::Label(
                "BlockParametersNode.parameters"
            )),
            locals: length_repeat(parse_varuint, parse_node).context(
                winnow::error::StrContext::Label("BlockParametersNode.locals")
            ),
            opening_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "BlockParametersNode.opening_loc"
            )),
            closing_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "BlockParametersNode.closing_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `break` keyword.

    break foo
    ^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct BreakNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The arguments to the break statement, if present. These can be any [non-void expressions](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        break foo
              ^^^
    */
    pub arguments: Option<NodeRef>,
    /**
    The location of the `break` keyword.

        break foo
        ^^^^^
    */
    pub keyword_loc: Location,
}
impl BreakNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::BreakNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![BreakNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("BreakNode.flags")),
            arguments: parse_optional_node
                .context(winnow::error::StrContext::Label("BreakNode.arguments")),
            keyword_loc: parse_location
                .context(winnow::error::StrContext::Label("BreakNode.keyword_loc")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `&&=` operator on a call.

    foo.bar &&= value
    ^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct CallAndWriteNode {
    pub flags: enumflags2::BitFlags<CallNodeFlags>,
    /**
    The object that the method is being called on. This can be either `nil` or any [non-void expression](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        foo.bar &&= value
        ^^^
    */
    pub receiver: Option<NodeRef>,
    /**
    Represents the location of the call operator.

        foo.bar &&= value
           ^
    */
    pub call_operator_loc: Option<Location>,
    /**
    Represents the location of the message.

        foo.bar &&= value
            ^^^
    */
    pub message_loc: Option<Location>,
    /**
    Represents the name of the method being called.

        foo.bar &&= value # read_name `:bar`
            ^^^
    */
    pub read_name: ConstantRef,
    /**
    Represents the name of the method being written to.

        foo.bar &&= value # write_name `:bar=`
            ^^^
    */
    pub write_name: ConstantRef,
    /**
    Represents the location of the operator.

        foo.bar &&= value
                ^^^
    */
    pub operator_loc: Location,
    /**
    Represents the value being assigned.

        foo.bar &&= value
                    ^^^^^
    */
    pub value: NodeRef,
}
impl CallAndWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::CallAndWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![CallAndWriteNode {
            flags: parse_varuint
                .map(CallNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("CallAndWriteNode.flags")),
            receiver: parse_optional_node.context(winnow::error::StrContext::Label(
                "CallAndWriteNode.receiver"
            )),
            call_operator_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "CallAndWriteNode.call_operator_loc"
            )),
            message_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "CallAndWriteNode.message_loc"
            )),
            read_name: parse_constant.context(winnow::error::StrContext::Label(
                "CallAndWriteNode.read_name"
            )),
            write_name: parse_constant.context(winnow::error::StrContext::Label(
                "CallAndWriteNode.write_name"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "CallAndWriteNode.operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label("CallAndWriteNode.value")),
        }]
        .parse_next(input)
    }
}

/**
Represents a method call, in all of the various forms that can take.

    foo
    ^^^

    foo()
    ^^^^^

    +foo
    ^^^^

    foo + bar
    ^^^^^^^^^

    foo.bar
    ^^^^^^^

    foo&.bar
    ^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct CallNode {
    pub flags: enumflags2::BitFlags<CallNodeFlags>,
    /**
    The object that the method is being called on. This can be either `nil` or any [non-void expression](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        foo.bar
        ^^^

        +foo
         ^^^

        foo + bar
        ^^^
    */
    pub receiver: Option<NodeRef>,
    /**
    Represents the location of the call operator.

        foo.bar
           ^

        foo&.bar
           ^^
    */
    pub call_operator_loc: Option<Location>,
    /**
    Represents the name of the method being called.

        foo.bar # name `:foo`
        ^^^
    */
    pub name: ConstantRef,
    /**
    Represents the location of the message.

        foo.bar
            ^^^
    */
    pub message_loc: Option<Location>,
    /**
    Represents the location of the left parenthesis.
        foo(bar)
           ^
    */
    pub opening_loc: Option<Location>,
    /**
    Represents the arguments to the method call. These can be any [non-void expressions](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        foo(bar)
            ^^^
    */
    pub arguments: Option<NodeRef>,
    /**
    Represents the location of the right parenthesis.

        foo(bar)
               ^
    */
    pub closing_loc: Option<Location>,
    /**
    Represents the block that is being passed to the method.

        foo { |a| a }
            ^^^^^^^^^
    */
    pub block: Option<NodeRef>,
}
impl CallNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::CallNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![CallNode {
            flags: parse_varuint
                .map(CallNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("CallNode.flags")),
            receiver: parse_optional_node
                .context(winnow::error::StrContext::Label("CallNode.receiver")),
            call_operator_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "CallNode.call_operator_loc"
            )),
            name: parse_constant.context(winnow::error::StrContext::Label("CallNode.name")),
            message_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("CallNode.message_loc")),
            opening_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("CallNode.opening_loc")),
            arguments: parse_optional_node
                .context(winnow::error::StrContext::Label("CallNode.arguments")),
            closing_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("CallNode.closing_loc")),
            block: parse_optional_node.context(winnow::error::StrContext::Label("CallNode.block")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of an assignment operator on a call.

    foo.bar += baz
    ^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct CallOperatorWriteNode {
    pub flags: enumflags2::BitFlags<CallNodeFlags>,
    /**
    The object that the method is being called on. This can be either `nil` or any [non-void expressions](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        foo.bar += value
        ^^^
    */
    pub receiver: Option<NodeRef>,
    /**
    Represents the location of the call operator.

        foo.bar += value
           ^
    */
    pub call_operator_loc: Option<Location>,
    /**
    Represents the location of the message.

        foo.bar += value
            ^^^
    */
    pub message_loc: Option<Location>,
    /**
    Represents the name of the method being called.

        foo.bar += value # read_name `:bar`
            ^^^
    */
    pub read_name: ConstantRef,
    /**
    Represents the name of the method being written to.

        foo.bar += value # write_name `:bar=`
            ^^^
    */
    pub write_name: ConstantRef,
    /**
    Represents the binary operator being used.

        foo.bar += value # binary_operator `:+`
                ^
    */
    pub binary_operator: ConstantRef,
    /**
    Represents the location of the binary operator.

        foo.bar += value
                ^^
    */
    pub binary_operator_loc: Location,
    /**
    Represents the value being assigned.

        foo.bar += value
                   ^^^^^
    */
    pub value: NodeRef,
}
impl CallOperatorWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::CallOperatorWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![CallOperatorWriteNode {
            flags: parse_varuint
                .map(CallNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "CallOperatorWriteNode.flags"
                )),
            receiver: parse_optional_node.context(winnow::error::StrContext::Label(
                "CallOperatorWriteNode.receiver"
            )),
            call_operator_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "CallOperatorWriteNode.call_operator_loc"
            )),
            message_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "CallOperatorWriteNode.message_loc"
            )),
            read_name: parse_constant.context(winnow::error::StrContext::Label(
                "CallOperatorWriteNode.read_name"
            )),
            write_name: parse_constant.context(winnow::error::StrContext::Label(
                "CallOperatorWriteNode.write_name"
            )),
            binary_operator: parse_constant.context(winnow::error::StrContext::Label(
                "CallOperatorWriteNode.binary_operator"
            )),
            binary_operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "CallOperatorWriteNode.binary_operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "CallOperatorWriteNode.value"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `||=` operator on a call.

    foo.bar ||= value
    ^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct CallOrWriteNode {
    pub flags: enumflags2::BitFlags<CallNodeFlags>,
    /**
    The object that the method is being called on. This can be either `nil` or any [non-void expressions](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        foo.bar ||= value
        ^^^
    */
    pub receiver: Option<NodeRef>,
    /**
    Represents the location of the call operator.

        foo.bar ||= value
           ^
    */
    pub call_operator_loc: Option<Location>,
    /**
    Represents the location of the message.

        foo.bar ||= value
            ^^^
    */
    pub message_loc: Option<Location>,
    /**
    Represents the name of the method being called.

        foo.bar ||= value # read_name `:bar`
            ^^^
    */
    pub read_name: ConstantRef,
    /**
    Represents the name of the method being written to.

        foo.bar ||= value # write_name `:bar=`
            ^^^
    */
    pub write_name: ConstantRef,
    /**
    Represents the location of the operator.

        foo.bar ||= value
                ^^^
    */
    pub operator_loc: Location,
    /**
    Represents the value being assigned.

        foo.bar ||= value
                    ^^^^^
    */
    pub value: NodeRef,
}
impl CallOrWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::CallOrWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![CallOrWriteNode {
            flags: parse_varuint
                .map(CallNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("CallOrWriteNode.flags")),
            receiver: parse_optional_node
                .context(winnow::error::StrContext::Label("CallOrWriteNode.receiver")),
            call_operator_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "CallOrWriteNode.call_operator_loc"
            )),
            message_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "CallOrWriteNode.message_loc"
            )),
            read_name: parse_constant.context(winnow::error::StrContext::Label(
                "CallOrWriteNode.read_name"
            )),
            write_name: parse_constant.context(winnow::error::StrContext::Label(
                "CallOrWriteNode.write_name"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "CallOrWriteNode.operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label("CallOrWriteNode.value")),
        }]
        .parse_next(input)
    }
}

/**
Represents assigning to a method call.

    foo.bar, = 1
    ^^^^^^^

    begin
    rescue => foo.bar
              ^^^^^^^
    end

    for foo.bar in baz do end
        ^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct CallTargetNode {
    pub flags: enumflags2::BitFlags<CallNodeFlags>,
    /**
    The object that the method is being called on. This can be any [non-void expression](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        foo.bar = 1
        ^^^
    */
    pub receiver: NodeRef,
    /**
    Represents the location of the call operator.

        foo.bar = 1
           ^
    */
    pub call_operator_loc: Location,
    /**
    Represents the name of the method being called.

        foo.bar = 1 # name `:foo`
        ^^^
    */
    pub name: ConstantRef,
    /**
    Represents the location of the message.

        foo.bar = 1
            ^^^
    */
    pub message_loc: Location,
}
impl CallTargetNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::CallTargetNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![CallTargetNode {
            flags: parse_varuint
                .map(CallNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("CallTargetNode.flags")),
            receiver: parse_node
                .context(winnow::error::StrContext::Label("CallTargetNode.receiver")),
            call_operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "CallTargetNode.call_operator_loc"
            )),
            name: parse_constant.context(winnow::error::StrContext::Label("CallTargetNode.name")),
            message_loc: parse_location.context(winnow::error::StrContext::Label(
                "CallTargetNode.message_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents assigning to a local variable in pattern matching.

    foo => [bar => baz]
           ^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct CapturePatternNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    Represents the value to capture.

        foo => bar
               ^^^
    */
    pub value: NodeRef,
    /**
    Represents the target of the capture.

        foo => bar
        ^^^
    */
    pub target: NodeRef,
    /**
    Represents the location of the `=>` operator.

        foo => bar
            ^^
    */
    pub operator_loc: Location,
}
impl CapturePatternNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::CapturePatternNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![CapturePatternNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("CapturePatternNode.flags")),
            value: parse_node.context(winnow::error::StrContext::Label("CapturePatternNode.value")),
            target: parse_node.context(winnow::error::StrContext::Label(
                "CapturePatternNode.target"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "CapturePatternNode.operator_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of a case statement for pattern matching.

    case true
    in false
    end
    ^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct CaseMatchNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    Represents the predicate of the case match. This can be either `nil` or any [non-void expressions](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        case true; in false; end
        ^^^^
    */
    pub predicate: Option<NodeRef>,
    /**
    Represents the conditions of the case match.

        case true; in false; end
                   ^^^^^^^^
    */
    pub conditions: Vec<NodeRef>,
    /**
    Represents the else clause of the case match.

        case true; in false; else; end
                             ^^^^
    */
    pub else_clause: Option<NodeRef>,
    /**
    Represents the location of the `case` keyword.

        case true; in false; end
        ^^^^
    */
    pub case_keyword_loc: Location,
    /**
    Represents the location of the `end` keyword.

        case true; in false; end
                             ^^^
    */
    pub end_keyword_loc: Location,
}
impl CaseMatchNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::CaseMatchNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![CaseMatchNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("CaseMatchNode.flags")),
            predicate: parse_optional_node
                .context(winnow::error::StrContext::Label("CaseMatchNode.predicate")),
            conditions: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("CaseMatchNode.conditions")),
            else_clause: parse_optional_node.context(winnow::error::StrContext::Label(
                "CaseMatchNode.else_clause"
            )),
            case_keyword_loc: parse_location.context(winnow::error::StrContext::Label(
                "CaseMatchNode.case_keyword_loc"
            )),
            end_keyword_loc: parse_location.context(winnow::error::StrContext::Label(
                "CaseMatchNode.end_keyword_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of a case statement.

    case true
    when false
    end
    ^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct CaseNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    Represents the predicate of the case statement. This can be either `nil` or any [non-void expressions](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        case true; when false; end
        ^^^^
    */
    pub predicate: Option<NodeRef>,
    /**
    Represents the conditions of the case statement.

        case true; when false; end
                   ^^^^^^^^^^
    */
    pub conditions: Vec<NodeRef>,
    /**
    Represents the else clause of the case statement.

        case true; when false; else; end
                               ^^^^
    */
    pub else_clause: Option<NodeRef>,
    /**
    Represents the location of the `case` keyword.

        case true; when false; end
        ^^^^
    */
    pub case_keyword_loc: Location,
    /**
    Represents the location of the `end` keyword.

        case true; when false; end
                               ^^^
    */
    pub end_keyword_loc: Location,
}
impl CaseNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::CaseNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![CaseNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("CaseNode.flags")),
            predicate: parse_optional_node
                .context(winnow::error::StrContext::Label("CaseNode.predicate")),
            conditions: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("CaseNode.conditions")),
            else_clause: parse_optional_node
                .context(winnow::error::StrContext::Label("CaseNode.else_clause")),
            case_keyword_loc: parse_location.context(winnow::error::StrContext::Label(
                "CaseNode.case_keyword_loc"
            )),
            end_keyword_loc: parse_location
                .context(winnow::error::StrContext::Label("CaseNode.end_keyword_loc")),
        }]
        .parse_next(input)
    }
}

/**
Represents a class declaration involving the `class` keyword.

    class Foo end
    ^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ClassNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub locals: Vec<ConstantRef>,
    pub class_keyword_loc: Location,
    pub constant_path: NodeRef,
    pub inheritance_operator_loc: Option<Location>,
    pub superclass: Option<NodeRef>,
    pub body: Option<NodeRef>,
    pub end_keyword_loc: Location,
    pub name: ConstantRef,
}
impl ClassNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ClassNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ClassNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("ClassNode.flags")),
            locals: length_repeat(parse_varuint, parse_constant)
                .context(winnow::error::StrContext::Label("ClassNode.locals")),
            class_keyword_loc: parse_location.context(winnow::error::StrContext::Label(
                "ClassNode.class_keyword_loc"
            )),
            constant_path: parse_node
                .context(winnow::error::StrContext::Label("ClassNode.constant_path")),
            inheritance_operator_loc: parse_optional_location.context(
                winnow::error::StrContext::Label("ClassNode.inheritance_operator_loc")
            ),
            superclass: parse_optional_node
                .context(winnow::error::StrContext::Label("ClassNode.superclass")),
            body: parse_optional_node.context(winnow::error::StrContext::Label("ClassNode.body")),
            end_keyword_loc: parse_location.context(winnow::error::StrContext::Label(
                "ClassNode.end_keyword_loc"
            )),
            name: parse_constant.context(winnow::error::StrContext::Label("ClassNode.name")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `&&=` operator for assignment to a class variable.

    @@target &&= value
    ^^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ClassVariableAndWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The name of the class variable, which is a `@@` followed by an [identifier](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#identifiers).

        @@target &&= value # name `:@@target`
        ^^^^^^^^
    */
    pub name: ConstantRef,
    /**
    Represents the location of the variable name.

        @@target &&= value
        ^^^^^^^^
    */
    pub name_loc: Location,
    /**
    Represents the location of the `&&=` operator.

        @@target &&= value
                 ^^^
    */
    pub operator_loc: Location,
    /**
    Represents the value being assigned. This can be any [non-void expression](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        @@target &&= value
                     ^^^^^
    */
    pub value: NodeRef,
}
impl ClassVariableAndWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ClassVariableAndWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ClassVariableAndWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "ClassVariableAndWriteNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "ClassVariableAndWriteNode.name"
            )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "ClassVariableAndWriteNode.name_loc"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "ClassVariableAndWriteNode.operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "ClassVariableAndWriteNode.value"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents assigning to a class variable using an operator that isn't `=`.

    @@target += value
    ^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ClassVariableOperatorWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub name: ConstantRef,
    pub name_loc: Location,
    pub binary_operator_loc: Location,
    pub value: NodeRef,
    pub binary_operator: ConstantRef,
}
impl ClassVariableOperatorWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ClassVariableOperatorWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ClassVariableOperatorWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "ClassVariableOperatorWriteNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "ClassVariableOperatorWriteNode.name"
            )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "ClassVariableOperatorWriteNode.name_loc"
            )),
            binary_operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "ClassVariableOperatorWriteNode.binary_operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "ClassVariableOperatorWriteNode.value"
            )),
            binary_operator: parse_constant.context(winnow::error::StrContext::Label(
                "ClassVariableOperatorWriteNode.binary_operator"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `||=` operator for assignment to a class variable.

    @@target ||= value
    ^^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ClassVariableOrWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub name: ConstantRef,
    pub name_loc: Location,
    pub operator_loc: Location,
    pub value: NodeRef,
}
impl ClassVariableOrWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ClassVariableOrWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ClassVariableOrWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "ClassVariableOrWriteNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "ClassVariableOrWriteNode.name"
            )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "ClassVariableOrWriteNode.name_loc"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "ClassVariableOrWriteNode.operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "ClassVariableOrWriteNode.value"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents referencing a class variable.

    @@foo
    ^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ClassVariableReadNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The name of the class variable, which is a `@@` followed by an [identifier](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#identifiers).

        @@abc   # name `:@@abc`

        @@_test # name `:@@_test`
    */
    pub name: ConstantRef,
}
impl ClassVariableReadNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ClassVariableReadNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ClassVariableReadNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "ClassVariableReadNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "ClassVariableReadNode.name"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents writing to a class variable in a context that doesn't have an explicit value.

    @@foo, @@bar = baz
    ^^^^^  ^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ClassVariableTargetNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub name: ConstantRef,
}
impl ClassVariableTargetNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ClassVariableTargetNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ClassVariableTargetNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "ClassVariableTargetNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "ClassVariableTargetNode.name"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents writing to a class variable.

    @@foo = 1
    ^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ClassVariableWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The name of the class variable, which is a `@@` followed by an [identifier](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#identifiers).

        @@abc = 123     # name `@@abc`

        @@_test = :test # name `@@_test`
    */
    pub name: ConstantRef,
    /**
    The location of the variable name.

        @@foo = :bar
        ^^^^^
    */
    pub name_loc: Location,
    /**
    The value to write to the class variable. This can be any [non-void expression](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        @@foo = :bar
                ^^^^

        @@_xyz = 123
                 ^^^
    */
    pub value: NodeRef,
    /**
    The location of the `=` operator.

        @@foo = :bar
              ^
    */
    pub operator_loc: Location,
}
impl ClassVariableWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ClassVariableWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ClassVariableWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "ClassVariableWriteNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "ClassVariableWriteNode.name"
            )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "ClassVariableWriteNode.name_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "ClassVariableWriteNode.value"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "ClassVariableWriteNode.operator_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `&&=` operator for assignment to a constant.

    Target &&= value
    ^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantAndWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub name: ConstantRef,
    pub name_loc: Location,
    pub operator_loc: Location,
    pub value: NodeRef,
}
impl ConstantAndWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantAndWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantAndWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "ConstantAndWriteNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "ConstantAndWriteNode.name"
            )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "ConstantAndWriteNode.name_loc"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "ConstantAndWriteNode.operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "ConstantAndWriteNode.value"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents assigning to a constant using an operator that isn't `=`.

    Target += value
    ^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantOperatorWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub name: ConstantRef,
    pub name_loc: Location,
    pub binary_operator_loc: Location,
    pub value: NodeRef,
    pub binary_operator: ConstantRef,
}
impl ConstantOperatorWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantOperatorWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantOperatorWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "ConstantOperatorWriteNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "ConstantOperatorWriteNode.name"
            )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "ConstantOperatorWriteNode.name_loc"
            )),
            binary_operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "ConstantOperatorWriteNode.binary_operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "ConstantOperatorWriteNode.value"
            )),
            binary_operator: parse_constant.context(winnow::error::StrContext::Label(
                "ConstantOperatorWriteNode.binary_operator"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `||=` operator for assignment to a constant.

    Target ||= value
    ^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantOrWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub name: ConstantRef,
    pub name_loc: Location,
    pub operator_loc: Location,
    pub value: NodeRef,
}
impl ConstantOrWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantOrWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantOrWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "ConstantOrWriteNode.flags"
                )),
            name: parse_constant
                .context(winnow::error::StrContext::Label("ConstantOrWriteNode.name")),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "ConstantOrWriteNode.name_loc"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "ConstantOrWriteNode.operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "ConstantOrWriteNode.value"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `&&=` operator for assignment to a constant path.

    Parent::Child &&= value
    ^^^^^^^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantPathAndWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub target: NodeRef,
    pub operator_loc: Location,
    pub value: NodeRef,
}
impl ConstantPathAndWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantPathAndWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantPathAndWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "ConstantPathAndWriteNode.flags"
                )),
            target: parse_node.context(winnow::error::StrContext::Label(
                "ConstantPathAndWriteNode.target"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "ConstantPathAndWriteNode.operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "ConstantPathAndWriteNode.value"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents accessing a constant through a path of `::` operators.

    Foo::Bar
    ^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantPathNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The left-hand node of the path, if present. It can be `nil` or any [non-void expression](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression). It will be `nil` when the constant lookup is at the root of the module tree.

        Foo::Bar
        ^^^

        self::Test
        ^^^^

        a.b::C
        ^^^
    */
    pub parent: Option<NodeRef>,
    /**
    The name of the constant being accessed. This could be `nil` in the event of a syntax error.    */
    pub name: Option<ConstantRef>,
    /**
    The location of the `::` delimiter.

        ::Foo
        ^^

        One::Two
           ^^
    */
    pub delimiter_loc: Location,
    /**
    The location of the name of the constant.

        ::Foo
          ^^^

        One::Two
             ^^^
    */
    pub name_loc: Location,
}
impl ConstantPathNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantPathNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantPathNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("ConstantPathNode.flags")),
            parent: parse_optional_node
                .context(winnow::error::StrContext::Label("ConstantPathNode.parent")),
            name: parse_optional_constant
                .context(winnow::error::StrContext::Label("ConstantPathNode.name")),
            delimiter_loc: parse_location.context(winnow::error::StrContext::Label(
                "ConstantPathNode.delimiter_loc"
            )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "ConstantPathNode.name_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents assigning to a constant path using an operator that isn't `=`.

    Parent::Child += value
    ^^^^^^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantPathOperatorWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub target: NodeRef,
    pub binary_operator_loc: Location,
    pub value: NodeRef,
    pub binary_operator: ConstantRef,
}
impl ConstantPathOperatorWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantPathOperatorWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantPathOperatorWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "ConstantPathOperatorWriteNode.flags"
                )),
            target: parse_node.context(winnow::error::StrContext::Label(
                "ConstantPathOperatorWriteNode.target"
            )),
            binary_operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "ConstantPathOperatorWriteNode.binary_operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "ConstantPathOperatorWriteNode.value"
            )),
            binary_operator: parse_constant.context(winnow::error::StrContext::Label(
                "ConstantPathOperatorWriteNode.binary_operator"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `||=` operator for assignment to a constant path.

    Parent::Child ||= value
    ^^^^^^^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantPathOrWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub target: NodeRef,
    pub operator_loc: Location,
    pub value: NodeRef,
}
impl ConstantPathOrWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantPathOrWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantPathOrWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "ConstantPathOrWriteNode.flags"
                )),
            target: parse_node.context(winnow::error::StrContext::Label(
                "ConstantPathOrWriteNode.target"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "ConstantPathOrWriteNode.operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "ConstantPathOrWriteNode.value"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents writing to a constant path in a context that doesn't have an explicit value.

    Foo::Foo, Bar::Bar = baz
    ^^^^^^^^  ^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantPathTargetNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub parent: Option<NodeRef>,
    pub name: Option<ConstantRef>,
    pub delimiter_loc: Location,
    pub name_loc: Location,
}
impl ConstantPathTargetNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantPathTargetNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantPathTargetNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "ConstantPathTargetNode.flags"
                )),
            parent: parse_optional_node.context(winnow::error::StrContext::Label(
                "ConstantPathTargetNode.parent"
            )),
            name: parse_optional_constant.context(winnow::error::StrContext::Label(
                "ConstantPathTargetNode.name"
            )),
            delimiter_loc: parse_location.context(winnow::error::StrContext::Label(
                "ConstantPathTargetNode.delimiter_loc"
            )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "ConstantPathTargetNode.name_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents writing to a constant path.

    ::Foo = 1
    ^^^^^^^^^

    Foo::Bar = 1
    ^^^^^^^^^^^^

    ::Foo::Bar = 1
    ^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantPathWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    A node representing the constant path being written to.

        Foo::Bar = 1
        ^^^^^^^^

        ::Foo = :abc
        ^^^^^
    */
    pub target: NodeRef,
    /**
    The location of the `=` operator.

        ::ABC = 123
              ^
    */
    pub operator_loc: Location,
    /**
    The value to write to the constant path. It can be any [non-void expression](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        FOO::BAR = :abc
                   ^^^^
    */
    pub value: NodeRef,
}
impl ConstantPathWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantPathWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantPathWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "ConstantPathWriteNode.flags"
                )),
            target: parse_node.context(winnow::error::StrContext::Label(
                "ConstantPathWriteNode.target"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "ConstantPathWriteNode.operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "ConstantPathWriteNode.value"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents referencing a constant.

    Foo
    ^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantReadNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The name of the [constant](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#constants).

        X              # name `:X`

        SOME_CONSTANT  # name `:SOME_CONSTANT`
    */
    pub name: ConstantRef,
}
impl ConstantReadNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantReadNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantReadNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("ConstantReadNode.flags")),
            name: parse_constant.context(winnow::error::StrContext::Label("ConstantReadNode.name")),
        }]
        .parse_next(input)
    }
}

/**
Represents writing to a constant in a context that doesn't have an explicit value.

    Foo, Bar = baz
    ^^^  ^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantTargetNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub name: ConstantRef,
}
impl ConstantTargetNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantTargetNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantTargetNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("ConstantTargetNode.flags")),
            name: parse_constant
                .context(winnow::error::StrContext::Label("ConstantTargetNode.name")),
        }]
        .parse_next(input)
    }
}

/**
Represents writing to a constant.

    Foo = 1
    ^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The name of the [constant](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#constants).

        Foo = :bar # name `:Foo`

        XYZ = 1    # name `:XYZ`
    */
    pub name: ConstantRef,
    /**
    The location of the constant name.

        FOO = 1
        ^^^
    */
    pub name_loc: Location,
    /**
    The value to write to the constant. It can be any [non-void expression](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        FOO = :bar
              ^^^^

        MyClass = Class.new
                  ^^^^^^^^^
    */
    pub value: NodeRef,
    /**
    The location of the `=` operator.

        FOO = :bar
            ^
    */
    pub operator_loc: Location,
}
impl ConstantWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("ConstantWriteNode.flags")),
            name: parse_constant
                .context(winnow::error::StrContext::Label("ConstantWriteNode.name")),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "ConstantWriteNode.name_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label("ConstantWriteNode.value")),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "ConstantWriteNode.operator_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents a method definition.

    def method
    end
    ^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct DefNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub name: ConstantRef,
    pub name_loc: Location,
    pub receiver: Option<NodeRef>,
    pub parameters: Option<NodeRef>,
    pub body: Option<NodeRef>,
    pub locals: Vec<ConstantRef>,
    pub def_keyword_loc: Location,
    pub operator_loc: Option<Location>,
    pub lparen_loc: Option<Location>,
    pub rparen_loc: Option<Location>,
    pub equal_loc: Option<Location>,
    pub end_keyword_loc: Option<Location>,
}
impl DefNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::DefNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![DefNode{
        _: winnow::binary::u32(winnow::binary::Endianness::Native),
        flags: parse_varuint.map(DefaultNodeFlags::from_bits_truncate).context(winnow::error::StrContext::Label("DefNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("DefNode.name")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("DefNode.name_loc")),
        receiver: parse_optional_node.context(winnow::error::StrContext::Label("DefNode.receiver")),
        parameters: parse_optional_node.context(winnow::error::StrContext::Label("DefNode.parameters")),
        body: parse_optional_node.context(winnow::error::StrContext::Label("DefNode.body")),
        locals: length_repeat(parse_varuint, parse_constant).context(winnow::error::StrContext::Label("DefNode.locals")),
        def_keyword_loc: parse_location.context(winnow::error::StrContext::Label("DefNode.def_keyword_loc")),
        operator_loc: parse_optional_location.context(winnow::error::StrContext::Label("DefNode.operator_loc")),
        lparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("DefNode.lparen_loc")),
        rparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("DefNode.rparen_loc")),
        equal_loc: parse_optional_location.context(winnow::error::StrContext::Label("DefNode.equal_loc")),
        end_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("DefNode.end_keyword_loc")),
    }].parse_next(input)
    }
}

/**
Represents the use of the `defined?` keyword.

    defined?(a)
    ^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct DefinedNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub lparen_loc: Option<Location>,
    pub value: NodeRef,
    pub rparen_loc: Option<Location>,
    pub keyword_loc: Location,
}
impl DefinedNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::DefinedNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![DefinedNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("DefinedNode.flags")),
            lparen_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("DefinedNode.lparen_loc")),
            value: parse_node.context(winnow::error::StrContext::Label("DefinedNode.value")),
            rparen_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("DefinedNode.rparen_loc")),
            keyword_loc: parse_location
                .context(winnow::error::StrContext::Label("DefinedNode.keyword_loc")),
        }]
        .parse_next(input)
    }
}

/**
Represents an `else` clause in a `case`, `if`, or `unless` statement.

    if a then b else c end
                ^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ElseNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub else_keyword_loc: Location,
    pub statements: Option<NodeRef>,
    pub end_keyword_loc: Option<Location>,
}
impl ElseNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ElseNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ElseNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("ElseNode.flags")),
            else_keyword_loc: parse_location.context(winnow::error::StrContext::Label(
                "ElseNode.else_keyword_loc"
            )),
            statements: parse_optional_node
                .context(winnow::error::StrContext::Label("ElseNode.statements")),
            end_keyword_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("ElseNode.end_keyword_loc")),
        }]
        .parse_next(input)
    }
}

/**
Represents an interpolated set of statements.

    "foo #{bar}"
         ^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct EmbeddedStatementsNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub opening_loc: Location,
    pub statements: Option<NodeRef>,
    pub closing_loc: Location,
}
impl EmbeddedStatementsNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::EmbeddedStatementsNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![EmbeddedStatementsNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "EmbeddedStatementsNode.flags"
                )),
            opening_loc: parse_location.context(winnow::error::StrContext::Label(
                "EmbeddedStatementsNode.opening_loc"
            )),
            statements: parse_optional_node.context(winnow::error::StrContext::Label(
                "EmbeddedStatementsNode.statements"
            )),
            closing_loc: parse_location.context(winnow::error::StrContext::Label(
                "EmbeddedStatementsNode.closing_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents an interpolated variable.

    "foo #@bar"
         ^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct EmbeddedVariableNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub operator_loc: Location,
    pub variable: NodeRef,
}
impl EmbeddedVariableNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::EmbeddedVariableNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![EmbeddedVariableNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "EmbeddedVariableNode.flags"
                )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "EmbeddedVariableNode.operator_loc"
            )),
            variable: parse_node.context(winnow::error::StrContext::Label(
                "EmbeddedVariableNode.variable"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents an `ensure` clause in a `begin` statement.

    begin
      foo
    ensure
    ^^^^^^
      bar
    end
*/
#[derive(Debug, Clone, PartialEq)]
pub struct EnsureNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub ensure_keyword_loc: Location,
    pub statements: Option<NodeRef>,
    pub end_keyword_loc: Location,
}
impl EnsureNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::EnsureNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![EnsureNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("EnsureNode.flags")),
            ensure_keyword_loc: parse_location.context(winnow::error::StrContext::Label(
                "EnsureNode.ensure_keyword_loc"
            )),
            statements: parse_optional_node
                .context(winnow::error::StrContext::Label("EnsureNode.statements")),
            end_keyword_loc: parse_location.context(winnow::error::StrContext::Label(
                "EnsureNode.end_keyword_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the literal `false` keyword.

    false
    ^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct FalseNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
}
impl FalseNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::FalseNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![FalseNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("FalseNode.flags")),
        }]
        .parse_next(input)
    }
}

/**
Represents a find pattern in pattern matching.

    foo in *bar, baz, *qux
           ^^^^^^^^^^^^^^^

    foo in [*bar, baz, *qux]
           ^^^^^^^^^^^^^^^^^

    foo in Foo(*bar, baz, *qux)
           ^^^^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct FindPatternNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub constant: Option<NodeRef>,
    pub left: NodeRef,
    pub requireds: Vec<NodeRef>,
    pub right: NodeRef,
    pub opening_loc: Option<Location>,
    pub closing_loc: Option<Location>,
}
impl FindPatternNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::FindPatternNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![FindPatternNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("FindPatternNode.flags")),
            constant: parse_optional_node
                .context(winnow::error::StrContext::Label("FindPatternNode.constant")),
            left: parse_node.context(winnow::error::StrContext::Label("FindPatternNode.left")),
            requireds: length_repeat(parse_varuint, parse_node).context(
                winnow::error::StrContext::Label("FindPatternNode.requireds")
            ),
            right: parse_node.context(winnow::error::StrContext::Label("FindPatternNode.right")),
            opening_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "FindPatternNode.opening_loc"
            )),
            closing_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "FindPatternNode.closing_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `..` or `...` operators to create flip flops.

    baz if foo .. bar
           ^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct FlipFlopNode {
    pub flags: enumflags2::BitFlags<RangeFlags>,
    pub left: Option<NodeRef>,
    pub right: Option<NodeRef>,
    pub operator_loc: Location,
}
impl FlipFlopNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::FlipFlopNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![FlipFlopNode {
            flags: parse_varuint
                .map(RangeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("FlipFlopNode.flags")),
            left: parse_optional_node
                .context(winnow::error::StrContext::Label("FlipFlopNode.left")),
            right: parse_optional_node
                .context(winnow::error::StrContext::Label("FlipFlopNode.right")),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "FlipFlopNode.operator_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents a floating point number literal.

    1.0
    ^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct FloatNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The value of the floating point number as a Float.    */
    pub value: f64,
}
impl FloatNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::FloatNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![FloatNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("FloatNode.flags")),
            value: parse_double.context(winnow::error::StrContext::Label("FloatNode.value")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `for` keyword.

    for i in a end
    ^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ForNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The index expression for `for` loops.

        for i in a end
            ^
    */
    pub index: NodeRef,
    /**
    The collection to iterate over.

        for i in a end
                 ^
    */
    pub collection: NodeRef,
    /**
    Represents the body of statements to execute for each iteration of the loop.

        for i in a
          foo(i)
          ^^^^^^
        end
    */
    pub statements: Option<NodeRef>,
    /**
    The location of the `for` keyword.

        for i in a end
        ^^^
    */
    pub for_keyword_loc: Location,
    /**
    The location of the `in` keyword.

        for i in a end
              ^^
    */
    pub in_keyword_loc: Location,
    /**
    The location of the `do` keyword, if present.

        for i in a do end
                   ^^
    */
    pub do_keyword_loc: Option<Location>,
    /**
    The location of the `end` keyword.

        for i in a end
                   ^^^
    */
    pub end_keyword_loc: Location,
}
impl ForNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ForNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ForNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("ForNode.flags")),
            index: parse_node.context(winnow::error::StrContext::Label("ForNode.index")),
            collection: parse_node.context(winnow::error::StrContext::Label("ForNode.collection")),
            statements: parse_optional_node
                .context(winnow::error::StrContext::Label("ForNode.statements")),
            for_keyword_loc: parse_location
                .context(winnow::error::StrContext::Label("ForNode.for_keyword_loc")),
            in_keyword_loc: parse_location
                .context(winnow::error::StrContext::Label("ForNode.in_keyword_loc")),
            do_keyword_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("ForNode.do_keyword_loc")),
            end_keyword_loc: parse_location
                .context(winnow::error::StrContext::Label("ForNode.end_keyword_loc")),
        }]
        .parse_next(input)
    }
}

/**
Represents forwarding all arguments to this method to another method.

    def foo(...)
      bar(...)
          ^^^
    end
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ForwardingArgumentsNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
}
impl ForwardingArgumentsNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ForwardingArgumentsNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ForwardingArgumentsNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "ForwardingArgumentsNode.flags"
                )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the forwarding parameter in a method, block, or lambda declaration.

    def foo(...)
            ^^^
    end
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ForwardingParameterNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
}
impl ForwardingParameterNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ForwardingParameterNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ForwardingParameterNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "ForwardingParameterNode.flags"
                )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `super` keyword without parentheses or arguments.

    super
    ^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ForwardingSuperNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub block: Option<NodeRef>,
}
impl ForwardingSuperNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ForwardingSuperNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ForwardingSuperNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "ForwardingSuperNode.flags"
                )),
            block: parse_optional_node.context(winnow::error::StrContext::Label(
                "ForwardingSuperNode.block"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `&&=` operator for assignment to a global variable.

    $target &&= value
    ^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalVariableAndWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub name: ConstantRef,
    pub name_loc: Location,
    pub operator_loc: Location,
    pub value: NodeRef,
}
impl GlobalVariableAndWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::GlobalVariableAndWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![GlobalVariableAndWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "GlobalVariableAndWriteNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "GlobalVariableAndWriteNode.name"
            )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "GlobalVariableAndWriteNode.name_loc"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "GlobalVariableAndWriteNode.operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "GlobalVariableAndWriteNode.value"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents assigning to a global variable using an operator that isn't `=`.

    $target += value
    ^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalVariableOperatorWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub name: ConstantRef,
    pub name_loc: Location,
    pub binary_operator_loc: Location,
    pub value: NodeRef,
    pub binary_operator: ConstantRef,
}
impl GlobalVariableOperatorWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::GlobalVariableOperatorWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![GlobalVariableOperatorWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "GlobalVariableOperatorWriteNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "GlobalVariableOperatorWriteNode.name"
            )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "GlobalVariableOperatorWriteNode.name_loc"
            )),
            binary_operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "GlobalVariableOperatorWriteNode.binary_operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "GlobalVariableOperatorWriteNode.value"
            )),
            binary_operator: parse_constant.context(winnow::error::StrContext::Label(
                "GlobalVariableOperatorWriteNode.binary_operator"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `||=` operator for assignment to a global variable.

    $target ||= value
    ^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalVariableOrWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub name: ConstantRef,
    pub name_loc: Location,
    pub operator_loc: Location,
    pub value: NodeRef,
}
impl GlobalVariableOrWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::GlobalVariableOrWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![GlobalVariableOrWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "GlobalVariableOrWriteNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "GlobalVariableOrWriteNode.name"
            )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "GlobalVariableOrWriteNode.name_loc"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "GlobalVariableOrWriteNode.operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "GlobalVariableOrWriteNode.value"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents referencing a global variable.

    $foo
    ^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalVariableReadNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The name of the global variable, which is a `$` followed by an [identifier](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#identifier). Alternatively, it can be one of the special global variables designated by a symbol.

        $foo   # name `:$foo`

        $_Test # name `:$_Test`
    */
    pub name: ConstantRef,
}
impl GlobalVariableReadNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::GlobalVariableReadNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![GlobalVariableReadNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "GlobalVariableReadNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "GlobalVariableReadNode.name"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents writing to a global variable in a context that doesn't have an explicit value.

    $foo, $bar = baz
    ^^^^  ^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalVariableTargetNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub name: ConstantRef,
}
impl GlobalVariableTargetNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::GlobalVariableTargetNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![GlobalVariableTargetNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "GlobalVariableTargetNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "GlobalVariableTargetNode.name"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents writing to a global variable.

    $foo = 1
    ^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalVariableWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The name of the global variable, which is a `$` followed by an [identifier](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#identifier). Alternatively, it can be one of the special global variables designated by a symbol.

        $foo = :bar  # name `:$foo`

        $_Test = 123 # name `:$_Test`
    */
    pub name: ConstantRef,
    /**
    The location of the global variable's name.

        $foo = :bar
        ^^^^
    */
    pub name_loc: Location,
    /**
    The value to write to the global variable. It can be any [non-void expression](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        $foo = :bar
               ^^^^

        $-xyz = 123
                ^^^
    */
    pub value: NodeRef,
    /**
    The location of the `=` operator.

        $foo = :bar
             ^
    */
    pub operator_loc: Location,
}
impl GlobalVariableWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::GlobalVariableWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![GlobalVariableWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "GlobalVariableWriteNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "GlobalVariableWriteNode.name"
            )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "GlobalVariableWriteNode.name_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "GlobalVariableWriteNode.value"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "GlobalVariableWriteNode.operator_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents a hash literal.

    { a => b }
    ^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct HashNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The location of the opening brace.

        { a => b }
        ^
    */
    pub opening_loc: Location,
    /**
    The elements of the hash. These can be either `AssocNode`s or `AssocSplatNode`s.

        { a: b }
          ^^^^

        { **foo }
          ^^^^^
    */
    pub elements: Vec<NodeRef>,
    /**
    The location of the closing brace.

        { a => b }
                 ^
    */
    pub closing_loc: Location,
}
impl HashNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::HashNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![HashNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("HashNode.flags")),
            opening_loc: parse_location
                .context(winnow::error::StrContext::Label("HashNode.opening_loc")),
            elements: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("HashNode.elements")),
            closing_loc: parse_location
                .context(winnow::error::StrContext::Label("HashNode.closing_loc")),
        }]
        .parse_next(input)
    }
}

/**
Represents a hash pattern in pattern matching.

    foo => { a: 1, b: 2 }
           ^^^^^^^^^^^^^^

    foo => { a: 1, b: 2, **c }
           ^^^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct HashPatternNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub constant: Option<NodeRef>,
    pub elements: Vec<NodeRef>,
    pub rest: Option<NodeRef>,
    pub opening_loc: Option<Location>,
    pub closing_loc: Option<Location>,
}
impl HashPatternNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::HashPatternNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![HashPatternNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("HashPatternNode.flags")),
            constant: parse_optional_node
                .context(winnow::error::StrContext::Label("HashPatternNode.constant")),
            elements: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("HashPatternNode.elements")),
            rest: parse_optional_node
                .context(winnow::error::StrContext::Label("HashPatternNode.rest")),
            opening_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "HashPatternNode.opening_loc"
            )),
            closing_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "HashPatternNode.closing_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `if` keyword, either in the block form or the modifier form, or a ternary expression.

    bar if foo
    ^^^^^^^^^^

    if foo then bar end
    ^^^^^^^^^^^^^^^^^^^

    foo ? bar : baz
    ^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct IfNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The location of the `if` keyword if present.

        bar if foo
            ^^

    The `if_keyword_loc` field will be `nil` when the `IfNode` represents a ternary expression.
    */
    pub if_keyword_loc: Option<Location>,
    /**
    The node for the condition the `IfNode` is testing.

        if foo
           ^^^
          bar
        end

        bar if foo
               ^^^

        foo ? bar : baz
        ^^^
    */
    pub predicate: NodeRef,
    /**
    The location of the `then` keyword (if present) or the `?` in a ternary expression, `nil` otherwise.

        if foo then bar end
               ^^^^

        a ? b : c
          ^
    */
    pub then_keyword_loc: Option<Location>,
    /**
    Represents the body of statements that will be executed when the predicate is evaluated as truthy. Will be `nil` when no body is provided.

        if foo
          bar
          ^^^
          baz
          ^^^
        end
    */
    pub statements: Option<NodeRef>,
    /**
    Represents an `ElseNode` or an `IfNode` when there is an `else` or an `elsif` in the `if` statement.

        if foo
          bar
        elsif baz
        ^^^^^^^^^
          qux
          ^^^
        end
        ^^^

        if foo then bar else baz end
                        ^^^^^^^^^^^^
    */
    pub subsequent: Option<NodeRef>,
    /**
    The location of the `end` keyword if present, `nil` otherwise.

        if foo
          bar
        end
        ^^^
    */
    pub end_keyword_loc: Option<Location>,
}
impl IfNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::IfNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![IfNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("IfNode.flags")),
            if_keyword_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("IfNode.if_keyword_loc")),
            predicate: parse_node.context(winnow::error::StrContext::Label("IfNode.predicate")),
            then_keyword_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("IfNode.then_keyword_loc")),
            statements: parse_optional_node
                .context(winnow::error::StrContext::Label("IfNode.statements")),
            subsequent: parse_optional_node
                .context(winnow::error::StrContext::Label("IfNode.subsequent")),
            end_keyword_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("IfNode.end_keyword_loc")),
        }]
        .parse_next(input)
    }
}

/**
Represents an imaginary number literal.

    1.0i
    ^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ImaginaryNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub numeric: NodeRef,
}
impl ImaginaryNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ImaginaryNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ImaginaryNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("ImaginaryNode.flags")),
            numeric: parse_node.context(winnow::error::StrContext::Label("ImaginaryNode.numeric")),
        }]
        .parse_next(input)
    }
}

/**
Represents a node that is implicitly being added to the tree but doesn't correspond directly to a node in the source.

    { foo: }
      ^^^^

    { Foo: }
      ^^^^

    foo in { bar: }
             ^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ImplicitNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub value: NodeRef,
}
impl ImplicitNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ImplicitNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ImplicitNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("ImplicitNode.flags")),
            value: parse_node.context(winnow::error::StrContext::Label("ImplicitNode.value")),
        }]
        .parse_next(input)
    }
}

/**
Represents using a trailing comma to indicate an implicit rest parameter.

    foo { |bar,| }
              ^

    foo in [bar,]
               ^

    for foo, in bar do end
           ^

    foo, = bar
       ^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ImplicitRestNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
}
impl ImplicitRestNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ImplicitRestNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ImplicitRestNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("ImplicitRestNode.flags")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `in` keyword in a case statement.

    case a; in b then c end
            ^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct InNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub pattern: NodeRef,
    pub statements: Option<NodeRef>,
    pub in_loc: Location,
    pub then_loc: Option<Location>,
}
impl InNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("InNode.flags")),
            pattern: parse_node.context(winnow::error::StrContext::Label("InNode.pattern")),
            statements: parse_optional_node
                .context(winnow::error::StrContext::Label("InNode.statements")),
            in_loc: parse_location.context(winnow::error::StrContext::Label("InNode.in_loc")),
            then_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("InNode.then_loc")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `&&=` operator on a call to the `[]` method.

    foo.bar[baz] &&= value
    ^^^^^^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct IndexAndWriteNode {
    pub flags: enumflags2::BitFlags<CallNodeFlags>,
    pub receiver: Option<NodeRef>,
    pub call_operator_loc: Option<Location>,
    pub opening_loc: Location,
    pub arguments: Option<NodeRef>,
    pub closing_loc: Location,
    pub block: Option<NodeRef>,
    pub operator_loc: Location,
    pub value: NodeRef,
}
impl IndexAndWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::IndexAndWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![IndexAndWriteNode {
            flags: parse_varuint
                .map(CallNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("IndexAndWriteNode.flags")),
            receiver: parse_optional_node.context(winnow::error::StrContext::Label(
                "IndexAndWriteNode.receiver"
            )),
            call_operator_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "IndexAndWriteNode.call_operator_loc"
            )),
            opening_loc: parse_location.context(winnow::error::StrContext::Label(
                "IndexAndWriteNode.opening_loc"
            )),
            arguments: parse_optional_node.context(winnow::error::StrContext::Label(
                "IndexAndWriteNode.arguments"
            )),
            closing_loc: parse_location.context(winnow::error::StrContext::Label(
                "IndexAndWriteNode.closing_loc"
            )),
            block: parse_optional_node
                .context(winnow::error::StrContext::Label("IndexAndWriteNode.block")),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "IndexAndWriteNode.operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label("IndexAndWriteNode.value")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of an assignment operator on a call to `[]`.

    foo.bar[baz] += value
    ^^^^^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct IndexOperatorWriteNode {
    pub flags: enumflags2::BitFlags<CallNodeFlags>,
    pub receiver: Option<NodeRef>,
    pub call_operator_loc: Option<Location>,
    pub opening_loc: Location,
    pub arguments: Option<NodeRef>,
    pub closing_loc: Location,
    pub block: Option<NodeRef>,
    pub binary_operator: ConstantRef,
    pub binary_operator_loc: Location,
    pub value: NodeRef,
}
impl IndexOperatorWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::IndexOperatorWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![IndexOperatorWriteNode {
            flags: parse_varuint
                .map(CallNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "IndexOperatorWriteNode.flags"
                )),
            receiver: parse_optional_node.context(winnow::error::StrContext::Label(
                "IndexOperatorWriteNode.receiver"
            )),
            call_operator_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "IndexOperatorWriteNode.call_operator_loc"
            )),
            opening_loc: parse_location.context(winnow::error::StrContext::Label(
                "IndexOperatorWriteNode.opening_loc"
            )),
            arguments: parse_optional_node.context(winnow::error::StrContext::Label(
                "IndexOperatorWriteNode.arguments"
            )),
            closing_loc: parse_location.context(winnow::error::StrContext::Label(
                "IndexOperatorWriteNode.closing_loc"
            )),
            block: parse_optional_node.context(winnow::error::StrContext::Label(
                "IndexOperatorWriteNode.block"
            )),
            binary_operator: parse_constant.context(winnow::error::StrContext::Label(
                "IndexOperatorWriteNode.binary_operator"
            )),
            binary_operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "IndexOperatorWriteNode.binary_operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "IndexOperatorWriteNode.value"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `||=` operator on a call to `[]`.

    foo.bar[baz] ||= value
    ^^^^^^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct IndexOrWriteNode {
    pub flags: enumflags2::BitFlags<CallNodeFlags>,
    pub receiver: Option<NodeRef>,
    pub call_operator_loc: Option<Location>,
    pub opening_loc: Location,
    pub arguments: Option<NodeRef>,
    pub closing_loc: Location,
    pub block: Option<NodeRef>,
    pub operator_loc: Location,
    pub value: NodeRef,
}
impl IndexOrWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::IndexOrWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![IndexOrWriteNode {
            flags: parse_varuint
                .map(CallNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("IndexOrWriteNode.flags")),
            receiver: parse_optional_node.context(winnow::error::StrContext::Label(
                "IndexOrWriteNode.receiver"
            )),
            call_operator_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "IndexOrWriteNode.call_operator_loc"
            )),
            opening_loc: parse_location.context(winnow::error::StrContext::Label(
                "IndexOrWriteNode.opening_loc"
            )),
            arguments: parse_optional_node.context(winnow::error::StrContext::Label(
                "IndexOrWriteNode.arguments"
            )),
            closing_loc: parse_location.context(winnow::error::StrContext::Label(
                "IndexOrWriteNode.closing_loc"
            )),
            block: parse_optional_node
                .context(winnow::error::StrContext::Label("IndexOrWriteNode.block")),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "IndexOrWriteNode.operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label("IndexOrWriteNode.value")),
        }]
        .parse_next(input)
    }
}

/**
Represents assigning to an index.

    foo[bar], = 1
    ^^^^^^^^

    begin
    rescue => foo[bar]
              ^^^^^^^^
    end

    for foo[bar] in baz do end
        ^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct IndexTargetNode {
    pub flags: enumflags2::BitFlags<CallNodeFlags>,
    pub receiver: NodeRef,
    pub opening_loc: Location,
    pub arguments: Option<NodeRef>,
    pub closing_loc: Location,
    pub block: Option<NodeRef>,
}
impl IndexTargetNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::IndexTargetNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![IndexTargetNode {
            flags: parse_varuint
                .map(CallNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("IndexTargetNode.flags")),
            receiver: parse_node
                .context(winnow::error::StrContext::Label("IndexTargetNode.receiver")),
            opening_loc: parse_location.context(winnow::error::StrContext::Label(
                "IndexTargetNode.opening_loc"
            )),
            arguments: parse_optional_node.context(winnow::error::StrContext::Label(
                "IndexTargetNode.arguments"
            )),
            closing_loc: parse_location.context(winnow::error::StrContext::Label(
                "IndexTargetNode.closing_loc"
            )),
            block: parse_optional_node
                .context(winnow::error::StrContext::Label("IndexTargetNode.block")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `&&=` operator for assignment to an instance variable.

    @target &&= value
    ^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct InstanceVariableAndWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub name: ConstantRef,
    pub name_loc: Location,
    pub operator_loc: Location,
    pub value: NodeRef,
}
impl InstanceVariableAndWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InstanceVariableAndWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InstanceVariableAndWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "InstanceVariableAndWriteNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "InstanceVariableAndWriteNode.name"
            )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "InstanceVariableAndWriteNode.name_loc"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "InstanceVariableAndWriteNode.operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "InstanceVariableAndWriteNode.value"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents assigning to an instance variable using an operator that isn't `=`.

    @target += value
    ^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct InstanceVariableOperatorWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub name: ConstantRef,
    pub name_loc: Location,
    pub binary_operator_loc: Location,
    pub value: NodeRef,
    pub binary_operator: ConstantRef,
}
impl InstanceVariableOperatorWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InstanceVariableOperatorWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InstanceVariableOperatorWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "InstanceVariableOperatorWriteNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "InstanceVariableOperatorWriteNode.name"
            )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "InstanceVariableOperatorWriteNode.name_loc"
            )),
            binary_operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "InstanceVariableOperatorWriteNode.binary_operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "InstanceVariableOperatorWriteNode.value"
            )),
            binary_operator: parse_constant.context(winnow::error::StrContext::Label(
                "InstanceVariableOperatorWriteNode.binary_operator"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `||=` operator for assignment to an instance variable.

    @target ||= value
    ^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct InstanceVariableOrWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub name: ConstantRef,
    pub name_loc: Location,
    pub operator_loc: Location,
    pub value: NodeRef,
}
impl InstanceVariableOrWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InstanceVariableOrWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InstanceVariableOrWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "InstanceVariableOrWriteNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "InstanceVariableOrWriteNode.name"
            )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "InstanceVariableOrWriteNode.name_loc"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "InstanceVariableOrWriteNode.operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "InstanceVariableOrWriteNode.value"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents referencing an instance variable.

    @foo
    ^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct InstanceVariableReadNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The name of the instance variable, which is a `@` followed by an [identifier](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#identifiers).

        @x     # name `:@x`

        @_test # name `:@_test`
    */
    pub name: ConstantRef,
}
impl InstanceVariableReadNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InstanceVariableReadNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InstanceVariableReadNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "InstanceVariableReadNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "InstanceVariableReadNode.name"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents writing to an instance variable in a context that doesn't have an explicit value.

    @foo, @bar = baz
    ^^^^  ^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct InstanceVariableTargetNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub name: ConstantRef,
}
impl InstanceVariableTargetNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InstanceVariableTargetNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InstanceVariableTargetNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "InstanceVariableTargetNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "InstanceVariableTargetNode.name"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents writing to an instance variable.

    @foo = 1
    ^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct InstanceVariableWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The name of the instance variable, which is a `@` followed by an [identifier](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#identifiers).

        @x = :y       # name `:@x`

        @_foo = "bar" # name `@_foo`
    */
    pub name: ConstantRef,
    /**
    The location of the variable name.

        @_x = 1
        ^^^
    */
    pub name_loc: Location,
    /**
    The value to write to the instance variable. It can be any [non-void expression](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        @foo = :bar
               ^^^^

        @_x = 1234
              ^^^^
    */
    pub value: NodeRef,
    /**
    The location of the `=` operator.

        @x = y
           ^
    */
    pub operator_loc: Location,
}
impl InstanceVariableWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InstanceVariableWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InstanceVariableWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "InstanceVariableWriteNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "InstanceVariableWriteNode.name"
            )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "InstanceVariableWriteNode.name_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "InstanceVariableWriteNode.value"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "InstanceVariableWriteNode.operator_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents an integer number literal.

    1
    ^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct IntegerNode {
    pub flags: enumflags2::BitFlags<IntegerBaseFlags>,
    /**
    The value of the integer literal as a number.    */
    pub value: Integer,
}
impl IntegerNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::IntegerNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![IntegerNode {
            flags: parse_varuint
                .map(IntegerBaseFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("IntegerNode.flags")),
            value: parse_integer.context(winnow::error::StrContext::Label("IntegerNode.value")),
        }]
        .parse_next(input)
    }
}

/**
Represents a regular expression literal that contains interpolation that is being used in the predicate of a conditional to implicitly match against the last line read by an IO object.

    if /foo #{bar} baz/ then end
       ^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct InterpolatedMatchLastLineNode {
    pub flags: enumflags2::BitFlags<RegularExpressionFlags>,
    pub opening_loc: Location,
    pub parts: Vec<NodeRef>,
    pub closing_loc: Location,
}
impl InterpolatedMatchLastLineNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InterpolatedMatchLastLineNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InterpolatedMatchLastLineNode {
            flags: parse_varuint
                .map(RegularExpressionFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "InterpolatedMatchLastLineNode.flags"
                )),
            opening_loc: parse_location.context(winnow::error::StrContext::Label(
                "InterpolatedMatchLastLineNode.opening_loc"
            )),
            parts: length_repeat(parse_varuint, parse_node).context(
                winnow::error::StrContext::Label("InterpolatedMatchLastLineNode.parts")
            ),
            closing_loc: parse_location.context(winnow::error::StrContext::Label(
                "InterpolatedMatchLastLineNode.closing_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents a regular expression literal that contains interpolation.

    /foo #{bar} baz/
    ^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct InterpolatedRegularExpressionNode {
    pub flags: enumflags2::BitFlags<RegularExpressionFlags>,
    pub opening_loc: Location,
    pub parts: Vec<NodeRef>,
    pub closing_loc: Location,
}
impl InterpolatedRegularExpressionNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InterpolatedRegularExpressionNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InterpolatedRegularExpressionNode {
            flags: parse_varuint
                .map(RegularExpressionFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "InterpolatedRegularExpressionNode.flags"
                )),
            opening_loc: parse_location.context(winnow::error::StrContext::Label(
                "InterpolatedRegularExpressionNode.opening_loc"
            )),
            parts: length_repeat(parse_varuint, parse_node).context(
                winnow::error::StrContext::Label("InterpolatedRegularExpressionNode.parts")
            ),
            closing_loc: parse_location.context(winnow::error::StrContext::Label(
                "InterpolatedRegularExpressionNode.closing_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents a string literal that contains interpolation.

    "foo #{bar} baz"
    ^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct InterpolatedStringNode {
    pub flags: enumflags2::BitFlags<InterpolatedStringNodeFlags>,
    pub opening_loc: Option<Location>,
    pub parts: Vec<NodeRef>,
    pub closing_loc: Option<Location>,
}
impl InterpolatedStringNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InterpolatedStringNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InterpolatedStringNode {
            flags: parse_varuint
                .map(InterpolatedStringNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "InterpolatedStringNode.flags"
                )),
            opening_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "InterpolatedStringNode.opening_loc"
            )),
            parts: length_repeat(parse_varuint, parse_node).context(
                winnow::error::StrContext::Label("InterpolatedStringNode.parts")
            ),
            closing_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "InterpolatedStringNode.closing_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents a symbol literal that contains interpolation.

    :"foo #{bar} baz"
    ^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct InterpolatedSymbolNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub opening_loc: Option<Location>,
    pub parts: Vec<NodeRef>,
    pub closing_loc: Option<Location>,
}
impl InterpolatedSymbolNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InterpolatedSymbolNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InterpolatedSymbolNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "InterpolatedSymbolNode.flags"
                )),
            opening_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "InterpolatedSymbolNode.opening_loc"
            )),
            parts: length_repeat(parse_varuint, parse_node).context(
                winnow::error::StrContext::Label("InterpolatedSymbolNode.parts")
            ),
            closing_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "InterpolatedSymbolNode.closing_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents an xstring literal that contains interpolation.

    `foo #{bar} baz`
    ^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct InterpolatedXStringNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub opening_loc: Location,
    pub parts: Vec<NodeRef>,
    pub closing_loc: Location,
}
impl InterpolatedXStringNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InterpolatedXStringNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InterpolatedXStringNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "InterpolatedXStringNode.flags"
                )),
            opening_loc: parse_location.context(winnow::error::StrContext::Label(
                "InterpolatedXStringNode.opening_loc"
            )),
            parts: length_repeat(parse_varuint, parse_node).context(
                winnow::error::StrContext::Label("InterpolatedXStringNode.parts")
            ),
            closing_loc: parse_location.context(winnow::error::StrContext::Label(
                "InterpolatedXStringNode.closing_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents reading from the implicit `it` local variable.

    -> { it }
         ^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ItLocalVariableReadNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
}
impl ItLocalVariableReadNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ItLocalVariableReadNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ItLocalVariableReadNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "ItLocalVariableReadNode.flags"
                )),
        }]
        .parse_next(input)
    }
}

/**
Represents an implicit set of parameters through the use of the `it` keyword within a block or lambda.

    -> { it + it }
    ^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ItParametersNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
}
impl ItParametersNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ItParametersNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ItParametersNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("ItParametersNode.flags")),
        }]
        .parse_next(input)
    }
}

/**
Represents a hash literal without opening and closing braces.

    foo(a: b)
        ^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct KeywordHashNode {
    pub flags: enumflags2::BitFlags<KeywordHashNodeFlags>,
    pub elements: Vec<NodeRef>,
}
impl KeywordHashNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::KeywordHashNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![KeywordHashNode {
            flags: parse_varuint
                .map(KeywordHashNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("KeywordHashNode.flags")),
            elements: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("KeywordHashNode.elements")),
        }]
        .parse_next(input)
    }
}

/**
Represents a keyword rest parameter to a method, block, or lambda definition.

    def a(**b)
          ^^^
    end
*/
#[derive(Debug, Clone, PartialEq)]
pub struct KeywordRestParameterNode {
    pub flags: enumflags2::BitFlags<ParameterFlags>,
    pub name: Option<ConstantRef>,
    pub name_loc: Option<Location>,
    pub operator_loc: Location,
}
impl KeywordRestParameterNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::KeywordRestParameterNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![KeywordRestParameterNode {
            flags: parse_varuint
                .map(ParameterFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "KeywordRestParameterNode.flags"
                )),
            name: parse_optional_constant.context(winnow::error::StrContext::Label(
                "KeywordRestParameterNode.name"
            )),
            name_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "KeywordRestParameterNode.name_loc"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "KeywordRestParameterNode.operator_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents using a lambda literal (not the lambda method call).

    ->(value) { value * 2 }
    ^^^^^^^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct LambdaNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub locals: Vec<ConstantRef>,
    pub operator_loc: Location,
    pub opening_loc: Location,
    pub closing_loc: Location,
    pub parameters: Option<NodeRef>,
    pub body: Option<NodeRef>,
}
impl LambdaNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::LambdaNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![LambdaNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("LambdaNode.flags")),
            locals: length_repeat(parse_varuint, parse_constant)
                .context(winnow::error::StrContext::Label("LambdaNode.locals")),
            operator_loc: parse_location
                .context(winnow::error::StrContext::Label("LambdaNode.operator_loc")),
            opening_loc: parse_location
                .context(winnow::error::StrContext::Label("LambdaNode.opening_loc")),
            closing_loc: parse_location
                .context(winnow::error::StrContext::Label("LambdaNode.closing_loc")),
            parameters: parse_optional_node
                .context(winnow::error::StrContext::Label("LambdaNode.parameters")),
            body: parse_optional_node.context(winnow::error::StrContext::Label("LambdaNode.body")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `&&=` operator for assignment to a local variable.

    target &&= value
    ^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariableAndWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub name_loc: Location,
    pub operator_loc: Location,
    pub value: NodeRef,
    pub name: ConstantRef,
    pub depth: u32,
}
impl LocalVariableAndWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::LocalVariableAndWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![LocalVariableAndWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "LocalVariableAndWriteNode.flags"
                )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "LocalVariableAndWriteNode.name_loc"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "LocalVariableAndWriteNode.operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "LocalVariableAndWriteNode.value"
            )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "LocalVariableAndWriteNode.name"
            )),
            depth: parse_varuint.context(winnow::error::StrContext::Label(
                "LocalVariableAndWriteNode.depth"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents assigning to a local variable using an operator that isn't `=`.

    target += value
    ^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariableOperatorWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub name_loc: Location,
    pub binary_operator_loc: Location,
    pub value: NodeRef,
    pub name: ConstantRef,
    pub binary_operator: ConstantRef,
    pub depth: u32,
}
impl LocalVariableOperatorWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::LocalVariableOperatorWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![LocalVariableOperatorWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "LocalVariableOperatorWriteNode.flags"
                )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "LocalVariableOperatorWriteNode.name_loc"
            )),
            binary_operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "LocalVariableOperatorWriteNode.binary_operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "LocalVariableOperatorWriteNode.value"
            )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "LocalVariableOperatorWriteNode.name"
            )),
            binary_operator: parse_constant.context(winnow::error::StrContext::Label(
                "LocalVariableOperatorWriteNode.binary_operator"
            )),
            depth: parse_varuint.context(winnow::error::StrContext::Label(
                "LocalVariableOperatorWriteNode.depth"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `||=` operator for assignment to a local variable.

    target ||= value
    ^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariableOrWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub name_loc: Location,
    pub operator_loc: Location,
    pub value: NodeRef,
    pub name: ConstantRef,
    pub depth: u32,
}
impl LocalVariableOrWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::LocalVariableOrWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![LocalVariableOrWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "LocalVariableOrWriteNode.flags"
                )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "LocalVariableOrWriteNode.name_loc"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "LocalVariableOrWriteNode.operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "LocalVariableOrWriteNode.value"
            )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "LocalVariableOrWriteNode.name"
            )),
            depth: parse_varuint.context(winnow::error::StrContext::Label(
                "LocalVariableOrWriteNode.depth"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents reading a local variable. Note that this requires that a local variable of the same name has already been written to in the same scope, otherwise it is parsed as a method call.

    foo
    ^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariableReadNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The name of the local variable, which is an [identifier](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#identifiers).

        x      # name `:x`

        _Test  # name `:_Test`

    Note that this can also be an underscore followed by a number for the default block parameters.

        _1     # name `:_1`
    */
    pub name: ConstantRef,
    /**
    The number of visible scopes that should be searched to find the origin of this local variable.

        foo = 1; foo # depth 0

        bar = 2; tap { bar } # depth 1

    The specific rules for calculating the depth may differ from individual Ruby implementations, as they are not specified by the language. For more information, see [the Prism documentation](https://github.com/ruby/prism/blob/main/docs/local_variable_depth.md).
    */
    pub depth: u32,
}
impl LocalVariableReadNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::LocalVariableReadNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![LocalVariableReadNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "LocalVariableReadNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "LocalVariableReadNode.name"
            )),
            depth: parse_varuint.context(winnow::error::StrContext::Label(
                "LocalVariableReadNode.depth"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents writing to a local variable in a context that doesn't have an explicit value.

    foo, bar = baz
    ^^^  ^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariableTargetNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub name: ConstantRef,
    pub depth: u32,
}
impl LocalVariableTargetNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::LocalVariableTargetNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![LocalVariableTargetNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "LocalVariableTargetNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "LocalVariableTargetNode.name"
            )),
            depth: parse_varuint.context(winnow::error::StrContext::Label(
                "LocalVariableTargetNode.depth"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents writing to a local variable.

    foo = 1
    ^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariableWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The name of the local variable, which is an [identifier](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#identifiers).

        foo = :bar # name `:foo`

        abc = 123  # name `:abc`
    */
    pub name: ConstantRef,
    /**
    The number of semantic scopes we have to traverse to find the declaration of this variable.

        foo = 1         # depth 0

        tap { foo = 1 } # depth 1

    The specific rules for calculating the depth may differ from individual Ruby implementations, as they are not specified by the language. For more information, see [the Prism documentation](https://github.com/ruby/prism/blob/main/docs/local_variable_depth.md).
    */
    pub depth: u32,
    /**
    The location of the variable name.

        foo = :bar
        ^^^
    */
    pub name_loc: Location,
    /**
    The value to write to the local variable. It can be any [non-void expression](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        foo = :bar
              ^^^^

        abc = 1234
              ^^^^

    Note that since the name of a local variable is known before the value is parsed, it is valid for a local variable to appear within the value of its own write.

        foo = foo
    */
    pub value: NodeRef,
    /**
    The location of the `=` operator.

        x = :y
          ^
    */
    pub operator_loc: Location,
}
impl LocalVariableWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::LocalVariableWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![LocalVariableWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "LocalVariableWriteNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "LocalVariableWriteNode.name"
            )),
            depth: parse_varuint.context(winnow::error::StrContext::Label(
                "LocalVariableWriteNode.depth"
            )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "LocalVariableWriteNode.name_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "LocalVariableWriteNode.value"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "LocalVariableWriteNode.operator_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents a regular expression literal used in the predicate of a conditional to implicitly match against the last line read by an IO object.

    if /foo/i then end
       ^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct MatchLastLineNode {
    pub flags: enumflags2::BitFlags<RegularExpressionFlags>,
    pub opening_loc: Location,
    pub content_loc: Location,
    pub closing_loc: Location,
    pub unescaped: StringField,
}
impl MatchLastLineNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::MatchLastLineNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![MatchLastLineNode {
            flags: parse_varuint
                .map(RegularExpressionFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("MatchLastLineNode.flags")),
            opening_loc: parse_location.context(winnow::error::StrContext::Label(
                "MatchLastLineNode.opening_loc"
            )),
            content_loc: parse_location.context(winnow::error::StrContext::Label(
                "MatchLastLineNode.content_loc"
            )),
            closing_loc: parse_location.context(winnow::error::StrContext::Label(
                "MatchLastLineNode.closing_loc"
            )),
            unescaped: parse_string_field.context(winnow::error::StrContext::Label(
                "MatchLastLineNode.unescaped"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the modifier `in` operator.

    foo in bar
    ^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct MatchPredicateNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub value: NodeRef,
    pub pattern: NodeRef,
    pub operator_loc: Location,
}
impl MatchPredicateNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::MatchPredicateNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![MatchPredicateNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("MatchPredicateNode.flags")),
            value: parse_node.context(winnow::error::StrContext::Label("MatchPredicateNode.value")),
            pattern: parse_node.context(winnow::error::StrContext::Label(
                "MatchPredicateNode.pattern"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "MatchPredicateNode.operator_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `=>` operator.

    foo => bar
    ^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct MatchRequiredNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub value: NodeRef,
    pub pattern: NodeRef,
    pub operator_loc: Location,
}
impl MatchRequiredNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::MatchRequiredNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![MatchRequiredNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("MatchRequiredNode.flags")),
            value: parse_node.context(winnow::error::StrContext::Label("MatchRequiredNode.value")),
            pattern: parse_node.context(winnow::error::StrContext::Label(
                "MatchRequiredNode.pattern"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "MatchRequiredNode.operator_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents writing local variables using a regular expression match with named capture groups.

    /(?<foo>bar)/ =~ baz
    ^^^^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct MatchWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub call: NodeRef,
    pub targets: Vec<NodeRef>,
}
impl MatchWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::MatchWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![MatchWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("MatchWriteNode.flags")),
            call: parse_node.context(winnow::error::StrContext::Label("MatchWriteNode.call")),
            targets: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("MatchWriteNode.targets")),
        }]
        .parse_next(input)
    }
}

/**
Represents a node that is missing from the source and results in a syntax error.
*/
#[derive(Debug, Clone, PartialEq)]
pub struct MissingNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
}
impl MissingNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::MissingNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![MissingNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("MissingNode.flags")),
        }]
        .parse_next(input)
    }
}

/**
Represents a module declaration involving the `module` keyword.

    module Foo end
    ^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub locals: Vec<ConstantRef>,
    pub module_keyword_loc: Location,
    pub constant_path: NodeRef,
    pub body: Option<NodeRef>,
    pub end_keyword_loc: Location,
    pub name: ConstantRef,
}
impl ModuleNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ModuleNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ModuleNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("ModuleNode.flags")),
            locals: length_repeat(parse_varuint, parse_constant)
                .context(winnow::error::StrContext::Label("ModuleNode.locals")),
            module_keyword_loc: parse_location.context(winnow::error::StrContext::Label(
                "ModuleNode.module_keyword_loc"
            )),
            constant_path: parse_node
                .context(winnow::error::StrContext::Label("ModuleNode.constant_path")),
            body: parse_optional_node.context(winnow::error::StrContext::Label("ModuleNode.body")),
            end_keyword_loc: parse_location.context(winnow::error::StrContext::Label(
                "ModuleNode.end_keyword_loc"
            )),
            name: parse_constant.context(winnow::error::StrContext::Label("ModuleNode.name")),
        }]
        .parse_next(input)
    }
}

/**
Represents a multi-target expression.

    a, (b, c) = 1, 2, 3
       ^^^^^^

This can be a part of `MultiWriteNode` as above, or the target of a `for` loop

    for a, b in [[1, 2], [3, 4]]
        ^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct MultiTargetNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    Represents the targets expressions before a splat node.

        a, (b, c, *) = 1, 2, 3, 4, 5
            ^^^^

    The splat node can be absent, in that case all target expressions are in the left field.

        a, (b, c) = 1, 2, 3, 4, 5
            ^^^^
    */
    pub lefts: Vec<NodeRef>,
    /**
    Represents a splat node in the target expression.

        a, (b, *c) = 1, 2, 3, 4
               ^^

    The variable can be empty, this results in a `SplatNode` with a `nil` expression field.

        a, (b, *) = 1, 2, 3, 4
               ^

    If the `*` is omitted, this field will contain an `ImplicitRestNode`

        a, (b,) = 1, 2, 3, 4
             ^
    */
    pub rest: Option<NodeRef>,
    /**
    Represents the targets expressions after a splat node.

        a, (*, b, c) = 1, 2, 3, 4, 5
               ^^^^
    */
    pub rights: Vec<NodeRef>,
    /**
    The location of the opening parenthesis.

        a, (b, c) = 1, 2, 3
           ^
    */
    pub lparen_loc: Option<Location>,
    /**
    The location of the closing parenthesis.

        a, (b, c) = 1, 2, 3
                ^
    */
    pub rparen_loc: Option<Location>,
}
impl MultiTargetNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::MultiTargetNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![MultiTargetNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("MultiTargetNode.flags")),
            lefts: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("MultiTargetNode.lefts")),
            rest: parse_optional_node
                .context(winnow::error::StrContext::Label("MultiTargetNode.rest")),
            rights: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("MultiTargetNode.rights")),
            lparen_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "MultiTargetNode.lparen_loc"
            )),
            rparen_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "MultiTargetNode.rparen_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents a write to a multi-target expression.

    a, b, c = 1, 2, 3
    ^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct MultiWriteNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    Represents the targets expressions before a splat node.

        a, b, * = 1, 2, 3, 4, 5
        ^^^^

    The splat node can be absent, in that case all target expressions are in the left field.

        a, b, c = 1, 2, 3, 4, 5
        ^^^^^^^
    */
    pub lefts: Vec<NodeRef>,
    /**
    Represents a splat node in the target expression.

        a, b, *c = 1, 2, 3, 4
              ^^

    The variable can be empty, this results in a `SplatNode` with a `nil` expression field.

        a, b, * = 1, 2, 3, 4
              ^

    If the `*` is omitted, this field will contain an `ImplicitRestNode`

        a, b, = 1, 2, 3, 4
            ^
    */
    pub rest: Option<NodeRef>,
    /**
    Represents the targets expressions after a splat node.

        a, *, b, c = 1, 2, 3, 4, 5
              ^^^^
    */
    pub rights: Vec<NodeRef>,
    /**
    The location of the opening parenthesis.

        (a, b, c) = 1, 2, 3
        ^
    */
    pub lparen_loc: Option<Location>,
    /**
    The location of the closing parenthesis.

        (a, b, c) = 1, 2, 3
                ^
    */
    pub rparen_loc: Option<Location>,
    /**
    The location of the operator.

        a, b, c = 1, 2, 3
                ^
    */
    pub operator_loc: Location,
    /**
    The value to write to the targets. It can be any [non-void expression](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        a, b, c = 1, 2, 3
                  ^^^^^^^
    */
    pub value: NodeRef,
}
impl MultiWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::MultiWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![MultiWriteNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("MultiWriteNode.flags")),
            lefts: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("MultiWriteNode.lefts")),
            rest: parse_optional_node
                .context(winnow::error::StrContext::Label("MultiWriteNode.rest")),
            rights: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("MultiWriteNode.rights")),
            lparen_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "MultiWriteNode.lparen_loc"
            )),
            rparen_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "MultiWriteNode.rparen_loc"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "MultiWriteNode.operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label("MultiWriteNode.value")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `next` keyword.

    next 1
    ^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct NextNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub arguments: Option<NodeRef>,
    pub keyword_loc: Location,
}
impl NextNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::NextNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![NextNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("NextNode.flags")),
            arguments: parse_optional_node
                .context(winnow::error::StrContext::Label("NextNode.arguments")),
            keyword_loc: parse_location
                .context(winnow::error::StrContext::Label("NextNode.keyword_loc")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `nil` keyword.

    nil
    ^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct NilNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
}
impl NilNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::NilNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![NilNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("NilNode.flags")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of `**nil` inside method arguments.

    def a(**nil)
          ^^^^^
    end
*/
#[derive(Debug, Clone, PartialEq)]
pub struct NoKeywordsParameterNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub operator_loc: Location,
    pub keyword_loc: Location,
}
impl NoKeywordsParameterNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::NoKeywordsParameterNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![NoKeywordsParameterNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "NoKeywordsParameterNode.flags"
                )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "NoKeywordsParameterNode.operator_loc"
            )),
            keyword_loc: parse_location.context(winnow::error::StrContext::Label(
                "NoKeywordsParameterNode.keyword_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents an implicit set of parameters through the use of numbered parameters within a block or lambda.

    -> { _1 + _2 }
    ^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct NumberedParametersNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub maximum: u8,
}
impl NumberedParametersNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::NumberedParametersNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![NumberedParametersNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "NumberedParametersNode.flags"
                )),
            maximum: winnow::binary::u8.context(winnow::error::StrContext::Label(
                "NumberedParametersNode.maximum"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents reading a numbered reference to a capture in the previous match.

    $1
    ^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct NumberedReferenceReadNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The (1-indexed, from the left) number of the capture group. Numbered references that are too large result in this value being `0`.

        $1          # number `1`

        $5432       # number `5432`

        $4294967296 # number `0`
    */
    pub number: u32,
}
impl NumberedReferenceReadNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::NumberedReferenceReadNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![NumberedReferenceReadNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "NumberedReferenceReadNode.flags"
                )),
            number: parse_varuint.context(winnow::error::StrContext::Label(
                "NumberedReferenceReadNode.number"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents an optional keyword parameter to a method, block, or lambda definition.

    def a(b: 1)
          ^^^^
    end
*/
#[derive(Debug, Clone, PartialEq)]
pub struct OptionalKeywordParameterNode {
    pub flags: enumflags2::BitFlags<ParameterFlags>,
    pub name: ConstantRef,
    pub name_loc: Location,
    pub value: NodeRef,
}
impl OptionalKeywordParameterNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::OptionalKeywordParameterNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![OptionalKeywordParameterNode {
            flags: parse_varuint
                .map(ParameterFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "OptionalKeywordParameterNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "OptionalKeywordParameterNode.name"
            )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "OptionalKeywordParameterNode.name_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "OptionalKeywordParameterNode.value"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents an optional parameter to a method, block, or lambda definition.

    def a(b = 1)
          ^^^^^
    end
*/
#[derive(Debug, Clone, PartialEq)]
pub struct OptionalParameterNode {
    pub flags: enumflags2::BitFlags<ParameterFlags>,
    pub name: ConstantRef,
    pub name_loc: Location,
    pub operator_loc: Location,
    pub value: NodeRef,
}
impl OptionalParameterNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::OptionalParameterNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![OptionalParameterNode {
            flags: parse_varuint
                .map(ParameterFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "OptionalParameterNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "OptionalParameterNode.name"
            )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "OptionalParameterNode.name_loc"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "OptionalParameterNode.operator_loc"
            )),
            value: parse_node.context(winnow::error::StrContext::Label(
                "OptionalParameterNode.value"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `||` operator or the `or` keyword.

    left or right
    ^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct OrNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    Represents the left side of the expression. It can be any [non-void expression](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        left or right
        ^^^^

        1 || 2
        ^
    */
    pub left: NodeRef,
    /**
    Represents the right side of the expression.

        left || right
                ^^^^^

        1 or 2
             ^
    */
    pub right: NodeRef,
    /**
    The location of the `or` keyword or the `||` operator.

        left or right
             ^^
    */
    pub operator_loc: Location,
}
impl OrNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::OrNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![OrNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("OrNode.flags")),
            left: parse_node.context(winnow::error::StrContext::Label("OrNode.left")),
            right: parse_node.context(winnow::error::StrContext::Label("OrNode.right")),
            operator_loc: parse_location
                .context(winnow::error::StrContext::Label("OrNode.operator_loc")),
        }]
        .parse_next(input)
    }
}

/**
Represents the list of parameters on a method, block, or lambda definition.

    def a(b, c, d)
          ^^^^^^^
    end
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ParametersNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub requireds: Vec<NodeRef>,
    pub optionals: Vec<NodeRef>,
    pub rest: Option<NodeRef>,
    pub posts: Vec<NodeRef>,
    pub keywords: Vec<NodeRef>,
    pub keyword_rest: Option<NodeRef>,
    pub block: Option<NodeRef>,
}
impl ParametersNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ParametersNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ParametersNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("ParametersNode.flags")),
            requireds: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("ParametersNode.requireds")),
            optionals: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("ParametersNode.optionals")),
            rest: parse_optional_node
                .context(winnow::error::StrContext::Label("ParametersNode.rest")),
            posts: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("ParametersNode.posts")),
            keywords: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("ParametersNode.keywords")),
            keyword_rest: parse_optional_node.context(winnow::error::StrContext::Label(
                "ParametersNode.keyword_rest"
            )),
            block: parse_optional_node
                .context(winnow::error::StrContext::Label("ParametersNode.block")),
        }]
        .parse_next(input)
    }
}

/**
Represents a parenthesized expression

    (10 + 34)
    ^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ParenthesesNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub body: Option<NodeRef>,
    pub opening_loc: Location,
    pub closing_loc: Location,
}
impl ParenthesesNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ParenthesesNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ParenthesesNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("ParenthesesNode.flags")),
            body: parse_optional_node
                .context(winnow::error::StrContext::Label("ParenthesesNode.body")),
            opening_loc: parse_location.context(winnow::error::StrContext::Label(
                "ParenthesesNode.opening_loc"
            )),
            closing_loc: parse_location.context(winnow::error::StrContext::Label(
                "ParenthesesNode.closing_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `^` operator for pinning an expression in a pattern matching expression.

    foo in ^(bar)
           ^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct PinnedExpressionNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub expression: NodeRef,
    pub operator_loc: Location,
    pub lparen_loc: Location,
    pub rparen_loc: Location,
}
impl PinnedExpressionNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::PinnedExpressionNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![PinnedExpressionNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "PinnedExpressionNode.flags"
                )),
            expression: parse_node.context(winnow::error::StrContext::Label(
                "PinnedExpressionNode.expression"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "PinnedExpressionNode.operator_loc"
            )),
            lparen_loc: parse_location.context(winnow::error::StrContext::Label(
                "PinnedExpressionNode.lparen_loc"
            )),
            rparen_loc: parse_location.context(winnow::error::StrContext::Label(
                "PinnedExpressionNode.rparen_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `^` operator for pinning a variable in a pattern matching expression.

    foo in ^bar
           ^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct PinnedVariableNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub variable: NodeRef,
    pub operator_loc: Location,
}
impl PinnedVariableNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::PinnedVariableNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![PinnedVariableNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("PinnedVariableNode.flags")),
            variable: parse_node.context(winnow::error::StrContext::Label(
                "PinnedVariableNode.variable"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "PinnedVariableNode.operator_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `END` keyword.

    END { foo }
    ^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct PostExecutionNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub statements: Option<NodeRef>,
    pub keyword_loc: Location,
    pub opening_loc: Location,
    pub closing_loc: Location,
}
impl PostExecutionNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::PostExecutionNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![PostExecutionNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("PostExecutionNode.flags")),
            statements: parse_optional_node.context(winnow::error::StrContext::Label(
                "PostExecutionNode.statements"
            )),
            keyword_loc: parse_location.context(winnow::error::StrContext::Label(
                "PostExecutionNode.keyword_loc"
            )),
            opening_loc: parse_location.context(winnow::error::StrContext::Label(
                "PostExecutionNode.opening_loc"
            )),
            closing_loc: parse_location.context(winnow::error::StrContext::Label(
                "PostExecutionNode.closing_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `BEGIN` keyword.

    BEGIN { foo }
    ^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct PreExecutionNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub statements: Option<NodeRef>,
    pub keyword_loc: Location,
    pub opening_loc: Location,
    pub closing_loc: Location,
}
impl PreExecutionNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::PreExecutionNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![PreExecutionNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("PreExecutionNode.flags")),
            statements: parse_optional_node.context(winnow::error::StrContext::Label(
                "PreExecutionNode.statements"
            )),
            keyword_loc: parse_location.context(winnow::error::StrContext::Label(
                "PreExecutionNode.keyword_loc"
            )),
            opening_loc: parse_location.context(winnow::error::StrContext::Label(
                "PreExecutionNode.opening_loc"
            )),
            closing_loc: parse_location.context(winnow::error::StrContext::Label(
                "PreExecutionNode.closing_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
The top level node of any parse tree.  */
#[derive(Debug, Clone, PartialEq)]
pub struct ProgramNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub locals: Vec<ConstantRef>,
    pub statements: NodeRef,
}
impl ProgramNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ProgramNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ProgramNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("ProgramNode.flags")),
            locals: length_repeat(parse_varuint, parse_constant)
                .context(winnow::error::StrContext::Label("ProgramNode.locals")),
            statements: parse_node
                .context(winnow::error::StrContext::Label("ProgramNode.statements")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `..` or `...` operators.

    1..2
    ^^^^

    c if a =~ /left/ ... b =~ /right/
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct RangeNode {
    pub flags: enumflags2::BitFlags<RangeFlags>,
    /**
    The left-hand side of the range, if present. It can be either `nil` or any [non-void expression](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        1...
        ^

        hello...goodbye
        ^^^^^
    */
    pub left: Option<NodeRef>,
    /**
    The right-hand side of the range, if present. It can be either `nil` or any [non-void expression](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        ..5
          ^

        1...foo
            ^^^
    If neither right-hand or left-hand side was included, this will be a MissingNode.
    */
    pub right: Option<NodeRef>,
    /**
    The location of the `..` or `...` operator.
    */
    pub operator_loc: Location,
}
impl RangeNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::RangeNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![RangeNode {
            flags: parse_varuint
                .map(RangeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("RangeNode.flags")),
            left: parse_optional_node.context(winnow::error::StrContext::Label("RangeNode.left")),
            right: parse_optional_node.context(winnow::error::StrContext::Label("RangeNode.right")),
            operator_loc: parse_location
                .context(winnow::error::StrContext::Label("RangeNode.operator_loc")),
        }]
        .parse_next(input)
    }
}

/**
Represents a rational number literal.

    1.0r
    ^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct RationalNode {
    pub flags: enumflags2::BitFlags<IntegerBaseFlags>,
    /**
    The numerator of the rational number.

        1.5r # numerator 3
    */
    pub numerator: Integer,
    /**
    The denominator of the rational number.

        1.5r # denominator 2
    */
    pub denominator: Integer,
}
impl RationalNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::RationalNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![RationalNode {
            flags: parse_varuint
                .map(IntegerBaseFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("RationalNode.flags")),
            numerator: parse_integer
                .context(winnow::error::StrContext::Label("RationalNode.numerator")),
            denominator: parse_integer
                .context(winnow::error::StrContext::Label("RationalNode.denominator")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `redo` keyword.

    redo
    ^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct RedoNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
}
impl RedoNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::RedoNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![RedoNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("RedoNode.flags")),
        }]
        .parse_next(input)
    }
}

/**
Represents a regular expression literal with no interpolation.

    /foo/i
    ^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct RegularExpressionNode {
    pub flags: enumflags2::BitFlags<RegularExpressionFlags>,
    pub opening_loc: Location,
    pub content_loc: Location,
    pub closing_loc: Location,
    pub unescaped: StringField,
}
impl RegularExpressionNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::RegularExpressionNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![RegularExpressionNode {
            flags: parse_varuint
                .map(RegularExpressionFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "RegularExpressionNode.flags"
                )),
            opening_loc: parse_location.context(winnow::error::StrContext::Label(
                "RegularExpressionNode.opening_loc"
            )),
            content_loc: parse_location.context(winnow::error::StrContext::Label(
                "RegularExpressionNode.content_loc"
            )),
            closing_loc: parse_location.context(winnow::error::StrContext::Label(
                "RegularExpressionNode.closing_loc"
            )),
            unescaped: parse_string_field.context(winnow::error::StrContext::Label(
                "RegularExpressionNode.unescaped"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents a required keyword parameter to a method, block, or lambda definition.

    def a(b: )
          ^^
    end
*/
#[derive(Debug, Clone, PartialEq)]
pub struct RequiredKeywordParameterNode {
    pub flags: enumflags2::BitFlags<ParameterFlags>,
    pub name: ConstantRef,
    pub name_loc: Location,
}
impl RequiredKeywordParameterNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::RequiredKeywordParameterNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![RequiredKeywordParameterNode {
            flags: parse_varuint
                .map(ParameterFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "RequiredKeywordParameterNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "RequiredKeywordParameterNode.name"
            )),
            name_loc: parse_location.context(winnow::error::StrContext::Label(
                "RequiredKeywordParameterNode.name_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents a required parameter to a method, block, or lambda definition.

    def a(b)
          ^
    end
*/
#[derive(Debug, Clone, PartialEq)]
pub struct RequiredParameterNode {
    pub flags: enumflags2::BitFlags<ParameterFlags>,
    pub name: ConstantRef,
}
impl RequiredParameterNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::RequiredParameterNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![RequiredParameterNode {
            flags: parse_varuint
                .map(ParameterFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "RequiredParameterNode.flags"
                )),
            name: parse_constant.context(winnow::error::StrContext::Label(
                "RequiredParameterNode.name"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents an expression modified with a rescue.

    foo rescue nil
    ^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct RescueModifierNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub expression: NodeRef,
    pub keyword_loc: Location,
    pub rescue_expression: NodeRef,
}
impl RescueModifierNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::RescueModifierNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![RescueModifierNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("RescueModifierNode.flags")),
            expression: parse_node.context(winnow::error::StrContext::Label(
                "RescueModifierNode.expression"
            )),
            keyword_loc: parse_location.context(winnow::error::StrContext::Label(
                "RescueModifierNode.keyword_loc"
            )),
            rescue_expression: parse_node.context(winnow::error::StrContext::Label(
                "RescueModifierNode.rescue_expression"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents a rescue statement.

    begin
    rescue Foo, *splat, Bar => ex
      foo
    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    end

`Foo, *splat, Bar` are in the `exceptions` field. `ex` is in the `reference` field.
*/
#[derive(Debug, Clone, PartialEq)]
pub struct RescueNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub keyword_loc: Location,
    pub exceptions: Vec<NodeRef>,
    pub operator_loc: Option<Location>,
    pub reference: Option<NodeRef>,
    pub then_keyword_loc: Option<Location>,
    pub statements: Option<NodeRef>,
    pub subsequent: Option<NodeRef>,
}
impl RescueNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::RescueNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![RescueNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("RescueNode.flags")),
            keyword_loc: parse_location
                .context(winnow::error::StrContext::Label("RescueNode.keyword_loc")),
            exceptions: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("RescueNode.exceptions")),
            operator_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("RescueNode.operator_loc")),
            reference: parse_optional_node
                .context(winnow::error::StrContext::Label("RescueNode.reference")),
            then_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "RescueNode.then_keyword_loc"
            )),
            statements: parse_optional_node
                .context(winnow::error::StrContext::Label("RescueNode.statements")),
            subsequent: parse_optional_node
                .context(winnow::error::StrContext::Label("RescueNode.subsequent")),
        }]
        .parse_next(input)
    }
}

/**
Represents a rest parameter to a method, block, or lambda definition.

    def a(*b)
          ^^
    end
*/
#[derive(Debug, Clone, PartialEq)]
pub struct RestParameterNode {
    pub flags: enumflags2::BitFlags<ParameterFlags>,
    pub name: Option<ConstantRef>,
    pub name_loc: Option<Location>,
    pub operator_loc: Location,
}
impl RestParameterNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::RestParameterNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![RestParameterNode {
            flags: parse_varuint
                .map(ParameterFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("RestParameterNode.flags")),
            name: parse_optional_constant
                .context(winnow::error::StrContext::Label("RestParameterNode.name")),
            name_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "RestParameterNode.name_loc"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "RestParameterNode.operator_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `retry` keyword.

    retry
    ^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct RetryNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
}
impl RetryNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::RetryNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![RetryNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("RetryNode.flags")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `return` keyword.

    return 1
    ^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub keyword_loc: Location,
    pub arguments: Option<NodeRef>,
}
impl ReturnNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ReturnNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ReturnNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("ReturnNode.flags")),
            keyword_loc: parse_location
                .context(winnow::error::StrContext::Label("ReturnNode.keyword_loc")),
            arguments: parse_optional_node
                .context(winnow::error::StrContext::Label("ReturnNode.arguments")),
        }]
        .parse_next(input)
    }
}

/**
Represents the `self` keyword.

    self
    ^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct SelfNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
}
impl SelfNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::SelfNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![SelfNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("SelfNode.flags")),
        }]
        .parse_next(input)
    }
}

/**
This node wraps a constant write to indicate that when the value is written, it should have its shareability state modified.

    # shareable_constant_value: literal
    C = { a: 1 }
    ^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ShareableConstantNode {
    pub flags: enumflags2::BitFlags<ShareableConstantNodeFlags>,
    /**
    The constant write that should be modified with the shareability state.    */
    pub write: NodeRef,
}
impl ShareableConstantNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ShareableConstantNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ShareableConstantNode {
            flags: parse_varuint
                .map(ShareableConstantNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label(
                    "ShareableConstantNode.flags"
                )),
            write: parse_node.context(winnow::error::StrContext::Label(
                "ShareableConstantNode.write"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents a singleton class declaration involving the `class` keyword.

    class << self end
    ^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct SingletonClassNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub locals: Vec<ConstantRef>,
    pub class_keyword_loc: Location,
    pub operator_loc: Location,
    pub expression: NodeRef,
    pub body: Option<NodeRef>,
    pub end_keyword_loc: Location,
}
impl SingletonClassNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::SingletonClassNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![SingletonClassNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("SingletonClassNode.flags")),
            locals: length_repeat(parse_varuint, parse_constant).context(
                winnow::error::StrContext::Label("SingletonClassNode.locals")
            ),
            class_keyword_loc: parse_location.context(winnow::error::StrContext::Label(
                "SingletonClassNode.class_keyword_loc"
            )),
            operator_loc: parse_location.context(winnow::error::StrContext::Label(
                "SingletonClassNode.operator_loc"
            )),
            expression: parse_node.context(winnow::error::StrContext::Label(
                "SingletonClassNode.expression"
            )),
            body: parse_optional_node
                .context(winnow::error::StrContext::Label("SingletonClassNode.body")),
            end_keyword_loc: parse_location.context(winnow::error::StrContext::Label(
                "SingletonClassNode.end_keyword_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `__ENCODING__` keyword.

    __ENCODING__
    ^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct SourceEncodingNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
}
impl SourceEncodingNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::SourceEncodingNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![SourceEncodingNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("SourceEncodingNode.flags")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `__FILE__` keyword.

    __FILE__
    ^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct SourceFileNode {
    pub flags: enumflags2::BitFlags<StringFlags>,
    /**
    Represents the file path being parsed. This corresponds directly to the `filepath` option given to the various `Prism::parse*` APIs.    */
    pub filepath: StringField,
}
impl SourceFileNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::SourceFileNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![SourceFileNode {
            flags: parse_varuint
                .map(StringFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("SourceFileNode.flags")),
            filepath: parse_string_field
                .context(winnow::error::StrContext::Label("SourceFileNode.filepath")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `__LINE__` keyword.

    __LINE__
    ^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLineNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
}
impl SourceLineNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::SourceLineNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![SourceLineNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("SourceLineNode.flags")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the splat operator.

    [*a]
     ^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct SplatNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub operator_loc: Location,
    pub expression: Option<NodeRef>,
}
impl SplatNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::SplatNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![SplatNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("SplatNode.flags")),
            operator_loc: parse_location
                .context(winnow::error::StrContext::Label("SplatNode.operator_loc")),
            expression: parse_optional_node
                .context(winnow::error::StrContext::Label("SplatNode.expression")),
        }]
        .parse_next(input)
    }
}

/**
Represents a set of statements contained within some scope.

    foo; bar; baz
    ^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct StatementsNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub body: Vec<NodeRef>,
}
impl StatementsNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::StatementsNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![StatementsNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("StatementsNode.flags")),
            body: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("StatementsNode.body")),
        }]
        .parse_next(input)
    }
}

/**
Represents a string literal, a string contained within a `%w` list, or plain string content within an interpolated string.

    "foo"
    ^^^^^

    %w[foo]
       ^^^

    "foo #{bar} baz"
     ^^^^      ^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct StringNode {
    pub flags: enumflags2::BitFlags<StringFlags>,
    pub opening_loc: Option<Location>,
    pub content_loc: Location,
    pub closing_loc: Option<Location>,
    pub unescaped: StringField,
}
impl StringNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::StringNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![StringNode {
            flags: parse_varuint
                .map(StringFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("StringNode.flags")),
            opening_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("StringNode.opening_loc")),
            content_loc: parse_location
                .context(winnow::error::StrContext::Label("StringNode.content_loc")),
            closing_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("StringNode.closing_loc")),
            unescaped: parse_string_field
                .context(winnow::error::StrContext::Label("StringNode.unescaped")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `super` keyword with parentheses or arguments.

    super()
    ^^^^^^^

    super foo, bar
    ^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct SuperNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub keyword_loc: Location,
    pub lparen_loc: Option<Location>,
    pub arguments: Option<NodeRef>,
    pub rparen_loc: Option<Location>,
    pub block: Option<NodeRef>,
}
impl SuperNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::SuperNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![SuperNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("SuperNode.flags")),
            keyword_loc: parse_location
                .context(winnow::error::StrContext::Label("SuperNode.keyword_loc")),
            lparen_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("SuperNode.lparen_loc")),
            arguments: parse_optional_node
                .context(winnow::error::StrContext::Label("SuperNode.arguments")),
            rparen_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("SuperNode.rparen_loc")),
            block: parse_optional_node.context(winnow::error::StrContext::Label("SuperNode.block")),
        }]
        .parse_next(input)
    }
}

/**
Represents a symbol literal or a symbol contained within a `%i` list.

    :foo
    ^^^^

    %i[foo]
       ^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct SymbolNode {
    pub flags: enumflags2::BitFlags<SymbolFlags>,
    pub opening_loc: Option<Location>,
    pub value_loc: Option<Location>,
    pub closing_loc: Option<Location>,
    pub unescaped: StringField,
}
impl SymbolNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::SymbolNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![SymbolNode {
            flags: parse_varuint
                .map(SymbolFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("SymbolNode.flags")),
            opening_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("SymbolNode.opening_loc")),
            value_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("SymbolNode.value_loc")),
            closing_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("SymbolNode.closing_loc")),
            unescaped: parse_string_field
                .context(winnow::error::StrContext::Label("SymbolNode.unescaped")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the literal `true` keyword.

    true
    ^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct TrueNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
}
impl TrueNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::TrueNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![TrueNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("TrueNode.flags")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `undef` keyword.

    undef :foo, :bar, :baz
    ^^^^^^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct UndefNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub names: Vec<NodeRef>,
    pub keyword_loc: Location,
}
impl UndefNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::UndefNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![UndefNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("UndefNode.flags")),
            names: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("UndefNode.names")),
            keyword_loc: parse_location
                .context(winnow::error::StrContext::Label("UndefNode.keyword_loc")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `unless` keyword, either in the block form or the modifier form.

    bar unless foo
    ^^^^^^^^^^^^^^

    unless foo then bar end
    ^^^^^^^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct UnlessNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    /**
    The location of the `unless` keyword.

        unless cond then bar end
        ^^^^^^

        bar unless cond
            ^^^^^^
    */
    pub keyword_loc: Location,
    /**
    The condition to be evaluated for the unless expression. It can be any [non-void expression](https://github.com/ruby/prism/blob/main/docs/parsing_rules.md#non-void-expression).

        unless cond then bar end
               ^^^^

        bar unless cond
                   ^^^^
    */
    pub predicate: NodeRef,
    /**
    The location of the `then` keyword, if present.

        unless cond then bar end
                    ^^^^
    */
    pub then_keyword_loc: Option<Location>,
    /**
    The body of statements that will executed if the unless condition is
    falsey. Will be `nil` if no body is provided.

        unless cond then bar end
                         ^^^
    */
    pub statements: Option<NodeRef>,
    /**
    The else clause of the unless expression, if present.

        unless cond then bar else baz end
                             ^^^^^^^^
    */
    pub else_clause: Option<NodeRef>,
    /**
    The location of the `end` keyword, if present.

        unless cond then bar end
                             ^^^
    */
    pub end_keyword_loc: Option<Location>,
}
impl UnlessNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::UnlessNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![UnlessNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("UnlessNode.flags")),
            keyword_loc: parse_location
                .context(winnow::error::StrContext::Label("UnlessNode.keyword_loc")),
            predicate: parse_node.context(winnow::error::StrContext::Label("UnlessNode.predicate")),
            then_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "UnlessNode.then_keyword_loc"
            )),
            statements: parse_optional_node
                .context(winnow::error::StrContext::Label("UnlessNode.statements")),
            else_clause: parse_optional_node
                .context(winnow::error::StrContext::Label("UnlessNode.else_clause")),
            end_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "UnlessNode.end_keyword_loc"
            )),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `until` keyword, either in the block form or the modifier form.

    bar until foo
    ^^^^^^^^^^^^^

    until foo do bar end
    ^^^^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct UntilNode {
    pub flags: enumflags2::BitFlags<LoopFlags>,
    pub keyword_loc: Location,
    pub do_keyword_loc: Option<Location>,
    pub closing_loc: Option<Location>,
    pub predicate: NodeRef,
    pub statements: Option<NodeRef>,
}
impl UntilNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::UntilNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![UntilNode {
            flags: parse_varuint
                .map(LoopFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("UntilNode.flags")),
            keyword_loc: parse_location
                .context(winnow::error::StrContext::Label("UntilNode.keyword_loc")),
            do_keyword_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("UntilNode.do_keyword_loc")),
            closing_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("UntilNode.closing_loc")),
            predicate: parse_node.context(winnow::error::StrContext::Label("UntilNode.predicate")),
            statements: parse_optional_node
                .context(winnow::error::StrContext::Label("UntilNode.statements")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `when` keyword within a case statement.

    case true
    when true
    ^^^^^^^^^
    end
*/
#[derive(Debug, Clone, PartialEq)]
pub struct WhenNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub keyword_loc: Location,
    pub conditions: Vec<NodeRef>,
    pub then_keyword_loc: Option<Location>,
    pub statements: Option<NodeRef>,
}
impl WhenNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::WhenNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![WhenNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("WhenNode.flags")),
            keyword_loc: parse_location
                .context(winnow::error::StrContext::Label("WhenNode.keyword_loc")),
            conditions: length_repeat(parse_varuint, parse_node)
                .context(winnow::error::StrContext::Label("WhenNode.conditions")),
            then_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label(
                "WhenNode.then_keyword_loc"
            )),
            statements: parse_optional_node
                .context(winnow::error::StrContext::Label("WhenNode.statements")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `while` keyword, either in the block form or the modifier form.

    bar while foo
    ^^^^^^^^^^^^^

    while foo do bar end
    ^^^^^^^^^^^^^^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct WhileNode {
    pub flags: enumflags2::BitFlags<LoopFlags>,
    pub keyword_loc: Location,
    pub do_keyword_loc: Option<Location>,
    pub closing_loc: Option<Location>,
    pub predicate: NodeRef,
    pub statements: Option<NodeRef>,
}
impl WhileNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::WhileNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![WhileNode {
            flags: parse_varuint
                .map(LoopFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("WhileNode.flags")),
            keyword_loc: parse_location
                .context(winnow::error::StrContext::Label("WhileNode.keyword_loc")),
            do_keyword_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("WhileNode.do_keyword_loc")),
            closing_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("WhileNode.closing_loc")),
            predicate: parse_node.context(winnow::error::StrContext::Label("WhileNode.predicate")),
            statements: parse_optional_node
                .context(winnow::error::StrContext::Label("WhileNode.statements")),
        }]
        .parse_next(input)
    }
}

/**
Represents an xstring literal with no interpolation.

    `foo`
    ^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct XStringNode {
    pub flags: enumflags2::BitFlags<EncodingFlags>,
    pub opening_loc: Location,
    pub content_loc: Location,
    pub closing_loc: Location,
    pub unescaped: StringField,
}
impl XStringNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::XStringNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![XStringNode {
            flags: parse_varuint
                .map(EncodingFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("XStringNode.flags")),
            opening_loc: parse_location
                .context(winnow::error::StrContext::Label("XStringNode.opening_loc")),
            content_loc: parse_location
                .context(winnow::error::StrContext::Label("XStringNode.content_loc")),
            closing_loc: parse_location
                .context(winnow::error::StrContext::Label("XStringNode.closing_loc")),
            unescaped: parse_string_field
                .context(winnow::error::StrContext::Label("XStringNode.unescaped")),
        }]
        .parse_next(input)
    }
}

/**
Represents the use of the `yield` keyword.

    yield 1
    ^^^^^^^
*/
#[derive(Debug, Clone, PartialEq)]
pub struct YieldNode {
    pub flags: enumflags2::BitFlags<DefaultNodeFlags>,
    pub keyword_loc: Location,
    pub lparen_loc: Option<Location>,
    pub arguments: Option<NodeRef>,
    pub rparen_loc: Option<Location>,
}
impl YieldNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::YieldNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![YieldNode {
            flags: parse_varuint
                .map(DefaultNodeFlags::from_bits_truncate)
                .context(winnow::error::StrContext::Label("YieldNode.flags")),
            keyword_loc: parse_location
                .context(winnow::error::StrContext::Label("YieldNode.keyword_loc")),
            lparen_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("YieldNode.lparen_loc")),
            arguments: parse_optional_node
                .context(winnow::error::StrContext::Label("YieldNode.arguments")),
            rparen_loc: parse_optional_location
                .context(winnow::error::StrContext::Label("YieldNode.rparen_loc")),
        }]
        .parse_next(input)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    AliasGlobalVariableNode(AliasGlobalVariableNode),
    AliasMethodNode(AliasMethodNode),
    AlternationPatternNode(AlternationPatternNode),
    AndNode(AndNode),
    ArgumentsNode(ArgumentsNode),
    ArrayNode(ArrayNode),
    ArrayPatternNode(ArrayPatternNode),
    AssocNode(AssocNode),
    AssocSplatNode(AssocSplatNode),
    BackReferenceReadNode(BackReferenceReadNode),
    BeginNode(BeginNode),
    BlockArgumentNode(BlockArgumentNode),
    BlockLocalVariableNode(BlockLocalVariableNode),
    BlockNode(BlockNode),
    BlockParameterNode(BlockParameterNode),
    BlockParametersNode(BlockParametersNode),
    BreakNode(BreakNode),
    CallAndWriteNode(CallAndWriteNode),
    CallNode(CallNode),
    CallOperatorWriteNode(CallOperatorWriteNode),
    CallOrWriteNode(CallOrWriteNode),
    CallTargetNode(CallTargetNode),
    CapturePatternNode(CapturePatternNode),
    CaseMatchNode(CaseMatchNode),
    CaseNode(CaseNode),
    ClassNode(ClassNode),
    ClassVariableAndWriteNode(ClassVariableAndWriteNode),
    ClassVariableOperatorWriteNode(ClassVariableOperatorWriteNode),
    ClassVariableOrWriteNode(ClassVariableOrWriteNode),
    ClassVariableReadNode(ClassVariableReadNode),
    ClassVariableTargetNode(ClassVariableTargetNode),
    ClassVariableWriteNode(ClassVariableWriteNode),
    ConstantAndWriteNode(ConstantAndWriteNode),
    ConstantOperatorWriteNode(ConstantOperatorWriteNode),
    ConstantOrWriteNode(ConstantOrWriteNode),
    ConstantPathAndWriteNode(ConstantPathAndWriteNode),
    ConstantPathNode(ConstantPathNode),
    ConstantPathOperatorWriteNode(ConstantPathOperatorWriteNode),
    ConstantPathOrWriteNode(ConstantPathOrWriteNode),
    ConstantPathTargetNode(ConstantPathTargetNode),
    ConstantPathWriteNode(ConstantPathWriteNode),
    ConstantReadNode(ConstantReadNode),
    ConstantTargetNode(ConstantTargetNode),
    ConstantWriteNode(ConstantWriteNode),
    DefNode(DefNode),
    DefinedNode(DefinedNode),
    ElseNode(ElseNode),
    EmbeddedStatementsNode(EmbeddedStatementsNode),
    EmbeddedVariableNode(EmbeddedVariableNode),
    EnsureNode(EnsureNode),
    FalseNode(FalseNode),
    FindPatternNode(FindPatternNode),
    FlipFlopNode(FlipFlopNode),
    FloatNode(FloatNode),
    ForNode(ForNode),
    ForwardingArgumentsNode(ForwardingArgumentsNode),
    ForwardingParameterNode(ForwardingParameterNode),
    ForwardingSuperNode(ForwardingSuperNode),
    GlobalVariableAndWriteNode(GlobalVariableAndWriteNode),
    GlobalVariableOperatorWriteNode(GlobalVariableOperatorWriteNode),
    GlobalVariableOrWriteNode(GlobalVariableOrWriteNode),
    GlobalVariableReadNode(GlobalVariableReadNode),
    GlobalVariableTargetNode(GlobalVariableTargetNode),
    GlobalVariableWriteNode(GlobalVariableWriteNode),
    HashNode(HashNode),
    HashPatternNode(HashPatternNode),
    IfNode(IfNode),
    ImaginaryNode(ImaginaryNode),
    ImplicitNode(ImplicitNode),
    ImplicitRestNode(ImplicitRestNode),
    InNode(InNode),
    IndexAndWriteNode(IndexAndWriteNode),
    IndexOperatorWriteNode(IndexOperatorWriteNode),
    IndexOrWriteNode(IndexOrWriteNode),
    IndexTargetNode(IndexTargetNode),
    InstanceVariableAndWriteNode(InstanceVariableAndWriteNode),
    InstanceVariableOperatorWriteNode(InstanceVariableOperatorWriteNode),
    InstanceVariableOrWriteNode(InstanceVariableOrWriteNode),
    InstanceVariableReadNode(InstanceVariableReadNode),
    InstanceVariableTargetNode(InstanceVariableTargetNode),
    InstanceVariableWriteNode(InstanceVariableWriteNode),
    IntegerNode(IntegerNode),
    InterpolatedMatchLastLineNode(InterpolatedMatchLastLineNode),
    InterpolatedRegularExpressionNode(InterpolatedRegularExpressionNode),
    InterpolatedStringNode(InterpolatedStringNode),
    InterpolatedSymbolNode(InterpolatedSymbolNode),
    InterpolatedXStringNode(InterpolatedXStringNode),
    ItLocalVariableReadNode(ItLocalVariableReadNode),
    ItParametersNode(ItParametersNode),
    KeywordHashNode(KeywordHashNode),
    KeywordRestParameterNode(KeywordRestParameterNode),
    LambdaNode(LambdaNode),
    LocalVariableAndWriteNode(LocalVariableAndWriteNode),
    LocalVariableOperatorWriteNode(LocalVariableOperatorWriteNode),
    LocalVariableOrWriteNode(LocalVariableOrWriteNode),
    LocalVariableReadNode(LocalVariableReadNode),
    LocalVariableTargetNode(LocalVariableTargetNode),
    LocalVariableWriteNode(LocalVariableWriteNode),
    MatchLastLineNode(MatchLastLineNode),
    MatchPredicateNode(MatchPredicateNode),
    MatchRequiredNode(MatchRequiredNode),
    MatchWriteNode(MatchWriteNode),
    MissingNode(MissingNode),
    ModuleNode(ModuleNode),
    MultiTargetNode(MultiTargetNode),
    MultiWriteNode(MultiWriteNode),
    NextNode(NextNode),
    NilNode(NilNode),
    NoKeywordsParameterNode(NoKeywordsParameterNode),
    NumberedParametersNode(NumberedParametersNode),
    NumberedReferenceReadNode(NumberedReferenceReadNode),
    OptionalKeywordParameterNode(OptionalKeywordParameterNode),
    OptionalParameterNode(OptionalParameterNode),
    OrNode(OrNode),
    ParametersNode(ParametersNode),
    ParenthesesNode(ParenthesesNode),
    PinnedExpressionNode(PinnedExpressionNode),
    PinnedVariableNode(PinnedVariableNode),
    PostExecutionNode(PostExecutionNode),
    PreExecutionNode(PreExecutionNode),
    ProgramNode(ProgramNode),
    RangeNode(RangeNode),
    RationalNode(RationalNode),
    RedoNode(RedoNode),
    RegularExpressionNode(RegularExpressionNode),
    RequiredKeywordParameterNode(RequiredKeywordParameterNode),
    RequiredParameterNode(RequiredParameterNode),
    RescueModifierNode(RescueModifierNode),
    RescueNode(RescueNode),
    RestParameterNode(RestParameterNode),
    RetryNode(RetryNode),
    ReturnNode(ReturnNode),
    SelfNode(SelfNode),
    ShareableConstantNode(ShareableConstantNode),
    SingletonClassNode(SingletonClassNode),
    SourceEncodingNode(SourceEncodingNode),
    SourceFileNode(SourceFileNode),
    SourceLineNode(SourceLineNode),
    SplatNode(SplatNode),
    StatementsNode(StatementsNode),
    StringNode(StringNode),
    SuperNode(SuperNode),
    SymbolNode(SymbolNode),
    TrueNode(TrueNode),
    UndefNode(UndefNode),
    UnlessNode(UnlessNode),
    UntilNode(UntilNode),
    WhenNode(WhenNode),
    WhileNode(WhileNode),
    XStringNode(XStringNode),
    YieldNode(YieldNode),
}

impl NodeKind {
    pub fn name(&self) -> &str {
        match self {
            NodeKind::AliasGlobalVariableNode(_) => "AliasGlobalVariableNode",
            NodeKind::AliasMethodNode(_) => "AliasMethodNode",
            NodeKind::AlternationPatternNode(_) => "AlternationPatternNode",
            NodeKind::AndNode(_) => "AndNode",
            NodeKind::ArgumentsNode(_) => "ArgumentsNode",
            NodeKind::ArrayNode(_) => "ArrayNode",
            NodeKind::ArrayPatternNode(_) => "ArrayPatternNode",
            NodeKind::AssocNode(_) => "AssocNode",
            NodeKind::AssocSplatNode(_) => "AssocSplatNode",
            NodeKind::BackReferenceReadNode(_) => "BackReferenceReadNode",
            NodeKind::BeginNode(_) => "BeginNode",
            NodeKind::BlockArgumentNode(_) => "BlockArgumentNode",
            NodeKind::BlockLocalVariableNode(_) => "BlockLocalVariableNode",
            NodeKind::BlockNode(_) => "BlockNode",
            NodeKind::BlockParameterNode(_) => "BlockParameterNode",
            NodeKind::BlockParametersNode(_) => "BlockParametersNode",
            NodeKind::BreakNode(_) => "BreakNode",
            NodeKind::CallAndWriteNode(_) => "CallAndWriteNode",
            NodeKind::CallNode(_) => "CallNode",
            NodeKind::CallOperatorWriteNode(_) => "CallOperatorWriteNode",
            NodeKind::CallOrWriteNode(_) => "CallOrWriteNode",
            NodeKind::CallTargetNode(_) => "CallTargetNode",
            NodeKind::CapturePatternNode(_) => "CapturePatternNode",
            NodeKind::CaseMatchNode(_) => "CaseMatchNode",
            NodeKind::CaseNode(_) => "CaseNode",
            NodeKind::ClassNode(_) => "ClassNode",
            NodeKind::ClassVariableAndWriteNode(_) => "ClassVariableAndWriteNode",
            NodeKind::ClassVariableOperatorWriteNode(_) => "ClassVariableOperatorWriteNode",
            NodeKind::ClassVariableOrWriteNode(_) => "ClassVariableOrWriteNode",
            NodeKind::ClassVariableReadNode(_) => "ClassVariableReadNode",
            NodeKind::ClassVariableTargetNode(_) => "ClassVariableTargetNode",
            NodeKind::ClassVariableWriteNode(_) => "ClassVariableWriteNode",
            NodeKind::ConstantAndWriteNode(_) => "ConstantAndWriteNode",
            NodeKind::ConstantOperatorWriteNode(_) => "ConstantOperatorWriteNode",
            NodeKind::ConstantOrWriteNode(_) => "ConstantOrWriteNode",
            NodeKind::ConstantPathAndWriteNode(_) => "ConstantPathAndWriteNode",
            NodeKind::ConstantPathNode(_) => "ConstantPathNode",
            NodeKind::ConstantPathOperatorWriteNode(_) => "ConstantPathOperatorWriteNode",
            NodeKind::ConstantPathOrWriteNode(_) => "ConstantPathOrWriteNode",
            NodeKind::ConstantPathTargetNode(_) => "ConstantPathTargetNode",
            NodeKind::ConstantPathWriteNode(_) => "ConstantPathWriteNode",
            NodeKind::ConstantReadNode(_) => "ConstantReadNode",
            NodeKind::ConstantTargetNode(_) => "ConstantTargetNode",
            NodeKind::ConstantWriteNode(_) => "ConstantWriteNode",
            NodeKind::DefNode(_) => "DefNode",
            NodeKind::DefinedNode(_) => "DefinedNode",
            NodeKind::ElseNode(_) => "ElseNode",
            NodeKind::EmbeddedStatementsNode(_) => "EmbeddedStatementsNode",
            NodeKind::EmbeddedVariableNode(_) => "EmbeddedVariableNode",
            NodeKind::EnsureNode(_) => "EnsureNode",
            NodeKind::FalseNode(_) => "FalseNode",
            NodeKind::FindPatternNode(_) => "FindPatternNode",
            NodeKind::FlipFlopNode(_) => "FlipFlopNode",
            NodeKind::FloatNode(_) => "FloatNode",
            NodeKind::ForNode(_) => "ForNode",
            NodeKind::ForwardingArgumentsNode(_) => "ForwardingArgumentsNode",
            NodeKind::ForwardingParameterNode(_) => "ForwardingParameterNode",
            NodeKind::ForwardingSuperNode(_) => "ForwardingSuperNode",
            NodeKind::GlobalVariableAndWriteNode(_) => "GlobalVariableAndWriteNode",
            NodeKind::GlobalVariableOperatorWriteNode(_) => "GlobalVariableOperatorWriteNode",
            NodeKind::GlobalVariableOrWriteNode(_) => "GlobalVariableOrWriteNode",
            NodeKind::GlobalVariableReadNode(_) => "GlobalVariableReadNode",
            NodeKind::GlobalVariableTargetNode(_) => "GlobalVariableTargetNode",
            NodeKind::GlobalVariableWriteNode(_) => "GlobalVariableWriteNode",
            NodeKind::HashNode(_) => "HashNode",
            NodeKind::HashPatternNode(_) => "HashPatternNode",
            NodeKind::IfNode(_) => "IfNode",
            NodeKind::ImaginaryNode(_) => "ImaginaryNode",
            NodeKind::ImplicitNode(_) => "ImplicitNode",
            NodeKind::ImplicitRestNode(_) => "ImplicitRestNode",
            NodeKind::InNode(_) => "InNode",
            NodeKind::IndexAndWriteNode(_) => "IndexAndWriteNode",
            NodeKind::IndexOperatorWriteNode(_) => "IndexOperatorWriteNode",
            NodeKind::IndexOrWriteNode(_) => "IndexOrWriteNode",
            NodeKind::IndexTargetNode(_) => "IndexTargetNode",
            NodeKind::InstanceVariableAndWriteNode(_) => "InstanceVariableAndWriteNode",
            NodeKind::InstanceVariableOperatorWriteNode(_) => "InstanceVariableOperatorWriteNode",
            NodeKind::InstanceVariableOrWriteNode(_) => "InstanceVariableOrWriteNode",
            NodeKind::InstanceVariableReadNode(_) => "InstanceVariableReadNode",
            NodeKind::InstanceVariableTargetNode(_) => "InstanceVariableTargetNode",
            NodeKind::InstanceVariableWriteNode(_) => "InstanceVariableWriteNode",
            NodeKind::IntegerNode(_) => "IntegerNode",
            NodeKind::InterpolatedMatchLastLineNode(_) => "InterpolatedMatchLastLineNode",
            NodeKind::InterpolatedRegularExpressionNode(_) => "InterpolatedRegularExpressionNode",
            NodeKind::InterpolatedStringNode(_) => "InterpolatedStringNode",
            NodeKind::InterpolatedSymbolNode(_) => "InterpolatedSymbolNode",
            NodeKind::InterpolatedXStringNode(_) => "InterpolatedXStringNode",
            NodeKind::ItLocalVariableReadNode(_) => "ItLocalVariableReadNode",
            NodeKind::ItParametersNode(_) => "ItParametersNode",
            NodeKind::KeywordHashNode(_) => "KeywordHashNode",
            NodeKind::KeywordRestParameterNode(_) => "KeywordRestParameterNode",
            NodeKind::LambdaNode(_) => "LambdaNode",
            NodeKind::LocalVariableAndWriteNode(_) => "LocalVariableAndWriteNode",
            NodeKind::LocalVariableOperatorWriteNode(_) => "LocalVariableOperatorWriteNode",
            NodeKind::LocalVariableOrWriteNode(_) => "LocalVariableOrWriteNode",
            NodeKind::LocalVariableReadNode(_) => "LocalVariableReadNode",
            NodeKind::LocalVariableTargetNode(_) => "LocalVariableTargetNode",
            NodeKind::LocalVariableWriteNode(_) => "LocalVariableWriteNode",
            NodeKind::MatchLastLineNode(_) => "MatchLastLineNode",
            NodeKind::MatchPredicateNode(_) => "MatchPredicateNode",
            NodeKind::MatchRequiredNode(_) => "MatchRequiredNode",
            NodeKind::MatchWriteNode(_) => "MatchWriteNode",
            NodeKind::MissingNode(_) => "MissingNode",
            NodeKind::ModuleNode(_) => "ModuleNode",
            NodeKind::MultiTargetNode(_) => "MultiTargetNode",
            NodeKind::MultiWriteNode(_) => "MultiWriteNode",
            NodeKind::NextNode(_) => "NextNode",
            NodeKind::NilNode(_) => "NilNode",
            NodeKind::NoKeywordsParameterNode(_) => "NoKeywordsParameterNode",
            NodeKind::NumberedParametersNode(_) => "NumberedParametersNode",
            NodeKind::NumberedReferenceReadNode(_) => "NumberedReferenceReadNode",
            NodeKind::OptionalKeywordParameterNode(_) => "OptionalKeywordParameterNode",
            NodeKind::OptionalParameterNode(_) => "OptionalParameterNode",
            NodeKind::OrNode(_) => "OrNode",
            NodeKind::ParametersNode(_) => "ParametersNode",
            NodeKind::ParenthesesNode(_) => "ParenthesesNode",
            NodeKind::PinnedExpressionNode(_) => "PinnedExpressionNode",
            NodeKind::PinnedVariableNode(_) => "PinnedVariableNode",
            NodeKind::PostExecutionNode(_) => "PostExecutionNode",
            NodeKind::PreExecutionNode(_) => "PreExecutionNode",
            NodeKind::ProgramNode(_) => "ProgramNode",
            NodeKind::RangeNode(_) => "RangeNode",
            NodeKind::RationalNode(_) => "RationalNode",
            NodeKind::RedoNode(_) => "RedoNode",
            NodeKind::RegularExpressionNode(_) => "RegularExpressionNode",
            NodeKind::RequiredKeywordParameterNode(_) => "RequiredKeywordParameterNode",
            NodeKind::RequiredParameterNode(_) => "RequiredParameterNode",
            NodeKind::RescueModifierNode(_) => "RescueModifierNode",
            NodeKind::RescueNode(_) => "RescueNode",
            NodeKind::RestParameterNode(_) => "RestParameterNode",
            NodeKind::RetryNode(_) => "RetryNode",
            NodeKind::ReturnNode(_) => "ReturnNode",
            NodeKind::SelfNode(_) => "SelfNode",
            NodeKind::ShareableConstantNode(_) => "ShareableConstantNode",
            NodeKind::SingletonClassNode(_) => "SingletonClassNode",
            NodeKind::SourceEncodingNode(_) => "SourceEncodingNode",
            NodeKind::SourceFileNode(_) => "SourceFileNode",
            NodeKind::SourceLineNode(_) => "SourceLineNode",
            NodeKind::SplatNode(_) => "SplatNode",
            NodeKind::StatementsNode(_) => "StatementsNode",
            NodeKind::StringNode(_) => "StringNode",
            NodeKind::SuperNode(_) => "SuperNode",
            NodeKind::SymbolNode(_) => "SymbolNode",
            NodeKind::TrueNode(_) => "TrueNode",
            NodeKind::UndefNode(_) => "UndefNode",
            NodeKind::UnlessNode(_) => "UnlessNode",
            NodeKind::UntilNode(_) => "UntilNode",
            NodeKind::WhenNode(_) => "WhenNode",
            NodeKind::WhileNode(_) => "WhileNode",
            NodeKind::XStringNode(_) => "XStringNode",
            NodeKind::YieldNode(_) => "YieldNode",
        }
    }
}

pub fn parse_node(input: &mut Stream) -> winnow::ModalResult<NodeRef> {
    use winnow::Parser;

    let (kind, identifier, location) =
        winnow::combinator::seq![(winnow::binary::u8, parse_varuint, parse_location,)]
            .parse_next(input)?;

    let node_kind = match kind {
        1 => AliasGlobalVariableNode::parser
            .map(AliasGlobalVariableNode::into_node_kind)
            .parse_next(input),
        2 => AliasMethodNode::parser
            .map(AliasMethodNode::into_node_kind)
            .parse_next(input),
        3 => AlternationPatternNode::parser
            .map(AlternationPatternNode::into_node_kind)
            .parse_next(input),
        4 => AndNode::parser
            .map(AndNode::into_node_kind)
            .parse_next(input),
        5 => ArgumentsNode::parser
            .map(ArgumentsNode::into_node_kind)
            .parse_next(input),
        6 => ArrayNode::parser
            .map(ArrayNode::into_node_kind)
            .parse_next(input),
        7 => ArrayPatternNode::parser
            .map(ArrayPatternNode::into_node_kind)
            .parse_next(input),
        8 => AssocNode::parser
            .map(AssocNode::into_node_kind)
            .parse_next(input),
        9 => AssocSplatNode::parser
            .map(AssocSplatNode::into_node_kind)
            .parse_next(input),
        10 => BackReferenceReadNode::parser
            .map(BackReferenceReadNode::into_node_kind)
            .parse_next(input),
        11 => BeginNode::parser
            .map(BeginNode::into_node_kind)
            .parse_next(input),
        12 => BlockArgumentNode::parser
            .map(BlockArgumentNode::into_node_kind)
            .parse_next(input),
        13 => BlockLocalVariableNode::parser
            .map(BlockLocalVariableNode::into_node_kind)
            .parse_next(input),
        14 => BlockNode::parser
            .map(BlockNode::into_node_kind)
            .parse_next(input),
        15 => BlockParameterNode::parser
            .map(BlockParameterNode::into_node_kind)
            .parse_next(input),
        16 => BlockParametersNode::parser
            .map(BlockParametersNode::into_node_kind)
            .parse_next(input),
        17 => BreakNode::parser
            .map(BreakNode::into_node_kind)
            .parse_next(input),
        18 => CallAndWriteNode::parser
            .map(CallAndWriteNode::into_node_kind)
            .parse_next(input),
        19 => CallNode::parser
            .map(CallNode::into_node_kind)
            .parse_next(input),
        20 => CallOperatorWriteNode::parser
            .map(CallOperatorWriteNode::into_node_kind)
            .parse_next(input),
        21 => CallOrWriteNode::parser
            .map(CallOrWriteNode::into_node_kind)
            .parse_next(input),
        22 => CallTargetNode::parser
            .map(CallTargetNode::into_node_kind)
            .parse_next(input),
        23 => CapturePatternNode::parser
            .map(CapturePatternNode::into_node_kind)
            .parse_next(input),
        24 => CaseMatchNode::parser
            .map(CaseMatchNode::into_node_kind)
            .parse_next(input),
        25 => CaseNode::parser
            .map(CaseNode::into_node_kind)
            .parse_next(input),
        26 => ClassNode::parser
            .map(ClassNode::into_node_kind)
            .parse_next(input),
        27 => ClassVariableAndWriteNode::parser
            .map(ClassVariableAndWriteNode::into_node_kind)
            .parse_next(input),
        28 => ClassVariableOperatorWriteNode::parser
            .map(ClassVariableOperatorWriteNode::into_node_kind)
            .parse_next(input),
        29 => ClassVariableOrWriteNode::parser
            .map(ClassVariableOrWriteNode::into_node_kind)
            .parse_next(input),
        30 => ClassVariableReadNode::parser
            .map(ClassVariableReadNode::into_node_kind)
            .parse_next(input),
        31 => ClassVariableTargetNode::parser
            .map(ClassVariableTargetNode::into_node_kind)
            .parse_next(input),
        32 => ClassVariableWriteNode::parser
            .map(ClassVariableWriteNode::into_node_kind)
            .parse_next(input),
        33 => ConstantAndWriteNode::parser
            .map(ConstantAndWriteNode::into_node_kind)
            .parse_next(input),
        34 => ConstantOperatorWriteNode::parser
            .map(ConstantOperatorWriteNode::into_node_kind)
            .parse_next(input),
        35 => ConstantOrWriteNode::parser
            .map(ConstantOrWriteNode::into_node_kind)
            .parse_next(input),
        36 => ConstantPathAndWriteNode::parser
            .map(ConstantPathAndWriteNode::into_node_kind)
            .parse_next(input),
        37 => ConstantPathNode::parser
            .map(ConstantPathNode::into_node_kind)
            .parse_next(input),
        38 => ConstantPathOperatorWriteNode::parser
            .map(ConstantPathOperatorWriteNode::into_node_kind)
            .parse_next(input),
        39 => ConstantPathOrWriteNode::parser
            .map(ConstantPathOrWriteNode::into_node_kind)
            .parse_next(input),
        40 => ConstantPathTargetNode::parser
            .map(ConstantPathTargetNode::into_node_kind)
            .parse_next(input),
        41 => ConstantPathWriteNode::parser
            .map(ConstantPathWriteNode::into_node_kind)
            .parse_next(input),
        42 => ConstantReadNode::parser
            .map(ConstantReadNode::into_node_kind)
            .parse_next(input),
        43 => ConstantTargetNode::parser
            .map(ConstantTargetNode::into_node_kind)
            .parse_next(input),
        44 => ConstantWriteNode::parser
            .map(ConstantWriteNode::into_node_kind)
            .parse_next(input),
        45 => DefNode::parser
            .map(DefNode::into_node_kind)
            .parse_next(input),
        46 => DefinedNode::parser
            .map(DefinedNode::into_node_kind)
            .parse_next(input),
        47 => ElseNode::parser
            .map(ElseNode::into_node_kind)
            .parse_next(input),
        48 => EmbeddedStatementsNode::parser
            .map(EmbeddedStatementsNode::into_node_kind)
            .parse_next(input),
        49 => EmbeddedVariableNode::parser
            .map(EmbeddedVariableNode::into_node_kind)
            .parse_next(input),
        50 => EnsureNode::parser
            .map(EnsureNode::into_node_kind)
            .parse_next(input),
        51 => FalseNode::parser
            .map(FalseNode::into_node_kind)
            .parse_next(input),
        52 => FindPatternNode::parser
            .map(FindPatternNode::into_node_kind)
            .parse_next(input),
        53 => FlipFlopNode::parser
            .map(FlipFlopNode::into_node_kind)
            .parse_next(input),
        54 => FloatNode::parser
            .map(FloatNode::into_node_kind)
            .parse_next(input),
        55 => ForNode::parser
            .map(ForNode::into_node_kind)
            .parse_next(input),
        56 => ForwardingArgumentsNode::parser
            .map(ForwardingArgumentsNode::into_node_kind)
            .parse_next(input),
        57 => ForwardingParameterNode::parser
            .map(ForwardingParameterNode::into_node_kind)
            .parse_next(input),
        58 => ForwardingSuperNode::parser
            .map(ForwardingSuperNode::into_node_kind)
            .parse_next(input),
        59 => GlobalVariableAndWriteNode::parser
            .map(GlobalVariableAndWriteNode::into_node_kind)
            .parse_next(input),
        60 => GlobalVariableOperatorWriteNode::parser
            .map(GlobalVariableOperatorWriteNode::into_node_kind)
            .parse_next(input),
        61 => GlobalVariableOrWriteNode::parser
            .map(GlobalVariableOrWriteNode::into_node_kind)
            .parse_next(input),
        62 => GlobalVariableReadNode::parser
            .map(GlobalVariableReadNode::into_node_kind)
            .parse_next(input),
        63 => GlobalVariableTargetNode::parser
            .map(GlobalVariableTargetNode::into_node_kind)
            .parse_next(input),
        64 => GlobalVariableWriteNode::parser
            .map(GlobalVariableWriteNode::into_node_kind)
            .parse_next(input),
        65 => HashNode::parser
            .map(HashNode::into_node_kind)
            .parse_next(input),
        66 => HashPatternNode::parser
            .map(HashPatternNode::into_node_kind)
            .parse_next(input),
        67 => IfNode::parser.map(IfNode::into_node_kind).parse_next(input),
        68 => ImaginaryNode::parser
            .map(ImaginaryNode::into_node_kind)
            .parse_next(input),
        69 => ImplicitNode::parser
            .map(ImplicitNode::into_node_kind)
            .parse_next(input),
        70 => ImplicitRestNode::parser
            .map(ImplicitRestNode::into_node_kind)
            .parse_next(input),
        71 => InNode::parser.map(InNode::into_node_kind).parse_next(input),
        72 => IndexAndWriteNode::parser
            .map(IndexAndWriteNode::into_node_kind)
            .parse_next(input),
        73 => IndexOperatorWriteNode::parser
            .map(IndexOperatorWriteNode::into_node_kind)
            .parse_next(input),
        74 => IndexOrWriteNode::parser
            .map(IndexOrWriteNode::into_node_kind)
            .parse_next(input),
        75 => IndexTargetNode::parser
            .map(IndexTargetNode::into_node_kind)
            .parse_next(input),
        76 => InstanceVariableAndWriteNode::parser
            .map(InstanceVariableAndWriteNode::into_node_kind)
            .parse_next(input),
        77 => InstanceVariableOperatorWriteNode::parser
            .map(InstanceVariableOperatorWriteNode::into_node_kind)
            .parse_next(input),
        78 => InstanceVariableOrWriteNode::parser
            .map(InstanceVariableOrWriteNode::into_node_kind)
            .parse_next(input),
        79 => InstanceVariableReadNode::parser
            .map(InstanceVariableReadNode::into_node_kind)
            .parse_next(input),
        80 => InstanceVariableTargetNode::parser
            .map(InstanceVariableTargetNode::into_node_kind)
            .parse_next(input),
        81 => InstanceVariableWriteNode::parser
            .map(InstanceVariableWriteNode::into_node_kind)
            .parse_next(input),
        82 => IntegerNode::parser
            .map(IntegerNode::into_node_kind)
            .parse_next(input),
        83 => InterpolatedMatchLastLineNode::parser
            .map(InterpolatedMatchLastLineNode::into_node_kind)
            .parse_next(input),
        84 => InterpolatedRegularExpressionNode::parser
            .map(InterpolatedRegularExpressionNode::into_node_kind)
            .parse_next(input),
        85 => InterpolatedStringNode::parser
            .map(InterpolatedStringNode::into_node_kind)
            .parse_next(input),
        86 => InterpolatedSymbolNode::parser
            .map(InterpolatedSymbolNode::into_node_kind)
            .parse_next(input),
        87 => InterpolatedXStringNode::parser
            .map(InterpolatedXStringNode::into_node_kind)
            .parse_next(input),
        88 => ItLocalVariableReadNode::parser
            .map(ItLocalVariableReadNode::into_node_kind)
            .parse_next(input),
        89 => ItParametersNode::parser
            .map(ItParametersNode::into_node_kind)
            .parse_next(input),
        90 => KeywordHashNode::parser
            .map(KeywordHashNode::into_node_kind)
            .parse_next(input),
        91 => KeywordRestParameterNode::parser
            .map(KeywordRestParameterNode::into_node_kind)
            .parse_next(input),
        92 => LambdaNode::parser
            .map(LambdaNode::into_node_kind)
            .parse_next(input),
        93 => LocalVariableAndWriteNode::parser
            .map(LocalVariableAndWriteNode::into_node_kind)
            .parse_next(input),
        94 => LocalVariableOperatorWriteNode::parser
            .map(LocalVariableOperatorWriteNode::into_node_kind)
            .parse_next(input),
        95 => LocalVariableOrWriteNode::parser
            .map(LocalVariableOrWriteNode::into_node_kind)
            .parse_next(input),
        96 => LocalVariableReadNode::parser
            .map(LocalVariableReadNode::into_node_kind)
            .parse_next(input),
        97 => LocalVariableTargetNode::parser
            .map(LocalVariableTargetNode::into_node_kind)
            .parse_next(input),
        98 => LocalVariableWriteNode::parser
            .map(LocalVariableWriteNode::into_node_kind)
            .parse_next(input),
        99 => MatchLastLineNode::parser
            .map(MatchLastLineNode::into_node_kind)
            .parse_next(input),
        100 => MatchPredicateNode::parser
            .map(MatchPredicateNode::into_node_kind)
            .parse_next(input),
        101 => MatchRequiredNode::parser
            .map(MatchRequiredNode::into_node_kind)
            .parse_next(input),
        102 => MatchWriteNode::parser
            .map(MatchWriteNode::into_node_kind)
            .parse_next(input),
        103 => MissingNode::parser
            .map(MissingNode::into_node_kind)
            .parse_next(input),
        104 => ModuleNode::parser
            .map(ModuleNode::into_node_kind)
            .parse_next(input),
        105 => MultiTargetNode::parser
            .map(MultiTargetNode::into_node_kind)
            .parse_next(input),
        106 => MultiWriteNode::parser
            .map(MultiWriteNode::into_node_kind)
            .parse_next(input),
        107 => NextNode::parser
            .map(NextNode::into_node_kind)
            .parse_next(input),
        108 => NilNode::parser
            .map(NilNode::into_node_kind)
            .parse_next(input),
        109 => NoKeywordsParameterNode::parser
            .map(NoKeywordsParameterNode::into_node_kind)
            .parse_next(input),
        110 => NumberedParametersNode::parser
            .map(NumberedParametersNode::into_node_kind)
            .parse_next(input),
        111 => NumberedReferenceReadNode::parser
            .map(NumberedReferenceReadNode::into_node_kind)
            .parse_next(input),
        112 => OptionalKeywordParameterNode::parser
            .map(OptionalKeywordParameterNode::into_node_kind)
            .parse_next(input),
        113 => OptionalParameterNode::parser
            .map(OptionalParameterNode::into_node_kind)
            .parse_next(input),
        114 => OrNode::parser.map(OrNode::into_node_kind).parse_next(input),
        115 => ParametersNode::parser
            .map(ParametersNode::into_node_kind)
            .parse_next(input),
        116 => ParenthesesNode::parser
            .map(ParenthesesNode::into_node_kind)
            .parse_next(input),
        117 => PinnedExpressionNode::parser
            .map(PinnedExpressionNode::into_node_kind)
            .parse_next(input),
        118 => PinnedVariableNode::parser
            .map(PinnedVariableNode::into_node_kind)
            .parse_next(input),
        119 => PostExecutionNode::parser
            .map(PostExecutionNode::into_node_kind)
            .parse_next(input),
        120 => PreExecutionNode::parser
            .map(PreExecutionNode::into_node_kind)
            .parse_next(input),
        121 => ProgramNode::parser
            .map(ProgramNode::into_node_kind)
            .parse_next(input),
        122 => RangeNode::parser
            .map(RangeNode::into_node_kind)
            .parse_next(input),
        123 => RationalNode::parser
            .map(RationalNode::into_node_kind)
            .parse_next(input),
        124 => RedoNode::parser
            .map(RedoNode::into_node_kind)
            .parse_next(input),
        125 => RegularExpressionNode::parser
            .map(RegularExpressionNode::into_node_kind)
            .parse_next(input),
        126 => RequiredKeywordParameterNode::parser
            .map(RequiredKeywordParameterNode::into_node_kind)
            .parse_next(input),
        127 => RequiredParameterNode::parser
            .map(RequiredParameterNode::into_node_kind)
            .parse_next(input),
        128 => RescueModifierNode::parser
            .map(RescueModifierNode::into_node_kind)
            .parse_next(input),
        129 => RescueNode::parser
            .map(RescueNode::into_node_kind)
            .parse_next(input),
        130 => RestParameterNode::parser
            .map(RestParameterNode::into_node_kind)
            .parse_next(input),
        131 => RetryNode::parser
            .map(RetryNode::into_node_kind)
            .parse_next(input),
        132 => ReturnNode::parser
            .map(ReturnNode::into_node_kind)
            .parse_next(input),
        133 => SelfNode::parser
            .map(SelfNode::into_node_kind)
            .parse_next(input),
        134 => ShareableConstantNode::parser
            .map(ShareableConstantNode::into_node_kind)
            .parse_next(input),
        135 => SingletonClassNode::parser
            .map(SingletonClassNode::into_node_kind)
            .parse_next(input),
        136 => SourceEncodingNode::parser
            .map(SourceEncodingNode::into_node_kind)
            .parse_next(input),
        137 => SourceFileNode::parser
            .map(SourceFileNode::into_node_kind)
            .parse_next(input),
        138 => SourceLineNode::parser
            .map(SourceLineNode::into_node_kind)
            .parse_next(input),
        139 => SplatNode::parser
            .map(SplatNode::into_node_kind)
            .parse_next(input),
        140 => StatementsNode::parser
            .map(StatementsNode::into_node_kind)
            .parse_next(input),
        141 => StringNode::parser
            .map(StringNode::into_node_kind)
            .parse_next(input),
        142 => SuperNode::parser
            .map(SuperNode::into_node_kind)
            .parse_next(input),
        143 => SymbolNode::parser
            .map(SymbolNode::into_node_kind)
            .parse_next(input),
        144 => TrueNode::parser
            .map(TrueNode::into_node_kind)
            .parse_next(input),
        145 => UndefNode::parser
            .map(UndefNode::into_node_kind)
            .parse_next(input),
        146 => UnlessNode::parser
            .map(UnlessNode::into_node_kind)
            .parse_next(input),
        147 => UntilNode::parser
            .map(UntilNode::into_node_kind)
            .parse_next(input),
        148 => WhenNode::parser
            .map(WhenNode::into_node_kind)
            .parse_next(input),
        149 => WhileNode::parser
            .map(WhileNode::into_node_kind)
            .parse_next(input),
        150 => XStringNode::parser
            .map(XStringNode::into_node_kind)
            .parse_next(input),
        151 => YieldNode::parser
            .map(YieldNode::into_node_kind)
            .parse_next(input),
        _ => winnow::combinator::fail
            .complete_err()
            //.context(StrContext::Expected(StrContextValue::CharLiteral(
            //    kind as char,
            //)))
            .parse_next(input),
    }?;
    Ok(input.state.add_node(Node {
        identifier,
        location,
        node_kind,
    }))
}

pub(super) struct NodeSnapshot<'a> {
    pub program: &'a Program,
    pub node: NodeRef,
}

impl std::fmt::Debug for NodeSnapshot<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let node = self.program.node(&self.node);
        writeln!(
            f,
            "@ {} (location: {:?})",
            node.node_kind.name(),
            LocationSnapshot {
                program: self.program,
                location: &node.location
            }
        )?;
        /*if node.flags == 0 {
          writeln!(f, "├── flags: ∅")?;
        } else {
          writeln!(f, "├── flags: {:?}", node.flags)?;
        }*/

        match &node.node_kind {
            NodeKind::AliasGlobalVariableNode(node) => {
                write!(f, "├── new_name:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.new_name
                    }
                )?;
                drop(pad);
                write!(f, "├── old_name:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.old_name
                    }
                )?;
                drop(pad);
                write!(f, "└── keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.keyword_loc
                    }
                )?;
            }
            NodeKind::AliasMethodNode(node) => {
                write!(f, "├── new_name:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.new_name
                    }
                )?;
                drop(pad);
                write!(f, "├── old_name:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.old_name
                    }
                )?;
                drop(pad);
                write!(f, "└── keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.keyword_loc
                    }
                )?;
            }
            NodeKind::AlternationPatternNode(node) => {
                write!(f, "├── left:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.left
                    }
                )?;
                drop(pad);
                write!(f, "├── right:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.right
                    }
                )?;
                drop(pad);
                write!(f, "└── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
            }
            NodeKind::AndNode(node) => {
                write!(f, "├── left:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.left
                    }
                )?;
                drop(pad);
                write!(f, "├── right:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.right
                    }
                )?;
                drop(pad);
                write!(f, "└── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
            }
            NodeKind::ArgumentsNode(node) => {
                write!(f, "└── arguments:")?;
                writeln!(f, " (length: {})", node.arguments.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.arguments {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
            }
            NodeKind::ArrayNode(node) => {
                write!(f, "├── elements:")?;
                writeln!(f, " (length: {})", node.elements.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.elements {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "├── opening_loc:")?;
                match &node.opening_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "└── closing_loc:")?;
                match &node.closing_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
            }
            NodeKind::ArrayPatternNode(node) => {
                write!(f, "├── constant:")?;
                writeln!(f, "# {:?}", node.constant)?;
                write!(f, "├── requireds:")?;
                writeln!(f, " (length: {})", node.requireds.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.requireds {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "├── rest:")?;
                writeln!(f, "# {:?}", node.rest)?;
                write!(f, "├── posts:")?;
                writeln!(f, " (length: {})", node.posts.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.posts {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "├── opening_loc:")?;
                match &node.opening_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "└── closing_loc:")?;
                match &node.closing_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
            }
            NodeKind::AssocNode(node) => {
                write!(f, "├── key:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.key
                    }
                )?;
                drop(pad);
                write!(f, "├── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
                write!(f, "└── operator_loc:")?;
                match &node.operator_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
            }
            NodeKind::AssocSplatNode(node) => {
                write!(f, "├── value:")?;
                writeln!(f, "# {:?}", node.value)?;
                write!(f, "└── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
            }
            NodeKind::BackReferenceReadNode(node) => {
                write!(f, "└── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
            }
            NodeKind::BeginNode(node) => {
                write!(f, "├── begin_keyword_loc:")?;
                match &node.begin_keyword_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── statements:")?;
                writeln!(f, "# {:?}", node.statements)?;
                write!(f, "├── rescue_clause:")?;
                writeln!(f, "# {:?}", node.rescue_clause)?;
                write!(f, "├── else_clause:")?;
                writeln!(f, "# {:?}", node.else_clause)?;
                write!(f, "├── ensure_clause:")?;
                writeln!(f, "# {:?}", node.ensure_clause)?;
                write!(f, "└── end_keyword_loc:")?;
                match &node.end_keyword_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
            }
            NodeKind::BlockArgumentNode(node) => {
                write!(f, "├── expression:")?;
                writeln!(f, "# {:?}", node.expression)?;
                write!(f, "└── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
            }
            NodeKind::BlockLocalVariableNode(node) => {
                write!(f, "└── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
            }
            NodeKind::BlockNode(node) => {
                write!(f, "├── locals:")?;
                writeln!(
                    f,
                    " {:?}",
                    node.locals
                        .iter()
                        .map(|r| self.program.constant(r))
                        .collect::<Vec<_>>()
                )?;
                write!(f, "├── parameters:")?;
                writeln!(f, "# {:?}", node.parameters)?;
                write!(f, "├── body:")?;
                writeln!(f, "# {:?}", node.body)?;
                write!(f, "├── opening_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.opening_loc
                    }
                )?;
                write!(f, "└── closing_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.closing_loc
                    }
                )?;
            }
            NodeKind::BlockParameterNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, "# {:?}", node.name)?;
                write!(f, "├── name_loc:")?;
                match &node.name_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "└── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
            }
            NodeKind::BlockParametersNode(node) => {
                write!(f, "├── parameters:")?;
                writeln!(f, "# {:?}", node.parameters)?;
                write!(f, "├── locals:")?;
                writeln!(f, " (length: {})", node.locals.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.locals {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "├── opening_loc:")?;
                match &node.opening_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "└── closing_loc:")?;
                match &node.closing_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
            }
            NodeKind::BreakNode(node) => {
                write!(f, "├── arguments:")?;
                writeln!(f, "# {:?}", node.arguments)?;
                write!(f, "└── keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.keyword_loc
                    }
                )?;
            }
            NodeKind::CallAndWriteNode(node) => {
                write!(f, "├── receiver:")?;
                writeln!(f, "# {:?}", node.receiver)?;
                write!(f, "├── call_operator_loc:")?;
                match &node.call_operator_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── message_loc:")?;
                match &node.message_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── read_name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.read_name))?;
                write!(f, "├── write_name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.write_name))?;
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::CallNode(node) => {
                write!(f, "├── receiver:")?;
                writeln!(f, "# {:?}", node.receiver)?;
                write!(f, "├── call_operator_loc:")?;
                match &node.call_operator_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── message_loc:")?;
                match &node.message_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── opening_loc:")?;
                match &node.opening_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── arguments:")?;
                writeln!(f, "# {:?}", node.arguments)?;
                write!(f, "├── closing_loc:")?;
                match &node.closing_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "└── block:")?;
                writeln!(f, "# {:?}", node.block)?;
            }
            NodeKind::CallOperatorWriteNode(node) => {
                write!(f, "├── receiver:")?;
                writeln!(f, "# {:?}", node.receiver)?;
                write!(f, "├── call_operator_loc:")?;
                match &node.call_operator_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── message_loc:")?;
                match &node.message_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── read_name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.read_name))?;
                write!(f, "├── write_name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.write_name))?;
                write!(f, "├── binary_operator:")?;
                writeln!(f, " {:?}", self.program.constant(&node.binary_operator))?;
                write!(f, "├── binary_operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.binary_operator_loc
                    }
                )?;
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::CallOrWriteNode(node) => {
                write!(f, "├── receiver:")?;
                writeln!(f, "# {:?}", node.receiver)?;
                write!(f, "├── call_operator_loc:")?;
                match &node.call_operator_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── message_loc:")?;
                match &node.message_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── read_name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.read_name))?;
                write!(f, "├── write_name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.write_name))?;
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::CallTargetNode(node) => {
                write!(f, "├── receiver:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.receiver
                    }
                )?;
                drop(pad);
                write!(f, "├── call_operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.call_operator_loc
                    }
                )?;
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "└── message_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.message_loc
                    }
                )?;
            }
            NodeKind::CapturePatternNode(node) => {
                write!(f, "├── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
                write!(f, "├── target:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.target
                    }
                )?;
                drop(pad);
                write!(f, "└── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
            }
            NodeKind::CaseMatchNode(node) => {
                write!(f, "├── predicate:")?;
                writeln!(f, "# {:?}", node.predicate)?;
                write!(f, "├── conditions:")?;
                writeln!(f, " (length: {})", node.conditions.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.conditions {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "├── else_clause:")?;
                writeln!(f, "# {:?}", node.else_clause)?;
                write!(f, "├── case_keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.case_keyword_loc
                    }
                )?;
                write!(f, "└── end_keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.end_keyword_loc
                    }
                )?;
            }
            NodeKind::CaseNode(node) => {
                write!(f, "├── predicate:")?;
                writeln!(f, "# {:?}", node.predicate)?;
                write!(f, "├── conditions:")?;
                writeln!(f, " (length: {})", node.conditions.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.conditions {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "├── else_clause:")?;
                writeln!(f, "# {:?}", node.else_clause)?;
                write!(f, "├── case_keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.case_keyword_loc
                    }
                )?;
                write!(f, "└── end_keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.end_keyword_loc
                    }
                )?;
            }
            NodeKind::ClassNode(node) => {
                write!(f, "├── locals:")?;
                writeln!(
                    f,
                    " {:?}",
                    node.locals
                        .iter()
                        .map(|r| self.program.constant(r))
                        .collect::<Vec<_>>()
                )?;
                write!(f, "├── class_keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.class_keyword_loc
                    }
                )?;
                write!(f, "├── constant_path:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.constant_path
                    }
                )?;
                drop(pad);
                write!(f, "├── inheritance_operator_loc:")?;
                match &node.inheritance_operator_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── superclass:")?;
                writeln!(f, "# {:?}", node.superclass)?;
                write!(f, "├── body:")?;
                writeln!(f, "# {:?}", node.body)?;
                write!(f, "├── end_keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.end_keyword_loc
                    }
                )?;
                write!(f, "└── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
            }
            NodeKind::ClassVariableAndWriteNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::ClassVariableOperatorWriteNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── binary_operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.binary_operator_loc
                    }
                )?;
                write!(f, "├── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
                write!(f, "└── binary_operator:")?;
                writeln!(f, " {:?}", self.program.constant(&node.binary_operator))?;
            }
            NodeKind::ClassVariableOrWriteNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::ClassVariableReadNode(node) => {
                write!(f, "└── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
            }
            NodeKind::ClassVariableTargetNode(node) => {
                write!(f, "└── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
            }
            NodeKind::ClassVariableWriteNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
                write!(f, "└── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
            }
            NodeKind::ConstantAndWriteNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::ConstantOperatorWriteNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── binary_operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.binary_operator_loc
                    }
                )?;
                write!(f, "├── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
                write!(f, "└── binary_operator:")?;
                writeln!(f, " {:?}", self.program.constant(&node.binary_operator))?;
            }
            NodeKind::ConstantOrWriteNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::ConstantPathAndWriteNode(node) => {
                write!(f, "├── target:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.target
                    }
                )?;
                drop(pad);
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::ConstantPathNode(node) => {
                write!(f, "├── parent:")?;
                writeln!(f, "# {:?}", node.parent)?;
                write!(f, "├── name:")?;
                writeln!(f, "# {:?}", node.name)?;
                write!(f, "├── delimiter_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.delimiter_loc
                    }
                )?;
                write!(f, "└── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
            }
            NodeKind::ConstantPathOperatorWriteNode(node) => {
                write!(f, "├── target:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.target
                    }
                )?;
                drop(pad);
                write!(f, "├── binary_operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.binary_operator_loc
                    }
                )?;
                write!(f, "├── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
                write!(f, "└── binary_operator:")?;
                writeln!(f, " {:?}", self.program.constant(&node.binary_operator))?;
            }
            NodeKind::ConstantPathOrWriteNode(node) => {
                write!(f, "├── target:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.target
                    }
                )?;
                drop(pad);
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::ConstantPathTargetNode(node) => {
                write!(f, "├── parent:")?;
                writeln!(f, "# {:?}", node.parent)?;
                write!(f, "├── name:")?;
                writeln!(f, "# {:?}", node.name)?;
                write!(f, "├── delimiter_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.delimiter_loc
                    }
                )?;
                write!(f, "└── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
            }
            NodeKind::ConstantPathWriteNode(node) => {
                write!(f, "├── target:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.target
                    }
                )?;
                drop(pad);
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::ConstantReadNode(node) => {
                write!(f, "└── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
            }
            NodeKind::ConstantTargetNode(node) => {
                write!(f, "└── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
            }
            NodeKind::ConstantWriteNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
                write!(f, "└── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
            }
            NodeKind::DefNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── receiver:")?;
                writeln!(f, "# {:?}", node.receiver)?;
                write!(f, "├── parameters:")?;
                writeln!(f, "# {:?}", node.parameters)?;
                write!(f, "├── body:")?;
                writeln!(f, "# {:?}", node.body)?;
                write!(f, "├── locals:")?;
                writeln!(
                    f,
                    " {:?}",
                    node.locals
                        .iter()
                        .map(|r| self.program.constant(r))
                        .collect::<Vec<_>>()
                )?;
                write!(f, "├── def_keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.def_keyword_loc
                    }
                )?;
                write!(f, "├── operator_loc:")?;
                match &node.operator_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── lparen_loc:")?;
                match &node.lparen_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── rparen_loc:")?;
                match &node.rparen_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── equal_loc:")?;
                match &node.equal_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "└── end_keyword_loc:")?;
                match &node.end_keyword_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
            }
            NodeKind::DefinedNode(node) => {
                write!(f, "├── lparen_loc:")?;
                match &node.lparen_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
                write!(f, "├── rparen_loc:")?;
                match &node.rparen_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "└── keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.keyword_loc
                    }
                )?;
            }
            NodeKind::ElseNode(node) => {
                write!(f, "├── else_keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.else_keyword_loc
                    }
                )?;
                write!(f, "├── statements:")?;
                writeln!(f, "# {:?}", node.statements)?;
                write!(f, "└── end_keyword_loc:")?;
                match &node.end_keyword_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
            }
            NodeKind::EmbeddedStatementsNode(node) => {
                write!(f, "├── opening_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.opening_loc
                    }
                )?;
                write!(f, "├── statements:")?;
                writeln!(f, "# {:?}", node.statements)?;
                write!(f, "└── closing_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.closing_loc
                    }
                )?;
            }
            NodeKind::EmbeddedVariableNode(node) => {
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "└── variable:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.variable
                    }
                )?;
                drop(pad);
            }
            NodeKind::EnsureNode(node) => {
                write!(f, "├── ensure_keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.ensure_keyword_loc
                    }
                )?;
                write!(f, "├── statements:")?;
                writeln!(f, "# {:?}", node.statements)?;
                write!(f, "└── end_keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.end_keyword_loc
                    }
                )?;
            }
            NodeKind::FalseNode(node) => {}
            NodeKind::FindPatternNode(node) => {
                write!(f, "├── constant:")?;
                writeln!(f, "# {:?}", node.constant)?;
                write!(f, "├── left:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.left
                    }
                )?;
                drop(pad);
                write!(f, "├── requireds:")?;
                writeln!(f, " (length: {})", node.requireds.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.requireds {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "├── right:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.right
                    }
                )?;
                drop(pad);
                write!(f, "├── opening_loc:")?;
                match &node.opening_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "└── closing_loc:")?;
                match &node.closing_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
            }
            NodeKind::FlipFlopNode(node) => {
                write!(f, "├── left:")?;
                writeln!(f, "# {:?}", node.left)?;
                write!(f, "├── right:")?;
                writeln!(f, "# {:?}", node.right)?;
                write!(f, "└── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
            }
            NodeKind::FloatNode(node) => {
                write!(f, "└── value:")?;
                writeln!(f, "# {:?}", node.value)?;
            }
            NodeKind::ForNode(node) => {
                write!(f, "├── index:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.index
                    }
                )?;
                drop(pad);
                write!(f, "├── collection:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.collection
                    }
                )?;
                drop(pad);
                write!(f, "├── statements:")?;
                writeln!(f, "# {:?}", node.statements)?;
                write!(f, "├── for_keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.for_keyword_loc
                    }
                )?;
                write!(f, "├── in_keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.in_keyword_loc
                    }
                )?;
                write!(f, "├── do_keyword_loc:")?;
                match &node.do_keyword_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "└── end_keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.end_keyword_loc
                    }
                )?;
            }
            NodeKind::ForwardingArgumentsNode(node) => {}
            NodeKind::ForwardingParameterNode(node) => {}
            NodeKind::ForwardingSuperNode(node) => {
                write!(f, "└── block:")?;
                writeln!(f, "# {:?}", node.block)?;
            }
            NodeKind::GlobalVariableAndWriteNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::GlobalVariableOperatorWriteNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── binary_operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.binary_operator_loc
                    }
                )?;
                write!(f, "├── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
                write!(f, "└── binary_operator:")?;
                writeln!(f, " {:?}", self.program.constant(&node.binary_operator))?;
            }
            NodeKind::GlobalVariableOrWriteNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::GlobalVariableReadNode(node) => {
                write!(f, "└── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
            }
            NodeKind::GlobalVariableTargetNode(node) => {
                write!(f, "└── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
            }
            NodeKind::GlobalVariableWriteNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
                write!(f, "└── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
            }
            NodeKind::HashNode(node) => {
                write!(f, "├── opening_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.opening_loc
                    }
                )?;
                write!(f, "├── elements:")?;
                writeln!(f, " (length: {})", node.elements.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.elements {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "└── closing_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.closing_loc
                    }
                )?;
            }
            NodeKind::HashPatternNode(node) => {
                write!(f, "├── constant:")?;
                writeln!(f, "# {:?}", node.constant)?;
                write!(f, "├── elements:")?;
                writeln!(f, " (length: {})", node.elements.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.elements {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "├── rest:")?;
                writeln!(f, "# {:?}", node.rest)?;
                write!(f, "├── opening_loc:")?;
                match &node.opening_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "└── closing_loc:")?;
                match &node.closing_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
            }
            NodeKind::IfNode(node) => {
                write!(f, "├── if_keyword_loc:")?;
                match &node.if_keyword_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── predicate:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.predicate
                    }
                )?;
                drop(pad);
                write!(f, "├── then_keyword_loc:")?;
                match &node.then_keyword_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── statements:")?;
                writeln!(f, "# {:?}", node.statements)?;
                write!(f, "├── subsequent:")?;
                writeln!(f, "# {:?}", node.subsequent)?;
                write!(f, "└── end_keyword_loc:")?;
                match &node.end_keyword_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
            }
            NodeKind::ImaginaryNode(node) => {
                write!(f, "└── numeric:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.numeric
                    }
                )?;
                drop(pad);
            }
            NodeKind::ImplicitNode(node) => {
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::ImplicitRestNode(node) => {}
            NodeKind::InNode(node) => {
                write!(f, "├── pattern:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.pattern
                    }
                )?;
                drop(pad);
                write!(f, "├── statements:")?;
                writeln!(f, "# {:?}", node.statements)?;
                write!(f, "├── in_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.in_loc
                    }
                )?;
                write!(f, "└── then_loc:")?;
                match &node.then_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
            }
            NodeKind::IndexAndWriteNode(node) => {
                write!(f, "├── receiver:")?;
                writeln!(f, "# {:?}", node.receiver)?;
                write!(f, "├── call_operator_loc:")?;
                match &node.call_operator_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── opening_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.opening_loc
                    }
                )?;
                write!(f, "├── arguments:")?;
                writeln!(f, "# {:?}", node.arguments)?;
                write!(f, "├── closing_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.closing_loc
                    }
                )?;
                write!(f, "├── block:")?;
                writeln!(f, "# {:?}", node.block)?;
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::IndexOperatorWriteNode(node) => {
                write!(f, "├── receiver:")?;
                writeln!(f, "# {:?}", node.receiver)?;
                write!(f, "├── call_operator_loc:")?;
                match &node.call_operator_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── opening_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.opening_loc
                    }
                )?;
                write!(f, "├── arguments:")?;
                writeln!(f, "# {:?}", node.arguments)?;
                write!(f, "├── closing_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.closing_loc
                    }
                )?;
                write!(f, "├── block:")?;
                writeln!(f, "# {:?}", node.block)?;
                write!(f, "├── binary_operator:")?;
                writeln!(f, " {:?}", self.program.constant(&node.binary_operator))?;
                write!(f, "├── binary_operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.binary_operator_loc
                    }
                )?;
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::IndexOrWriteNode(node) => {
                write!(f, "├── receiver:")?;
                writeln!(f, "# {:?}", node.receiver)?;
                write!(f, "├── call_operator_loc:")?;
                match &node.call_operator_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── opening_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.opening_loc
                    }
                )?;
                write!(f, "├── arguments:")?;
                writeln!(f, "# {:?}", node.arguments)?;
                write!(f, "├── closing_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.closing_loc
                    }
                )?;
                write!(f, "├── block:")?;
                writeln!(f, "# {:?}", node.block)?;
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::IndexTargetNode(node) => {
                write!(f, "├── receiver:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.receiver
                    }
                )?;
                drop(pad);
                write!(f, "├── opening_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.opening_loc
                    }
                )?;
                write!(f, "├── arguments:")?;
                writeln!(f, "# {:?}", node.arguments)?;
                write!(f, "├── closing_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.closing_loc
                    }
                )?;
                write!(f, "└── block:")?;
                writeln!(f, "# {:?}", node.block)?;
            }
            NodeKind::InstanceVariableAndWriteNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::InstanceVariableOperatorWriteNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── binary_operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.binary_operator_loc
                    }
                )?;
                write!(f, "├── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
                write!(f, "└── binary_operator:")?;
                writeln!(f, " {:?}", self.program.constant(&node.binary_operator))?;
            }
            NodeKind::InstanceVariableOrWriteNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::InstanceVariableReadNode(node) => {
                write!(f, "└── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
            }
            NodeKind::InstanceVariableTargetNode(node) => {
                write!(f, "└── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
            }
            NodeKind::InstanceVariableWriteNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
                write!(f, "└── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
            }
            NodeKind::IntegerNode(node) => {
                write!(f, "└── value:")?;
                writeln!(f, "# {:?}", node.value)?;
            }
            NodeKind::InterpolatedMatchLastLineNode(node) => {
                write!(f, "├── opening_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.opening_loc
                    }
                )?;
                write!(f, "├── parts:")?;
                writeln!(f, " (length: {})", node.parts.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.parts {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "└── closing_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.closing_loc
                    }
                )?;
            }
            NodeKind::InterpolatedRegularExpressionNode(node) => {
                write!(f, "├── opening_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.opening_loc
                    }
                )?;
                write!(f, "├── parts:")?;
                writeln!(f, " (length: {})", node.parts.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.parts {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "└── closing_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.closing_loc
                    }
                )?;
            }
            NodeKind::InterpolatedStringNode(node) => {
                write!(f, "├── opening_loc:")?;
                match &node.opening_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── parts:")?;
                writeln!(f, " (length: {})", node.parts.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.parts {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "└── closing_loc:")?;
                match &node.closing_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
            }
            NodeKind::InterpolatedSymbolNode(node) => {
                write!(f, "├── opening_loc:")?;
                match &node.opening_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── parts:")?;
                writeln!(f, " (length: {})", node.parts.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.parts {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "└── closing_loc:")?;
                match &node.closing_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
            }
            NodeKind::InterpolatedXStringNode(node) => {
                write!(f, "├── opening_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.opening_loc
                    }
                )?;
                write!(f, "├── parts:")?;
                writeln!(f, " (length: {})", node.parts.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.parts {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "└── closing_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.closing_loc
                    }
                )?;
            }
            NodeKind::ItLocalVariableReadNode(node) => {}
            NodeKind::ItParametersNode(node) => {}
            NodeKind::KeywordHashNode(node) => {
                write!(f, "└── elements:")?;
                writeln!(f, " (length: {})", node.elements.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.elements {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
            }
            NodeKind::KeywordRestParameterNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, "# {:?}", node.name)?;
                write!(f, "├── name_loc:")?;
                match &node.name_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "└── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
            }
            NodeKind::LambdaNode(node) => {
                write!(f, "├── locals:")?;
                writeln!(
                    f,
                    " {:?}",
                    node.locals
                        .iter()
                        .map(|r| self.program.constant(r))
                        .collect::<Vec<_>>()
                )?;
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "├── opening_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.opening_loc
                    }
                )?;
                write!(f, "├── closing_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.closing_loc
                    }
                )?;
                write!(f, "├── parameters:")?;
                writeln!(f, "# {:?}", node.parameters)?;
                write!(f, "└── body:")?;
                writeln!(f, "# {:?}", node.body)?;
            }
            NodeKind::LocalVariableAndWriteNode(node) => {
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "├── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "└── depth:")?;
                writeln!(f, "# {:?}", node.depth)?;
            }
            NodeKind::LocalVariableOperatorWriteNode(node) => {
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── binary_operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.binary_operator_loc
                    }
                )?;
                write!(f, "├── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── binary_operator:")?;
                writeln!(f, " {:?}", self.program.constant(&node.binary_operator))?;
                write!(f, "└── depth:")?;
                writeln!(f, "# {:?}", node.depth)?;
            }
            NodeKind::LocalVariableOrWriteNode(node) => {
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "├── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "└── depth:")?;
                writeln!(f, "# {:?}", node.depth)?;
            }
            NodeKind::LocalVariableReadNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "└── depth:")?;
                writeln!(f, "# {:?}", node.depth)?;
            }
            NodeKind::LocalVariableTargetNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "└── depth:")?;
                writeln!(f, "# {:?}", node.depth)?;
            }
            NodeKind::LocalVariableWriteNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── depth:")?;
                writeln!(f, "# {:?}", node.depth)?;
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
                write!(f, "└── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
            }
            NodeKind::MatchLastLineNode(node) => {
                write!(f, "├── opening_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.opening_loc
                    }
                )?;
                write!(f, "├── content_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.content_loc
                    }
                )?;
                write!(f, "├── closing_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.closing_loc
                    }
                )?;
                write!(f, "└── unescaped:")?;
                writeln!(f, " {:?}", node.unescaped)?;
            }
            NodeKind::MatchPredicateNode(node) => {
                write!(f, "├── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
                write!(f, "├── pattern:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.pattern
                    }
                )?;
                drop(pad);
                write!(f, "└── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
            }
            NodeKind::MatchRequiredNode(node) => {
                write!(f, "├── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
                write!(f, "├── pattern:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.pattern
                    }
                )?;
                drop(pad);
                write!(f, "└── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
            }
            NodeKind::MatchWriteNode(node) => {
                write!(f, "├── call:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.call
                    }
                )?;
                drop(pad);
                write!(f, "└── targets:")?;
                writeln!(f, " (length: {})", node.targets.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.targets {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
            }
            NodeKind::MissingNode(node) => {}
            NodeKind::ModuleNode(node) => {
                write!(f, "├── locals:")?;
                writeln!(
                    f,
                    " {:?}",
                    node.locals
                        .iter()
                        .map(|r| self.program.constant(r))
                        .collect::<Vec<_>>()
                )?;
                write!(f, "├── module_keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.module_keyword_loc
                    }
                )?;
                write!(f, "├── constant_path:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.constant_path
                    }
                )?;
                drop(pad);
                write!(f, "├── body:")?;
                writeln!(f, "# {:?}", node.body)?;
                write!(f, "├── end_keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.end_keyword_loc
                    }
                )?;
                write!(f, "└── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
            }
            NodeKind::MultiTargetNode(node) => {
                write!(f, "├── lefts:")?;
                writeln!(f, " (length: {})", node.lefts.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.lefts {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "├── rest:")?;
                writeln!(f, "# {:?}", node.rest)?;
                write!(f, "├── rights:")?;
                writeln!(f, " (length: {})", node.rights.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.rights {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "├── lparen_loc:")?;
                match &node.lparen_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "└── rparen_loc:")?;
                match &node.rparen_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
            }
            NodeKind::MultiWriteNode(node) => {
                write!(f, "├── lefts:")?;
                writeln!(f, " (length: {})", node.lefts.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.lefts {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "├── rest:")?;
                writeln!(f, "# {:?}", node.rest)?;
                write!(f, "├── rights:")?;
                writeln!(f, " (length: {})", node.rights.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.rights {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "├── lparen_loc:")?;
                match &node.lparen_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── rparen_loc:")?;
                match &node.rparen_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::NextNode(node) => {
                write!(f, "├── arguments:")?;
                writeln!(f, "# {:?}", node.arguments)?;
                write!(f, "└── keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.keyword_loc
                    }
                )?;
            }
            NodeKind::NilNode(node) => {}
            NodeKind::NoKeywordsParameterNode(node) => {
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "└── keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.keyword_loc
                    }
                )?;
            }
            NodeKind::NumberedParametersNode(node) => {
                write!(f, "└── maximum:")?;
                writeln!(f, "# {:?}", node.maximum)?;
            }
            NodeKind::NumberedReferenceReadNode(node) => {
                write!(f, "└── number:")?;
                writeln!(f, "# {:?}", node.number)?;
            }
            NodeKind::OptionalKeywordParameterNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::OptionalParameterNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "├── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "└── value:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.value
                    }
                )?;
                drop(pad);
            }
            NodeKind::OrNode(node) => {
                write!(f, "├── left:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.left
                    }
                )?;
                drop(pad);
                write!(f, "├── right:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.right
                    }
                )?;
                drop(pad);
                write!(f, "└── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
            }
            NodeKind::ParametersNode(node) => {
                write!(f, "├── requireds:")?;
                writeln!(f, " (length: {})", node.requireds.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.requireds {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "├── optionals:")?;
                writeln!(f, " (length: {})", node.optionals.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.optionals {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "├── rest:")?;
                writeln!(f, "# {:?}", node.rest)?;
                write!(f, "├── posts:")?;
                writeln!(f, " (length: {})", node.posts.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.posts {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "├── keywords:")?;
                writeln!(f, " (length: {})", node.keywords.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.keywords {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "├── keyword_rest:")?;
                writeln!(f, "# {:?}", node.keyword_rest)?;
                write!(f, "└── block:")?;
                writeln!(f, "# {:?}", node.block)?;
            }
            NodeKind::ParenthesesNode(node) => {
                write!(f, "├── body:")?;
                writeln!(f, "# {:?}", node.body)?;
                write!(f, "├── opening_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.opening_loc
                    }
                )?;
                write!(f, "└── closing_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.closing_loc
                    }
                )?;
            }
            NodeKind::PinnedExpressionNode(node) => {
                write!(f, "├── expression:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.expression
                    }
                )?;
                drop(pad);
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "├── lparen_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.lparen_loc
                    }
                )?;
                write!(f, "└── rparen_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.rparen_loc
                    }
                )?;
            }
            NodeKind::PinnedVariableNode(node) => {
                write!(f, "├── variable:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.variable
                    }
                )?;
                drop(pad);
                write!(f, "└── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
            }
            NodeKind::PostExecutionNode(node) => {
                write!(f, "├── statements:")?;
                writeln!(f, "# {:?}", node.statements)?;
                write!(f, "├── keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.keyword_loc
                    }
                )?;
                write!(f, "├── opening_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.opening_loc
                    }
                )?;
                write!(f, "└── closing_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.closing_loc
                    }
                )?;
            }
            NodeKind::PreExecutionNode(node) => {
                write!(f, "├── statements:")?;
                writeln!(f, "# {:?}", node.statements)?;
                write!(f, "├── keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.keyword_loc
                    }
                )?;
                write!(f, "├── opening_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.opening_loc
                    }
                )?;
                write!(f, "└── closing_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.closing_loc
                    }
                )?;
            }
            NodeKind::ProgramNode(node) => {
                write!(f, "├── locals:")?;
                writeln!(
                    f,
                    " {:?}",
                    node.locals
                        .iter()
                        .map(|r| self.program.constant(r))
                        .collect::<Vec<_>>()
                )?;
                write!(f, "└── statements:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.statements
                    }
                )?;
                drop(pad);
            }
            NodeKind::RangeNode(node) => {
                write!(f, "├── left:")?;
                writeln!(f, "# {:?}", node.left)?;
                write!(f, "├── right:")?;
                writeln!(f, "# {:?}", node.right)?;
                write!(f, "└── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
            }
            NodeKind::RationalNode(node) => {
                write!(f, "├── numerator:")?;
                writeln!(f, "# {:?}", node.numerator)?;
                write!(f, "└── denominator:")?;
                writeln!(f, "# {:?}", node.denominator)?;
            }
            NodeKind::RedoNode(node) => {}
            NodeKind::RegularExpressionNode(node) => {
                write!(f, "├── opening_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.opening_loc
                    }
                )?;
                write!(f, "├── content_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.content_loc
                    }
                )?;
                write!(f, "├── closing_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.closing_loc
                    }
                )?;
                write!(f, "└── unescaped:")?;
                writeln!(f, " {:?}", node.unescaped)?;
            }
            NodeKind::RequiredKeywordParameterNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
                write!(f, "└── name_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.name_loc
                    }
                )?;
            }
            NodeKind::RequiredParameterNode(node) => {
                write!(f, "└── name:")?;
                writeln!(f, " {:?}", self.program.constant(&node.name))?;
            }
            NodeKind::RescueModifierNode(node) => {
                write!(f, "├── expression:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.expression
                    }
                )?;
                drop(pad);
                write!(f, "├── keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.keyword_loc
                    }
                )?;
                write!(f, "└── rescue_expression:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.rescue_expression
                    }
                )?;
                drop(pad);
            }
            NodeKind::RescueNode(node) => {
                write!(f, "├── keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.keyword_loc
                    }
                )?;
                write!(f, "├── exceptions:")?;
                writeln!(f, " (length: {})", node.exceptions.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.exceptions {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "├── operator_loc:")?;
                match &node.operator_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── reference:")?;
                writeln!(f, "# {:?}", node.reference)?;
                write!(f, "├── then_keyword_loc:")?;
                match &node.then_keyword_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── statements:")?;
                writeln!(f, "# {:?}", node.statements)?;
                write!(f, "└── subsequent:")?;
                writeln!(f, "# {:?}", node.subsequent)?;
            }
            NodeKind::RestParameterNode(node) => {
                write!(f, "├── name:")?;
                writeln!(f, "# {:?}", node.name)?;
                write!(f, "├── name_loc:")?;
                match &node.name_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "└── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
            }
            NodeKind::RetryNode(node) => {}
            NodeKind::ReturnNode(node) => {
                write!(f, "├── keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.keyword_loc
                    }
                )?;
                write!(f, "└── arguments:")?;
                writeln!(f, "# {:?}", node.arguments)?;
            }
            NodeKind::SelfNode(node) => {}
            NodeKind::ShareableConstantNode(node) => {
                write!(f, "└── write:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.write
                    }
                )?;
                drop(pad);
            }
            NodeKind::SingletonClassNode(node) => {
                write!(f, "├── locals:")?;
                writeln!(
                    f,
                    " {:?}",
                    node.locals
                        .iter()
                        .map(|r| self.program.constant(r))
                        .collect::<Vec<_>>()
                )?;
                write!(f, "├── class_keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.class_keyword_loc
                    }
                )?;
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "├── expression:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.expression
                    }
                )?;
                drop(pad);
                write!(f, "├── body:")?;
                writeln!(f, "# {:?}", node.body)?;
                write!(f, "└── end_keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.end_keyword_loc
                    }
                )?;
            }
            NodeKind::SourceEncodingNode(node) => {}
            NodeKind::SourceFileNode(node) => {
                write!(f, "└── filepath:")?;
                writeln!(f, " {:?}", node.filepath)?;
            }
            NodeKind::SourceLineNode(node) => {}
            NodeKind::SplatNode(node) => {
                write!(f, "├── operator_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.operator_loc
                    }
                )?;
                write!(f, "└── expression:")?;
                writeln!(f, "# {:?}", node.expression)?;
            }
            NodeKind::StatementsNode(node) => {
                write!(f, "└── body:")?;
                writeln!(f, " (length: {})", node.body.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.body {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
            }
            NodeKind::StringNode(node) => {
                write!(f, "├── opening_loc:")?;
                match &node.opening_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── content_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.content_loc
                    }
                )?;
                write!(f, "├── closing_loc:")?;
                match &node.closing_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "└── unescaped:")?;
                writeln!(f, " {:?}", node.unescaped)?;
            }
            NodeKind::SuperNode(node) => {
                write!(f, "├── keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.keyword_loc
                    }
                )?;
                write!(f, "├── lparen_loc:")?;
                match &node.lparen_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── arguments:")?;
                writeln!(f, "# {:?}", node.arguments)?;
                write!(f, "├── rparen_loc:")?;
                match &node.rparen_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "└── block:")?;
                writeln!(f, "# {:?}", node.block)?;
            }
            NodeKind::SymbolNode(node) => {
                write!(f, "├── opening_loc:")?;
                match &node.opening_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── value_loc:")?;
                match &node.value_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── closing_loc:")?;
                match &node.closing_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "└── unescaped:")?;
                writeln!(f, " {:?}", node.unescaped)?;
            }
            NodeKind::TrueNode(node) => {}
            NodeKind::UndefNode(node) => {
                write!(f, "├── names:")?;
                writeln!(f, " (length: {})", node.names.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.names {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "└── keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.keyword_loc
                    }
                )?;
            }
            NodeKind::UnlessNode(node) => {
                write!(f, "├── keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.keyword_loc
                    }
                )?;
                write!(f, "├── predicate:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.predicate
                    }
                )?;
                drop(pad);
                write!(f, "├── then_keyword_loc:")?;
                match &node.then_keyword_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── statements:")?;
                writeln!(f, "# {:?}", node.statements)?;
                write!(f, "├── else_clause:")?;
                writeln!(f, "# {:?}", node.else_clause)?;
                write!(f, "└── end_keyword_loc:")?;
                match &node.end_keyword_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
            }
            NodeKind::UntilNode(node) => {
                write!(f, "├── keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.keyword_loc
                    }
                )?;
                write!(f, "├── do_keyword_loc:")?;
                match &node.do_keyword_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── closing_loc:")?;
                match &node.closing_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── predicate:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.predicate
                    }
                )?;
                drop(pad);
                write!(f, "└── statements:")?;
                writeln!(f, "# {:?}", node.statements)?;
            }
            NodeKind::WhenNode(node) => {
                write!(f, "├── keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.keyword_loc
                    }
                )?;
                write!(f, "├── conditions:")?;
                writeln!(f, " (length: {})", node.conditions.len())?;
                let mut pad = PadWriter::new(f, "|   ", true);
                for node in &node.conditions {
                    writeln!(
                        &mut pad,
                        "├── {:?}",
                        NodeSnapshot {
                            program: self.program,
                            node: *node
                        }
                    )?;
                }
                drop(pad);
                write!(f, "├── then_keyword_loc:")?;
                match &node.then_keyword_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "└── statements:")?;
                writeln!(f, "# {:?}", node.statements)?;
            }
            NodeKind::WhileNode(node) => {
                write!(f, "├── keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.keyword_loc
                    }
                )?;
                write!(f, "├── do_keyword_loc:")?;
                match &node.do_keyword_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── closing_loc:")?;
                match &node.closing_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── predicate:")?;
                writeln!(f)?;
                let mut pad = PadWriter::new(f, "    ", true);
                writeln!(
                    &mut pad,
                    "{:?}",
                    NodeSnapshot {
                        program: self.program,
                        node: node.predicate
                    }
                )?;
                drop(pad);
                write!(f, "└── statements:")?;
                writeln!(f, "# {:?}", node.statements)?;
            }
            NodeKind::XStringNode(node) => {
                write!(f, "├── opening_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.opening_loc
                    }
                )?;
                write!(f, "├── content_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.content_loc
                    }
                )?;
                write!(f, "├── closing_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.closing_loc
                    }
                )?;
                write!(f, "└── unescaped:")?;
                writeln!(f, " {:?}", node.unescaped)?;
            }
            NodeKind::YieldNode(node) => {
                write!(f, "├── keyword_loc:")?;
                writeln!(
                    f,
                    "{:?}",
                    LocationSnapshot {
                        program: self.program,
                        location: &node.keyword_loc
                    }
                )?;
                write!(f, "├── lparen_loc:")?;
                match &node.lparen_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
                write!(f, "├── arguments:")?;
                writeln!(f, "# {:?}", node.arguments)?;
                write!(f, "└── rparen_loc:")?;
                match &node.rparen_loc {
                    Some(loc) => writeln!(
                        f,
                        " {:?}",
                        LocationSnapshot {
                            program: self.program,
                            location: loc
                        }
                    ),
                    None => writeln!(f, " ∅"),
                }?;
            }
        };

        Ok(())
    }
}

#[derive(derive_new::new)]
struct PadWriter<'a, T: std::fmt::Write> {
    writer: &'a mut T,
    pad: &'static str,
    at_newline: bool,
}

impl<T: std::fmt::Write> std::fmt::Write for PadWriter<'_, T> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let mut buf = s;
        loop {
            let l = buf.find('\n');
            if self.at_newline {
                self.writer.write_str(self.pad)?;
                self.at_newline = false;
            }
            match l {
                None => {
                    self.writer.write_str(buf)?;
                    break;
                }
                Some(l) => {
                    self.writer.write_str(&buf[..l])?;
                    self.writer.write_str("\n")?;
                    buf = &buf[l + 1..];
                    self.at_newline = true;
                    if buf.is_empty() {
                        break;
                    }
                }
            };
        }
        Ok(())
    }
}
