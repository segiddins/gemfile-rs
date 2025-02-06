use super::deserialize::*;

// 0
#[derive(Debug, Clone)]
pub struct AliasGlobalVariableNode {
  new_name: NodeRef,
  old_name: NodeRef,
  keyword_loc: Location,
}
impl AliasGlobalVariableNode {
}

// 1
#[derive(Debug, Clone)]
pub struct AliasMethodNode {
  new_name: NodeRef,
  old_name: NodeRef,
  keyword_loc: Location,
}
impl AliasMethodNode {
}

// 2
#[derive(Debug, Clone)]
pub struct AlternationPatternNode {
  left: NodeRef,
  right: NodeRef,
  operator_loc: Location,
}
impl AlternationPatternNode {
}

// 3
#[derive(Debug, Clone)]
pub struct AndNode {
  left: NodeRef,
  right: NodeRef,
  operator_loc: Location,
}
impl AndNode {
}

// 4
#[derive(Debug, Clone)]
pub struct ArgumentsNode {
  arguments: Vec<NodeRef>,
}
impl ArgumentsNode {
}

// 5
#[derive(Debug, Clone)]
pub struct ArrayNode {
  elements: Vec<NodeRef>,
  opening_loc: Option<Location>,
  closing_loc: Option<Location>,
}
impl ArrayNode {
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
}

// 7
#[derive(Debug, Clone)]
pub struct AssocNode {
  key: NodeRef,
  value: NodeRef,
  operator_loc: Option<Location>,
}
impl AssocNode {
}

// 8
#[derive(Debug, Clone)]
pub struct AssocSplatNode {
  value: Option<NodeRef>,
  operator_loc: Location,
}
impl AssocSplatNode {
}

// 9
#[derive(Debug, Clone)]
pub struct BackReferenceReadNode {
  name: ConstantRef,
}
impl BackReferenceReadNode {
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
}

// 11
#[derive(Debug, Clone)]
pub struct BlockArgumentNode {
  expression: Option<NodeRef>,
  operator_loc: Location,
}
impl BlockArgumentNode {
}

// 12
#[derive(Debug, Clone)]
pub struct BlockLocalVariableNode {
  name: ConstantRef,
}
impl BlockLocalVariableNode {
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
}

// 14
#[derive(Debug, Clone)]
pub struct BlockParameterNode {
  name: Option<ConstantRef>,
  name_loc: Option<Location>,
  operator_loc: Location,
}
impl BlockParameterNode {
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
}

// 16
#[derive(Debug, Clone)]
pub struct BreakNode {
  arguments: Option<NodeRef>,
  keyword_loc: Location,
}
impl BreakNode {
}

// 17
#[derive(Debug, Clone)]
pub struct CallAndWriteNode {
  receiver: Option<NodeRef>,
  call_operator_loc: Option<Location>,
  message_loc: Option<Location>,
  read_name: ConstantRef,
  write_name: ConstantRef,
  operator_loc: Location,
  value: NodeRef,
}
impl CallAndWriteNode {
}

// 18
#[derive(Debug, Clone)]
pub struct CallNode {
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
}

// 19
#[derive(Debug, Clone)]
pub struct CallOperatorWriteNode {
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
}

// 20
#[derive(Debug, Clone)]
pub struct CallOrWriteNode {
  receiver: Option<NodeRef>,
  call_operator_loc: Option<Location>,
  message_loc: Option<Location>,
  read_name: ConstantRef,
  write_name: ConstantRef,
  operator_loc: Location,
  value: NodeRef,
}
impl CallOrWriteNode {
}

// 21
#[derive(Debug, Clone)]
pub struct CallTargetNode {
  receiver: NodeRef,
  call_operator_loc: Location,
  name: ConstantRef,
  message_loc: Location,
}
impl CallTargetNode {
}

// 22
#[derive(Debug, Clone)]
pub struct CapturePatternNode {
  value: NodeRef,
  target: NodeRef,
  operator_loc: Location,
}
impl CapturePatternNode {
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
}

// 29
#[derive(Debug, Clone)]
pub struct ClassVariableReadNode {
  name: ConstantRef,
}
impl ClassVariableReadNode {
}

// 30
#[derive(Debug, Clone)]
pub struct ClassVariableTargetNode {
  name: ConstantRef,
}
impl ClassVariableTargetNode {
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
}

// 35
#[derive(Debug, Clone)]
pub struct ConstantPathAndWriteNode {
  target: NodeRef,
  operator_loc: Location,
  value: NodeRef,
}
impl ConstantPathAndWriteNode {
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
}

// 38
#[derive(Debug, Clone)]
pub struct ConstantPathOrWriteNode {
  target: NodeRef,
  operator_loc: Location,
  value: NodeRef,
}
impl ConstantPathOrWriteNode {
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
}

// 40
#[derive(Debug, Clone)]
pub struct ConstantPathWriteNode {
  target: NodeRef,
  operator_loc: Location,
  value: NodeRef,
}
impl ConstantPathWriteNode {
}

// 41
#[derive(Debug, Clone)]
pub struct ConstantReadNode {
  name: ConstantRef,
}
impl ConstantReadNode {
}

// 42
#[derive(Debug, Clone)]
pub struct ConstantTargetNode {
  name: ConstantRef,
}
impl ConstantTargetNode {
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
}

// 46
#[derive(Debug, Clone)]
pub struct ElseNode {
  else_keyword_loc: Location,
  statements: Option<NodeRef>,
  end_keyword_loc: Option<Location>,
}
impl ElseNode {
}

// 47
#[derive(Debug, Clone)]
pub struct EmbeddedStatementsNode {
  opening_loc: Location,
  statements: Option<NodeRef>,
  closing_loc: Location,
}
impl EmbeddedStatementsNode {
}

// 48
#[derive(Debug, Clone)]
pub struct EmbeddedVariableNode {
  operator_loc: Location,
  variable: NodeRef,
}
impl EmbeddedVariableNode {
}

// 49
#[derive(Debug, Clone)]
pub struct EnsureNode {
  ensure_keyword_loc: Location,
  statements: Option<NodeRef>,
  end_keyword_loc: Location,
}
impl EnsureNode {
}

// 50
#[derive(Debug, Clone)]
pub struct FalseNode(());
impl FalseNode {
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
}

// 52
#[derive(Debug, Clone)]
pub struct FlipFlopNode {
  left: Option<NodeRef>,
  right: Option<NodeRef>,
  operator_loc: Location,
}
impl FlipFlopNode {
}

// 53
#[derive(Debug, Clone)]
pub struct FloatNode {
  value: f64,
}
impl FloatNode {
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
}

// 55
#[derive(Debug, Clone)]
pub struct ForwardingArgumentsNode(());
impl ForwardingArgumentsNode {
}

// 56
#[derive(Debug, Clone)]
pub struct ForwardingParameterNode(());
impl ForwardingParameterNode {
}

// 57
#[derive(Debug, Clone)]
pub struct ForwardingSuperNode {
  block: Option<NodeRef>,
}
impl ForwardingSuperNode {
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
}

// 61
#[derive(Debug, Clone)]
pub struct GlobalVariableReadNode {
  name: ConstantRef,
}
impl GlobalVariableReadNode {
}

// 62
#[derive(Debug, Clone)]
pub struct GlobalVariableTargetNode {
  name: ConstantRef,
}
impl GlobalVariableTargetNode {
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
}

// 64
#[derive(Debug, Clone)]
pub struct HashNode {
  opening_loc: Location,
  elements: Vec<NodeRef>,
  closing_loc: Location,
}
impl HashNode {
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
}

// 67
#[derive(Debug, Clone)]
pub struct ImaginaryNode {
  numeric: NodeRef,
}
impl ImaginaryNode {
}

// 68
#[derive(Debug, Clone)]
pub struct ImplicitNode {
  value: NodeRef,
}
impl ImplicitNode {
}

// 69
#[derive(Debug, Clone)]
pub struct ImplicitRestNode(());
impl ImplicitRestNode {
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
}

// 71
#[derive(Debug, Clone)]
pub struct IndexAndWriteNode {
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
}

// 72
#[derive(Debug, Clone)]
pub struct IndexOperatorWriteNode {
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
}

// 73
#[derive(Debug, Clone)]
pub struct IndexOrWriteNode {
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
}

// 74
#[derive(Debug, Clone)]
pub struct IndexTargetNode {
  receiver: NodeRef,
  opening_loc: Location,
  arguments: Option<NodeRef>,
  closing_loc: Location,
  block: Option<NodeRef>,
}
impl IndexTargetNode {
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
}

// 78
#[derive(Debug, Clone)]
pub struct InstanceVariableReadNode {
  name: ConstantRef,
}
impl InstanceVariableReadNode {
}

// 79
#[derive(Debug, Clone)]
pub struct InstanceVariableTargetNode {
  name: ConstantRef,
}
impl InstanceVariableTargetNode {
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
}

// 81
#[derive(Debug, Clone)]
pub struct IntegerNode {
  value: Integer,
}
impl IntegerNode {
}

// 82
#[derive(Debug, Clone)]
pub struct InterpolatedMatchLastLineNode {
  opening_loc: Location,
  parts: Vec<NodeRef>,
  closing_loc: Location,
}
impl InterpolatedMatchLastLineNode {
}

// 83
#[derive(Debug, Clone)]
pub struct InterpolatedRegularExpressionNode {
  opening_loc: Location,
  parts: Vec<NodeRef>,
  closing_loc: Location,
}
impl InterpolatedRegularExpressionNode {
}

// 84
#[derive(Debug, Clone)]
pub struct InterpolatedStringNode {
  opening_loc: Option<Location>,
  parts: Vec<NodeRef>,
  closing_loc: Option<Location>,
}
impl InterpolatedStringNode {
}

// 85
#[derive(Debug, Clone)]
pub struct InterpolatedSymbolNode {
  opening_loc: Option<Location>,
  parts: Vec<NodeRef>,
  closing_loc: Option<Location>,
}
impl InterpolatedSymbolNode {
}

// 86
#[derive(Debug, Clone)]
pub struct InterpolatedXStringNode {
  opening_loc: Location,
  parts: Vec<NodeRef>,
  closing_loc: Location,
}
impl InterpolatedXStringNode {
}

// 87
#[derive(Debug, Clone)]
pub struct ItLocalVariableReadNode(());
impl ItLocalVariableReadNode {
}

// 88
#[derive(Debug, Clone)]
pub struct ItParametersNode(());
impl ItParametersNode {
}

// 89
#[derive(Debug, Clone)]
pub struct KeywordHashNode {
  elements: Vec<NodeRef>,
}
impl KeywordHashNode {
}

// 90
#[derive(Debug, Clone)]
pub struct KeywordRestParameterNode {
  name: Option<ConstantRef>,
  name_loc: Option<Location>,
  operator_loc: Location,
}
impl KeywordRestParameterNode {
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
}

