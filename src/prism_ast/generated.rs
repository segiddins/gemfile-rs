use std::fmt::Write;

use super::deserialize::*;
use enumflags2::BitFlag;
use winnow::binary::length_repeat;

/* Flags for arguments nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgumentsNodeFlags {
    /* Flags for arguments nodes. */
    CONTAINS_FORWARDING = 1 << 0,
    /* Flags for arguments nodes. */
    CONTAINS_KEYWORDS = 1 << 1,
    /* Flags for arguments nodes. */
    CONTAINS_KEYWORD_SPLAT = 1 << 2,
    /* Flags for arguments nodes. */
    CONTAINS_SPLAT = 1 << 3,
    /* Flags for arguments nodes. */
    CONTAINS_MULTIPLE_SPLATS = 1 << 4,
}

/* Flags for array nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayNodeFlags {
    /* Flags for array nodes. */
    CONTAINS_SPLAT = 1 << 0,
}

/* Flags for call nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallNodeFlags {
    /* Flags for call nodes. */
    SAFE_NAVIGATION = 1 << 0,
    /* Flags for call nodes. */
    VARIABLE_CALL = 1 << 1,
    /* Flags for call nodes. */
    ATTRIBUTE_WRITE = 1 << 2,
    /* Flags for call nodes. */
    IGNORE_VISIBILITY = 1 << 3,
}

/* Flags for nodes that have unescaped content. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodingFlags {
    /* Flags for nodes that have unescaped content. */
    FORCED_UTF8_ENCODING = 1 << 0,
    /* Flags for nodes that have unescaped content. */
    FORCED_BINARY_ENCODING = 1 << 1,
}

/* Flags for integer nodes that correspond to the base of the integer. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerBaseFlags {
    /* Flags for integer nodes that correspond to the base of the integer. */
    BINARY = 1 << 0,
    /* Flags for integer nodes that correspond to the base of the integer. */
    DECIMAL = 1 << 1,
    /* Flags for integer nodes that correspond to the base of the integer. */
    OCTAL = 1 << 2,
    /* Flags for integer nodes that correspond to the base of the integer. */
    HEXADECIMAL = 1 << 3,
}

/* Flags for interpolated string nodes that indicated mutability if they are also marked as literals. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpolatedStringNodeFlags {
    /* Flags for interpolated string nodes that indicated mutability if they are also marked as literals. */
    FROZEN = 1 << 0,
    /* Flags for interpolated string nodes that indicated mutability if they are also marked as literals. */
    MUTABLE = 1 << 1,
}

/* Flags for keyword hash nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeywordHashNodeFlags {
    /* Flags for keyword hash nodes. */
    SYMBOL_KEYS = 1 << 0,
}

/* Flags for while and until loop nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopFlags {
    /* Flags for while and until loop nodes. */
    BEGIN_MODIFIER = 1 << 0,
}

/* Flags for parameter nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterFlags {
    /* Flags for parameter nodes. */
    REPEATED_PARAMETER = 1 << 0,
}

/* Flags for range and flip-flop nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeFlags {
    /* Flags for range and flip-flop nodes. */
    EXCLUDE_END = 1 << 0,
}

/* Flags for regular expression and match last line nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegularExpressionFlags {
    /* Flags for regular expression and match last line nodes. */
    IGNORE_CASE = 1 << 0,
    /* Flags for regular expression and match last line nodes. */
    EXTENDED = 1 << 1,
    /* Flags for regular expression and match last line nodes. */
    MULTI_LINE = 1 << 2,
    /* Flags for regular expression and match last line nodes. */
    ONCE = 1 << 3,
    /* Flags for regular expression and match last line nodes. */
    EUC_JP = 1 << 4,
    /* Flags for regular expression and match last line nodes. */
    ASCII_8BIT = 1 << 5,
    /* Flags for regular expression and match last line nodes. */
    WINDOWS_31J = 1 << 6,
    /* Flags for regular expression and match last line nodes. */
    UTF_8 = 1 << 7,
    /* Flags for regular expression and match last line nodes. */
    FORCED_UTF8_ENCODING = 1 << 8,
    /* Flags for regular expression and match last line nodes. */
    FORCED_BINARY_ENCODING = 1 << 9,
    /* Flags for regular expression and match last line nodes. */
    FORCED_US_ASCII_ENCODING = 1 << 10,
}

/* Flags for shareable constant nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShareableConstantNodeFlags {
    /* Flags for shareable constant nodes. */
    LITERAL = 1 << 0,
    /* Flags for shareable constant nodes. */
    EXPERIMENTAL_EVERYTHING = 1 << 1,
    /* Flags for shareable constant nodes. */
    EXPERIMENTAL_COPY = 1 << 2,
}

/* Flags for string nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringFlags {
    /* Flags for string nodes. */
    FORCED_UTF8_ENCODING = 1 << 0,
    /* Flags for string nodes. */
    FORCED_BINARY_ENCODING = 1 << 1,
    /* Flags for string nodes. */
    FROZEN = 1 << 2,
    /* Flags for string nodes. */
    MUTABLE = 1 << 3,
}

/* Flags for symbol nodes. */
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolFlags {
    /* Flags for symbol nodes. */
    FORCED_UTF8_ENCODING = 1 << 0,
    /* Flags for symbol nodes. */
    FORCED_BINARY_ENCODING = 1 << 1,
    /* Flags for symbol nodes. */
    FORCED_US_ASCII_ENCODING = 1 << 2,
}

// 0
#[derive(Debug, Clone)]
pub struct AliasGlobalVariableNode {
    new_name: NodeRef,
    old_name: NodeRef,
    keyword_loc: Location,
}
impl AliasGlobalVariableNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::AliasGlobalVariableNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![AliasGlobalVariableNode{
        _: zero_flags.context(winnow::error::StrContext::Label("AliasGlobalVariableNode.flags")),
        new_name: parse_node.context(winnow::error::StrContext::Label("AliasGlobalVariableNode.new_name")),
        old_name: parse_node.context(winnow::error::StrContext::Label("AliasGlobalVariableNode.old_name")),
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("AliasGlobalVariableNode.keyword_loc")),
    }].parse_next(input)
    }
}

// 1
#[derive(Debug, Clone)]
pub struct AliasMethodNode {
    new_name: NodeRef,
    old_name: NodeRef,
    keyword_loc: Location,
}
impl AliasMethodNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::AliasMethodNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![AliasMethodNode{
        _: zero_flags.context(winnow::error::StrContext::Label("AliasMethodNode.flags")),
        new_name: parse_node.context(winnow::error::StrContext::Label("AliasMethodNode.new_name")),
        old_name: parse_node.context(winnow::error::StrContext::Label("AliasMethodNode.old_name")),
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("AliasMethodNode.keyword_loc")),
    }].parse_next(input)
    }
}

// 2
#[derive(Debug, Clone)]
pub struct AlternationPatternNode {
    left: NodeRef,
    right: NodeRef,
    operator_loc: Location,
}
impl AlternationPatternNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::AlternationPatternNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![AlternationPatternNode{
        _: zero_flags.context(winnow::error::StrContext::Label("AlternationPatternNode.flags")),
        left: parse_node.context(winnow::error::StrContext::Label("AlternationPatternNode.left")),
        right: parse_node.context(winnow::error::StrContext::Label("AlternationPatternNode.right")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("AlternationPatternNode.operator_loc")),
    }].parse_next(input)
    }
}

// 3
#[derive(Debug, Clone)]
pub struct AndNode {
    left: NodeRef,
    right: NodeRef,
    operator_loc: Location,
}
impl AndNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::AndNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![AndNode{
        _: zero_flags.context(winnow::error::StrContext::Label("AndNode.flags")),
        left: parse_node.context(winnow::error::StrContext::Label("AndNode.left")),
        right: parse_node.context(winnow::error::StrContext::Label("AndNode.right")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("AndNode.operator_loc")),
    }].parse_next(input)
    }
}

// 4
#[derive(Debug, Clone)]
pub struct ArgumentsNode {
    pub flags: enumflags2::BitFlags<ArgumentsNodeFlags>,
    arguments: Vec<NodeRef>,
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

// 5
#[derive(Debug, Clone)]
pub struct ArrayNode {
    pub flags: enumflags2::BitFlags<ArrayNodeFlags>,
    elements: Vec<NodeRef>,
    opening_loc: Option<Location>,
    closing_loc: Option<Location>,
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

// 6
#[derive(Debug, Clone)]
pub struct ArrayPatternNode {
    constant: Option<NodeRef>,
    requireds: Vec<NodeRef>,
    rest: Option<NodeRef>,
    posts: Vec<NodeRef>,
    opening_loc: Option<Location>,
    closing_loc: Option<Location>,
}
impl ArrayPatternNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ArrayPatternNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ArrayPatternNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ArrayPatternNode.flags")),
        constant: parse_optional_node.context(winnow::error::StrContext::Label("ArrayPatternNode.constant")),
        requireds: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("ArrayPatternNode.requireds")),
        rest: parse_optional_node.context(winnow::error::StrContext::Label("ArrayPatternNode.rest")),
        posts: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("ArrayPatternNode.posts")),
        opening_loc: parse_optional_location.context(winnow::error::StrContext::Label("ArrayPatternNode.opening_loc")),
        closing_loc: parse_optional_location.context(winnow::error::StrContext::Label("ArrayPatternNode.closing_loc")),
    }].parse_next(input)
    }
}

// 7
#[derive(Debug, Clone)]
pub struct AssocNode {
    key: NodeRef,
    value: NodeRef,
    operator_loc: Option<Location>,
}
impl AssocNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::AssocNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![AssocNode{
        _: zero_flags.context(winnow::error::StrContext::Label("AssocNode.flags")),
        key: parse_node.context(winnow::error::StrContext::Label("AssocNode.key")),
        value: parse_node.context(winnow::error::StrContext::Label("AssocNode.value")),
        operator_loc: parse_optional_location.context(winnow::error::StrContext::Label("AssocNode.operator_loc")),
    }].parse_next(input)
    }
}

// 8
#[derive(Debug, Clone)]
pub struct AssocSplatNode {
    value: Option<NodeRef>,
    operator_loc: Location,
}
impl AssocSplatNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::AssocSplatNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![AssocSplatNode{
        _: zero_flags.context(winnow::error::StrContext::Label("AssocSplatNode.flags")),
        value: parse_optional_node.context(winnow::error::StrContext::Label("AssocSplatNode.value")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("AssocSplatNode.operator_loc")),
    }].parse_next(input)
    }
}

// 9
#[derive(Debug, Clone)]
pub struct BackReferenceReadNode {
    name: ConstantRef,
}
impl BackReferenceReadNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::BackReferenceReadNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![BackReferenceReadNode{
        _: zero_flags.context(winnow::error::StrContext::Label("BackReferenceReadNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("BackReferenceReadNode.name")),
    }].parse_next(input)
    }
}

