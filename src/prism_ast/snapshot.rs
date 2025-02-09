use core::fmt;
use std::fmt::Debug;

use enumflags2::BitFlag;
use enumflags2::BitFlags;
#[cfg(test)]
use expect_test::expect;
use pretty::*;

use super::deserialize::*;
use super::generated::*;

trait Snapshot {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()>;
}

impl Snapshot for Location {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        _hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        let (start_line, start_column) = program.line_column(self.start);
        let (end_line, end_column) = program.line_column(self.start + self.length);

        RcDoc::text(format!(
            "({start_line},{start_column})-({end_line},{end_column})",
        ))
    }
}

impl Snapshot for NodeRef {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        program.node(self).snapshot(program, hardline)
    }
}

impl Snapshot for ConstantRef {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        _hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        match program.constant(self) {
            Constant::Owned(offset, length) => RcDoc::text(":").append(
                program
                    .source(&Location {
                        start: *offset,
                        length: *length,
                    })
                    .escape_default()
                    .to_string(),
            ),
            Constant::Shared(offset, length) => {
                debug_assert!(
                    offset + length <= program.content_pool.len() as u32,
                    "offset: {}, length: {}, content_pool.len(): {}",
                    offset,
                    length,
                    program.content_pool.len()
                );
                RcDoc::text(":").append(
                    program.content_pool[*offset as usize..((offset + length) as usize)]
                        .escape_ascii()
                        .to_string(),
                )
            }
        }
    }
}

impl Snapshot for Node {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        RcDoc::text("@ ")
            .append(self.node_kind.name())
            .append(" (location: ")
            .append(Snapshot::snapshot(
                &self.location,
                program,
                hardline.clone(),
            ))
            .append(")")
            .append(self.node_kind.snapshot(program, hardline))
    }
}

impl Snapshot for Vec<NodeRef> {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        let mut doc = RcDoc::nil();
        let Some((last, head)) = self.split_last() else {
            return doc;
        };
        // self.hardline
        //     .clone()
        //     .append(if self.last {
        //         "└── "
        //     } else {
        //         "├── "
        //     })
        for n in head.iter() {
            doc = doc.append(
                hardline
                    .clone()
                    .append("├── ")
                    .append(n.snapshot(program, hardline.clone().append("│   "))),
            );
        }
        doc = doc.append(
            hardline
                .clone()
                .append("└── ")
                .append(last.snapshot(program, hardline.append("    "))),
        );
        doc
    }
}

#[derive(Debug, derive_new::new)]
struct Field<'a, T> {
    name: &'static str,
    last: bool,
    value: &'a T,
    program: &'a Program,
    hardline: RcDoc<'a, ()>,
}

impl<'a, T> Field<'a, T> {
    fn snapshot_prefix(&'_ self, space: bool) -> RcDoc<'a, ()> {
        self.hardline
            .clone()
            .append(if self.last {
                "└── "
            } else {
                "├── "
            })
            .append(self.name)
            .append(if space { ": " } else { ":" })
    }
}

impl<'a> Pretty<'a, pretty::RcAllocator> for Field<'a, Location> {
    fn pretty(
        self,
        allocator: &'a pretty::RcAllocator,
    ) -> pretty::DocBuilder<'a, pretty::RcAllocator, ()> {
        self.snapshot_prefix(true)
            .append(Snapshot::snapshot(
                self.value,
                self.program,
                self.hardline.clone(),
            ))
            .append(RcDoc::text(" = \""))
            .append(self.program.source(self.value).escape_default().to_string())
            .append("\"")
            .pretty(allocator)
    }
}

impl<'a> Pretty<'a, pretty::RcAllocator> for Field<'a, Vec<NodeRef>> {
    fn pretty(
        self,
        allocator: &'a pretty::RcAllocator,
    ) -> pretty::DocBuilder<'a, pretty::RcAllocator, ()> {
        let hardline = self
            .hardline
            .clone()
            .append(if self.last { "    " } else { "│   " });
        self.snapshot_prefix(true)
            .append(format!("(length: {})", self.value.len()))
            .append(self.value.snapshot(self.program, hardline))
            .pretty(allocator)
    }
}

impl<'a> Pretty<'a, pretty::RcAllocator> for Field<'a, &'static str> {
    fn pretty(
        self,
        allocator: &'a pretty::RcAllocator,
    ) -> pretty::DocBuilder<'a, pretty::RcAllocator, ()> {
        self.snapshot_prefix(true)
            .append(self.value.to_string())
            .pretty(allocator)
    }
}