// 95
#[derive(Debug, Clone)]
pub struct LocalVariableReadNode {
  name: ConstantRef,
  depth: u32,
}
impl LocalVariableReadNode {
}

// 96
#[derive(Debug, Clone)]
pub struct LocalVariableTargetNode {
  name: ConstantRef,
  depth: u32,
}
impl LocalVariableTargetNode {
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
}

// 98
#[derive(Debug, Clone)]
pub struct MatchLastLineNode {
  opening_loc: Location,
  content_loc: Location,
  closing_loc: Location,
  unescaped: String,
}
impl MatchLastLineNode {
}

// 99
#[derive(Debug, Clone)]
pub struct MatchPredicateNode {
  value: NodeRef,
  pattern: NodeRef,
  operator_loc: Location,
}
impl MatchPredicateNode {
}

// 100
#[derive(Debug, Clone)]
pub struct MatchRequiredNode {
  value: NodeRef,
  pattern: NodeRef,
  operator_loc: Location,
}
impl MatchRequiredNode {
}

// 101
#[derive(Debug, Clone)]
pub struct MatchWriteNode {
  call: NodeRef,
  targets: Vec<NodeRef>,
}
impl MatchWriteNode {
}

// 102
#[derive(Debug, Clone)]
pub struct MissingNode(());
impl MissingNode {
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
}

// 106
#[derive(Debug, Clone)]
pub struct NextNode {
  arguments: Option<NodeRef>,
  keyword_loc: Location,
}
impl NextNode {
}

// 107
#[derive(Debug, Clone)]
pub struct NilNode(());
impl NilNode {
}

// 108
#[derive(Debug, Clone)]
pub struct NoKeywordsParameterNode {
  operator_loc: Location,
  keyword_loc: Location,
}
impl NoKeywordsParameterNode {
}

// 109
#[derive(Debug, Clone)]
pub struct NumberedParametersNode {
  maximum: u8,
}
impl NumberedParametersNode {
}

// 110
#[derive(Debug, Clone)]
pub struct NumberedReferenceReadNode {
  number: u32,
}
impl NumberedReferenceReadNode {
}

// 111
#[derive(Debug, Clone)]
pub struct OptionalKeywordParameterNode {
  name: ConstantRef,
  name_loc: Location,
  value: NodeRef,
}
impl OptionalKeywordParameterNode {
}

// 112
#[derive(Debug, Clone)]
pub struct OptionalParameterNode {
  name: ConstantRef,
  name_loc: Location,
  operator_loc: Location,
  value: NodeRef,
}
impl OptionalParameterNode {
}

// 113
#[derive(Debug, Clone)]
pub struct OrNode {
  left: NodeRef,
  right: NodeRef,
  operator_loc: Location,
}
impl OrNode {
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
}

// 115
#[derive(Debug, Clone)]
pub struct ParenthesesNode {
  body: Option<NodeRef>,
  opening_loc: Location,
  closing_loc: Location,
}
impl ParenthesesNode {
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
}

// 117
#[derive(Debug, Clone)]
pub struct PinnedVariableNode {
  variable: NodeRef,
  operator_loc: Location,
}
impl PinnedVariableNode {
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
}

// 120
#[derive(Debug, Clone)]
pub struct ProgramNode {
  locals: Vec<ConstantRef>,
  statements: NodeRef,
}
impl ProgramNode {
}

// 121
#[derive(Debug, Clone)]
pub struct RangeNode {
  left: Option<NodeRef>,
  right: Option<NodeRef>,
  operator_loc: Location,
}
impl RangeNode {
}

// 122
#[derive(Debug, Clone)]
pub struct RationalNode {
  numerator: Integer,
  denominator: Integer,
}
impl RationalNode {
}

// 123
#[derive(Debug, Clone)]
pub struct RedoNode(());
impl RedoNode {
}

// 124
#[derive(Debug, Clone)]
pub struct RegularExpressionNode {
  opening_loc: Location,
  content_loc: Location,
  closing_loc: Location,
  unescaped: String,
}
impl RegularExpressionNode {
}

// 125
#[derive(Debug, Clone)]
pub struct RequiredKeywordParameterNode {
  name: ConstantRef,
  name_loc: Location,
}
impl RequiredKeywordParameterNode {
}

// 126
#[derive(Debug, Clone)]
pub struct RequiredParameterNode {
  name: ConstantRef,
}
impl RequiredParameterNode {
}

// 127
#[derive(Debug, Clone)]
pub struct RescueModifierNode {
  expression: NodeRef,
  keyword_loc: Location,
  rescue_expression: NodeRef,
}
impl RescueModifierNode {
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
}

// 129
#[derive(Debug, Clone)]
pub struct RestParameterNode {
  name: Option<ConstantRef>,
  name_loc: Option<Location>,
  operator_loc: Location,
}
impl RestParameterNode {
}

// 130
#[derive(Debug, Clone)]
pub struct RetryNode(());
impl RetryNode {
}

// 131
#[derive(Debug, Clone)]
pub struct ReturnNode {
  keyword_loc: Location,
  arguments: Option<NodeRef>,
}
impl ReturnNode {
}

// 132
#[derive(Debug, Clone)]
pub struct SelfNode(());
impl SelfNode {
}

// 133
#[derive(Debug, Clone)]
pub struct ShareableConstantNode {
  write: NodeRef,
}
impl ShareableConstantNode {
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
}

// 135
#[derive(Debug, Clone)]
pub struct SourceEncodingNode(());
impl SourceEncodingNode {
}

// 136
#[derive(Debug, Clone)]
pub struct SourceFileNode {
  filepath: String,
}
impl SourceFileNode {
}

// 137
#[derive(Debug, Clone)]
pub struct SourceLineNode(());
impl SourceLineNode {
}

// 138
#[derive(Debug, Clone)]
pub struct SplatNode {
  operator_loc: Location,
  expression: Option<NodeRef>,
}
impl SplatNode {
}

// 139
#[derive(Debug, Clone)]
pub struct StatementsNode {
  body: Vec<NodeRef>,
}
impl StatementsNode {
}

// 140
#[derive(Debug, Clone)]
pub struct StringNode {
  opening_loc: Option<Location>,
  content_loc: Location,
  closing_loc: Option<Location>,
  unescaped: String,
}
impl StringNode {
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
}

// 142
#[derive(Debug, Clone)]
pub struct SymbolNode {
  opening_loc: Option<Location>,
  value_loc: Option<Location>,
  closing_loc: Option<Location>,
  unescaped: String,
}
impl SymbolNode {
}

// 143
#[derive(Debug, Clone)]
pub struct TrueNode(());
impl TrueNode {
}

// 144
#[derive(Debug, Clone)]
pub struct UndefNode {
  names: Vec<NodeRef>,
  keyword_loc: Location,
}
impl UndefNode {
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
}

// 146
#[derive(Debug, Clone)]
pub struct UntilNode {
  keyword_loc: Location,
  do_keyword_loc: Option<Location>,
  closing_loc: Option<Location>,
  predicate: NodeRef,
  statements: Option<NodeRef>,
}
impl UntilNode {
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
}

// 148
#[derive(Debug, Clone)]
pub struct WhileNode {
  keyword_loc: Location,
  do_keyword_loc: Option<Location>,
  closing_loc: Option<Location>,
  predicate: NodeRef,
  statements: Option<NodeRef>,
}
impl WhileNode {
}