// 10
#[derive(Debug, Clone)]
pub struct BeginNode {
    begin_keyword_loc: Option<Location>,
    statements: Option<NodeRef>,
    rescue_clause: Option<NodeRef>,
    else_clause: Option<NodeRef>,
    ensure_clause: Option<NodeRef>,
    end_keyword_loc: Option<Location>,
}
impl BeginNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::BeginNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![BeginNode{
        _: zero_flags.context(winnow::error::StrContext::Label("BeginNode.flags")),
        begin_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("BeginNode.begin_keyword_loc")),
        statements: parse_optional_node.context(winnow::error::StrContext::Label("BeginNode.statements")),
        rescue_clause: parse_optional_node.context(winnow::error::StrContext::Label("BeginNode.rescue_clause")),
        else_clause: parse_optional_node.context(winnow::error::StrContext::Label("BeginNode.else_clause")),
        ensure_clause: parse_optional_node.context(winnow::error::StrContext::Label("BeginNode.ensure_clause")),
        end_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("BeginNode.end_keyword_loc")),
    }].parse_next(input)
    }
}

// 11
#[derive(Debug, Clone)]
pub struct BlockArgumentNode {
    expression: Option<NodeRef>,
    operator_loc: Location,
}
impl BlockArgumentNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::BlockArgumentNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![BlockArgumentNode{
        _: zero_flags.context(winnow::error::StrContext::Label("BlockArgumentNode.flags")),
        expression: parse_optional_node.context(winnow::error::StrContext::Label("BlockArgumentNode.expression")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("BlockArgumentNode.operator_loc")),
    }].parse_next(input)
    }
}

// 12
#[derive(Debug, Clone)]
pub struct BlockLocalVariableNode {
    pub flags: enumflags2::BitFlags<ParameterFlags>,
    name: ConstantRef,
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

// 13
#[derive(Debug, Clone)]
pub struct BlockNode {
    locals: Vec<ConstantRef>,
    parameters: Option<NodeRef>,
    body: Option<NodeRef>,
    opening_loc: Location,
    closing_loc: Location,
}
impl BlockNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::BlockNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![BlockNode{
        _: zero_flags.context(winnow::error::StrContext::Label("BlockNode.flags")),
        locals: length_repeat(parse_varuint, parse_constant).context(winnow::error::StrContext::Label("BlockNode.locals")),
        parameters: parse_optional_node.context(winnow::error::StrContext::Label("BlockNode.parameters")),
        body: parse_optional_node.context(winnow::error::StrContext::Label("BlockNode.body")),
        opening_loc: parse_location.context(winnow::error::StrContext::Label("BlockNode.opening_loc")),
        closing_loc: parse_location.context(winnow::error::StrContext::Label("BlockNode.closing_loc")),
    }].parse_next(input)
    }
}

// 14
#[derive(Debug, Clone)]
pub struct BlockParameterNode {
    pub flags: enumflags2::BitFlags<ParameterFlags>,
    name: Option<ConstantRef>,
    name_loc: Option<Location>,
    operator_loc: Location,
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

// 15
#[derive(Debug, Clone)]
pub struct BlockParametersNode {
    parameters: Option<NodeRef>,
    locals: Vec<NodeRef>,
    opening_loc: Option<Location>,
    closing_loc: Option<Location>,
}
impl BlockParametersNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::BlockParametersNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![BlockParametersNode{
        _: zero_flags.context(winnow::error::StrContext::Label("BlockParametersNode.flags")),
        parameters: parse_optional_node.context(winnow::error::StrContext::Label("BlockParametersNode.parameters")),
        locals: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("BlockParametersNode.locals")),
        opening_loc: parse_optional_location.context(winnow::error::StrContext::Label("BlockParametersNode.opening_loc")),
        closing_loc: parse_optional_location.context(winnow::error::StrContext::Label("BlockParametersNode.closing_loc")),
    }].parse_next(input)
    }
}

// 16
#[derive(Debug, Clone)]
pub struct BreakNode {
    arguments: Option<NodeRef>,
    keyword_loc: Location,
}
impl BreakNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::BreakNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![BreakNode{
        _: zero_flags.context(winnow::error::StrContext::Label("BreakNode.flags")),
        arguments: parse_optional_node.context(winnow::error::StrContext::Label("BreakNode.arguments")),
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("BreakNode.keyword_loc")),
    }].parse_next(input)
    }
}

// 17
#[derive(Debug, Clone)]
pub struct CallAndWriteNode {
    pub flags: enumflags2::BitFlags<CallNodeFlags>,
    receiver: Option<NodeRef>,
    call_operator_loc: Option<Location>,
    message_loc: Option<Location>,
    read_name: ConstantRef,
    write_name: ConstantRef,
    operator_loc: Location,
    value: NodeRef,
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

// 18
#[derive(Debug, Clone)]
pub struct CallNode {
    pub flags: enumflags2::BitFlags<CallNodeFlags>,
    receiver: Option<NodeRef>,
    call_operator_loc: Option<Location>,
    name: ConstantRef,
    message_loc: Option<Location>,
    opening_loc: Option<Location>,
    arguments: Option<NodeRef>,
    closing_loc: Option<Location>,
    block: Option<NodeRef>,
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

// 19
#[derive(Debug, Clone)]
pub struct CallOperatorWriteNode {
    pub flags: enumflags2::BitFlags<CallNodeFlags>,
    receiver: Option<NodeRef>,
    call_operator_loc: Option<Location>,
    message_loc: Option<Location>,
    read_name: ConstantRef,
    write_name: ConstantRef,
    binary_operator: ConstantRef,
    binary_operator_loc: Location,
    value: NodeRef,
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

// 20
#[derive(Debug, Clone)]
pub struct CallOrWriteNode {
    pub flags: enumflags2::BitFlags<CallNodeFlags>,
    receiver: Option<NodeRef>,
    call_operator_loc: Option<Location>,
    message_loc: Option<Location>,
    read_name: ConstantRef,
    write_name: ConstantRef,
    operator_loc: Location,
    value: NodeRef,
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

// 21
#[derive(Debug, Clone)]
pub struct CallTargetNode {
    pub flags: enumflags2::BitFlags<CallNodeFlags>,
    receiver: NodeRef,
    call_operator_loc: Location,
    name: ConstantRef,
    message_loc: Location,
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

// 22
#[derive(Debug, Clone)]
pub struct CapturePatternNode {
    value: NodeRef,
    target: NodeRef,
    operator_loc: Location,
}
impl CapturePatternNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::CapturePatternNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![CapturePatternNode{
        _: zero_flags.context(winnow::error::StrContext::Label("CapturePatternNode.flags")),
        value: parse_node.context(winnow::error::StrContext::Label("CapturePatternNode.value")),
        target: parse_node.context(winnow::error::StrContext::Label("CapturePatternNode.target")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("CapturePatternNode.operator_loc")),
    }].parse_next(input)
    }
}

// 23
#[derive(Debug, Clone)]
pub struct CaseMatchNode {
    predicate: Option<NodeRef>,
    conditions: Vec<NodeRef>,
    else_clause: Option<NodeRef>,
    case_keyword_loc: Location,
    end_keyword_loc: Location,
}
impl CaseMatchNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::CaseMatchNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![CaseMatchNode{
        _: zero_flags.context(winnow::error::StrContext::Label("CaseMatchNode.flags")),
        predicate: parse_optional_node.context(winnow::error::StrContext::Label("CaseMatchNode.predicate")),
        conditions: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("CaseMatchNode.conditions")),
        else_clause: parse_optional_node.context(winnow::error::StrContext::Label("CaseMatchNode.else_clause")),
        case_keyword_loc: parse_location.context(winnow::error::StrContext::Label("CaseMatchNode.case_keyword_loc")),
        end_keyword_loc: parse_location.context(winnow::error::StrContext::Label("CaseMatchNode.end_keyword_loc")),
    }].parse_next(input)
    }
}

// 24
#[derive(Debug, Clone)]
pub struct CaseNode {
    predicate: Option<NodeRef>,
    conditions: Vec<NodeRef>,
    else_clause: Option<NodeRef>,
    case_keyword_loc: Location,
    end_keyword_loc: Location,
}
impl CaseNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::CaseNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![CaseNode{
        _: zero_flags.context(winnow::error::StrContext::Label("CaseNode.flags")),
        predicate: parse_optional_node.context(winnow::error::StrContext::Label("CaseNode.predicate")),
        conditions: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("CaseNode.conditions")),
        else_clause: parse_optional_node.context(winnow::error::StrContext::Label("CaseNode.else_clause")),
        case_keyword_loc: parse_location.context(winnow::error::StrContext::Label("CaseNode.case_keyword_loc")),
        end_keyword_loc: parse_location.context(winnow::error::StrContext::Label("CaseNode.end_keyword_loc")),
    }].parse_next(input)
    }
}

// 25
#[derive(Debug, Clone)]
pub struct ClassNode {
    locals: Vec<ConstantRef>,
    class_keyword_loc: Location,
    constant_path: NodeRef,
    inheritance_operator_loc: Option<Location>,
    superclass: Option<NodeRef>,
    body: Option<NodeRef>,
    end_keyword_loc: Location,
    name: ConstantRef,
}
impl ClassNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ClassNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ClassNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ClassNode.flags")),
        locals: length_repeat(parse_varuint, parse_constant).context(winnow::error::StrContext::Label("ClassNode.locals")),
        class_keyword_loc: parse_location.context(winnow::error::StrContext::Label("ClassNode.class_keyword_loc")),
        constant_path: parse_node.context(winnow::error::StrContext::Label("ClassNode.constant_path")),
        inheritance_operator_loc: parse_optional_location.context(winnow::error::StrContext::Label("ClassNode.inheritance_operator_loc")),
        superclass: parse_optional_node.context(winnow::error::StrContext::Label("ClassNode.superclass")),
        body: parse_optional_node.context(winnow::error::StrContext::Label("ClassNode.body")),
        end_keyword_loc: parse_location.context(winnow::error::StrContext::Label("ClassNode.end_keyword_loc")),
        name: parse_constant.context(winnow::error::StrContext::Label("ClassNode.name")),
    }].parse_next(input)
    }
}

// 26
#[derive(Debug, Clone)]
pub struct ClassVariableAndWriteNode {
    name: ConstantRef,
    name_loc: Location,
    operator_loc: Location,
    value: NodeRef,
}
impl ClassVariableAndWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ClassVariableAndWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ClassVariableAndWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ClassVariableAndWriteNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("ClassVariableAndWriteNode.name")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("ClassVariableAndWriteNode.name_loc")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("ClassVariableAndWriteNode.operator_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("ClassVariableAndWriteNode.value")),
    }].parse_next(input)
    }
}