impl<'a, T: BitFlag + fmt::Debug> Pretty<'a, RcAllocator> for Field<'a, BitFlags<T>> {
    fn pretty(self, allocator: &'a RcAllocator) -> DocBuilder<'a, RcAllocator, ()> {
        self.snapshot_prefix(true)
            .append(if self.value.is_empty() {
                "∅".to_string()
            } else {
                self.value
                    .iter()
                    .map(|f| format!("{:?}", f).to_ascii_lowercase().to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            })
            .pretty(allocator)
    }
}

impl<'a> Pretty<'a, RcAllocator> for Field<'a, Vec<ConstantRef>> {
    fn pretty(self, allocator: &'a RcAllocator) -> DocBuilder<'a, RcAllocator, ()> {
        self.snapshot_prefix(true)
            .append("[")
            .append(RcDoc::intersperse(
                self.value
                    .iter()
                    .map(|c| c.snapshot(self.program, self.hardline.clone())),
                RcDoc::text(", "),
            ))
            .append("]")
            .pretty(allocator)
    }
}

impl<'a> Pretty<'a, RcAllocator> for Field<'a, ConstantRef> {
    fn pretty(self, allocator: &'a RcAllocator) -> DocBuilder<'a, RcAllocator, ()> {
        self.snapshot_prefix(true)
            .append(self.value.snapshot(self.program, self.hardline))
            .pretty(allocator)
    }
}

impl<'a, T> Pretty<'a, RcAllocator> for Field<'a, Option<T>>
where
    Field<'a, T>: Pretty<'a, RcAllocator>,
{
    fn pretty(self, allocator: &'a RcAllocator) -> DocBuilder<'a, RcAllocator, ()> {
        if let Some(value) = self.value {
            Field::new(self.name, self.last, value, self.program, self.hardline).pretty(allocator)
        } else {
            self.snapshot_prefix(true)
                .append(RcDoc::text("∅"))
                .pretty(allocator)
        }
    }
}

impl<'a> Pretty<'a, RcAllocator> for Field<'a, NodeRef> {
    fn pretty(self, allocator: &'a RcAllocator) -> DocBuilder<'a, RcAllocator, ()> {
        self.snapshot_prefix(false)
            .append(indent(
                self.hardline.clone(),
                if self.last { "    " } else { "│   " },
                |hardline| self.value.snapshot(self.program, hardline),
            ))
            .pretty(allocator)
    }
}

impl<'a> Pretty<'a, RcAllocator> for Field<'a, u8> {
    fn pretty(self, allocator: &'a RcAllocator) -> DocBuilder<'a, RcAllocator, ()> {
        self.snapshot_prefix(true)
            .append(self.value.to_string())
            .pretty(allocator)
    }
}
impl<'a> Pretty<'a, RcAllocator> for Field<'a, u32> {
    fn pretty(self, allocator: &'a RcAllocator) -> DocBuilder<'a, RcAllocator, ()> {
        self.snapshot_prefix(true)
            .append(self.value.to_string())
            .pretty(allocator)
    }
}
impl<'a> Pretty<'a, RcAllocator> for Field<'a, f64> {
    fn pretty(self, allocator: &'a RcAllocator) -> DocBuilder<'a, RcAllocator, ()> {
        self.snapshot_prefix(true)
            .append(format!("{:?}", self.value))
            .pretty(allocator)
    }
}
impl<'a> Pretty<'a, RcAllocator> for Field<'a, StringField> {
    fn pretty(self, allocator: &'a RcAllocator) -> DocBuilder<'a, RcAllocator, ()> {
        self.snapshot_prefix(true)
            .append("\"")
            .append(self.program.string(self.value))
            .append("\"")
            .pretty(allocator)
    }
}
impl<'a> Pretty<'a, RcAllocator> for Field<'a, Integer> {
    fn pretty(self, allocator: &'a RcAllocator) -> DocBuilder<'a, RcAllocator, ()> {
        self.snapshot_prefix(true)
            .append(format!("{}", self.value))
            .pretty(allocator)
    }
}

macro_rules! docs {

  (field $self:expr, $program:expr, $hardline:expr, empty_flag $(,)?) => {{
      Field::empty_flag($program, $hardline.clone())
  }};
  (last field $self:expr, $program:expr, $hardline:expr, empty_flag $(,)?) => {{
      Field::empty_flag($program, $hardline.clone())
  }};


  (last field $self:expr, $program:expr, $hardline:expr, $first: ident $(,)?) => {{
    Field::new(stringify!($first), true, &$self.$first, $program, $hardline.clone())
}};

(field $self:expr, $program:expr, $hardline:expr, $first: ident $(,)?) => {{
    Field::new(stringify!($first), false, &$self.$first, $program, $hardline.clone())
}};



($self:expr, $program:expr, $hardline:expr, $first: ident $(,)?) => {
    RcDoc::nil().append(docs![last field $self, $program, $hardline, $first])
};

($self:expr, $program:expr, $hardline:expr, $first: ident, $($rest:tt)+) => {{
  let mut doc = RcDoc::nil();
  doc = doc.append(docs![field $self, $program, $hardline, $first]);
  doc = doc.append(docs![$self, $program, $hardline, $($rest)+]);
  doc
}};
}

fn indent<'a, F>(hardline: RcDoc<'a, ()>, prefix: &'static str, f: F) -> RcDoc<'a, ()>
where
    F: FnOnce(RcDoc<'a, ()>) -> RcDoc<'a, ()>,
{
    let hardline = hardline.clone().append(prefix);
    hardline.clone().append(f(hardline))
}

// BEGIN GENERATED CODE. DO NOT EDIT
impl Snapshot for NodeKind {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        match self {
            NodeKind::AliasGlobalVariableNode(node) => node.snapshot(program, hardline),
            NodeKind::AliasMethodNode(node) => node.snapshot(program, hardline),
            NodeKind::AlternationPatternNode(node) => node.snapshot(program, hardline),
            NodeKind::AndNode(node) => node.snapshot(program, hardline),
            NodeKind::ArgumentsNode(node) => node.snapshot(program, hardline),
            NodeKind::ArrayNode(node) => node.snapshot(program, hardline),
            NodeKind::ArrayPatternNode(node) => node.snapshot(program, hardline),
            NodeKind::AssocNode(node) => node.snapshot(program, hardline),
            NodeKind::AssocSplatNode(node) => node.snapshot(program, hardline),
            NodeKind::BackReferenceReadNode(node) => node.snapshot(program, hardline),
            NodeKind::BeginNode(node) => node.snapshot(program, hardline),
            NodeKind::BlockArgumentNode(node) => node.snapshot(program, hardline),
            NodeKind::BlockLocalVariableNode(node) => node.snapshot(program, hardline),
            NodeKind::BlockNode(node) => node.snapshot(program, hardline),
            NodeKind::BlockParameterNode(node) => node.snapshot(program, hardline),
            NodeKind::BlockParametersNode(node) => node.snapshot(program, hardline),
            NodeKind::BreakNode(node) => node.snapshot(program, hardline),
            NodeKind::CallAndWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::CallNode(node) => node.snapshot(program, hardline),
            NodeKind::CallOperatorWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::CallOrWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::CallTargetNode(node) => node.snapshot(program, hardline),
            NodeKind::CapturePatternNode(node) => node.snapshot(program, hardline),
            NodeKind::CaseMatchNode(node) => node.snapshot(program, hardline),
            NodeKind::CaseNode(node) => node.snapshot(program, hardline),
            NodeKind::ClassNode(node) => node.snapshot(program, hardline),
            NodeKind::ClassVariableAndWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::ClassVariableOperatorWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::ClassVariableOrWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::ClassVariableReadNode(node) => node.snapshot(program, hardline),
            NodeKind::ClassVariableTargetNode(node) => node.snapshot(program, hardline),
            NodeKind::ClassVariableWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::ConstantAndWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::ConstantOperatorWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::ConstantOrWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::ConstantPathAndWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::ConstantPathNode(node) => node.snapshot(program, hardline),
            NodeKind::ConstantPathOperatorWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::ConstantPathOrWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::ConstantPathTargetNode(node) => node.snapshot(program, hardline),
            NodeKind::ConstantPathWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::ConstantReadNode(node) => node.snapshot(program, hardline),
            NodeKind::ConstantTargetNode(node) => node.snapshot(program, hardline),
            NodeKind::ConstantWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::DefNode(node) => node.snapshot(program, hardline),
            NodeKind::DefinedNode(node) => node.snapshot(program, hardline),
            NodeKind::ElseNode(node) => node.snapshot(program, hardline),
            NodeKind::EmbeddedStatementsNode(node) => node.snapshot(program, hardline),
            NodeKind::EmbeddedVariableNode(node) => node.snapshot(program, hardline),
            NodeKind::EnsureNode(node) => node.snapshot(program, hardline),
            NodeKind::FalseNode(node) => node.snapshot(program, hardline),
            NodeKind::FindPatternNode(node) => node.snapshot(program, hardline),
            NodeKind::FlipFlopNode(node) => node.snapshot(program, hardline),
            NodeKind::FloatNode(node) => node.snapshot(program, hardline),
            NodeKind::ForNode(node) => node.snapshot(program, hardline),
            NodeKind::ForwardingArgumentsNode(node) => node.snapshot(program, hardline),
            NodeKind::ForwardingParameterNode(node) => node.snapshot(program, hardline),
            NodeKind::ForwardingSuperNode(node) => node.snapshot(program, hardline),
            NodeKind::GlobalVariableAndWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::GlobalVariableOperatorWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::GlobalVariableOrWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::GlobalVariableReadNode(node) => node.snapshot(program, hardline),
            NodeKind::GlobalVariableTargetNode(node) => node.snapshot(program, hardline),
            NodeKind::GlobalVariableWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::HashNode(node) => node.snapshot(program, hardline),
            NodeKind::HashPatternNode(node) => node.snapshot(program, hardline),
            NodeKind::IfNode(node) => node.snapshot(program, hardline),
            NodeKind::ImaginaryNode(node) => node.snapshot(program, hardline),
            NodeKind::ImplicitNode(node) => node.snapshot(program, hardline),
            NodeKind::ImplicitRestNode(node) => node.snapshot(program, hardline),
            NodeKind::InNode(node) => node.snapshot(program, hardline),
            NodeKind::IndexAndWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::IndexOperatorWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::IndexOrWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::IndexTargetNode(node) => node.snapshot(program, hardline),
            NodeKind::InstanceVariableAndWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::InstanceVariableOperatorWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::InstanceVariableOrWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::InstanceVariableReadNode(node) => node.snapshot(program, hardline),
            NodeKind::InstanceVariableTargetNode(node) => node.snapshot(program, hardline),
            NodeKind::InstanceVariableWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::IntegerNode(node) => node.snapshot(program, hardline),
            NodeKind::InterpolatedMatchLastLineNode(node) => node.snapshot(program, hardline),
            NodeKind::InterpolatedRegularExpressionNode(node) => node.snapshot(program, hardline),
            NodeKind::InterpolatedStringNode(node) => node.snapshot(program, hardline),
            NodeKind::InterpolatedSymbolNode(node) => node.snapshot(program, hardline),
            NodeKind::InterpolatedXStringNode(node) => node.snapshot(program, hardline),
            NodeKind::ItLocalVariableReadNode(node) => node.snapshot(program, hardline),
            NodeKind::ItParametersNode(node) => node.snapshot(program, hardline),
            NodeKind::KeywordHashNode(node) => node.snapshot(program, hardline),
            NodeKind::KeywordRestParameterNode(node) => node.snapshot(program, hardline),
            NodeKind::LambdaNode(node) => node.snapshot(program, hardline),
            NodeKind::LocalVariableAndWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::LocalVariableOperatorWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::LocalVariableOrWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::LocalVariableReadNode(node) => node.snapshot(program, hardline),
            NodeKind::LocalVariableTargetNode(node) => node.snapshot(program, hardline),
            NodeKind::LocalVariableWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::MatchLastLineNode(node) => node.snapshot(program, hardline),
            NodeKind::MatchPredicateNode(node) => node.snapshot(program, hardline),
            NodeKind::MatchRequiredNode(node) => node.snapshot(program, hardline),
            NodeKind::MatchWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::MissingNode(node) => node.snapshot(program, hardline),
            NodeKind::ModuleNode(node) => node.snapshot(program, hardline),
            NodeKind::MultiTargetNode(node) => node.snapshot(program, hardline),
            NodeKind::MultiWriteNode(node) => node.snapshot(program, hardline),
            NodeKind::NextNode(node) => node.snapshot(program, hardline),
            NodeKind::NilNode(node) => node.snapshot(program, hardline),
            NodeKind::NoKeywordsParameterNode(node) => node.snapshot(program, hardline),
            NodeKind::NumberedParametersNode(node) => node.snapshot(program, hardline),
            NodeKind::NumberedReferenceReadNode(node) => node.snapshot(program, hardline),
            NodeKind::OptionalKeywordParameterNode(node) => node.snapshot(program, hardline),
            NodeKind::OptionalParameterNode(node) => node.snapshot(program, hardline),
            NodeKind::OrNode(node) => node.snapshot(program, hardline),
            NodeKind::ParametersNode(node) => node.snapshot(program, hardline),
            NodeKind::ParenthesesNode(node) => node.snapshot(program, hardline),
            NodeKind::PinnedExpressionNode(node) => node.snapshot(program, hardline),
            NodeKind::PinnedVariableNode(node) => node.snapshot(program, hardline),
            NodeKind::PostExecutionNode(node) => node.snapshot(program, hardline),
            NodeKind::PreExecutionNode(node) => node.snapshot(program, hardline),
            NodeKind::ProgramNode(node) => node.snapshot(program, hardline),
            NodeKind::RangeNode(node) => node.snapshot(program, hardline),
            NodeKind::RationalNode(node) => node.snapshot(program, hardline),
            NodeKind::RedoNode(node) => node.snapshot(program, hardline),
            NodeKind::RegularExpressionNode(node) => node.snapshot(program, hardline),
            NodeKind::RequiredKeywordParameterNode(node) => node.snapshot(program, hardline),
            NodeKind::RequiredParameterNode(node) => node.snapshot(program, hardline),
            NodeKind::RescueModifierNode(node) => node.snapshot(program, hardline),
            NodeKind::RescueNode(node) => node.snapshot(program, hardline),
            NodeKind::RestParameterNode(node) => node.snapshot(program, hardline),
            NodeKind::RetryNode(node) => node.snapshot(program, hardline),
            NodeKind::ReturnNode(node) => node.snapshot(program, hardline),
            NodeKind::SelfNode(node) => node.snapshot(program, hardline),
            NodeKind::ShareableConstantNode(node) => node.snapshot(program, hardline),
            NodeKind::SingletonClassNode(node) => node.snapshot(program, hardline),
            NodeKind::SourceEncodingNode(node) => node.snapshot(program, hardline),
            NodeKind::SourceFileNode(node) => node.snapshot(program, hardline),
            NodeKind::SourceLineNode(node) => node.snapshot(program, hardline),
            NodeKind::SplatNode(node) => node.snapshot(program, hardline),
            NodeKind::StatementsNode(node) => node.snapshot(program, hardline),
            NodeKind::StringNode(node) => node.snapshot(program, hardline),
            NodeKind::SuperNode(node) => node.snapshot(program, hardline),
            NodeKind::SymbolNode(node) => node.snapshot(program, hardline),
            NodeKind::TrueNode(node) => node.snapshot(program, hardline),
            NodeKind::UndefNode(node) => node.snapshot(program, hardline),
            NodeKind::UnlessNode(node) => node.snapshot(program, hardline),
            NodeKind::UntilNode(node) => node.snapshot(program, hardline),
            NodeKind::WhenNode(node) => node.snapshot(program, hardline),
            NodeKind::WhileNode(node) => node.snapshot(program, hardline),
            NodeKind::XStringNode(node) => node.snapshot(program, hardline),
            NodeKind::YieldNode(node) => node.snapshot(program, hardline),
        }
    }
}
impl Snapshot for AliasGlobalVariableNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            new_name,
            old_name,
            keyword_loc
        ]
    }
}
impl Snapshot for AliasMethodNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            new_name,
            old_name,
            keyword_loc
        ]
    }
}
impl Snapshot for AlternationPatternNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, left, right, operator_loc]
    }
}
impl Snapshot for AndNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, left, right, operator_loc]
    }
}
impl Snapshot for ArgumentsNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, arguments]
    }
}
impl Snapshot for ArrayNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            elements,
            opening_loc,
            closing_loc
        ]
    }
}
impl Snapshot for ArrayPatternNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            constant,
            requireds,
            rest,
            posts,
            opening_loc,
            closing_loc
        ]
    }
}
impl Snapshot for AssocNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, key, value, operator_loc]
    }
}
impl Snapshot for AssocSplatNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, value, operator_loc]
    }
}
impl Snapshot for BackReferenceReadNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, name]
    }
}
impl Snapshot for BeginNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            begin_keyword_loc,
            statements,
            rescue_clause,
            else_clause,
            ensure_clause,
            end_keyword_loc
        ]
    }
}
impl Snapshot for BlockArgumentNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, expression, operator_loc]
    }
}
impl Snapshot for BlockLocalVariableNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, name]
    }
}
impl Snapshot for BlockNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            locals,
            parameters,
            body,
            opening_loc,
            closing_loc
        ]
    }
}
impl Snapshot for BlockParameterNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, name, name_loc, operator_loc]
    }
}
impl Snapshot for BlockParametersNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            parameters,
            locals,
            opening_loc,
            closing_loc
        ]
    }
}
impl Snapshot for BreakNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, arguments, keyword_loc]
    }
}
impl Snapshot for CallAndWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            receiver,
            call_operator_loc,
            message_loc,
            read_name,
            write_name,
            operator_loc,
            value
        ]
    }
}
impl Snapshot for CallNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            receiver,
            call_operator_loc,
            name,
            message_loc,
            opening_loc,
            arguments,
            closing_loc,
            block
        ]
    }
}
impl Snapshot for CallOperatorWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            receiver,
            call_operator_loc,
            message_loc,
            read_name,
            write_name,
            binary_operator,
            binary_operator_loc,
            value
        ]
    }
}
impl Snapshot for CallOrWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            receiver,
            call_operator_loc,
            message_loc,
            read_name,
            write_name,
            operator_loc,
            value
        ]
    }
}
impl Snapshot for CallTargetNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            receiver,
            call_operator_loc,
            name,
            message_loc
        ]
    }
}
impl Snapshot for CapturePatternNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, value, target, operator_loc]
    }
}
impl Snapshot for CaseMatchNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            predicate,
            conditions,
            else_clause,
            case_keyword_loc,
            end_keyword_loc
        ]
    }
}
impl Snapshot for CaseNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            predicate,
            conditions,
            else_clause,
            case_keyword_loc,
            end_keyword_loc
        ]
    }
}
impl Snapshot for ClassNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            locals,
            class_keyword_loc,
            constant_path,
            inheritance_operator_loc,
            superclass,
            body,
            end_keyword_loc,
            name
        ]
    }
}
impl Snapshot for ClassVariableAndWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name,
            name_loc,
            operator_loc,
            value
        ]
    }
}
impl Snapshot for ClassVariableOperatorWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name,
            name_loc,
            binary_operator_loc,
            value,
            binary_operator
        ]
    }
}
impl Snapshot for ClassVariableOrWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name,
            name_loc,
            operator_loc,
            value
        ]
    }
}
impl Snapshot for ClassVariableReadNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, name]
    }
}
impl Snapshot for ClassVariableTargetNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, name]
    }
}
impl Snapshot for ClassVariableWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name,
            name_loc,
            value,
            operator_loc
        ]
    }
}
impl Snapshot for ConstantAndWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name,
            name_loc,
            operator_loc,
            value
        ]
    }
}
impl Snapshot for ConstantOperatorWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name,
            name_loc,
            binary_operator_loc,
            value,
            binary_operator
        ]
    }
}
impl Snapshot for ConstantOrWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name,
            name_loc,
            operator_loc,
            value
        ]
    }
}
impl Snapshot for ConstantPathAndWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, target, operator_loc, value]
    }
}
impl Snapshot for ConstantPathNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            parent,
            name,
            delimiter_loc,
            name_loc
        ]
    }
}
impl Snapshot for ConstantPathOperatorWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            target,
            binary_operator_loc,
            value,
            binary_operator
        ]
    }
}
impl Snapshot for ConstantPathOrWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, target, operator_loc, value]
    }
}
impl Snapshot for ConstantPathTargetNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            parent,
            name,
            delimiter_loc,
            name_loc
        ]
    }
}
impl Snapshot for ConstantPathWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, target, operator_loc, value]
    }
}
impl Snapshot for ConstantReadNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, name]
    }
}
impl Snapshot for ConstantTargetNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, name]
    }
}
impl Snapshot for ConstantWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name,
            name_loc,
            value,
            operator_loc
        ]
    }
}
impl Snapshot for DefNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name,
            name_loc,
            receiver,
            parameters,
            body,
            locals,
            def_keyword_loc,
            operator_loc,
            lparen_loc,
            rparen_loc,
            equal_loc,
            end_keyword_loc
        ]
    }
}
impl Snapshot for DefinedNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            lparen_loc,
            value,
            rparen_loc,
            keyword_loc
        ]
    }
}
impl Snapshot for ElseNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            else_keyword_loc,
            statements,
            end_keyword_loc
        ]
    }
}
impl Snapshot for EmbeddedStatementsNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            opening_loc,
            statements,
            closing_loc
        ]
    }
}
impl Snapshot for EmbeddedVariableNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, operator_loc, variable]
    }
}
impl Snapshot for EnsureNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            ensure_keyword_loc,
            statements,
            end_keyword_loc
        ]
    }
}
impl Snapshot for FalseNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags]
    }
}
impl Snapshot for FindPatternNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            constant,
            left,
            requireds,
            right,
            opening_loc,
            closing_loc
        ]
    }
}
impl Snapshot for FlipFlopNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, left, right, operator_loc]
    }
}
impl Snapshot for FloatNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, value]
    }
}
impl Snapshot for ForNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            index,
            collection,
            statements,
            for_keyword_loc,
            in_keyword_loc,
            do_keyword_loc,
            end_keyword_loc
        ]
    }
}
impl Snapshot for ForwardingArgumentsNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags]
    }
}
impl Snapshot for ForwardingParameterNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags]
    }
}
impl Snapshot for ForwardingSuperNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, block]
    }
}
impl Snapshot for GlobalVariableAndWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name,
            name_loc,
            operator_loc,
            value
        ]
    }
}
impl Snapshot for GlobalVariableOperatorWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name,
            name_loc,
            binary_operator_loc,
            value,
            binary_operator
        ]
    }
}
impl Snapshot for GlobalVariableOrWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name,
            name_loc,
            operator_loc,
            value
        ]
    }
}
impl Snapshot for GlobalVariableReadNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, name]
    }
}
impl Snapshot for GlobalVariableTargetNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, name]
    }
}
impl Snapshot for GlobalVariableWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name,
            name_loc,
            value,
            operator_loc
        ]
    }
}
impl Snapshot for HashNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            opening_loc,
            elements,
            closing_loc
        ]
    }
}
impl Snapshot for HashPatternNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            constant,
            elements,
            rest,
            opening_loc,
            closing_loc
        ]
    }
}
impl Snapshot for IfNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            if_keyword_loc,
            predicate,
            then_keyword_loc,
            statements,
            subsequent,
            end_keyword_loc
        ]
    }
}
impl Snapshot for ImaginaryNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, numeric]
    }
}
impl Snapshot for ImplicitNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, value]
    }
}
impl Snapshot for ImplicitRestNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags]
    }
}
impl Snapshot for InNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, pattern, statements, in_loc, then_loc]
    }
}
impl Snapshot for IndexAndWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            receiver,
            call_operator_loc,
            opening_loc,
            arguments,
            closing_loc,
            block,
            operator_loc,
            value
        ]
    }
}
impl Snapshot for IndexOperatorWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            receiver,
            call_operator_loc,
            opening_loc,
            arguments,
            closing_loc,
            block,
            binary_operator,
            binary_operator_loc,
            value
        ]
    }
}
impl Snapshot for IndexOrWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            receiver,
            call_operator_loc,
            opening_loc,
            arguments,
            closing_loc,
            block,
            operator_loc,
            value
        ]
    }
}
impl Snapshot for IndexTargetNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            receiver,
            opening_loc,
            arguments,
            closing_loc,
            block
        ]
    }
}
impl Snapshot for InstanceVariableAndWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name,
            name_loc,
            operator_loc,
            value
        ]
    }
}
impl Snapshot for InstanceVariableOperatorWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name,
            name_loc,
            binary_operator_loc,
            value,
            binary_operator
        ]
    }
}
impl Snapshot for InstanceVariableOrWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name,
            name_loc,
            operator_loc,
            value
        ]
    }
}
impl Snapshot for InstanceVariableReadNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, name]
    }
}
impl Snapshot for InstanceVariableTargetNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, name]
    }
}
impl Snapshot for InstanceVariableWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name,
            name_loc,
            value,
            operator_loc
        ]
    }
}
impl Snapshot for IntegerNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, value]
    }
}
impl Snapshot for InterpolatedMatchLastLineNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            opening_loc,
            parts,
            closing_loc
        ]
    }
}
impl Snapshot for InterpolatedRegularExpressionNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            opening_loc,
            parts,
            closing_loc
        ]
    }
}
impl Snapshot for InterpolatedStringNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            opening_loc,
            parts,
            closing_loc
        ]
    }
}
impl Snapshot for InterpolatedSymbolNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            opening_loc,
            parts,
            closing_loc
        ]
    }
}
impl Snapshot for InterpolatedXStringNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            opening_loc,
            parts,
            closing_loc
        ]
    }
}
impl Snapshot for ItLocalVariableReadNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags]
    }
}
impl Snapshot for ItParametersNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags]
    }
}
impl Snapshot for KeywordHashNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, elements]
    }
}
impl Snapshot for KeywordRestParameterNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, name, name_loc, operator_loc]
    }
}
impl Snapshot for LambdaNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            locals,
            operator_loc,
            opening_loc,
            closing_loc,
            parameters,
            body
        ]
    }
}
impl Snapshot for LocalVariableAndWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name_loc,
            operator_loc,
            value,
            name,
            depth
        ]
    }
}
impl Snapshot for LocalVariableOperatorWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name_loc,
            binary_operator_loc,
            value,
            name,
            binary_operator,
            depth
        ]
    }
}
impl Snapshot for LocalVariableOrWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name_loc,
            operator_loc,
            value,
            name,
            depth
        ]
    }
}
impl Snapshot for LocalVariableReadNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, name, depth]
    }
}
impl Snapshot for LocalVariableTargetNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, name, depth]
    }
}
impl Snapshot for LocalVariableWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name,
            depth,
            name_loc,
            value,
            operator_loc
        ]
    }
}
impl Snapshot for MatchLastLineNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            opening_loc,
            content_loc,
            closing_loc,
            unescaped
        ]
    }
}
impl Snapshot for MatchPredicateNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, value, pattern, operator_loc]
    }
}
impl Snapshot for MatchRequiredNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, value, pattern, operator_loc]
    }
}
impl Snapshot for MatchWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, call, targets]
    }
}
impl Snapshot for MissingNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags]
    }
}
impl Snapshot for ModuleNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            locals,
            module_keyword_loc,
            constant_path,
            body,
            end_keyword_loc,
            name
        ]
    }
}
impl Snapshot for MultiTargetNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, lefts, rest, rights, lparen_loc, rparen_loc]
    }
}
impl Snapshot for MultiWriteNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            lefts,
            rest,
            rights,
            lparen_loc,
            rparen_loc,
            operator_loc,
            value
        ]
    }
}
impl Snapshot for NextNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, arguments, keyword_loc]
    }
}
impl Snapshot for NilNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags]
    }
}
impl Snapshot for NoKeywordsParameterNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, operator_loc, keyword_loc]
    }
}
impl Snapshot for NumberedParametersNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, maximum]
    }
}
impl Snapshot for NumberedReferenceReadNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, number]
    }
}
impl Snapshot for OptionalKeywordParameterNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, name, name_loc, value]
    }
}
impl Snapshot for OptionalParameterNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            name,
            name_loc,
            operator_loc,
            value
        ]
    }
}
impl Snapshot for OrNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, left, right, operator_loc]
    }
}
impl Snapshot for ParametersNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            requireds,
            optionals,
            rest,
            posts,
            keywords,
            keyword_rest,
            block
        ]
    }
}
impl Snapshot for ParenthesesNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            body,
            opening_loc,
            closing_loc
        ]
    }
}
impl Snapshot for PinnedExpressionNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            expression,
            operator_loc,
            lparen_loc,
            rparen_loc
        ]
    }
}
impl Snapshot for PinnedVariableNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, variable, operator_loc]
    }
}
impl Snapshot for PostExecutionNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            statements,
            keyword_loc,
            opening_loc,
            closing_loc
        ]
    }
}
impl Snapshot for PreExecutionNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            statements,
            keyword_loc,
            opening_loc,
            closing_loc
        ]
    }
}
impl Snapshot for ProgramNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, locals, statements]
    }
}
impl Snapshot for RangeNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, left, right, operator_loc]
    }
}
impl Snapshot for RationalNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, numerator, denominator]
    }
}
impl Snapshot for RedoNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags]
    }
}
impl Snapshot for RegularExpressionNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            opening_loc,
            content_loc,
            closing_loc,
            unescaped
        ]
    }
}
impl Snapshot for RequiredKeywordParameterNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, name, name_loc]
    }
}
impl Snapshot for RequiredParameterNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, name]
    }
}
impl Snapshot for RescueModifierNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            expression,
            keyword_loc,
            rescue_expression
        ]
    }
}
impl Snapshot for RescueNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            keyword_loc,
            exceptions,
            operator_loc,
            reference,
            then_keyword_loc,
            statements,
            subsequent
        ]
    }
}
impl Snapshot for RestParameterNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, name, name_loc, operator_loc]
    }
}
impl Snapshot for RetryNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags]
    }
}
impl Snapshot for ReturnNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, keyword_loc, arguments]
    }
}
impl Snapshot for SelfNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags]
    }
}
impl Snapshot for ShareableConstantNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, write]
    }
}
impl Snapshot for SingletonClassNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            locals,
            class_keyword_loc,
            operator_loc,
            expression,
            body,
            end_keyword_loc
        ]
    }
}
impl Snapshot for SourceEncodingNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags]
    }
}
impl Snapshot for SourceFileNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, filepath]
    }
}
impl Snapshot for SourceLineNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags]
    }
}
impl Snapshot for SplatNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, operator_loc, expression]
    }
}
impl Snapshot for StatementsNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, body]
    }
}
impl Snapshot for StringNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            opening_loc,
            content_loc,
            closing_loc,
            unescaped
        ]
    }
}
impl Snapshot for SuperNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            keyword_loc,
            lparen_loc,
            arguments,
            rparen_loc,
            block
        ]
    }
}
impl Snapshot for SymbolNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            opening_loc,
            value_loc,
            closing_loc,
            unescaped
        ]
    }
}
impl Snapshot for TrueNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags]
    }
}
impl Snapshot for UndefNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, flags, names, keyword_loc]
    }
}
impl Snapshot for UnlessNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            keyword_loc,
            predicate,
            then_keyword_loc,
            statements,
            else_clause,
            end_keyword_loc
        ]
    }
}
impl Snapshot for UntilNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            keyword_loc,
            do_keyword_loc,
            closing_loc,
            predicate,
            statements
        ]
    }
}
impl Snapshot for WhenNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            keyword_loc,
            conditions,
            then_keyword_loc,
            statements
        ]
    }
}
impl Snapshot for WhileNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            keyword_loc,
            do_keyword_loc,
            closing_loc,
            predicate,
            statements
        ]
    }
}
impl Snapshot for XStringNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            opening_loc,
            content_loc,
            closing_loc,
            unescaped
        ]
    }
}
impl Snapshot for YieldNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![
            self,
            program,
            hardline,
            flags,
            keyword_loc,
            lparen_loc,
            arguments,
            rparen_loc
        ]
    }
}
// END GENERATED CODE. DO NOT EDIT

pub fn snapshot(program: &Program) -> String {
    let doc = program.root().snapshot(program, RcDoc::hardline());
    let mut out = vec![];
    doc.render(0, &mut out).unwrap();
    String::from_utf8(out).unwrap()
}

#[test]
fn empty_program() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::parse("".to_string())?;

    expect![["@ ProgramNode (location: (1,0)-(1,0))
├── flags: ∅
├── locals: []
└── statements:
    @ StatementsNode (location: (1,0)-(1,0))
    ├── flags: ∅
    └── body: (length: 0)"]]
    .assert_eq(&snapshot(&program));

    Ok(())
}