// 149
#[derive(Debug, Clone)]
pub struct XStringNode {
  opening_loc: Location,
  content_loc: Location,
  closing_loc: Location,
  unescaped: String,
}
impl XStringNode {
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


pub fn parse_node(
    input: &mut super::deserialize::Stream,
) -> winnow::ModalResult<NodeRef> {
    use winnow::binary::{length_repeat};
    use winnow::Parser;

    let (kind, identifier, location, flags) = winnow::Parser::parse_next(
        &mut winnow::combinator::seq![(
            winnow::binary::u8,
            parse_varuint,
            parse_location,
            parse_varuint,
        )],
        input,
    )?;

    let node_kind = match kind {
    1 => NodeKind::AliasGlobalVariableNode(
      AliasGlobalVariableNode {
        new_name: parse_node.context(winnow::error::StrContext::Label("AliasGlobalVariableNode.new_name")).parse_next(input)?,
        old_name: parse_node.context(winnow::error::StrContext::Label("AliasGlobalVariableNode.old_name")).parse_next(input)?,
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("AliasGlobalVariableNode.keyword_loc")).parse_next(input)?,
      }
    ),
    2 => NodeKind::AliasMethodNode(
      AliasMethodNode {
        new_name: parse_node.context(winnow::error::StrContext::Label("AliasMethodNode.new_name")).parse_next(input)?,
        old_name: parse_node.context(winnow::error::StrContext::Label("AliasMethodNode.old_name")).parse_next(input)?,
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("AliasMethodNode.keyword_loc")).parse_next(input)?,
      }
    ),
    3 => NodeKind::AlternationPatternNode(
      AlternationPatternNode {
        left: parse_node.context(winnow::error::StrContext::Label("AlternationPatternNode.left")).parse_next(input)?,
        right: parse_node.context(winnow::error::StrContext::Label("AlternationPatternNode.right")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("AlternationPatternNode.operator_loc")).parse_next(input)?,
      }
    ),
    4 => NodeKind::AndNode(
      AndNode {
        left: parse_node.context(winnow::error::StrContext::Label("AndNode.left")).parse_next(input)?,
        right: parse_node.context(winnow::error::StrContext::Label("AndNode.right")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("AndNode.operator_loc")).parse_next(input)?,
      }
    ),
    5 => NodeKind::ArgumentsNode(
      ArgumentsNode {
        arguments: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("ArgumentsNode.arguments")).parse_next(input)?,
      }
    ),
    6 => NodeKind::ArrayNode(
      ArrayNode {
        elements: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("ArrayNode.elements")).parse_next(input)?,
        opening_loc: parse_optional_location.context(winnow::error::StrContext::Label("ArrayNode.opening_loc")).parse_next(input)?,
        closing_loc: parse_optional_location.context(winnow::error::StrContext::Label("ArrayNode.closing_loc")).parse_next(input)?,
      }
    ),
    7 => NodeKind::ArrayPatternNode(
      ArrayPatternNode {
        constant: parse_optional_node.context(winnow::error::StrContext::Label("ArrayPatternNode.constant")).parse_next(input)?,
        requireds: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("ArrayPatternNode.requireds")).parse_next(input)?,
        rest: parse_optional_node.context(winnow::error::StrContext::Label("ArrayPatternNode.rest")).parse_next(input)?,
        posts: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("ArrayPatternNode.posts")).parse_next(input)?,
        opening_loc: parse_optional_location.context(winnow::error::StrContext::Label("ArrayPatternNode.opening_loc")).parse_next(input)?,
        closing_loc: parse_optional_location.context(winnow::error::StrContext::Label("ArrayPatternNode.closing_loc")).parse_next(input)?,
      }
    ),
    8 => NodeKind::AssocNode(
      AssocNode {
        key: parse_node.context(winnow::error::StrContext::Label("AssocNode.key")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("AssocNode.value")).parse_next(input)?,
        operator_loc: parse_optional_location.context(winnow::error::StrContext::Label("AssocNode.operator_loc")).parse_next(input)?,
      }
    ),
    9 => NodeKind::AssocSplatNode(
      AssocSplatNode {
        value: parse_optional_node.context(winnow::error::StrContext::Label("AssocSplatNode.value")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("AssocSplatNode.operator_loc")).parse_next(input)?,
      }
    ),
    10 => NodeKind::BackReferenceReadNode(
      BackReferenceReadNode {
        name: parse_constant.context(winnow::error::StrContext::Label("BackReferenceReadNode.name")).parse_next(input)?,
      }
    ),
    11 => NodeKind::BeginNode(
      BeginNode {
        begin_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("BeginNode.begin_keyword_loc")).parse_next(input)?,
        statements: parse_optional_node.context(winnow::error::StrContext::Label("BeginNode.statements")).parse_next(input)?,
        rescue_clause: parse_optional_node.context(winnow::error::StrContext::Label("BeginNode.rescue_clause")).parse_next(input)?,
        else_clause: parse_optional_node.context(winnow::error::StrContext::Label("BeginNode.else_clause")).parse_next(input)?,
        ensure_clause: parse_optional_node.context(winnow::error::StrContext::Label("BeginNode.ensure_clause")).parse_next(input)?,
        end_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("BeginNode.end_keyword_loc")).parse_next(input)?,
      }
    ),
    12 => NodeKind::BlockArgumentNode(
      BlockArgumentNode {
        expression: parse_optional_node.context(winnow::error::StrContext::Label("BlockArgumentNode.expression")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("BlockArgumentNode.operator_loc")).parse_next(input)?,
      }
    ),
    13 => NodeKind::BlockLocalVariableNode(
      BlockLocalVariableNode {
        name: parse_constant.context(winnow::error::StrContext::Label("BlockLocalVariableNode.name")).parse_next(input)?,
      }
    ),
    14 => NodeKind::BlockNode(
      BlockNode {
        locals: length_repeat(parse_varuint, parse_constant).context(winnow::error::StrContext::Label("BlockNode.locals")).parse_next(input)?,
        parameters: parse_optional_node.context(winnow::error::StrContext::Label("BlockNode.parameters")).parse_next(input)?,
        body: parse_optional_node.context(winnow::error::StrContext::Label("BlockNode.body")).parse_next(input)?,
        opening_loc: parse_location.context(winnow::error::StrContext::Label("BlockNode.opening_loc")).parse_next(input)?,
        closing_loc: parse_location.context(winnow::error::StrContext::Label("BlockNode.closing_loc")).parse_next(input)?,
      }
    ),
    15 => NodeKind::BlockParameterNode(
      BlockParameterNode {
        name: parse_optional_constant.context(winnow::error::StrContext::Label("BlockParameterNode.name")).parse_next(input)?,
        name_loc: parse_optional_location.context(winnow::error::StrContext::Label("BlockParameterNode.name_loc")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("BlockParameterNode.operator_loc")).parse_next(input)?,
      }
    ),
    16 => NodeKind::BlockParametersNode(
      BlockParametersNode {
        parameters: parse_optional_node.context(winnow::error::StrContext::Label("BlockParametersNode.parameters")).parse_next(input)?,
        locals: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("BlockParametersNode.locals")).parse_next(input)?,
        opening_loc: parse_optional_location.context(winnow::error::StrContext::Label("BlockParametersNode.opening_loc")).parse_next(input)?,
        closing_loc: parse_optional_location.context(winnow::error::StrContext::Label("BlockParametersNode.closing_loc")).parse_next(input)?,
      }
    ),
    17 => NodeKind::BreakNode(
      BreakNode {
        arguments: parse_optional_node.context(winnow::error::StrContext::Label("BreakNode.arguments")).parse_next(input)?,
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("BreakNode.keyword_loc")).parse_next(input)?,
      }
    ),
    18 => NodeKind::CallAndWriteNode(
      CallAndWriteNode {
        receiver: parse_optional_node.context(winnow::error::StrContext::Label("CallAndWriteNode.receiver")).parse_next(input)?,
        call_operator_loc: parse_optional_location.context(winnow::error::StrContext::Label("CallAndWriteNode.call_operator_loc")).parse_next(input)?,
        message_loc: parse_optional_location.context(winnow::error::StrContext::Label("CallAndWriteNode.message_loc")).parse_next(input)?,
        read_name: parse_constant.context(winnow::error::StrContext::Label("CallAndWriteNode.read_name")).parse_next(input)?,
        write_name: parse_constant.context(winnow::error::StrContext::Label("CallAndWriteNode.write_name")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("CallAndWriteNode.operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("CallAndWriteNode.value")).parse_next(input)?,
      }
    ),
    19 => NodeKind::CallNode(
      CallNode {
        receiver: parse_optional_node.context(winnow::error::StrContext::Label("CallNode.receiver")).parse_next(input)?,
        call_operator_loc: parse_optional_location.context(winnow::error::StrContext::Label("CallNode.call_operator_loc")).parse_next(input)?,
        name: parse_constant.context(winnow::error::StrContext::Label("CallNode.name")).parse_next(input)?,
        message_loc: parse_optional_location.context(winnow::error::StrContext::Label("CallNode.message_loc")).parse_next(input)?,
        opening_loc: parse_optional_location.context(winnow::error::StrContext::Label("CallNode.opening_loc")).parse_next(input)?,
        arguments: parse_optional_node.context(winnow::error::StrContext::Label("CallNode.arguments")).parse_next(input)?,
        closing_loc: parse_optional_location.context(winnow::error::StrContext::Label("CallNode.closing_loc")).parse_next(input)?,
        block: parse_optional_node.context(winnow::error::StrContext::Label("CallNode.block")).parse_next(input)?,
      }
    ),
    20 => NodeKind::CallOperatorWriteNode(
      CallOperatorWriteNode {
        receiver: parse_optional_node.context(winnow::error::StrContext::Label("CallOperatorWriteNode.receiver")).parse_next(input)?,
        call_operator_loc: parse_optional_location.context(winnow::error::StrContext::Label("CallOperatorWriteNode.call_operator_loc")).parse_next(input)?,
        message_loc: parse_optional_location.context(winnow::error::StrContext::Label("CallOperatorWriteNode.message_loc")).parse_next(input)?,
        read_name: parse_constant.context(winnow::error::StrContext::Label("CallOperatorWriteNode.read_name")).parse_next(input)?,
        write_name: parse_constant.context(winnow::error::StrContext::Label("CallOperatorWriteNode.write_name")).parse_next(input)?,
        binary_operator: parse_constant.context(winnow::error::StrContext::Label("CallOperatorWriteNode.binary_operator")).parse_next(input)?,
        binary_operator_loc: parse_location.context(winnow::error::StrContext::Label("CallOperatorWriteNode.binary_operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("CallOperatorWriteNode.value")).parse_next(input)?,
      }
    ),
    21 => NodeKind::CallOrWriteNode(
      CallOrWriteNode {
        receiver: parse_optional_node.context(winnow::error::StrContext::Label("CallOrWriteNode.receiver")).parse_next(input)?,
        call_operator_loc: parse_optional_location.context(winnow::error::StrContext::Label("CallOrWriteNode.call_operator_loc")).parse_next(input)?,
        message_loc: parse_optional_location.context(winnow::error::StrContext::Label("CallOrWriteNode.message_loc")).parse_next(input)?,
        read_name: parse_constant.context(winnow::error::StrContext::Label("CallOrWriteNode.read_name")).parse_next(input)?,
        write_name: parse_constant.context(winnow::error::StrContext::Label("CallOrWriteNode.write_name")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("CallOrWriteNode.operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("CallOrWriteNode.value")).parse_next(input)?,
      }
    ),
    22 => NodeKind::CallTargetNode(
      CallTargetNode {
        receiver: parse_node.context(winnow::error::StrContext::Label("CallTargetNode.receiver")).parse_next(input)?,
        call_operator_loc: parse_location.context(winnow::error::StrContext::Label("CallTargetNode.call_operator_loc")).parse_next(input)?,
        name: parse_constant.context(winnow::error::StrContext::Label("CallTargetNode.name")).parse_next(input)?,
        message_loc: parse_location.context(winnow::error::StrContext::Label("CallTargetNode.message_loc")).parse_next(input)?,
      }
    ),
    23 => NodeKind::CapturePatternNode(
      CapturePatternNode {
        value: parse_node.context(winnow::error::StrContext::Label("CapturePatternNode.value")).parse_next(input)?,
        target: parse_node.context(winnow::error::StrContext::Label("CapturePatternNode.target")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("CapturePatternNode.operator_loc")).parse_next(input)?,
      }
    ),
    24 => NodeKind::CaseMatchNode(
      CaseMatchNode {
        predicate: parse_optional_node.context(winnow::error::StrContext::Label("CaseMatchNode.predicate")).parse_next(input)?,
        conditions: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("CaseMatchNode.conditions")).parse_next(input)?,
        else_clause: parse_optional_node.context(winnow::error::StrContext::Label("CaseMatchNode.else_clause")).parse_next(input)?,
        case_keyword_loc: parse_location.context(winnow::error::StrContext::Label("CaseMatchNode.case_keyword_loc")).parse_next(input)?,
        end_keyword_loc: parse_location.context(winnow::error::StrContext::Label("CaseMatchNode.end_keyword_loc")).parse_next(input)?,
      }
    ),
    25 => NodeKind::CaseNode(
      CaseNode {
        predicate: parse_optional_node.context(winnow::error::StrContext::Label("CaseNode.predicate")).parse_next(input)?,
        conditions: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("CaseNode.conditions")).parse_next(input)?,
        else_clause: parse_optional_node.context(winnow::error::StrContext::Label("CaseNode.else_clause")).parse_next(input)?,
        case_keyword_loc: parse_location.context(winnow::error::StrContext::Label("CaseNode.case_keyword_loc")).parse_next(input)?,
        end_keyword_loc: parse_location.context(winnow::error::StrContext::Label("CaseNode.end_keyword_loc")).parse_next(input)?,
      }
    ),
    26 => NodeKind::ClassNode(
      ClassNode {
        locals: length_repeat(parse_varuint, parse_constant).context(winnow::error::StrContext::Label("ClassNode.locals")).parse_next(input)?,
        class_keyword_loc: parse_location.context(winnow::error::StrContext::Label("ClassNode.class_keyword_loc")).parse_next(input)?,
        constant_path: parse_node.context(winnow::error::StrContext::Label("ClassNode.constant_path")).parse_next(input)?,
        inheritance_operator_loc: parse_optional_location.context(winnow::error::StrContext::Label("ClassNode.inheritance_operator_loc")).parse_next(input)?,
        superclass: parse_optional_node.context(winnow::error::StrContext::Label("ClassNode.superclass")).parse_next(input)?,
        body: parse_optional_node.context(winnow::error::StrContext::Label("ClassNode.body")).parse_next(input)?,
        end_keyword_loc: parse_location.context(winnow::error::StrContext::Label("ClassNode.end_keyword_loc")).parse_next(input)?,
        name: parse_constant.context(winnow::error::StrContext::Label("ClassNode.name")).parse_next(input)?,
      }
    ),
    27 => NodeKind::ClassVariableAndWriteNode(
      ClassVariableAndWriteNode {
        name: parse_constant.context(winnow::error::StrContext::Label("ClassVariableAndWriteNode.name")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("ClassVariableAndWriteNode.name_loc")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("ClassVariableAndWriteNode.operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("ClassVariableAndWriteNode.value")).parse_next(input)?,
      }
    ),
    28 => NodeKind::ClassVariableOperatorWriteNode(
      ClassVariableOperatorWriteNode {
        name: parse_constant.context(winnow::error::StrContext::Label("ClassVariableOperatorWriteNode.name")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("ClassVariableOperatorWriteNode.name_loc")).parse_next(input)?,
        binary_operator_loc: parse_location.context(winnow::error::StrContext::Label("ClassVariableOperatorWriteNode.binary_operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("ClassVariableOperatorWriteNode.value")).parse_next(input)?,
        binary_operator: parse_constant.context(winnow::error::StrContext::Label("ClassVariableOperatorWriteNode.binary_operator")).parse_next(input)?,
      }
    ),
    29 => NodeKind::ClassVariableOrWriteNode(
      ClassVariableOrWriteNode {
        name: parse_constant.context(winnow::error::StrContext::Label("ClassVariableOrWriteNode.name")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("ClassVariableOrWriteNode.name_loc")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("ClassVariableOrWriteNode.operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("ClassVariableOrWriteNode.value")).parse_next(input)?,
      }
    ),
    30 => NodeKind::ClassVariableReadNode(
      ClassVariableReadNode {
        name: parse_constant.context(winnow::error::StrContext::Label("ClassVariableReadNode.name")).parse_next(input)?,
      }
    ),
    31 => NodeKind::ClassVariableTargetNode(
      ClassVariableTargetNode {
        name: parse_constant.context(winnow::error::StrContext::Label("ClassVariableTargetNode.name")).parse_next(input)?,
      }
    ),
    32 => NodeKind::ClassVariableWriteNode(
      ClassVariableWriteNode {
        name: parse_constant.context(winnow::error::StrContext::Label("ClassVariableWriteNode.name")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("ClassVariableWriteNode.name_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("ClassVariableWriteNode.value")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("ClassVariableWriteNode.operator_loc")).parse_next(input)?,
      }
    ),
    33 => NodeKind::ConstantAndWriteNode(
      ConstantAndWriteNode {
        name: parse_constant.context(winnow::error::StrContext::Label("ConstantAndWriteNode.name")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("ConstantAndWriteNode.name_loc")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("ConstantAndWriteNode.operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("ConstantAndWriteNode.value")).parse_next(input)?,
      }
    ),
    34 => NodeKind::ConstantOperatorWriteNode(
      ConstantOperatorWriteNode {
        name: parse_constant.context(winnow::error::StrContext::Label("ConstantOperatorWriteNode.name")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("ConstantOperatorWriteNode.name_loc")).parse_next(input)?,
        binary_operator_loc: parse_location.context(winnow::error::StrContext::Label("ConstantOperatorWriteNode.binary_operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("ConstantOperatorWriteNode.value")).parse_next(input)?,
        binary_operator: parse_constant.context(winnow::error::StrContext::Label("ConstantOperatorWriteNode.binary_operator")).parse_next(input)?,
      }
    ),
    35 => NodeKind::ConstantOrWriteNode(
      ConstantOrWriteNode {
        name: parse_constant.context(winnow::error::StrContext::Label("ConstantOrWriteNode.name")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("ConstantOrWriteNode.name_loc")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("ConstantOrWriteNode.operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("ConstantOrWriteNode.value")).parse_next(input)?,
      }
    ),
    36 => NodeKind::ConstantPathAndWriteNode(
      ConstantPathAndWriteNode {
        target: parse_node.context(winnow::error::StrContext::Label("ConstantPathAndWriteNode.target")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("ConstantPathAndWriteNode.operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("ConstantPathAndWriteNode.value")).parse_next(input)?,
      }
    ),
    37 => NodeKind::ConstantPathNode(
      ConstantPathNode {
        parent: parse_optional_node.context(winnow::error::StrContext::Label("ConstantPathNode.parent")).parse_next(input)?,
        name: parse_optional_constant.context(winnow::error::StrContext::Label("ConstantPathNode.name")).parse_next(input)?,
        delimiter_loc: parse_location.context(winnow::error::StrContext::Label("ConstantPathNode.delimiter_loc")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("ConstantPathNode.name_loc")).parse_next(input)?,
      }
    ),
    38 => NodeKind::ConstantPathOperatorWriteNode(
      ConstantPathOperatorWriteNode {
        target: parse_node.context(winnow::error::StrContext::Label("ConstantPathOperatorWriteNode.target")).parse_next(input)?,
        binary_operator_loc: parse_location.context(winnow::error::StrContext::Label("ConstantPathOperatorWriteNode.binary_operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("ConstantPathOperatorWriteNode.value")).parse_next(input)?,
        binary_operator: parse_constant.context(winnow::error::StrContext::Label("ConstantPathOperatorWriteNode.binary_operator")).parse_next(input)?,
      }
    ),
    39 => NodeKind::ConstantPathOrWriteNode(
      ConstantPathOrWriteNode {
        target: parse_node.context(winnow::error::StrContext::Label("ConstantPathOrWriteNode.target")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("ConstantPathOrWriteNode.operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("ConstantPathOrWriteNode.value")).parse_next(input)?,
      }
    ),
    40 => NodeKind::ConstantPathTargetNode(
      ConstantPathTargetNode {
        parent: parse_optional_node.context(winnow::error::StrContext::Label("ConstantPathTargetNode.parent")).parse_next(input)?,
        name: parse_optional_constant.context(winnow::error::StrContext::Label("ConstantPathTargetNode.name")).parse_next(input)?,
        delimiter_loc: parse_location.context(winnow::error::StrContext::Label("ConstantPathTargetNode.delimiter_loc")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("ConstantPathTargetNode.name_loc")).parse_next(input)?,
      }
    ),
    41 => NodeKind::ConstantPathWriteNode(
      ConstantPathWriteNode {
        target: parse_node.context(winnow::error::StrContext::Label("ConstantPathWriteNode.target")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("ConstantPathWriteNode.operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("ConstantPathWriteNode.value")).parse_next(input)?,
      }
    ),
    42 => NodeKind::ConstantReadNode(
      ConstantReadNode {
        name: parse_constant.context(winnow::error::StrContext::Label("ConstantReadNode.name")).parse_next(input)?,
      }
    ),
    43 => NodeKind::ConstantTargetNode(
      ConstantTargetNode {
        name: parse_constant.context(winnow::error::StrContext::Label("ConstantTargetNode.name")).parse_next(input)?,
      }
    ),
    44 => NodeKind::ConstantWriteNode(
      ConstantWriteNode {
        name: parse_constant.context(winnow::error::StrContext::Label("ConstantWriteNode.name")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("ConstantWriteNode.name_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("ConstantWriteNode.value")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("ConstantWriteNode.operator_loc")).parse_next(input)?,
      }
    ),
    45 => NodeKind::DefNode(
      DefNode {
        name: parse_constant.context(winnow::error::StrContext::Label("DefNode.name")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("DefNode.name_loc")).parse_next(input)?,
        receiver: parse_optional_node.context(winnow::error::StrContext::Label("DefNode.receiver")).parse_next(input)?,
        parameters: parse_optional_node.context(winnow::error::StrContext::Label("DefNode.parameters")).parse_next(input)?,
        body: parse_optional_node.context(winnow::error::StrContext::Label("DefNode.body")).parse_next(input)?,
        locals: length_repeat(parse_varuint, parse_constant).context(winnow::error::StrContext::Label("DefNode.locals")).parse_next(input)?,
        def_keyword_loc: parse_location.context(winnow::error::StrContext::Label("DefNode.def_keyword_loc")).parse_next(input)?,
        operator_loc: parse_optional_location.context(winnow::error::StrContext::Label("DefNode.operator_loc")).parse_next(input)?,
        lparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("DefNode.lparen_loc")).parse_next(input)?,
        rparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("DefNode.rparen_loc")).parse_next(input)?,
        equal_loc: parse_optional_location.context(winnow::error::StrContext::Label("DefNode.equal_loc")).parse_next(input)?,
        end_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("DefNode.end_keyword_loc")).parse_next(input)?,
      }
    ),
    46 => NodeKind::DefinedNode(
      DefinedNode {
        lparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("DefinedNode.lparen_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("DefinedNode.value")).parse_next(input)?,
        rparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("DefinedNode.rparen_loc")).parse_next(input)?,
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("DefinedNode.keyword_loc")).parse_next(input)?,
      }
    ),
    47 => NodeKind::ElseNode(
      ElseNode {
        else_keyword_loc: parse_location.context(winnow::error::StrContext::Label("ElseNode.else_keyword_loc")).parse_next(input)?,
        statements: parse_optional_node.context(winnow::error::StrContext::Label("ElseNode.statements")).parse_next(input)?,
        end_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("ElseNode.end_keyword_loc")).parse_next(input)?,
      }
    ),
    48 => NodeKind::EmbeddedStatementsNode(
      EmbeddedStatementsNode {
        opening_loc: parse_location.context(winnow::error::StrContext::Label("EmbeddedStatementsNode.opening_loc")).parse_next(input)?,
        statements: parse_optional_node.context(winnow::error::StrContext::Label("EmbeddedStatementsNode.statements")).parse_next(input)?,
        closing_loc: parse_location.context(winnow::error::StrContext::Label("EmbeddedStatementsNode.closing_loc")).parse_next(input)?,
      }
    ),
    49 => NodeKind::EmbeddedVariableNode(
      EmbeddedVariableNode {
        operator_loc: parse_location.context(winnow::error::StrContext::Label("EmbeddedVariableNode.operator_loc")).parse_next(input)?,
        variable: parse_node.context(winnow::error::StrContext::Label("EmbeddedVariableNode.variable")).parse_next(input)?,
      }
    ),
    50 => NodeKind::EnsureNode(
      EnsureNode {
        ensure_keyword_loc: parse_location.context(winnow::error::StrContext::Label("EnsureNode.ensure_keyword_loc")).parse_next(input)?,
        statements: parse_optional_node.context(winnow::error::StrContext::Label("EnsureNode.statements")).parse_next(input)?,
        end_keyword_loc: parse_location.context(winnow::error::StrContext::Label("EnsureNode.end_keyword_loc")).parse_next(input)?,
      }
    ),
    51 => NodeKind::FalseNode(
      FalseNode {
        0: (),
      }
    ),
    52 => NodeKind::FindPatternNode(
      FindPatternNode {
        constant: parse_optional_node.context(winnow::error::StrContext::Label("FindPatternNode.constant")).parse_next(input)?,
        left: parse_node.context(winnow::error::StrContext::Label("FindPatternNode.left")).parse_next(input)?,
        requireds: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("FindPatternNode.requireds")).parse_next(input)?,
        right: parse_node.context(winnow::error::StrContext::Label("FindPatternNode.right")).parse_next(input)?,
        opening_loc: parse_optional_location.context(winnow::error::StrContext::Label("FindPatternNode.opening_loc")).parse_next(input)?,
        closing_loc: parse_optional_location.context(winnow::error::StrContext::Label("FindPatternNode.closing_loc")).parse_next(input)?,
      }
    ),
    53 => NodeKind::FlipFlopNode(
      FlipFlopNode {
        left: parse_optional_node.context(winnow::error::StrContext::Label("FlipFlopNode.left")).parse_next(input)?,
        right: parse_optional_node.context(winnow::error::StrContext::Label("FlipFlopNode.right")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("FlipFlopNode.operator_loc")).parse_next(input)?,
      }
    ),
    54 => NodeKind::FloatNode(
      FloatNode {
        value: parse_double.context(winnow::error::StrContext::Label("FloatNode.value")).parse_next(input)?,
      }
    ),
    55 => NodeKind::ForNode(
      ForNode {
        index: parse_node.context(winnow::error::StrContext::Label("ForNode.index")).parse_next(input)?,
        collection: parse_node.context(winnow::error::StrContext::Label("ForNode.collection")).parse_next(input)?,
        statements: parse_optional_node.context(winnow::error::StrContext::Label("ForNode.statements")).parse_next(input)?,
        for_keyword_loc: parse_location.context(winnow::error::StrContext::Label("ForNode.for_keyword_loc")).parse_next(input)?,
        in_keyword_loc: parse_location.context(winnow::error::StrContext::Label("ForNode.in_keyword_loc")).parse_next(input)?,
        do_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("ForNode.do_keyword_loc")).parse_next(input)?,
        end_keyword_loc: parse_location.context(winnow::error::StrContext::Label("ForNode.end_keyword_loc")).parse_next(input)?,
      }
    ),
    56 => NodeKind::ForwardingArgumentsNode(
      ForwardingArgumentsNode {
        0: (),
      }
    ),
    57 => NodeKind::ForwardingParameterNode(
      ForwardingParameterNode {
        0: (),
      }
    ),
    58 => NodeKind::ForwardingSuperNode(
      ForwardingSuperNode {
        block: parse_optional_node.context(winnow::error::StrContext::Label("ForwardingSuperNode.block")).parse_next(input)?,
      }
    ),
    59 => NodeKind::GlobalVariableAndWriteNode(
      GlobalVariableAndWriteNode {
        name: parse_constant.context(winnow::error::StrContext::Label("GlobalVariableAndWriteNode.name")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("GlobalVariableAndWriteNode.name_loc")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("GlobalVariableAndWriteNode.operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("GlobalVariableAndWriteNode.value")).parse_next(input)?,
      }
    ),
    60 => NodeKind::GlobalVariableOperatorWriteNode(
      GlobalVariableOperatorWriteNode {
        name: parse_constant.context(winnow::error::StrContext::Label("GlobalVariableOperatorWriteNode.name")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("GlobalVariableOperatorWriteNode.name_loc")).parse_next(input)?,
        binary_operator_loc: parse_location.context(winnow::error::StrContext::Label("GlobalVariableOperatorWriteNode.binary_operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("GlobalVariableOperatorWriteNode.value")).parse_next(input)?,
        binary_operator: parse_constant.context(winnow::error::StrContext::Label("GlobalVariableOperatorWriteNode.binary_operator")).parse_next(input)?,
      }
    ),
    61 => NodeKind::GlobalVariableOrWriteNode(
      GlobalVariableOrWriteNode {
        name: parse_constant.context(winnow::error::StrContext::Label("GlobalVariableOrWriteNode.name")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("GlobalVariableOrWriteNode.name_loc")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("GlobalVariableOrWriteNode.operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("GlobalVariableOrWriteNode.value")).parse_next(input)?,
      }
    ),
    62 => NodeKind::GlobalVariableReadNode(
      GlobalVariableReadNode {
        name: parse_constant.context(winnow::error::StrContext::Label("GlobalVariableReadNode.name")).parse_next(input)?,
      }
    ),
    63 => NodeKind::GlobalVariableTargetNode(
      GlobalVariableTargetNode {
        name: parse_constant.context(winnow::error::StrContext::Label("GlobalVariableTargetNode.name")).parse_next(input)?,
      }
    ),
    64 => NodeKind::GlobalVariableWriteNode(
      GlobalVariableWriteNode {
        name: parse_constant.context(winnow::error::StrContext::Label("GlobalVariableWriteNode.name")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("GlobalVariableWriteNode.name_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("GlobalVariableWriteNode.value")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("GlobalVariableWriteNode.operator_loc")).parse_next(input)?,
      }
    ),
    65 => NodeKind::HashNode(
      HashNode {
        opening_loc: parse_location.context(winnow::error::StrContext::Label("HashNode.opening_loc")).parse_next(input)?,
        elements: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("HashNode.elements")).parse_next(input)?,
        closing_loc: parse_location.context(winnow::error::StrContext::Label("HashNode.closing_loc")).parse_next(input)?,
      }
    ),
    66 => NodeKind::HashPatternNode(
      HashPatternNode {
        constant: parse_optional_node.context(winnow::error::StrContext::Label("HashPatternNode.constant")).parse_next(input)?,
        elements: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("HashPatternNode.elements")).parse_next(input)?,
        rest: parse_optional_node.context(winnow::error::StrContext::Label("HashPatternNode.rest")).parse_next(input)?,
        opening_loc: parse_optional_location.context(winnow::error::StrContext::Label("HashPatternNode.opening_loc")).parse_next(input)?,
        closing_loc: parse_optional_location.context(winnow::error::StrContext::Label("HashPatternNode.closing_loc")).parse_next(input)?,
      }
    ),
    67 => NodeKind::IfNode(
      IfNode {
        if_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("IfNode.if_keyword_loc")).parse_next(input)?,
        predicate: parse_node.context(winnow::error::StrContext::Label("IfNode.predicate")).parse_next(input)?,
        then_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("IfNode.then_keyword_loc")).parse_next(input)?,
        statements: parse_optional_node.context(winnow::error::StrContext::Label("IfNode.statements")).parse_next(input)?,
        subsequent: parse_optional_node.context(winnow::error::StrContext::Label("IfNode.subsequent")).parse_next(input)?,
        end_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("IfNode.end_keyword_loc")).parse_next(input)?,
      }
    ),
    68 => NodeKind::ImaginaryNode(
      ImaginaryNode {
        numeric: parse_node.context(winnow::error::StrContext::Label("ImaginaryNode.numeric")).parse_next(input)?,
      }
    ),
    69 => NodeKind::ImplicitNode(
      ImplicitNode {
        value: parse_node.context(winnow::error::StrContext::Label("ImplicitNode.value")).parse_next(input)?,
      }
    ),
    70 => NodeKind::ImplicitRestNode(
      ImplicitRestNode {
        0: (),
      }
    ),
    71 => NodeKind::InNode(
      InNode {
        pattern: parse_node.context(winnow::error::StrContext::Label("InNode.pattern")).parse_next(input)?,
        statements: parse_optional_node.context(winnow::error::StrContext::Label("InNode.statements")).parse_next(input)?,
        in_loc: parse_location.context(winnow::error::StrContext::Label("InNode.in_loc")).parse_next(input)?,
        then_loc: parse_optional_location.context(winnow::error::StrContext::Label("InNode.then_loc")).parse_next(input)?,
      }
    ),
    72 => NodeKind::IndexAndWriteNode(
      IndexAndWriteNode {
        receiver: parse_optional_node.context(winnow::error::StrContext::Label("IndexAndWriteNode.receiver")).parse_next(input)?,
        call_operator_loc: parse_optional_location.context(winnow::error::StrContext::Label("IndexAndWriteNode.call_operator_loc")).parse_next(input)?,
        opening_loc: parse_location.context(winnow::error::StrContext::Label("IndexAndWriteNode.opening_loc")).parse_next(input)?,
        arguments: parse_optional_node.context(winnow::error::StrContext::Label("IndexAndWriteNode.arguments")).parse_next(input)?,
        closing_loc: parse_location.context(winnow::error::StrContext::Label("IndexAndWriteNode.closing_loc")).parse_next(input)?,
        block: parse_optional_node.context(winnow::error::StrContext::Label("IndexAndWriteNode.block")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("IndexAndWriteNode.operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("IndexAndWriteNode.value")).parse_next(input)?,
      }
    ),
    73 => NodeKind::IndexOperatorWriteNode(
      IndexOperatorWriteNode {
        receiver: parse_optional_node.context(winnow::error::StrContext::Label("IndexOperatorWriteNode.receiver")).parse_next(input)?,
        call_operator_loc: parse_optional_location.context(winnow::error::StrContext::Label("IndexOperatorWriteNode.call_operator_loc")).parse_next(input)?,
        opening_loc: parse_location.context(winnow::error::StrContext::Label("IndexOperatorWriteNode.opening_loc")).parse_next(input)?,
        arguments: parse_optional_node.context(winnow::error::StrContext::Label("IndexOperatorWriteNode.arguments")).parse_next(input)?,
        closing_loc: parse_location.context(winnow::error::StrContext::Label("IndexOperatorWriteNode.closing_loc")).parse_next(input)?,
        block: parse_optional_node.context(winnow::error::StrContext::Label("IndexOperatorWriteNode.block")).parse_next(input)?,
        binary_operator: parse_constant.context(winnow::error::StrContext::Label("IndexOperatorWriteNode.binary_operator")).parse_next(input)?,
        binary_operator_loc: parse_location.context(winnow::error::StrContext::Label("IndexOperatorWriteNode.binary_operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("IndexOperatorWriteNode.value")).parse_next(input)?,
      }
    ),
    74 => NodeKind::IndexOrWriteNode(
      IndexOrWriteNode {
        receiver: parse_optional_node.context(winnow::error::StrContext::Label("IndexOrWriteNode.receiver")).parse_next(input)?,
        call_operator_loc: parse_optional_location.context(winnow::error::StrContext::Label("IndexOrWriteNode.call_operator_loc")).parse_next(input)?,
        opening_loc: parse_location.context(winnow::error::StrContext::Label("IndexOrWriteNode.opening_loc")).parse_next(input)?,
        arguments: parse_optional_node.context(winnow::error::StrContext::Label("IndexOrWriteNode.arguments")).parse_next(input)?,
        closing_loc: parse_location.context(winnow::error::StrContext::Label("IndexOrWriteNode.closing_loc")).parse_next(input)?,
        block: parse_optional_node.context(winnow::error::StrContext::Label("IndexOrWriteNode.block")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("IndexOrWriteNode.operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("IndexOrWriteNode.value")).parse_next(input)?,
      }
    ),
    75 => NodeKind::IndexTargetNode(
      IndexTargetNode {
        receiver: parse_node.context(winnow::error::StrContext::Label("IndexTargetNode.receiver")).parse_next(input)?,
        opening_loc: parse_location.context(winnow::error::StrContext::Label("IndexTargetNode.opening_loc")).parse_next(input)?,
        arguments: parse_optional_node.context(winnow::error::StrContext::Label("IndexTargetNode.arguments")).parse_next(input)?,
        closing_loc: parse_location.context(winnow::error::StrContext::Label("IndexTargetNode.closing_loc")).parse_next(input)?,
        block: parse_optional_node.context(winnow::error::StrContext::Label("IndexTargetNode.block")).parse_next(input)?,
      }
    ),
    76 => NodeKind::InstanceVariableAndWriteNode(
      InstanceVariableAndWriteNode {
        name: parse_constant.context(winnow::error::StrContext::Label("InstanceVariableAndWriteNode.name")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("InstanceVariableAndWriteNode.name_loc")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("InstanceVariableAndWriteNode.operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("InstanceVariableAndWriteNode.value")).parse_next(input)?,
      }
    ),
    77 => NodeKind::InstanceVariableOperatorWriteNode(
      InstanceVariableOperatorWriteNode {
        name: parse_constant.context(winnow::error::StrContext::Label("InstanceVariableOperatorWriteNode.name")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("InstanceVariableOperatorWriteNode.name_loc")).parse_next(input)?,
        binary_operator_loc: parse_location.context(winnow::error::StrContext::Label("InstanceVariableOperatorWriteNode.binary_operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("InstanceVariableOperatorWriteNode.value")).parse_next(input)?,
        binary_operator: parse_constant.context(winnow::error::StrContext::Label("InstanceVariableOperatorWriteNode.binary_operator")).parse_next(input)?,
      }
    ),
    78 => NodeKind::InstanceVariableOrWriteNode(
      InstanceVariableOrWriteNode {
        name: parse_constant.context(winnow::error::StrContext::Label("InstanceVariableOrWriteNode.name")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("InstanceVariableOrWriteNode.name_loc")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("InstanceVariableOrWriteNode.operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("InstanceVariableOrWriteNode.value")).parse_next(input)?,
      }
    ),
    79 => NodeKind::InstanceVariableReadNode(
      InstanceVariableReadNode {
        name: parse_constant.context(winnow::error::StrContext::Label("InstanceVariableReadNode.name")).parse_next(input)?,
      }
    ),
    80 => NodeKind::InstanceVariableTargetNode(
      InstanceVariableTargetNode {
        name: parse_constant.context(winnow::error::StrContext::Label("InstanceVariableTargetNode.name")).parse_next(input)?,
      }
    ),
    81 => NodeKind::InstanceVariableWriteNode(
      InstanceVariableWriteNode {
        name: parse_constant.context(winnow::error::StrContext::Label("InstanceVariableWriteNode.name")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("InstanceVariableWriteNode.name_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("InstanceVariableWriteNode.value")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("InstanceVariableWriteNode.operator_loc")).parse_next(input)?,
      }
    ),
    82 => NodeKind::IntegerNode(
      IntegerNode {
        value: parse_integer.context(winnow::error::StrContext::Label("IntegerNode.value")).parse_next(input)?,
      }
    ),
    83 => NodeKind::InterpolatedMatchLastLineNode(
      InterpolatedMatchLastLineNode {
        opening_loc: parse_location.context(winnow::error::StrContext::Label("InterpolatedMatchLastLineNode.opening_loc")).parse_next(input)?,
        parts: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("InterpolatedMatchLastLineNode.parts")).parse_next(input)?,
        closing_loc: parse_location.context(winnow::error::StrContext::Label("InterpolatedMatchLastLineNode.closing_loc")).parse_next(input)?,
      }
    ),
    84 => NodeKind::InterpolatedRegularExpressionNode(
      InterpolatedRegularExpressionNode {
        opening_loc: parse_location.context(winnow::error::StrContext::Label("InterpolatedRegularExpressionNode.opening_loc")).parse_next(input)?,
        parts: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("InterpolatedRegularExpressionNode.parts")).parse_next(input)?,
        closing_loc: parse_location.context(winnow::error::StrContext::Label("InterpolatedRegularExpressionNode.closing_loc")).parse_next(input)?,
      }
    ),
    85 => NodeKind::InterpolatedStringNode(
      InterpolatedStringNode {
        opening_loc: parse_optional_location.context(winnow::error::StrContext::Label("InterpolatedStringNode.opening_loc")).parse_next(input)?,
        parts: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("InterpolatedStringNode.parts")).parse_next(input)?,
        closing_loc: parse_optional_location.context(winnow::error::StrContext::Label("InterpolatedStringNode.closing_loc")).parse_next(input)?,
      }
    ),
    86 => NodeKind::InterpolatedSymbolNode(
      InterpolatedSymbolNode {
        opening_loc: parse_optional_location.context(winnow::error::StrContext::Label("InterpolatedSymbolNode.opening_loc")).parse_next(input)?,
        parts: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("InterpolatedSymbolNode.parts")).parse_next(input)?,
        closing_loc: parse_optional_location.context(winnow::error::StrContext::Label("InterpolatedSymbolNode.closing_loc")).parse_next(input)?,
      }
    ),
    87 => NodeKind::InterpolatedXStringNode(
      InterpolatedXStringNode {
        opening_loc: parse_location.context(winnow::error::StrContext::Label("InterpolatedXStringNode.opening_loc")).parse_next(input)?,
        parts: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("InterpolatedXStringNode.parts")).parse_next(input)?,
        closing_loc: parse_location.context(winnow::error::StrContext::Label("InterpolatedXStringNode.closing_loc")).parse_next(input)?,
      }
    ),
    88 => NodeKind::ItLocalVariableReadNode(
      ItLocalVariableReadNode {
        0: (),
      }
    ),
    89 => NodeKind::ItParametersNode(
      ItParametersNode {
        0: (),
      }
    ),
    90 => NodeKind::KeywordHashNode(
      KeywordHashNode {
        elements: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("KeywordHashNode.elements")).parse_next(input)?,
      }
    ),
    91 => NodeKind::KeywordRestParameterNode(
      KeywordRestParameterNode {
        name: parse_optional_constant.context(winnow::error::StrContext::Label("KeywordRestParameterNode.name")).parse_next(input)?,
        name_loc: parse_optional_location.context(winnow::error::StrContext::Label("KeywordRestParameterNode.name_loc")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("KeywordRestParameterNode.operator_loc")).parse_next(input)?,
      }
    ),
    92 => NodeKind::LambdaNode(
      LambdaNode {
        locals: length_repeat(parse_varuint, parse_constant).context(winnow::error::StrContext::Label("LambdaNode.locals")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("LambdaNode.operator_loc")).parse_next(input)?,
        opening_loc: parse_location.context(winnow::error::StrContext::Label("LambdaNode.opening_loc")).parse_next(input)?,
        closing_loc: parse_location.context(winnow::error::StrContext::Label("LambdaNode.closing_loc")).parse_next(input)?,
        parameters: parse_optional_node.context(winnow::error::StrContext::Label("LambdaNode.parameters")).parse_next(input)?,
        body: parse_optional_node.context(winnow::error::StrContext::Label("LambdaNode.body")).parse_next(input)?,
      }
    ),
    93 => NodeKind::LocalVariableAndWriteNode(
      LocalVariableAndWriteNode {
        name_loc: parse_location.context(winnow::error::StrContext::Label("LocalVariableAndWriteNode.name_loc")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("LocalVariableAndWriteNode.operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("LocalVariableAndWriteNode.value")).parse_next(input)?,
        name: parse_constant.context(winnow::error::StrContext::Label("LocalVariableAndWriteNode.name")).parse_next(input)?,
        depth: parse_varuint.context(winnow::error::StrContext::Label("LocalVariableAndWriteNode.depth")).parse_next(input)?,
      }
    ),
    94 => NodeKind::LocalVariableOperatorWriteNode(
      LocalVariableOperatorWriteNode {
        name_loc: parse_location.context(winnow::error::StrContext::Label("LocalVariableOperatorWriteNode.name_loc")).parse_next(input)?,
        binary_operator_loc: parse_location.context(winnow::error::StrContext::Label("LocalVariableOperatorWriteNode.binary_operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("LocalVariableOperatorWriteNode.value")).parse_next(input)?,
        name: parse_constant.context(winnow::error::StrContext::Label("LocalVariableOperatorWriteNode.name")).parse_next(input)?,
        binary_operator: parse_constant.context(winnow::error::StrContext::Label("LocalVariableOperatorWriteNode.binary_operator")).parse_next(input)?,
        depth: parse_varuint.context(winnow::error::StrContext::Label("LocalVariableOperatorWriteNode.depth")).parse_next(input)?,
      }
    ),
    95 => NodeKind::LocalVariableOrWriteNode(
      LocalVariableOrWriteNode {
        name_loc: parse_location.context(winnow::error::StrContext::Label("LocalVariableOrWriteNode.name_loc")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("LocalVariableOrWriteNode.operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("LocalVariableOrWriteNode.value")).parse_next(input)?,
        name: parse_constant.context(winnow::error::StrContext::Label("LocalVariableOrWriteNode.name")).parse_next(input)?,
        depth: parse_varuint.context(winnow::error::StrContext::Label("LocalVariableOrWriteNode.depth")).parse_next(input)?,
      }
    ),
    96 => NodeKind::LocalVariableReadNode(
      LocalVariableReadNode {
        name: parse_constant.context(winnow::error::StrContext::Label("LocalVariableReadNode.name")).parse_next(input)?,
        depth: parse_varuint.context(winnow::error::StrContext::Label("LocalVariableReadNode.depth")).parse_next(input)?,
      }
    ),
    97 => NodeKind::LocalVariableTargetNode(
      LocalVariableTargetNode {
        name: parse_constant.context(winnow::error::StrContext::Label("LocalVariableTargetNode.name")).parse_next(input)?,
        depth: parse_varuint.context(winnow::error::StrContext::Label("LocalVariableTargetNode.depth")).parse_next(input)?,
      }
    ),
    98 => NodeKind::LocalVariableWriteNode(
      LocalVariableWriteNode {
        name: parse_constant.context(winnow::error::StrContext::Label("LocalVariableWriteNode.name")).parse_next(input)?,
        depth: parse_varuint.context(winnow::error::StrContext::Label("LocalVariableWriteNode.depth")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("LocalVariableWriteNode.name_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("LocalVariableWriteNode.value")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("LocalVariableWriteNode.operator_loc")).parse_next(input)?,
      }
    ),
    99 => NodeKind::MatchLastLineNode(
      MatchLastLineNode {
        opening_loc: parse_location.context(winnow::error::StrContext::Label("MatchLastLineNode.opening_loc")).parse_next(input)?,
        content_loc: parse_location.context(winnow::error::StrContext::Label("MatchLastLineNode.content_loc")).parse_next(input)?,
        closing_loc: parse_location.context(winnow::error::StrContext::Label("MatchLastLineNode.closing_loc")).parse_next(input)?,
        unescaped: parse_string_field.context(winnow::error::StrContext::Label("MatchLastLineNode.unescaped")).parse_next(input)?,
      }
    ),
    100 => NodeKind::MatchPredicateNode(
      MatchPredicateNode {
        value: parse_node.context(winnow::error::StrContext::Label("MatchPredicateNode.value")).parse_next(input)?,
        pattern: parse_node.context(winnow::error::StrContext::Label("MatchPredicateNode.pattern")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("MatchPredicateNode.operator_loc")).parse_next(input)?,
      }
    ),
    101 => NodeKind::MatchRequiredNode(
      MatchRequiredNode {
        value: parse_node.context(winnow::error::StrContext::Label("MatchRequiredNode.value")).parse_next(input)?,
        pattern: parse_node.context(winnow::error::StrContext::Label("MatchRequiredNode.pattern")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("MatchRequiredNode.operator_loc")).parse_next(input)?,
      }
    ),
    102 => NodeKind::MatchWriteNode(
      MatchWriteNode {
        call: parse_node.context(winnow::error::StrContext::Label("MatchWriteNode.call")).parse_next(input)?,
        targets: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("MatchWriteNode.targets")).parse_next(input)?,
      }
    ),
    103 => NodeKind::MissingNode(
      MissingNode {
        0: (),
      }
    ),
    104 => NodeKind::ModuleNode(
      ModuleNode {
        locals: length_repeat(parse_varuint, parse_constant).context(winnow::error::StrContext::Label("ModuleNode.locals")).parse_next(input)?,
        module_keyword_loc: parse_location.context(winnow::error::StrContext::Label("ModuleNode.module_keyword_loc")).parse_next(input)?,
        constant_path: parse_node.context(winnow::error::StrContext::Label("ModuleNode.constant_path")).parse_next(input)?,
        body: parse_optional_node.context(winnow::error::StrContext::Label("ModuleNode.body")).parse_next(input)?,
        end_keyword_loc: parse_location.context(winnow::error::StrContext::Label("ModuleNode.end_keyword_loc")).parse_next(input)?,
        name: parse_constant.context(winnow::error::StrContext::Label("ModuleNode.name")).parse_next(input)?,
      }
    ),
    105 => NodeKind::MultiTargetNode(
      MultiTargetNode {
        lefts: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("MultiTargetNode.lefts")).parse_next(input)?,
        rest: parse_optional_node.context(winnow::error::StrContext::Label("MultiTargetNode.rest")).parse_next(input)?,
        rights: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("MultiTargetNode.rights")).parse_next(input)?,
        lparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("MultiTargetNode.lparen_loc")).parse_next(input)?,
        rparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("MultiTargetNode.rparen_loc")).parse_next(input)?,
      }
    ),
    106 => NodeKind::MultiWriteNode(
      MultiWriteNode {
        lefts: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("MultiWriteNode.lefts")).parse_next(input)?,
        rest: parse_optional_node.context(winnow::error::StrContext::Label("MultiWriteNode.rest")).parse_next(input)?,
        rights: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("MultiWriteNode.rights")).parse_next(input)?,
        lparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("MultiWriteNode.lparen_loc")).parse_next(input)?,
        rparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("MultiWriteNode.rparen_loc")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("MultiWriteNode.operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("MultiWriteNode.value")).parse_next(input)?,
      }
    ),
    107 => NodeKind::NextNode(
      NextNode {
        arguments: parse_optional_node.context(winnow::error::StrContext::Label("NextNode.arguments")).parse_next(input)?,
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("NextNode.keyword_loc")).parse_next(input)?,
      }
    ),
    108 => NodeKind::NilNode(
      NilNode {
        0: (),
      }
    ),
    109 => NodeKind::NoKeywordsParameterNode(
      NoKeywordsParameterNode {
        operator_loc: parse_location.context(winnow::error::StrContext::Label("NoKeywordsParameterNode.operator_loc")).parse_next(input)?,
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("NoKeywordsParameterNode.keyword_loc")).parse_next(input)?,
      }
    ),
    110 => NodeKind::NumberedParametersNode(
      NumberedParametersNode {
        maximum: winnow::binary::u8.context(winnow::error::StrContext::Label("NumberedParametersNode.maximum")).parse_next(input)?,
      }
    ),
    111 => NodeKind::NumberedReferenceReadNode(
      NumberedReferenceReadNode {
        number: parse_varuint.context(winnow::error::StrContext::Label("NumberedReferenceReadNode.number")).parse_next(input)?,
      }
    ),
    112 => NodeKind::OptionalKeywordParameterNode(
      OptionalKeywordParameterNode {
        name: parse_constant.context(winnow::error::StrContext::Label("OptionalKeywordParameterNode.name")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("OptionalKeywordParameterNode.name_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("OptionalKeywordParameterNode.value")).parse_next(input)?,
      }
    ),
    113 => NodeKind::OptionalParameterNode(
      OptionalParameterNode {
        name: parse_constant.context(winnow::error::StrContext::Label("OptionalParameterNode.name")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("OptionalParameterNode.name_loc")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("OptionalParameterNode.operator_loc")).parse_next(input)?,
        value: parse_node.context(winnow::error::StrContext::Label("OptionalParameterNode.value")).parse_next(input)?,
      }
    ),
    114 => NodeKind::OrNode(
      OrNode {
        left: parse_node.context(winnow::error::StrContext::Label("OrNode.left")).parse_next(input)?,
        right: parse_node.context(winnow::error::StrContext::Label("OrNode.right")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("OrNode.operator_loc")).parse_next(input)?,
      }
    ),
    115 => NodeKind::ParametersNode(
      ParametersNode {
        requireds: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("ParametersNode.requireds")).parse_next(input)?,
        optionals: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("ParametersNode.optionals")).parse_next(input)?,
        rest: parse_optional_node.context(winnow::error::StrContext::Label("ParametersNode.rest")).parse_next(input)?,
        posts: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("ParametersNode.posts")).parse_next(input)?,
        keywords: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("ParametersNode.keywords")).parse_next(input)?,
        keyword_rest: parse_optional_node.context(winnow::error::StrContext::Label("ParametersNode.keyword_rest")).parse_next(input)?,
        block: parse_optional_node.context(winnow::error::StrContext::Label("ParametersNode.block")).parse_next(input)?,
      }
    ),
    116 => NodeKind::ParenthesesNode(
      ParenthesesNode {
        body: parse_optional_node.context(winnow::error::StrContext::Label("ParenthesesNode.body")).parse_next(input)?,
        opening_loc: parse_location.context(winnow::error::StrContext::Label("ParenthesesNode.opening_loc")).parse_next(input)?,
        closing_loc: parse_location.context(winnow::error::StrContext::Label("ParenthesesNode.closing_loc")).parse_next(input)?,
      }
    ),
    117 => NodeKind::PinnedExpressionNode(
      PinnedExpressionNode {
        expression: parse_node.context(winnow::error::StrContext::Label("PinnedExpressionNode.expression")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("PinnedExpressionNode.operator_loc")).parse_next(input)?,
        lparen_loc: parse_location.context(winnow::error::StrContext::Label("PinnedExpressionNode.lparen_loc")).parse_next(input)?,
        rparen_loc: parse_location.context(winnow::error::StrContext::Label("PinnedExpressionNode.rparen_loc")).parse_next(input)?,
      }
    ),
    118 => NodeKind::PinnedVariableNode(
      PinnedVariableNode {
        variable: parse_node.context(winnow::error::StrContext::Label("PinnedVariableNode.variable")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("PinnedVariableNode.operator_loc")).parse_next(input)?,
      }
    ),
    119 => NodeKind::PostExecutionNode(
      PostExecutionNode {
        statements: parse_optional_node.context(winnow::error::StrContext::Label("PostExecutionNode.statements")).parse_next(input)?,
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("PostExecutionNode.keyword_loc")).parse_next(input)?,
        opening_loc: parse_location.context(winnow::error::StrContext::Label("PostExecutionNode.opening_loc")).parse_next(input)?,
        closing_loc: parse_location.context(winnow::error::StrContext::Label("PostExecutionNode.closing_loc")).parse_next(input)?,
      }
    ),
    120 => NodeKind::PreExecutionNode(
      PreExecutionNode {
        statements: parse_optional_node.context(winnow::error::StrContext::Label("PreExecutionNode.statements")).parse_next(input)?,
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("PreExecutionNode.keyword_loc")).parse_next(input)?,
        opening_loc: parse_location.context(winnow::error::StrContext::Label("PreExecutionNode.opening_loc")).parse_next(input)?,
        closing_loc: parse_location.context(winnow::error::StrContext::Label("PreExecutionNode.closing_loc")).parse_next(input)?,
      }
    ),
    121 => NodeKind::ProgramNode(
      ProgramNode {
        locals: length_repeat(parse_varuint, parse_constant).context(winnow::error::StrContext::Label("ProgramNode.locals")).parse_next(input)?,
        statements: parse_node.context(winnow::error::StrContext::Label("ProgramNode.statements")).parse_next(input)?,
      }
    ),
    122 => NodeKind::RangeNode(
      RangeNode {
        left: parse_optional_node.context(winnow::error::StrContext::Label("RangeNode.left")).parse_next(input)?,
        right: parse_optional_node.context(winnow::error::StrContext::Label("RangeNode.right")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("RangeNode.operator_loc")).parse_next(input)?,
      }
    ),
    123 => NodeKind::RationalNode(
      RationalNode {
        numerator: parse_integer.context(winnow::error::StrContext::Label("RationalNode.numerator")).parse_next(input)?,
        denominator: parse_integer.context(winnow::error::StrContext::Label("RationalNode.denominator")).parse_next(input)?,
      }
    ),
    124 => NodeKind::RedoNode(
      RedoNode {
        0: (),
      }
    ),
    125 => NodeKind::RegularExpressionNode(
      RegularExpressionNode {
        opening_loc: parse_location.context(winnow::error::StrContext::Label("RegularExpressionNode.opening_loc")).parse_next(input)?,
        content_loc: parse_location.context(winnow::error::StrContext::Label("RegularExpressionNode.content_loc")).parse_next(input)?,
        closing_loc: parse_location.context(winnow::error::StrContext::Label("RegularExpressionNode.closing_loc")).parse_next(input)?,
        unescaped: parse_string_field.context(winnow::error::StrContext::Label("RegularExpressionNode.unescaped")).parse_next(input)?,
      }
    ),
    126 => NodeKind::RequiredKeywordParameterNode(
      RequiredKeywordParameterNode {
        name: parse_constant.context(winnow::error::StrContext::Label("RequiredKeywordParameterNode.name")).parse_next(input)?,
        name_loc: parse_location.context(winnow::error::StrContext::Label("RequiredKeywordParameterNode.name_loc")).parse_next(input)?,
      }
    ),
    127 => NodeKind::RequiredParameterNode(
      RequiredParameterNode {
        name: parse_constant.context(winnow::error::StrContext::Label("RequiredParameterNode.name")).parse_next(input)?,
      }
    ),
    128 => NodeKind::RescueModifierNode(
      RescueModifierNode {
        expression: parse_node.context(winnow::error::StrContext::Label("RescueModifierNode.expression")).parse_next(input)?,
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("RescueModifierNode.keyword_loc")).parse_next(input)?,
        rescue_expression: parse_node.context(winnow::error::StrContext::Label("RescueModifierNode.rescue_expression")).parse_next(input)?,
      }
    ),
    129 => NodeKind::RescueNode(
      RescueNode {
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("RescueNode.keyword_loc")).parse_next(input)?,
        exceptions: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("RescueNode.exceptions")).parse_next(input)?,
        operator_loc: parse_optional_location.context(winnow::error::StrContext::Label("RescueNode.operator_loc")).parse_next(input)?,
        reference: parse_optional_node.context(winnow::error::StrContext::Label("RescueNode.reference")).parse_next(input)?,
        then_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("RescueNode.then_keyword_loc")).parse_next(input)?,
        statements: parse_optional_node.context(winnow::error::StrContext::Label("RescueNode.statements")).parse_next(input)?,
        subsequent: parse_optional_node.context(winnow::error::StrContext::Label("RescueNode.subsequent")).parse_next(input)?,
      }
    ),
    130 => NodeKind::RestParameterNode(
      RestParameterNode {
        name: parse_optional_constant.context(winnow::error::StrContext::Label("RestParameterNode.name")).parse_next(input)?,
        name_loc: parse_optional_location.context(winnow::error::StrContext::Label("RestParameterNode.name_loc")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("RestParameterNode.operator_loc")).parse_next(input)?,
      }
    ),
    131 => NodeKind::RetryNode(
      RetryNode {
        0: (),
      }
    ),
    132 => NodeKind::ReturnNode(
      ReturnNode {
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("ReturnNode.keyword_loc")).parse_next(input)?,
        arguments: parse_optional_node.context(winnow::error::StrContext::Label("ReturnNode.arguments")).parse_next(input)?,
      }
    ),
    133 => NodeKind::SelfNode(
      SelfNode {
        0: (),
      }
    ),
    134 => NodeKind::ShareableConstantNode(
      ShareableConstantNode {
        write: parse_node.context(winnow::error::StrContext::Label("ShareableConstantNode.write")).parse_next(input)?,
      }
    ),
    135 => NodeKind::SingletonClassNode(
      SingletonClassNode {
        locals: length_repeat(parse_varuint, parse_constant).context(winnow::error::StrContext::Label("SingletonClassNode.locals")).parse_next(input)?,
        class_keyword_loc: parse_location.context(winnow::error::StrContext::Label("SingletonClassNode.class_keyword_loc")).parse_next(input)?,
        operator_loc: parse_location.context(winnow::error::StrContext::Label("SingletonClassNode.operator_loc")).parse_next(input)?,
        expression: parse_node.context(winnow::error::StrContext::Label("SingletonClassNode.expression")).parse_next(input)?,
        body: parse_optional_node.context(winnow::error::StrContext::Label("SingletonClassNode.body")).parse_next(input)?,
        end_keyword_loc: parse_location.context(winnow::error::StrContext::Label("SingletonClassNode.end_keyword_loc")).parse_next(input)?,
      }
    ),
    136 => NodeKind::SourceEncodingNode(
      SourceEncodingNode {
        0: (),
      }
    ),
    137 => NodeKind::SourceFileNode(
      SourceFileNode {
        filepath: parse_string_field.context(winnow::error::StrContext::Label("SourceFileNode.filepath")).parse_next(input)?,
      }
    ),
    138 => NodeKind::SourceLineNode(
      SourceLineNode {
        0: (),
      }
    ),
    139 => NodeKind::SplatNode(
      SplatNode {
        operator_loc: parse_location.context(winnow::error::StrContext::Label("SplatNode.operator_loc")).parse_next(input)?,
        expression: parse_optional_node.context(winnow::error::StrContext::Label("SplatNode.expression")).parse_next(input)?,
      }
    ),
    140 => NodeKind::StatementsNode(
      StatementsNode {
        body: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("StatementsNode.body")).parse_next(input)?,
      }
    ),
    141 => NodeKind::StringNode(
      StringNode {
        opening_loc: parse_optional_location.context(winnow::error::StrContext::Label("StringNode.opening_loc")).parse_next(input)?,
        content_loc: parse_location.context(winnow::error::StrContext::Label("StringNode.content_loc")).parse_next(input)?,
        closing_loc: parse_optional_location.context(winnow::error::StrContext::Label("StringNode.closing_loc")).parse_next(input)?,
        unescaped: parse_string_field.context(winnow::error::StrContext::Label("StringNode.unescaped")).parse_next(input)?,
      }
    ),
    142 => NodeKind::SuperNode(
      SuperNode {
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("SuperNode.keyword_loc")).parse_next(input)?,
        lparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("SuperNode.lparen_loc")).parse_next(input)?,
        arguments: parse_optional_node.context(winnow::error::StrContext::Label("SuperNode.arguments")).parse_next(input)?,
        rparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("SuperNode.rparen_loc")).parse_next(input)?,
        block: parse_optional_node.context(winnow::error::StrContext::Label("SuperNode.block")).parse_next(input)?,
      }
    ),
    143 => NodeKind::SymbolNode(
      SymbolNode {
        opening_loc: parse_optional_location.context(winnow::error::StrContext::Label("SymbolNode.opening_loc")).parse_next(input)?,
        value_loc: parse_optional_location.context(winnow::error::StrContext::Label("SymbolNode.value_loc")).parse_next(input)?,
        closing_loc: parse_optional_location.context(winnow::error::StrContext::Label("SymbolNode.closing_loc")).parse_next(input)?,
        unescaped: parse_string_field.context(winnow::error::StrContext::Label("SymbolNode.unescaped")).parse_next(input)?,
      }
    ),
    144 => NodeKind::TrueNode(
      TrueNode {
        0: (),
      }
    ),
    145 => NodeKind::UndefNode(
      UndefNode {
        names: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("UndefNode.names")).parse_next(input)?,
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("UndefNode.keyword_loc")).parse_next(input)?,
      }
    ),
    146 => NodeKind::UnlessNode(
      UnlessNode {
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("UnlessNode.keyword_loc")).parse_next(input)?,
        predicate: parse_node.context(winnow::error::StrContext::Label("UnlessNode.predicate")).parse_next(input)?,
        then_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("UnlessNode.then_keyword_loc")).parse_next(input)?,
        statements: parse_optional_node.context(winnow::error::StrContext::Label("UnlessNode.statements")).parse_next(input)?,
        else_clause: parse_optional_node.context(winnow::error::StrContext::Label("UnlessNode.else_clause")).parse_next(input)?,
        end_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("UnlessNode.end_keyword_loc")).parse_next(input)?,
      }
    ),
    147 => NodeKind::UntilNode(
      UntilNode {
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("UntilNode.keyword_loc")).parse_next(input)?,
        do_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("UntilNode.do_keyword_loc")).parse_next(input)?,
        closing_loc: parse_optional_location.context(winnow::error::StrContext::Label("UntilNode.closing_loc")).parse_next(input)?,
        predicate: parse_node.context(winnow::error::StrContext::Label("UntilNode.predicate")).parse_next(input)?,
        statements: parse_optional_node.context(winnow::error::StrContext::Label("UntilNode.statements")).parse_next(input)?,
      }
    ),
    148 => NodeKind::WhenNode(
      WhenNode {
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("WhenNode.keyword_loc")).parse_next(input)?,
        conditions: length_repeat(parse_varuint, parse_node).context(winnow::error::StrContext::Label("WhenNode.conditions")).parse_next(input)?,
        then_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("WhenNode.then_keyword_loc")).parse_next(input)?,
        statements: parse_optional_node.context(winnow::error::StrContext::Label("WhenNode.statements")).parse_next(input)?,
      }
    ),
    149 => NodeKind::WhileNode(
      WhileNode {
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("WhileNode.keyword_loc")).parse_next(input)?,
        do_keyword_loc: parse_optional_location.context(winnow::error::StrContext::Label("WhileNode.do_keyword_loc")).parse_next(input)?,
        closing_loc: parse_optional_location.context(winnow::error::StrContext::Label("WhileNode.closing_loc")).parse_next(input)?,
        predicate: parse_node.context(winnow::error::StrContext::Label("WhileNode.predicate")).parse_next(input)?,
        statements: parse_optional_node.context(winnow::error::StrContext::Label("WhileNode.statements")).parse_next(input)?,
      }
    ),
    150 => NodeKind::XStringNode(
      XStringNode {
        opening_loc: parse_location.context(winnow::error::StrContext::Label("XStringNode.opening_loc")).parse_next(input)?,
        content_loc: parse_location.context(winnow::error::StrContext::Label("XStringNode.content_loc")).parse_next(input)?,
        closing_loc: parse_location.context(winnow::error::StrContext::Label("XStringNode.closing_loc")).parse_next(input)?,
        unescaped: parse_string_field.context(winnow::error::StrContext::Label("XStringNode.unescaped")).parse_next(input)?,
      }
    ),
    151 => NodeKind::YieldNode(
      YieldNode {
        keyword_loc: parse_location.context(winnow::error::StrContext::Label("YieldNode.keyword_loc")).parse_next(input)?,
        lparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("YieldNode.lparen_loc")).parse_next(input)?,
        arguments: parse_optional_node.context(winnow::error::StrContext::Label("YieldNode.arguments")).parse_next(input)?,
        rparen_loc: parse_optional_location.context(winnow::error::StrContext::Label("YieldNode.rparen_loc")).parse_next(input)?,
      }
    ),
      _ => todo!("Unknown node kind: {}", kind),
    };
    Ok(input.state.add_node(Node {
        kind,
        identifier,
        location,
        flags,
        node_kind,
    }))
}