// 27
#[derive(Debug, Clone)]
pub struct ClassVariableOperatorWriteNode {
    name: ConstantRef,
    name_loc: Location,
    binary_operator_loc: Location,
    value: NodeRef,
    binary_operator: ConstantRef,
}
impl ClassVariableOperatorWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ClassVariableOperatorWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ClassVariableOperatorWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ClassVariableOperatorWriteNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("ClassVariableOperatorWriteNode.name")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("ClassVariableOperatorWriteNode.name_loc")),
        binary_operator_loc: parse_location.context(winnow::error::StrContext::Label("ClassVariableOperatorWriteNode.binary_operator_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("ClassVariableOperatorWriteNode.value")),
        binary_operator: parse_constant.context(winnow::error::StrContext::Label("ClassVariableOperatorWriteNode.binary_operator")),
    }].parse_next(input)
    }
}

// 28
#[derive(Debug, Clone)]
pub struct ClassVariableOrWriteNode {
    name: ConstantRef,
    name_loc: Location,
    operator_loc: Location,
    value: NodeRef,
}
impl ClassVariableOrWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ClassVariableOrWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ClassVariableOrWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ClassVariableOrWriteNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("ClassVariableOrWriteNode.name")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("ClassVariableOrWriteNode.name_loc")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("ClassVariableOrWriteNode.operator_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("ClassVariableOrWriteNode.value")),
    }].parse_next(input)
    }
}

// 29
#[derive(Debug, Clone)]
pub struct ClassVariableReadNode {
    name: ConstantRef,
}
impl ClassVariableReadNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ClassVariableReadNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ClassVariableReadNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ClassVariableReadNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("ClassVariableReadNode.name")),
    }].parse_next(input)
    }
}

// 30
#[derive(Debug, Clone)]
pub struct ClassVariableTargetNode {
    name: ConstantRef,
}
impl ClassVariableTargetNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ClassVariableTargetNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ClassVariableTargetNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ClassVariableTargetNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("ClassVariableTargetNode.name")),
    }].parse_next(input)
    }
}

// 31
#[derive(Debug, Clone)]
pub struct ClassVariableWriteNode {
    name: ConstantRef,
    name_loc: Location,
    value: NodeRef,
    operator_loc: Location,
}
impl ClassVariableWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ClassVariableWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ClassVariableWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ClassVariableWriteNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("ClassVariableWriteNode.name")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("ClassVariableWriteNode.name_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("ClassVariableWriteNode.value")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("ClassVariableWriteNode.operator_loc")),
    }].parse_next(input)
    }
}

// 32
#[derive(Debug, Clone)]
pub struct ConstantAndWriteNode {
    name: ConstantRef,
    name_loc: Location,
    operator_loc: Location,
    value: NodeRef,
}
impl ConstantAndWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantAndWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantAndWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ConstantAndWriteNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("ConstantAndWriteNode.name")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("ConstantAndWriteNode.name_loc")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("ConstantAndWriteNode.operator_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("ConstantAndWriteNode.value")),
    }].parse_next(input)
    }
}

// 33
#[derive(Debug, Clone)]
pub struct ConstantOperatorWriteNode {
    name: ConstantRef,
    name_loc: Location,
    binary_operator_loc: Location,
    value: NodeRef,
    binary_operator: ConstantRef,
}
impl ConstantOperatorWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantOperatorWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantOperatorWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ConstantOperatorWriteNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("ConstantOperatorWriteNode.name")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("ConstantOperatorWriteNode.name_loc")),
        binary_operator_loc: parse_location.context(winnow::error::StrContext::Label("ConstantOperatorWriteNode.binary_operator_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("ConstantOperatorWriteNode.value")),
        binary_operator: parse_constant.context(winnow::error::StrContext::Label("ConstantOperatorWriteNode.binary_operator")),
    }].parse_next(input)
    }
}

// 34
#[derive(Debug, Clone)]
pub struct ConstantOrWriteNode {
    name: ConstantRef,
    name_loc: Location,
    operator_loc: Location,
    value: NodeRef,
}
impl ConstantOrWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantOrWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantOrWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ConstantOrWriteNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("ConstantOrWriteNode.name")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("ConstantOrWriteNode.name_loc")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("ConstantOrWriteNode.operator_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("ConstantOrWriteNode.value")),
    }].parse_next(input)
    }
}

// 35
#[derive(Debug, Clone)]
pub struct ConstantPathAndWriteNode {
    target: NodeRef,
    operator_loc: Location,
    value: NodeRef,
}
impl ConstantPathAndWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantPathAndWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantPathAndWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ConstantPathAndWriteNode.flags")),
        target: parse_node.context(winnow::error::StrContext::Label("ConstantPathAndWriteNode.target")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("ConstantPathAndWriteNode.operator_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("ConstantPathAndWriteNode.value")),
    }].parse_next(input)
    }
}

// 36
#[derive(Debug, Clone)]
pub struct ConstantPathNode {
    parent: Option<NodeRef>,
    name: Option<ConstantRef>,
    delimiter_loc: Location,
    name_loc: Location,
}
impl ConstantPathNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantPathNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantPathNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ConstantPathNode.flags")),
        parent: parse_optional_node.context(winnow::error::StrContext::Label("ConstantPathNode.parent")),
        name: parse_optional_constant.context(winnow::error::StrContext::Label("ConstantPathNode.name")),
        delimiter_loc: parse_location.context(winnow::error::StrContext::Label("ConstantPathNode.delimiter_loc")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("ConstantPathNode.name_loc")),
    }].parse_next(input)
    }
}

// 37
#[derive(Debug, Clone)]
pub struct ConstantPathOperatorWriteNode {
    target: NodeRef,
    binary_operator_loc: Location,
    value: NodeRef,
    binary_operator: ConstantRef,
}
impl ConstantPathOperatorWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantPathOperatorWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantPathOperatorWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ConstantPathOperatorWriteNode.flags")),
        target: parse_node.context(winnow::error::StrContext::Label("ConstantPathOperatorWriteNode.target")),
        binary_operator_loc: parse_location.context(winnow::error::StrContext::Label("ConstantPathOperatorWriteNode.binary_operator_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("ConstantPathOperatorWriteNode.value")),
        binary_operator: parse_constant.context(winnow::error::StrContext::Label("ConstantPathOperatorWriteNode.binary_operator")),
    }].parse_next(input)
    }
}

// 38
#[derive(Debug, Clone)]
pub struct ConstantPathOrWriteNode {
    target: NodeRef,
    operator_loc: Location,
    value: NodeRef,
}
impl ConstantPathOrWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantPathOrWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantPathOrWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ConstantPathOrWriteNode.flags")),
        target: parse_node.context(winnow::error::StrContext::Label("ConstantPathOrWriteNode.target")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("ConstantPathOrWriteNode.operator_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("ConstantPathOrWriteNode.value")),
    }].parse_next(input)
    }
}

// 39
#[derive(Debug, Clone)]
pub struct ConstantPathTargetNode {
    parent: Option<NodeRef>,
    name: Option<ConstantRef>,
    delimiter_loc: Location,
    name_loc: Location,
}
impl ConstantPathTargetNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantPathTargetNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantPathTargetNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ConstantPathTargetNode.flags")),
        parent: parse_optional_node.context(winnow::error::StrContext::Label("ConstantPathTargetNode.parent")),
        name: parse_optional_constant.context(winnow::error::StrContext::Label("ConstantPathTargetNode.name")),
        delimiter_loc: parse_location.context(winnow::error::StrContext::Label("ConstantPathTargetNode.delimiter_loc")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("ConstantPathTargetNode.name_loc")),
    }].parse_next(input)
    }
}

// 40
#[derive(Debug, Clone)]
pub struct ConstantPathWriteNode {
    target: NodeRef,
    operator_loc: Location,
    value: NodeRef,
}
impl ConstantPathWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantPathWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantPathWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ConstantPathWriteNode.flags")),
        target: parse_node.context(winnow::error::StrContext::Label("ConstantPathWriteNode.target")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("ConstantPathWriteNode.operator_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("ConstantPathWriteNode.value")),
    }].parse_next(input)
    }
}

// 41
#[derive(Debug, Clone)]
pub struct ConstantReadNode {
    name: ConstantRef,
}
impl ConstantReadNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantReadNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantReadNode{
            _: zero_flags.context(winnow::error::StrContext::Label("ConstantReadNode.flags")),
            name: parse_constant.context(winnow::error::StrContext::Label("ConstantReadNode.name")),
        }]
        .parse_next(input)
    }
}

// 42
#[derive(Debug, Clone)]
pub struct ConstantTargetNode {
    name: ConstantRef,
}
impl ConstantTargetNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantTargetNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantTargetNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ConstantTargetNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("ConstantTargetNode.name")),
    }].parse_next(input)
    }
}

// 43
#[derive(Debug, Clone)]
pub struct ConstantWriteNode {
    name: ConstantRef,
    name_loc: Location,
    value: NodeRef,
    operator_loc: Location,
}
impl ConstantWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ConstantWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ConstantWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ConstantWriteNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("ConstantWriteNode.name")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("ConstantWriteNode.name_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("ConstantWriteNode.value")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("ConstantWriteNode.operator_loc")),
    }].parse_next(input)
    }
}

// 44
#[derive(Debug, Clone)]
pub struct DefNode {
    name: ConstantRef,
    name_loc: Location,
    receiver: Option<NodeRef>,
    parameters: Option<NodeRef>,
    body: Option<NodeRef>,
    locals: Vec<ConstantRef>,
    def_keyword_loc: Location,
    operator_loc: Option<Location>,
    lparen_loc: Option<Location>,
    rparen_loc: Option<Location>,
    equal_loc: Option<Location>,
    end_keyword_loc: Option<Location>,
}
impl DefNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::DefNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![DefNode{
        _: winnow::binary::u32(winnow::binary::Endianness::Native),
        _: zero_flags.context(winnow::error::StrContext::Label("DefNode.flags")),
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

// 45
#[derive(Debug, Clone)]
pub struct DefinedNode {
    lparen_loc: Option<Location>,
    value: NodeRef,
    rparen_loc: Option<Location>,
    keyword_loc: Location,
}
impl DefinedNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::DefinedNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![DefinedNode{
        _: zero_flags.context(winnow::error::StrContext::Label("DefinedNode.flags")),
        lparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("DefinedNode.lparen_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("DefinedNode.value")),
        rparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("DefinedNode.rparen_loc")),
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("DefinedNode.keyword_loc")),
    }].parse_next(input)
    }
}

// 46
#[derive(Debug, Clone)]
pub struct ElseNode {
    else_keyword_loc: Location,
    statements: Option<NodeRef>,
    end_keyword_loc: Option<Location>,
}
impl ElseNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ElseNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ElseNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ElseNode.flags")),
        else_keyword_loc: parse_location.context(winnow::error::StrContext::Label("ElseNode.else_keyword_loc")),
        statements: parse_optional_node.context(winnow::error::StrContext::Label("ElseNode.statements")),
        end_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("ElseNode.end_keyword_loc")),
    }].parse_next(input)
    }
}

// 47
#[derive(Debug, Clone)]
pub struct EmbeddedStatementsNode {
    opening_loc: Location,
    statements: Option<NodeRef>,
    closing_loc: Location,
}
impl EmbeddedStatementsNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::EmbeddedStatementsNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![EmbeddedStatementsNode{
        _: zero_flags.context(winnow::error::StrContext::Label("EmbeddedStatementsNode.flags")),
        opening_loc: parse_location.context(winnow::error::StrContext::Label("EmbeddedStatementsNode.opening_loc")),
        statements: parse_optional_node.context(winnow::error::StrContext::Label("EmbeddedStatementsNode.statements")),
        closing_loc: parse_location.context(winnow::error::StrContext::Label("EmbeddedStatementsNode.closing_loc")),
    }].parse_next(input)
    }
}

// 48
#[derive(Debug, Clone)]
pub struct EmbeddedVariableNode {
    operator_loc: Location,
    variable: NodeRef,
}
impl EmbeddedVariableNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::EmbeddedVariableNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![EmbeddedVariableNode{
        _: zero_flags.context(winnow::error::StrContext::Label("EmbeddedVariableNode.flags")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("EmbeddedVariableNode.operator_loc")),
        variable: parse_node.context(winnow::error::StrContext::Label("EmbeddedVariableNode.variable")),
    }].parse_next(input)
    }
}

// 49
#[derive(Debug, Clone)]
pub struct EnsureNode {
    ensure_keyword_loc: Location,
    statements: Option<NodeRef>,
    end_keyword_loc: Location,
}
impl EnsureNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::EnsureNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![EnsureNode{
        _: zero_flags.context(winnow::error::StrContext::Label("EnsureNode.flags")),
        ensure_keyword_loc: parse_location.context(winnow::error::StrContext::Label("EnsureNode.ensure_keyword_loc")),
        statements: parse_optional_node.context(winnow::error::StrContext::Label("EnsureNode.statements")),
        end_keyword_loc: parse_location.context(winnow::error::StrContext::Label("EnsureNode.end_keyword_loc")),
    }].parse_next(input)
    }
}

// 50
#[derive(Debug, Clone)]
pub struct FalseNode {}
impl FalseNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::FalseNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        zero_flags
            .context(winnow::error::StrContext::Label("FalseNode.flags"))
            .value(Self {})
            .parse_next(input)
    }
}

// 51
#[derive(Debug, Clone)]
pub struct FindPatternNode {
    constant: Option<NodeRef>,
    left: NodeRef,
    requireds: Vec<NodeRef>,
    right: NodeRef,
    opening_loc: Option<Location>,
    closing_loc: Option<Location>,
}
impl FindPatternNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::FindPatternNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![FindPatternNode{
        _: zero_flags.context(winnow::error::StrContext::Label("FindPatternNode.flags")),
        constant: parse_optional_node.context(winnow::error::StrContext::Label("FindPatternNode.constant")),
        left: parse_node.context(winnow::error::StrContext::Label("FindPatternNode.left")),
        requireds: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("FindPatternNode.requireds")),
        right: parse_node.context(winnow::error::StrContext::Label("FindPatternNode.right")),
        opening_loc: parse_optional_location.context(winnow::error::StrContext::Label("FindPatternNode.opening_loc")),
        closing_loc: parse_optional_location.context(winnow::error::StrContext::Label("FindPatternNode.closing_loc")),
    }].parse_next(input)
    }
}

// 52
#[derive(Debug, Clone)]
pub struct FlipFlopNode {
    pub flags: enumflags2::BitFlags<RangeFlags>,
    left: Option<NodeRef>,
    right: Option<NodeRef>,
    operator_loc: Location,
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

// 53
#[derive(Debug, Clone)]
pub struct FloatNode {
    value: f64,
}
impl FloatNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::FloatNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![FloatNode{
            _: zero_flags.context(winnow::error::StrContext::Label("FloatNode.flags")),
            value: parse_double.context(winnow::error::StrContext::Label("FloatNode.value")),
        }]
        .parse_next(input)
    }
}

// 54
#[derive(Debug, Clone)]
pub struct ForNode {
    index: NodeRef,
    collection: NodeRef,
    statements: Option<NodeRef>,
    for_keyword_loc: Location,
    in_keyword_loc: Location,
    do_keyword_loc: Option<Location>,
    end_keyword_loc: Location,
}
impl ForNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ForNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ForNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ForNode.flags")),
        index: parse_node.context(winnow::error::StrContext::Label("ForNode.index")),
        collection: parse_node.context(winnow::error::StrContext::Label("ForNode.collection")),
        statements: parse_optional_node.context(winnow::error::StrContext::Label("ForNode.statements")),
        for_keyword_loc: parse_location.context(winnow::error::StrContext::Label("ForNode.for_keyword_loc")),
        in_keyword_loc: parse_location.context(winnow::error::StrContext::Label("ForNode.in_keyword_loc")),
        do_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("ForNode.do_keyword_loc")),
        end_keyword_loc: parse_location.context(winnow::error::StrContext::Label("ForNode.end_keyword_loc")),
    }].parse_next(input)
    }
}

// 55
#[derive(Debug, Clone)]
pub struct ForwardingArgumentsNode {}
impl ForwardingArgumentsNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ForwardingArgumentsNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        zero_flags
            .context(winnow::error::StrContext::Label(
                "ForwardingArgumentsNode.flags",
            ))
            .value(Self {})
            .parse_next(input)
    }
}

// 56
#[derive(Debug, Clone)]
pub struct ForwardingParameterNode {}
impl ForwardingParameterNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ForwardingParameterNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        zero_flags
            .context(winnow::error::StrContext::Label(
                "ForwardingParameterNode.flags",
            ))
            .value(Self {})
            .parse_next(input)
    }
}

// 57
#[derive(Debug, Clone)]
pub struct ForwardingSuperNode {
    block: Option<NodeRef>,
}
impl ForwardingSuperNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ForwardingSuperNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ForwardingSuperNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ForwardingSuperNode.flags")),
        block: parse_optional_node.context(winnow::error::StrContext::Label("ForwardingSuperNode.block")),
    }].parse_next(input)
    }
}

// 58
#[derive(Debug, Clone)]
pub struct GlobalVariableAndWriteNode {
    name: ConstantRef,
    name_loc: Location,
    operator_loc: Location,
    value: NodeRef,
}
impl GlobalVariableAndWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::GlobalVariableAndWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![GlobalVariableAndWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("GlobalVariableAndWriteNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("GlobalVariableAndWriteNode.name")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("GlobalVariableAndWriteNode.name_loc")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("GlobalVariableAndWriteNode.operator_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("GlobalVariableAndWriteNode.value")),
    }].parse_next(input)
    }
}

// 59
#[derive(Debug, Clone)]
pub struct GlobalVariableOperatorWriteNode {
    name: ConstantRef,
    name_loc: Location,
    binary_operator_loc: Location,
    value: NodeRef,
    binary_operator: ConstantRef,
}
impl GlobalVariableOperatorWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::GlobalVariableOperatorWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![GlobalVariableOperatorWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("GlobalVariableOperatorWriteNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("GlobalVariableOperatorWriteNode.name")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("GlobalVariableOperatorWriteNode.name_loc")),
        binary_operator_loc: parse_location.context(winnow::error::StrContext::Label("GlobalVariableOperatorWriteNode.binary_operator_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("GlobalVariableOperatorWriteNode.value")),
        binary_operator: parse_constant.context(winnow::error::StrContext::Label("GlobalVariableOperatorWriteNode.binary_operator")),
    }].parse_next(input)
    }
}

// 60
#[derive(Debug, Clone)]
pub struct GlobalVariableOrWriteNode {
    name: ConstantRef,
    name_loc: Location,
    operator_loc: Location,
    value: NodeRef,
}
impl GlobalVariableOrWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::GlobalVariableOrWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![GlobalVariableOrWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("GlobalVariableOrWriteNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("GlobalVariableOrWriteNode.name")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("GlobalVariableOrWriteNode.name_loc")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("GlobalVariableOrWriteNode.operator_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("GlobalVariableOrWriteNode.value")),
    }].parse_next(input)
    }
}

// 61
#[derive(Debug, Clone)]
pub struct GlobalVariableReadNode {
    name: ConstantRef,
}
impl GlobalVariableReadNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::GlobalVariableReadNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![GlobalVariableReadNode{
        _: zero_flags.context(winnow::error::StrContext::Label("GlobalVariableReadNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("GlobalVariableReadNode.name")),
    }].parse_next(input)
    }
}

// 62
#[derive(Debug, Clone)]
pub struct GlobalVariableTargetNode {
    name: ConstantRef,
}
impl GlobalVariableTargetNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::GlobalVariableTargetNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![GlobalVariableTargetNode{
        _: zero_flags.context(winnow::error::StrContext::Label("GlobalVariableTargetNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("GlobalVariableTargetNode.name")),
    }].parse_next(input)
    }
}

// 63
#[derive(Debug, Clone)]
pub struct GlobalVariableWriteNode {
    name: ConstantRef,
    name_loc: Location,
    value: NodeRef,
    operator_loc: Location,
}
impl GlobalVariableWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::GlobalVariableWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![GlobalVariableWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("GlobalVariableWriteNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("GlobalVariableWriteNode.name")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("GlobalVariableWriteNode.name_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("GlobalVariableWriteNode.value")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("GlobalVariableWriteNode.operator_loc")),
    }].parse_next(input)
    }
}

// 64
#[derive(Debug, Clone)]
pub struct HashNode {
    opening_loc: Location,
    elements: Vec<NodeRef>,
    closing_loc: Location,
}
impl HashNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::HashNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![HashNode{
        _: zero_flags.context(winnow::error::StrContext::Label("HashNode.flags")),
        opening_loc: parse_location.context(winnow::error::StrContext::Label("HashNode.opening_loc")),
        elements: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("HashNode.elements")),
        closing_loc: parse_location.context(winnow::error::StrContext::Label("HashNode.closing_loc")),
    }].parse_next(input)
    }
}

// 65
#[derive(Debug, Clone)]
pub struct HashPatternNode {
    constant: Option<NodeRef>,
    elements: Vec<NodeRef>,
    rest: Option<NodeRef>,
    opening_loc: Option<Location>,
    closing_loc: Option<Location>,
}
impl HashPatternNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::HashPatternNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![HashPatternNode{
        _: zero_flags.context(winnow::error::StrContext::Label("HashPatternNode.flags")),
        constant: parse_optional_node.context(winnow::error::StrContext::Label("HashPatternNode.constant")),
        elements: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("HashPatternNode.elements")),
        rest: parse_optional_node.context(winnow::error::StrContext::Label("HashPatternNode.rest")),
        opening_loc: parse_optional_location.context(winnow::error::StrContext::Label("HashPatternNode.opening_loc")),
        closing_loc: parse_optional_location.context(winnow::error::StrContext::Label("HashPatternNode.closing_loc")),
    }].parse_next(input)
    }
}

// 66
#[derive(Debug, Clone)]
pub struct IfNode {
    if_keyword_loc: Option<Location>,
    predicate: NodeRef,
    then_keyword_loc: Option<Location>,
    statements: Option<NodeRef>,
    subsequent: Option<NodeRef>,
    end_keyword_loc: Option<Location>,
}
impl IfNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::IfNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![IfNode{
        _: zero_flags.context(winnow::error::StrContext::Label("IfNode.flags")),
        if_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("IfNode.if_keyword_loc")),
        predicate: parse_node.context(winnow::error::StrContext::Label("IfNode.predicate")),
        then_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("IfNode.then_keyword_loc")),
        statements: parse_optional_node.context(winnow::error::StrContext::Label("IfNode.statements")),
        subsequent: parse_optional_node.context(winnow::error::StrContext::Label("IfNode.subsequent")),
        end_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("IfNode.end_keyword_loc")),
    }].parse_next(input)
    }
}

// 67
#[derive(Debug, Clone)]
pub struct ImaginaryNode {
    numeric: NodeRef,
}
impl ImaginaryNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ImaginaryNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ImaginaryNode{
            _: zero_flags.context(winnow::error::StrContext::Label("ImaginaryNode.flags")),
            numeric: parse_node.context(winnow::error::StrContext::Label("ImaginaryNode.numeric")),
        }]
        .parse_next(input)
    }
}

// 68
#[derive(Debug, Clone)]
pub struct ImplicitNode {
    value: NodeRef,
}
impl ImplicitNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ImplicitNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ImplicitNode{
            _: zero_flags.context(winnow::error::StrContext::Label("ImplicitNode.flags")),
            value: parse_node.context(winnow::error::StrContext::Label("ImplicitNode.value")),
        }]
        .parse_next(input)
    }
}

// 69
#[derive(Debug, Clone)]
pub struct ImplicitRestNode {}
impl ImplicitRestNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ImplicitRestNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        zero_flags
            .context(winnow::error::StrContext::Label("ImplicitRestNode.flags"))
            .value(Self {})
            .parse_next(input)
    }
}

// 70
#[derive(Debug, Clone)]
pub struct InNode {
    pattern: NodeRef,
    statements: Option<NodeRef>,
    in_loc: Location,
    then_loc: Option<Location>,
}
impl InNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InNode{
        _: zero_flags.context(winnow::error::StrContext::Label("InNode.flags")),
        pattern: parse_node.context(winnow::error::StrContext::Label("InNode.pattern")),
        statements: parse_optional_node.context(winnow::error::StrContext::Label("InNode.statements")),
        in_loc: parse_location.context(winnow::error::StrContext::Label("InNode.in_loc")),
        then_loc: parse_optional_location.context(winnow::error::StrContext::Label("InNode.then_loc")),
    }].parse_next(input)
    }
}

// 71
#[derive(Debug, Clone)]
pub struct IndexAndWriteNode {
    pub flags: enumflags2::BitFlags<CallNodeFlags>,
    receiver: Option<NodeRef>,
    call_operator_loc: Option<Location>,
    opening_loc: Location,
    arguments: Option<NodeRef>,
    closing_loc: Location,
    block: Option<NodeRef>,
    operator_loc: Location,
    value: NodeRef,
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

// 72
#[derive(Debug, Clone)]
pub struct IndexOperatorWriteNode {
    pub flags: enumflags2::BitFlags<CallNodeFlags>,
    receiver: Option<NodeRef>,
    call_operator_loc: Option<Location>,
    opening_loc: Location,
    arguments: Option<NodeRef>,
    closing_loc: Location,
    block: Option<NodeRef>,
    binary_operator: ConstantRef,
    binary_operator_loc: Location,
    value: NodeRef,
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

// 73
#[derive(Debug, Clone)]
pub struct IndexOrWriteNode {
    pub flags: enumflags2::BitFlags<CallNodeFlags>,
    receiver: Option<NodeRef>,
    call_operator_loc: Option<Location>,
    opening_loc: Location,
    arguments: Option<NodeRef>,
    closing_loc: Location,
    block: Option<NodeRef>,
    operator_loc: Location,
    value: NodeRef,
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

// 74
#[derive(Debug, Clone)]
pub struct IndexTargetNode {
    pub flags: enumflags2::BitFlags<CallNodeFlags>,
    receiver: NodeRef,
    opening_loc: Location,
    arguments: Option<NodeRef>,
    closing_loc: Location,
    block: Option<NodeRef>,
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

// 75
#[derive(Debug, Clone)]
pub struct InstanceVariableAndWriteNode {
    name: ConstantRef,
    name_loc: Location,
    operator_loc: Location,
    value: NodeRef,
}
impl InstanceVariableAndWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InstanceVariableAndWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InstanceVariableAndWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("InstanceVariableAndWriteNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("InstanceVariableAndWriteNode.name")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("InstanceVariableAndWriteNode.name_loc")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("InstanceVariableAndWriteNode.operator_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("InstanceVariableAndWriteNode.value")),
    }].parse_next(input)
    }
}

// 76
#[derive(Debug, Clone)]
pub struct InstanceVariableOperatorWriteNode {
    name: ConstantRef,
    name_loc: Location,
    binary_operator_loc: Location,
    value: NodeRef,
    binary_operator: ConstantRef,
}
impl InstanceVariableOperatorWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InstanceVariableOperatorWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InstanceVariableOperatorWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("InstanceVariableOperatorWriteNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("InstanceVariableOperatorWriteNode.name")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("InstanceVariableOperatorWriteNode.name_loc")),
        binary_operator_loc: parse_location.context(winnow::error::StrContext::Label("InstanceVariableOperatorWriteNode.binary_operator_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("InstanceVariableOperatorWriteNode.value")),
        binary_operator: parse_constant.context(winnow::error::StrContext::Label("InstanceVariableOperatorWriteNode.binary_operator")),
    }].parse_next(input)
    }
}

// 77
#[derive(Debug, Clone)]
pub struct InstanceVariableOrWriteNode {
    name: ConstantRef,
    name_loc: Location,
    operator_loc: Location,
    value: NodeRef,
}
impl InstanceVariableOrWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InstanceVariableOrWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InstanceVariableOrWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("InstanceVariableOrWriteNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("InstanceVariableOrWriteNode.name")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("InstanceVariableOrWriteNode.name_loc")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("InstanceVariableOrWriteNode.operator_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("InstanceVariableOrWriteNode.value")),
    }].parse_next(input)
    }
}

// 78
#[derive(Debug, Clone)]
pub struct InstanceVariableReadNode {
    name: ConstantRef,
}
impl InstanceVariableReadNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InstanceVariableReadNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InstanceVariableReadNode{
        _: zero_flags.context(winnow::error::StrContext::Label("InstanceVariableReadNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("InstanceVariableReadNode.name")),
    }].parse_next(input)
    }
}

// 79
#[derive(Debug, Clone)]
pub struct InstanceVariableTargetNode {
    name: ConstantRef,
}
impl InstanceVariableTargetNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InstanceVariableTargetNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InstanceVariableTargetNode{
        _: zero_flags.context(winnow::error::StrContext::Label("InstanceVariableTargetNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("InstanceVariableTargetNode.name")),
    }].parse_next(input)
    }
}

// 80
#[derive(Debug, Clone)]
pub struct InstanceVariableWriteNode {
    name: ConstantRef,
    name_loc: Location,
    value: NodeRef,
    operator_loc: Location,
}
impl InstanceVariableWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InstanceVariableWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InstanceVariableWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("InstanceVariableWriteNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("InstanceVariableWriteNode.name")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("InstanceVariableWriteNode.name_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("InstanceVariableWriteNode.value")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("InstanceVariableWriteNode.operator_loc")),
    }].parse_next(input)
    }
}

// 81
#[derive(Debug, Clone)]
pub struct IntegerNode {
    pub flags: enumflags2::BitFlags<IntegerBaseFlags>,
    value: Integer,
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

// 82
#[derive(Debug, Clone)]
pub struct InterpolatedMatchLastLineNode {
    pub flags: enumflags2::BitFlags<RegularExpressionFlags>,
    opening_loc: Location,
    parts: Vec<NodeRef>,
    closing_loc: Location,
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

// 83
#[derive(Debug, Clone)]
pub struct InterpolatedRegularExpressionNode {
    pub flags: enumflags2::BitFlags<RegularExpressionFlags>,
    opening_loc: Location,
    parts: Vec<NodeRef>,
    closing_loc: Location,
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

// 84
#[derive(Debug, Clone)]
pub struct InterpolatedStringNode {
    pub flags: enumflags2::BitFlags<InterpolatedStringNodeFlags>,
    opening_loc: Option<Location>,
    parts: Vec<NodeRef>,
    closing_loc: Option<Location>,
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

// 85
#[derive(Debug, Clone)]
pub struct InterpolatedSymbolNode {
    opening_loc: Option<Location>,
    parts: Vec<NodeRef>,
    closing_loc: Option<Location>,
}
impl InterpolatedSymbolNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InterpolatedSymbolNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InterpolatedSymbolNode{
        _: zero_flags.context(winnow::error::StrContext::Label("InterpolatedSymbolNode.flags")),
        opening_loc: parse_optional_location.context(winnow::error::StrContext::Label("InterpolatedSymbolNode.opening_loc")),
        parts: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("InterpolatedSymbolNode.parts")),
        closing_loc: parse_optional_location.context(winnow::error::StrContext::Label("InterpolatedSymbolNode.closing_loc")),
    }].parse_next(input)
    }
}

// 86
#[derive(Debug, Clone)]
pub struct InterpolatedXStringNode {
    opening_loc: Location,
    parts: Vec<NodeRef>,
    closing_loc: Location,
}
impl InterpolatedXStringNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::InterpolatedXStringNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![InterpolatedXStringNode{
        _: zero_flags.context(winnow::error::StrContext::Label("InterpolatedXStringNode.flags")),
        opening_loc: parse_location.context(winnow::error::StrContext::Label("InterpolatedXStringNode.opening_loc")),
        parts: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("InterpolatedXStringNode.parts")),
        closing_loc: parse_location.context(winnow::error::StrContext::Label("InterpolatedXStringNode.closing_loc")),
    }].parse_next(input)
    }
}

// 87
#[derive(Debug, Clone)]
pub struct ItLocalVariableReadNode {}
impl ItLocalVariableReadNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ItLocalVariableReadNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        zero_flags
            .context(winnow::error::StrContext::Label(
                "ItLocalVariableReadNode.flags",
            ))
            .value(Self {})
            .parse_next(input)
    }
}

// 88
#[derive(Debug, Clone)]
pub struct ItParametersNode {}
impl ItParametersNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ItParametersNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        zero_flags
            .context(winnow::error::StrContext::Label("ItParametersNode.flags"))
            .value(Self {})
            .parse_next(input)
    }
}

// 89
#[derive(Debug, Clone)]
pub struct KeywordHashNode {
    pub flags: enumflags2::BitFlags<KeywordHashNodeFlags>,
    elements: Vec<NodeRef>,
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

// 90
#[derive(Debug, Clone)]
pub struct KeywordRestParameterNode {
    pub flags: enumflags2::BitFlags<ParameterFlags>,
    name: Option<ConstantRef>,
    name_loc: Option<Location>,
    operator_loc: Location,
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

// 91
#[derive(Debug, Clone)]
pub struct LambdaNode {
    locals: Vec<ConstantRef>,
    operator_loc: Location,
    opening_loc: Location,
    closing_loc: Location,
    parameters: Option<NodeRef>,
    body: Option<NodeRef>,
}
impl LambdaNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::LambdaNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![LambdaNode{
        _: zero_flags.context(winnow::error::StrContext::Label("LambdaNode.flags")),
        locals: length_repeat(parse_varuint, parse_constant).context(winnow::error::StrContext::Label("LambdaNode.locals")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("LambdaNode.operator_loc")),
        opening_loc: parse_location.context(winnow::error::StrContext::Label("LambdaNode.opening_loc")),
        closing_loc: parse_location.context(winnow::error::StrContext::Label("LambdaNode.closing_loc")),
        parameters: parse_optional_node.context(winnow::error::StrContext::Label("LambdaNode.parameters")),
        body: parse_optional_node.context(winnow::error::StrContext::Label("LambdaNode.body")),
    }].parse_next(input)
    }
}

// 92
#[derive(Debug, Clone)]
pub struct LocalVariableAndWriteNode {
    name_loc: Location,
    operator_loc: Location,
    value: NodeRef,
    name: ConstantRef,
    depth: u32,
}
impl LocalVariableAndWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::LocalVariableAndWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![LocalVariableAndWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("LocalVariableAndWriteNode.flags")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("LocalVariableAndWriteNode.name_loc")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("LocalVariableAndWriteNode.operator_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("LocalVariableAndWriteNode.value")),
        name: parse_constant.context(winnow::error::StrContext::Label("LocalVariableAndWriteNode.name")),
        depth: parse_varuint.context(winnow::error::StrContext::Label("LocalVariableAndWriteNode.depth")),
    }].parse_next(input)
    }
}

// 93
#[derive(Debug, Clone)]
pub struct LocalVariableOperatorWriteNode {
    name_loc: Location,
    binary_operator_loc: Location,
    value: NodeRef,
    name: ConstantRef,
    binary_operator: ConstantRef,
    depth: u32,
}
impl LocalVariableOperatorWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::LocalVariableOperatorWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![LocalVariableOperatorWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("LocalVariableOperatorWriteNode.flags")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("LocalVariableOperatorWriteNode.name_loc")),
        binary_operator_loc: parse_location.context(winnow::error::StrContext::Label("LocalVariableOperatorWriteNode.binary_operator_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("LocalVariableOperatorWriteNode.value")),
        name: parse_constant.context(winnow::error::StrContext::Label("LocalVariableOperatorWriteNode.name")),
        binary_operator: parse_constant.context(winnow::error::StrContext::Label("LocalVariableOperatorWriteNode.binary_operator")),
        depth: parse_varuint.context(winnow::error::StrContext::Label("LocalVariableOperatorWriteNode.depth")),
    }].parse_next(input)
    }
}

// 94
#[derive(Debug, Clone)]
pub struct LocalVariableOrWriteNode {
    name_loc: Location,
    operator_loc: Location,
    value: NodeRef,
    name: ConstantRef,
    depth: u32,
}
impl LocalVariableOrWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::LocalVariableOrWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![LocalVariableOrWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("LocalVariableOrWriteNode.flags")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("LocalVariableOrWriteNode.name_loc")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("LocalVariableOrWriteNode.operator_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("LocalVariableOrWriteNode.value")),
        name: parse_constant.context(winnow::error::StrContext::Label("LocalVariableOrWriteNode.name")),
        depth: parse_varuint.context(winnow::error::StrContext::Label("LocalVariableOrWriteNode.depth")),
    }].parse_next(input)
    }
}

// 95
#[derive(Debug, Clone)]
pub struct LocalVariableReadNode {
    name: ConstantRef,
    depth: u32,
}
impl LocalVariableReadNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::LocalVariableReadNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![LocalVariableReadNode{
        _: zero_flags.context(winnow::error::StrContext::Label("LocalVariableReadNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("LocalVariableReadNode.name")),
        depth: parse_varuint.context(winnow::error::StrContext::Label("LocalVariableReadNode.depth")),
    }].parse_next(input)
    }
}

// 96
#[derive(Debug, Clone)]
pub struct LocalVariableTargetNode {
    name: ConstantRef,
    depth: u32,
}
impl LocalVariableTargetNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::LocalVariableTargetNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![LocalVariableTargetNode{
        _: zero_flags.context(winnow::error::StrContext::Label("LocalVariableTargetNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("LocalVariableTargetNode.name")),
        depth: parse_varuint.context(winnow::error::StrContext::Label("LocalVariableTargetNode.depth")),
    }].parse_next(input)
    }
}

// 97
#[derive(Debug, Clone)]
pub struct LocalVariableWriteNode {
    name: ConstantRef,
    depth: u32,
    name_loc: Location,
    value: NodeRef,
    operator_loc: Location,
}
impl LocalVariableWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::LocalVariableWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![LocalVariableWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("LocalVariableWriteNode.flags")),
        name: parse_constant.context(winnow::error::StrContext::Label("LocalVariableWriteNode.name")),
        depth: parse_varuint.context(winnow::error::StrContext::Label("LocalVariableWriteNode.depth")),
        name_loc: parse_location.context(winnow::error::StrContext::Label("LocalVariableWriteNode.name_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("LocalVariableWriteNode.value")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("LocalVariableWriteNode.operator_loc")),
    }].parse_next(input)
    }
}

// 98
#[derive(Debug, Clone)]
pub struct MatchLastLineNode {
    pub flags: enumflags2::BitFlags<RegularExpressionFlags>,
    opening_loc: Location,
    content_loc: Location,
    closing_loc: Location,
    unescaped: String,
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

// 99
#[derive(Debug, Clone)]
pub struct MatchPredicateNode {
    value: NodeRef,
    pattern: NodeRef,
    operator_loc: Location,
}
impl MatchPredicateNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::MatchPredicateNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![MatchPredicateNode{
        _: zero_flags.context(winnow::error::StrContext::Label("MatchPredicateNode.flags")),
        value: parse_node.context(winnow::error::StrContext::Label("MatchPredicateNode.value")),
        pattern: parse_node.context(winnow::error::StrContext::Label("MatchPredicateNode.pattern")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("MatchPredicateNode.operator_loc")),
    }].parse_next(input)
    }
}

// 100
#[derive(Debug, Clone)]
pub struct MatchRequiredNode {
    value: NodeRef,
    pattern: NodeRef,
    operator_loc: Location,
}
impl MatchRequiredNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::MatchRequiredNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![MatchRequiredNode{
        _: zero_flags.context(winnow::error::StrContext::Label("MatchRequiredNode.flags")),
        value: parse_node.context(winnow::error::StrContext::Label("MatchRequiredNode.value")),
        pattern: parse_node.context(winnow::error::StrContext::Label("MatchRequiredNode.pattern")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("MatchRequiredNode.operator_loc")),
    }].parse_next(input)
    }
}

// 101
#[derive(Debug, Clone)]
pub struct MatchWriteNode {
    call: NodeRef,
    targets: Vec<NodeRef>,
}
impl MatchWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::MatchWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![MatchWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("MatchWriteNode.flags")),
        call: parse_node.context(winnow::error::StrContext::Label("MatchWriteNode.call")),
        targets: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("MatchWriteNode.targets")),
    }].parse_next(input)
    }
}

// 102
#[derive(Debug, Clone)]
pub struct MissingNode {}
impl MissingNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::MissingNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        zero_flags
            .context(winnow::error::StrContext::Label("MissingNode.flags"))
            .value(Self {})
            .parse_next(input)
    }
}

// 103
#[derive(Debug, Clone)]
pub struct ModuleNode {
    locals: Vec<ConstantRef>,
    module_keyword_loc: Location,
    constant_path: NodeRef,
    body: Option<NodeRef>,
    end_keyword_loc: Location,
    name: ConstantRef,
}
impl ModuleNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ModuleNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ModuleNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ModuleNode.flags")),
        locals: length_repeat(parse_varuint, parse_constant).context(winnow::error::StrContext::Label("ModuleNode.locals")),
        module_keyword_loc: parse_location.context(winnow::error::StrContext::Label("ModuleNode.module_keyword_loc")),
        constant_path: parse_node.context(winnow::error::StrContext::Label("ModuleNode.constant_path")),
        body: parse_optional_node.context(winnow::error::StrContext::Label("ModuleNode.body")),
        end_keyword_loc: parse_location.context(winnow::error::StrContext::Label("ModuleNode.end_keyword_loc")),
        name: parse_constant.context(winnow::error::StrContext::Label("ModuleNode.name")),
    }].parse_next(input)
    }
}

// 104
#[derive(Debug, Clone)]
pub struct MultiTargetNode {
    lefts: Vec<NodeRef>,
    rest: Option<NodeRef>,
    rights: Vec<NodeRef>,
    lparen_loc: Option<Location>,
    rparen_loc: Option<Location>,
}
impl MultiTargetNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::MultiTargetNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![MultiTargetNode{
        _: zero_flags.context(winnow::error::StrContext::Label("MultiTargetNode.flags")),
        lefts: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("MultiTargetNode.lefts")),
        rest: parse_optional_node.context(winnow::error::StrContext::Label("MultiTargetNode.rest")),
        rights: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("MultiTargetNode.rights")),
        lparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("MultiTargetNode.lparen_loc")),
        rparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("MultiTargetNode.rparen_loc")),
    }].parse_next(input)
    }
}

// 105
#[derive(Debug, Clone)]
pub struct MultiWriteNode {
    lefts: Vec<NodeRef>,
    rest: Option<NodeRef>,
    rights: Vec<NodeRef>,
    lparen_loc: Option<Location>,
    rparen_loc: Option<Location>,
    operator_loc: Location,
    value: NodeRef,
}
impl MultiWriteNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::MultiWriteNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![MultiWriteNode{
        _: zero_flags.context(winnow::error::StrContext::Label("MultiWriteNode.flags")),
        lefts: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("MultiWriteNode.lefts")),
        rest: parse_optional_node.context(winnow::error::StrContext::Label("MultiWriteNode.rest")),
        rights: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("MultiWriteNode.rights")),
        lparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("MultiWriteNode.lparen_loc")),
        rparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("MultiWriteNode.rparen_loc")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("MultiWriteNode.operator_loc")),
        value: parse_node.context(winnow::error::StrContext::Label("MultiWriteNode.value")),
    }].parse_next(input)
    }
}

// 106
#[derive(Debug, Clone)]
pub struct NextNode {
    arguments: Option<NodeRef>,
    keyword_loc: Location,
}
impl NextNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::NextNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![NextNode{
        _: zero_flags.context(winnow::error::StrContext::Label("NextNode.flags")),
        arguments: parse_optional_node.context(winnow::error::StrContext::Label("NextNode.arguments")),
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("NextNode.keyword_loc")),
    }].parse_next(input)
    }
}

// 107
#[derive(Debug, Clone)]
pub struct NilNode {}
impl NilNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::NilNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        zero_flags
            .context(winnow::error::StrContext::Label("NilNode.flags"))
            .value(Self {})
            .parse_next(input)
    }
}

// 108
#[derive(Debug, Clone)]
pub struct NoKeywordsParameterNode {
    operator_loc: Location,
    keyword_loc: Location,
}
impl NoKeywordsParameterNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::NoKeywordsParameterNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![NoKeywordsParameterNode{
        _: zero_flags.context(winnow::error::StrContext::Label("NoKeywordsParameterNode.flags")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("NoKeywordsParameterNode.operator_loc")),
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("NoKeywordsParameterNode.keyword_loc")),
    }].parse_next(input)
    }
}

// 109
#[derive(Debug, Clone)]
pub struct NumberedParametersNode {
    maximum: u8,
}
impl NumberedParametersNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::NumberedParametersNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![NumberedParametersNode{
        _: zero_flags.context(winnow::error::StrContext::Label("NumberedParametersNode.flags")),
        maximum: winnow::binary::u8.context(winnow::error::StrContext::Label("NumberedParametersNode.maximum")),
    }].parse_next(input)
    }
}

// 110
#[derive(Debug, Clone)]
pub struct NumberedReferenceReadNode {
    number: u32,
}
impl NumberedReferenceReadNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::NumberedReferenceReadNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![NumberedReferenceReadNode{
        _: zero_flags.context(winnow::error::StrContext::Label("NumberedReferenceReadNode.flags")),
        number: parse_varuint.context(winnow::error::StrContext::Label("NumberedReferenceReadNode.number")),
    }].parse_next(input)
    }
}

// 111
#[derive(Debug, Clone)]
pub struct OptionalKeywordParameterNode {
    pub flags: enumflags2::BitFlags<ParameterFlags>,
    name: ConstantRef,
    name_loc: Location,
    value: NodeRef,
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

// 112
#[derive(Debug, Clone)]
pub struct OptionalParameterNode {
    pub flags: enumflags2::BitFlags<ParameterFlags>,
    name: ConstantRef,
    name_loc: Location,
    operator_loc: Location,
    value: NodeRef,
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

// 113
#[derive(Debug, Clone)]
pub struct OrNode {
    left: NodeRef,
    right: NodeRef,
    operator_loc: Location,
}
impl OrNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::OrNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![OrNode{
        _: zero_flags.context(winnow::error::StrContext::Label("OrNode.flags")),
        left: parse_node.context(winnow::error::StrContext::Label("OrNode.left")),
        right: parse_node.context(winnow::error::StrContext::Label("OrNode.right")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("OrNode.operator_loc")),
    }].parse_next(input)
    }
}

// 114
#[derive(Debug, Clone)]
pub struct ParametersNode {
    requireds: Vec<NodeRef>,
    optionals: Vec<NodeRef>,
    rest: Option<NodeRef>,
    posts: Vec<NodeRef>,
    keywords: Vec<NodeRef>,
    keyword_rest: Option<NodeRef>,
    block: Option<NodeRef>,
}
impl ParametersNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ParametersNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ParametersNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ParametersNode.flags")),
        requireds: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("ParametersNode.requireds")),
        optionals: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("ParametersNode.optionals")),
        rest: parse_optional_node.context(winnow::error::StrContext::Label("ParametersNode.rest")),
        posts: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("ParametersNode.posts")),
        keywords: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("ParametersNode.keywords")),
        keyword_rest: parse_optional_node.context(winnow::error::StrContext::Label("ParametersNode.keyword_rest")),
        block: parse_optional_node.context(winnow::error::StrContext::Label("ParametersNode.block")),
    }].parse_next(input)
    }
}

// 115
#[derive(Debug, Clone)]
pub struct ParenthesesNode {
    body: Option<NodeRef>,
    opening_loc: Location,
    closing_loc: Location,
}
impl ParenthesesNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ParenthesesNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ParenthesesNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ParenthesesNode.flags")),
        body: parse_optional_node.context(winnow::error::StrContext::Label("ParenthesesNode.body")),
        opening_loc: parse_location.context(winnow::error::StrContext::Label("ParenthesesNode.opening_loc")),
        closing_loc: parse_location.context(winnow::error::StrContext::Label("ParenthesesNode.closing_loc")),
    }].parse_next(input)
    }
}

// 116
#[derive(Debug, Clone)]
pub struct PinnedExpressionNode {
    expression: NodeRef,
    operator_loc: Location,
    lparen_loc: Location,
    rparen_loc: Location,
}
impl PinnedExpressionNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::PinnedExpressionNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![PinnedExpressionNode{
        _: zero_flags.context(winnow::error::StrContext::Label("PinnedExpressionNode.flags")),
        expression: parse_node.context(winnow::error::StrContext::Label("PinnedExpressionNode.expression")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("PinnedExpressionNode.operator_loc")),
        lparen_loc: parse_location.context(winnow::error::StrContext::Label("PinnedExpressionNode.lparen_loc")),
        rparen_loc: parse_location.context(winnow::error::StrContext::Label("PinnedExpressionNode.rparen_loc")),
    }].parse_next(input)
    }
}

// 117
#[derive(Debug, Clone)]
pub struct PinnedVariableNode {
    variable: NodeRef,
    operator_loc: Location,
}
impl PinnedVariableNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::PinnedVariableNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![PinnedVariableNode{
        _: zero_flags.context(winnow::error::StrContext::Label("PinnedVariableNode.flags")),
        variable: parse_node.context(winnow::error::StrContext::Label("PinnedVariableNode.variable")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("PinnedVariableNode.operator_loc")),
    }].parse_next(input)
    }
}

// 118
#[derive(Debug, Clone)]
pub struct PostExecutionNode {
    statements: Option<NodeRef>,
    keyword_loc: Location,
    opening_loc: Location,
    closing_loc: Location,
}
impl PostExecutionNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::PostExecutionNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![PostExecutionNode{
        _: zero_flags.context(winnow::error::StrContext::Label("PostExecutionNode.flags")),
        statements: parse_optional_node.context(winnow::error::StrContext::Label("PostExecutionNode.statements")),
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("PostExecutionNode.keyword_loc")),
        opening_loc: parse_location.context(winnow::error::StrContext::Label("PostExecutionNode.opening_loc")),
        closing_loc: parse_location.context(winnow::error::StrContext::Label("PostExecutionNode.closing_loc")),
    }].parse_next(input)
    }
}

// 119
#[derive(Debug, Clone)]
pub struct PreExecutionNode {
    statements: Option<NodeRef>,
    keyword_loc: Location,
    opening_loc: Location,
    closing_loc: Location,
}
impl PreExecutionNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::PreExecutionNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![PreExecutionNode{
        _: zero_flags.context(winnow::error::StrContext::Label("PreExecutionNode.flags")),
        statements: parse_optional_node.context(winnow::error::StrContext::Label("PreExecutionNode.statements")),
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("PreExecutionNode.keyword_loc")),
        opening_loc: parse_location.context(winnow::error::StrContext::Label("PreExecutionNode.opening_loc")),
        closing_loc: parse_location.context(winnow::error::StrContext::Label("PreExecutionNode.closing_loc")),
    }].parse_next(input)
    }
}

// 120
#[derive(Debug, Clone)]
pub struct ProgramNode {
    locals: Vec<ConstantRef>,
    statements: NodeRef,
}
impl ProgramNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ProgramNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ProgramNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ProgramNode.flags")),
        locals: length_repeat(parse_varuint, parse_constant).context(winnow::error::StrContext::Label("ProgramNode.locals")),
        statements: parse_node.context(winnow::error::StrContext::Label("ProgramNode.statements")),
    }].parse_next(input)
    }
}

// 121
#[derive(Debug, Clone)]
pub struct RangeNode {
    pub flags: enumflags2::BitFlags<RangeFlags>,
    left: Option<NodeRef>,
    right: Option<NodeRef>,
    operator_loc: Location,
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

// 122
#[derive(Debug, Clone)]
pub struct RationalNode {
    pub flags: enumflags2::BitFlags<IntegerBaseFlags>,
    numerator: Integer,
    denominator: Integer,
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

// 123
#[derive(Debug, Clone)]
pub struct RedoNode {}
impl RedoNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::RedoNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        zero_flags
            .context(winnow::error::StrContext::Label("RedoNode.flags"))
            .value(Self {})
            .parse_next(input)
    }
}

// 124
#[derive(Debug, Clone)]
pub struct RegularExpressionNode {
    pub flags: enumflags2::BitFlags<RegularExpressionFlags>,
    opening_loc: Location,
    content_loc: Location,
    closing_loc: Location,
    unescaped: String,
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

// 125
#[derive(Debug, Clone)]
pub struct RequiredKeywordParameterNode {
    pub flags: enumflags2::BitFlags<ParameterFlags>,
    name: ConstantRef,
    name_loc: Location,
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

// 126
#[derive(Debug, Clone)]
pub struct RequiredParameterNode {
    pub flags: enumflags2::BitFlags<ParameterFlags>,
    name: ConstantRef,
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

// 127
#[derive(Debug, Clone)]
pub struct RescueModifierNode {
    expression: NodeRef,
    keyword_loc: Location,
    rescue_expression: NodeRef,
}
impl RescueModifierNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::RescueModifierNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![RescueModifierNode{
        _: zero_flags.context(winnow::error::StrContext::Label("RescueModifierNode.flags")),
        expression: parse_node.context(winnow::error::StrContext::Label("RescueModifierNode.expression")),
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("RescueModifierNode.keyword_loc")),
        rescue_expression: parse_node.context(winnow::error::StrContext::Label("RescueModifierNode.rescue_expression")),
    }].parse_next(input)
    }
}

// 128
#[derive(Debug, Clone)]
pub struct RescueNode {
    keyword_loc: Location,
    exceptions: Vec<NodeRef>,
    operator_loc: Option<Location>,
    reference: Option<NodeRef>,
    then_keyword_loc: Option<Location>,
    statements: Option<NodeRef>,
    subsequent: Option<NodeRef>,
}
impl RescueNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::RescueNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![RescueNode{
        _: zero_flags.context(winnow::error::StrContext::Label("RescueNode.flags")),
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("RescueNode.keyword_loc")),
        exceptions: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("RescueNode.exceptions")),
        operator_loc: parse_optional_location.context(winnow::error::StrContext::Label("RescueNode.operator_loc")),
        reference: parse_optional_node.context(winnow::error::StrContext::Label("RescueNode.reference")),
        then_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("RescueNode.then_keyword_loc")),
        statements: parse_optional_node.context(winnow::error::StrContext::Label("RescueNode.statements")),
        subsequent: parse_optional_node.context(winnow::error::StrContext::Label("RescueNode.subsequent")),
    }].parse_next(input)
    }
}

// 129
#[derive(Debug, Clone)]
pub struct RestParameterNode {
    pub flags: enumflags2::BitFlags<ParameterFlags>,
    name: Option<ConstantRef>,
    name_loc: Option<Location>,
    operator_loc: Location,
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

// 130
#[derive(Debug, Clone)]
pub struct RetryNode {}
impl RetryNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::RetryNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        zero_flags
            .context(winnow::error::StrContext::Label("RetryNode.flags"))
            .value(Self {})
            .parse_next(input)
    }
}

// 131
#[derive(Debug, Clone)]
pub struct ReturnNode {
    keyword_loc: Location,
    arguments: Option<NodeRef>,
}
impl ReturnNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::ReturnNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![ReturnNode{
        _: zero_flags.context(winnow::error::StrContext::Label("ReturnNode.flags")),
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("ReturnNode.keyword_loc")),
        arguments: parse_optional_node.context(winnow::error::StrContext::Label("ReturnNode.arguments")),
    }].parse_next(input)
    }
}

// 132
#[derive(Debug, Clone)]
pub struct SelfNode {}
impl SelfNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::SelfNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        zero_flags
            .context(winnow::error::StrContext::Label("SelfNode.flags"))
            .value(Self {})
            .parse_next(input)
    }
}

// 133
#[derive(Debug, Clone)]
pub struct ShareableConstantNode {
    pub flags: enumflags2::BitFlags<ShareableConstantNodeFlags>,
    write: NodeRef,
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

// 134
#[derive(Debug, Clone)]
pub struct SingletonClassNode {
    locals: Vec<ConstantRef>,
    class_keyword_loc: Location,
    operator_loc: Location,
    expression: NodeRef,
    body: Option<NodeRef>,
    end_keyword_loc: Location,
}
impl SingletonClassNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::SingletonClassNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![SingletonClassNode{
        _: zero_flags.context(winnow::error::StrContext::Label("SingletonClassNode.flags")),
        locals: length_repeat(parse_varuint, parse_constant).context(winnow::error::StrContext::Label("SingletonClassNode.locals")),
        class_keyword_loc: parse_location.context(winnow::error::StrContext::Label("SingletonClassNode.class_keyword_loc")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("SingletonClassNode.operator_loc")),
        expression: parse_node.context(winnow::error::StrContext::Label("SingletonClassNode.expression")),
        body: parse_optional_node.context(winnow::error::StrContext::Label("SingletonClassNode.body")),
        end_keyword_loc: parse_location.context(winnow::error::StrContext::Label("SingletonClassNode.end_keyword_loc")),
    }].parse_next(input)
    }
}

// 135
#[derive(Debug, Clone)]
pub struct SourceEncodingNode {}
impl SourceEncodingNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::SourceEncodingNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        zero_flags
            .context(winnow::error::StrContext::Label("SourceEncodingNode.flags"))
            .value(Self {})
            .parse_next(input)
    }
}

// 136
#[derive(Debug, Clone)]
pub struct SourceFileNode {
    pub flags: enumflags2::BitFlags<StringFlags>,
    filepath: String,
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

// 137
#[derive(Debug, Clone)]
pub struct SourceLineNode {}
impl SourceLineNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::SourceLineNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        zero_flags
            .context(winnow::error::StrContext::Label("SourceLineNode.flags"))
            .value(Self {})
            .parse_next(input)
    }
}

// 138
#[derive(Debug, Clone)]
pub struct SplatNode {
    operator_loc: Location,
    expression: Option<NodeRef>,
}
impl SplatNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::SplatNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![SplatNode{
        _: zero_flags.context(winnow::error::StrContext::Label("SplatNode.flags")),
        operator_loc: parse_location.context(winnow::error::StrContext::Label("SplatNode.operator_loc")),
        expression: parse_optional_node.context(winnow::error::StrContext::Label("SplatNode.expression")),
    }].parse_next(input)
    }
}

// 139
#[derive(Debug, Clone)]
pub struct StatementsNode {
    body: Vec<NodeRef>,
}
impl StatementsNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::StatementsNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![StatementsNode{
        _: zero_flags.context(winnow::error::StrContext::Label("StatementsNode.flags")),
        body: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("StatementsNode.body")),
    }].parse_next(input)
    }
}

// 140
#[derive(Debug, Clone)]
pub struct StringNode {
    pub flags: enumflags2::BitFlags<StringFlags>,
    opening_loc: Option<Location>,
    content_loc: Location,
    closing_loc: Option<Location>,
    unescaped: String,
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

// 141
#[derive(Debug, Clone)]
pub struct SuperNode {
    keyword_loc: Location,
    lparen_loc: Option<Location>,
    arguments: Option<NodeRef>,
    rparen_loc: Option<Location>,
    block: Option<NodeRef>,
}
impl SuperNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::SuperNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![SuperNode{
        _: zero_flags.context(winnow::error::StrContext::Label("SuperNode.flags")),
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("SuperNode.keyword_loc")),
        lparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("SuperNode.lparen_loc")),
        arguments: parse_optional_node.context(winnow::error::StrContext::Label("SuperNode.arguments")),
        rparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("SuperNode.rparen_loc")),
        block: parse_optional_node.context(winnow::error::StrContext::Label("SuperNode.block")),
    }].parse_next(input)
    }
}

// 142
#[derive(Debug, Clone)]
pub struct SymbolNode {
    pub flags: enumflags2::BitFlags<SymbolFlags>,
    opening_loc: Option<Location>,
    value_loc: Option<Location>,
    closing_loc: Option<Location>,
    unescaped: String,
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

// 143
#[derive(Debug, Clone)]
pub struct TrueNode {}
impl TrueNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::TrueNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        zero_flags
            .context(winnow::error::StrContext::Label("TrueNode.flags"))
            .value(Self {})
            .parse_next(input)
    }
}

// 144
#[derive(Debug, Clone)]
pub struct UndefNode {
    names: Vec<NodeRef>,
    keyword_loc: Location,
}
impl UndefNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::UndefNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![UndefNode{
        _: zero_flags.context(winnow::error::StrContext::Label("UndefNode.flags")),
        names: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("UndefNode.names")),
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("UndefNode.keyword_loc")),
    }].parse_next(input)
    }
}

// 145
#[derive(Debug, Clone)]
pub struct UnlessNode {
    keyword_loc: Location,
    predicate: NodeRef,
    then_keyword_loc: Option<Location>,
    statements: Option<NodeRef>,
    else_clause: Option<NodeRef>,
    end_keyword_loc: Option<Location>,
}
impl UnlessNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::UnlessNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![UnlessNode{
        _: zero_flags.context(winnow::error::StrContext::Label("UnlessNode.flags")),
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("UnlessNode.keyword_loc")),
        predicate: parse_node.context(winnow::error::StrContext::Label("UnlessNode.predicate")),
        then_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("UnlessNode.then_keyword_loc")),
        statements: parse_optional_node.context(winnow::error::StrContext::Label("UnlessNode.statements")),
        else_clause: parse_optional_node.context(winnow::error::StrContext::Label("UnlessNode.else_clause")),
        end_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("UnlessNode.end_keyword_loc")),
    }].parse_next(input)
    }
}

// 146
#[derive(Debug, Clone)]
pub struct UntilNode {
    pub flags: enumflags2::BitFlags<LoopFlags>,
    keyword_loc: Location,
    do_keyword_loc: Option<Location>,
    closing_loc: Option<Location>,
    predicate: NodeRef,
    statements: Option<NodeRef>,
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

// 147
#[derive(Debug, Clone)]
pub struct WhenNode {
    keyword_loc: Location,
    conditions: Vec<NodeRef>,
    then_keyword_loc: Option<Location>,
    statements: Option<NodeRef>,
}
impl WhenNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::WhenNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![WhenNode{
        _: zero_flags.context(winnow::error::StrContext::Label("WhenNode.flags")),
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("WhenNode.keyword_loc")),
        conditions: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("WhenNode.conditions")),
        then_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("WhenNode.then_keyword_loc")),
        statements: parse_optional_node.context(winnow::error::StrContext::Label("WhenNode.statements")),
    }].parse_next(input)
    }
}

// 148
#[derive(Debug, Clone)]
pub struct WhileNode {
    pub flags: enumflags2::BitFlags<LoopFlags>,
    keyword_loc: Location,
    do_keyword_loc: Option<Location>,
    closing_loc: Option<Location>,
    predicate: NodeRef,
    statements: Option<NodeRef>,
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

// 149
#[derive(Debug, Clone)]
pub struct XStringNode {
    pub flags: enumflags2::BitFlags<EncodingFlags>,
    opening_loc: Location,
    content_loc: Location,
    closing_loc: Location,
    unescaped: String,
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

// 150
#[derive(Debug, Clone)]
pub struct YieldNode {
    keyword_loc: Location,
    lparen_loc: Option<Location>,
    arguments: Option<NodeRef>,
    rparen_loc: Option<Location>,
}
impl YieldNode {
    pub fn into_node_kind(self) -> NodeKind {
        NodeKind::YieldNode(self)
    }
    pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {
        use winnow::Parser;
        winnow::combinator::seq![YieldNode{
        _: zero_flags.context(winnow::error::StrContext::Label("YieldNode.flags")),
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("YieldNode.keyword_loc")),
        lparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("YieldNode.lparen_loc")),
        arguments: parse_optional_node.context(winnow::error::StrContext::Label("YieldNode.arguments")),
        rparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("YieldNode.rparen_loc")),
    }].parse_next(input)
    }
}

#[derive(Debug, Clone)]
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
