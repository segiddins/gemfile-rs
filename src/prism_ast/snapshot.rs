use std::fmt::format;
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

impl<'a> Field<'a, ()> {
    fn empty_flag(program: &'a Program) -> Field<'a, ()> {
        Field {
            name: "flags",
            last: false,
            value: &(),
            program,
            hardline: RcDoc::nil(),
        }
    }
}

impl<'a> Pretty<'a, RcAllocator> for Field<'a, ()> {
    fn pretty(self, allocator: &'a RcAllocator) -> DocBuilder<'a, RcAllocator, ()> {
        self.snapshot_prefix(true)
            .append("∅")
            .append(self.hardline)
            .pretty(allocator)
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
            .append(
                self.program
                    .source(&self.value)
                    .escape_default()
                    .to_string(),
            )
            .append("\"")
            .pretty(allocator)
    }
}

impl<'a> Pretty<'a, pretty::RcAllocator> for Field<'a, Vec<NodeRef>> {
    fn pretty(
        self,
        allocator: &'a pretty::RcAllocator,
    ) -> pretty::DocBuilder<'a, pretty::RcAllocator, ()> {
        self.snapshot_prefix(true)
            .append(format!("(length: {})", self.value.len()))
            .append(self.value.snapshot(self.program, self.hardline))
            .pretty(allocator)
    }
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
            Constant::Shared(offset, length) => RcDoc::text(
                program.content_pool[*offset as usize..=(offset + length) as usize]
                    .escape_ascii()
                    .to_string(),
            ),
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
            .append(hardline.clone())
            .append(self.node_kind.snapshot(program, hardline))
    }
}

impl Snapshot for NodeKind {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        match self {
            NodeKind::ProgramNode(node) => node.snapshot(program, hardline),
            NodeKind::StatementsNode(node) => node.snapshot(program, hardline),
            NodeKind::ModuleNode(node) => node.snapshot(program, hardline),
            NodeKind::ConstantReadNode(node) => node.snapshot(program, hardline),
            NodeKind::ConstantPathNode(node) => node.snapshot(program, hardline),
            _ => RcDoc::text(format!("{:?}", self)),
        }
    }
}

impl<'a, T: BitFlag + Debug> Pretty<'a, RcAllocator> for Field<'a, BitFlags<T>> {
    fn pretty(self, allocator: &'a RcAllocator) -> DocBuilder<'a, RcAllocator, ()> {
        self.snapshot_prefix(true).append("∅").pretty(allocator)
    }
}

impl<'a> Pretty<'a, RcAllocator> for Field<'a, Vec<ConstantRef>> {
    fn pretty(self, allocator: &'a RcAllocator) -> DocBuilder<'a, RcAllocator, ()> {
        self.snapshot_prefix(true)
            .append(RcDoc::text(format!("{:?}", self.value)))
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

macro_rules! docs {

  (field $self:expr, $program:expr, $hardline:expr, empty_flag $(,)?) => {{
      Field::empty_flag($program)
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

impl Snapshot for ProgramNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, empty_flag, locals, statements]
    }
}

impl Snapshot for StatementsNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, empty_flag, body]
    }
}

impl Snapshot for ModuleNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        RcDoc::text("├── flags: ")
            .append(RcDoc::text(format!("{}", "∅")))
            .append(hardline.clone())
            .append(RcDoc::text("├── locals: "))
            .append(RcDoc::text(format!("{:?}", self.locals)))
            .append(Field::new(
                "module_keyword_loc",
                false,
                &self.module_keyword_loc,
                &program,
                hardline.clone(),
            ))
            .append(RcDoc::text("├── constant_path:"))
            .append(indent(hardline.clone(), "│   ", |hardline| {
                self.constant_path.snapshot(program, hardline)
            }))
            .append(hardline.clone())
            .append(RcDoc::text("├── body: "))
            .append(RcDoc::text(format!("{:?}", self.body)))
            .append(Field::new(
                "end_keyword_loc",
                false,
                &self.end_keyword_loc,
                &program,
                hardline.clone(),
            ))
            .append(RcDoc::text("└── name: "))
            .append(self.name.snapshot(program, hardline.clone()));
        docs![
            self,
            program,
            hardline,
            empty_flag,
            locals,
            module_keyword_loc,
            constant_path,
            body,
            end_keyword_loc,
            name
        ]
    }
}

impl Snapshot for Vec<NodeRef> {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        let Some((last, nodes)) = self.split_last() else {
            return RcDoc::nil();
        };
        hardline
            .clone()
            .append(RcDoc::intersperse(
                nodes.iter().map(|r| {
                    RcDoc::text("    ├── ").append(
                        program
                            .node(r)
                            .snapshot(program, hardline.clone().append("    │   ")),
                    )
                }),
                hardline.clone(),
            ))
            .append(hardline.clone())
            .append(
                RcDoc::text("    └── ").append(
                    program
                        .node(last)
                        .snapshot(program, hardline.clone().append("        ")),
                ),
            )
    }
}

impl Snapshot for ConstantReadNode {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        docs![self, program, hardline, empty_flag, name]
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
            empty_flag,
            parent,
            name,
            delimiter_loc,
            name_loc
        ]
    }
}

fn snapshot(program: &Program) -> String {
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

#[test]
fn test_module() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::parse(
        r"module A
end

module A::B
end

module A::B::C
end

module A
  include(B.new)

  def foo
    :bar
  end
end
"
        .to_string(),
    )?;

    let doc = snapshot(&program);

    expect![[r#"@ ProgramNode (location: (1,0)-(16,3))
├── flags: ∅
├── locals: []
└── statements:
    @ StatementsNode (location: (1,0)-(16,3))
    ├── flags: ∅
    └── body: (length: 4)
        ├── @ ModuleNode (location: (1,0)-(2,3))
        │   ├── flags: ∅
        │   ├── locals: []
        │   ├── module_keyword_loc: (1,0)-(1,6) = "module"
        │   ├── constant_path:
        │   │   @ ConstantReadNode (location: (1,7)-(1,8))
        │   │   ├── flags: ∅
        │   │   └── name: :A
        │   ├── body: ∅
        │   ├── end_keyword_loc: (2,0)-(2,3) = "end"
        │   └── name: :A
        ├── @ ModuleNode (location: (4,0)-(5,3))
        │   ├── flags: ∅
        │   ├── locals: []
        │   ├── module_keyword_loc: (4,0)-(4,6) = "module"
        │   ├── constant_path:
        │   │   @ ConstantPathNode (location: (4,7)-(4,11))
        │   │   ├── flags: ∅
        │   │   ├── parent:
        │   │   │   @ ConstantReadNode (location: (4,7)-(4,8))
        │   │   │   ├── flags: ∅
        │   │   │   └── name: :A
        │   │   ├── name: :B
        │   │   ├── delimiter_loc: (4,8)-(4,10) = "::"
        │   │   └── name_loc: (4,10)-(4,11) = "B"
        │   ├── body: ∅
        │   ├── end_keyword_loc: (5,0)-(5,3) = "end"
        │   └── name: :B
        ├── @ ModuleNode (location: (7,0)-(8,3))
        │   ├── flags: ∅
        │   ├── locals: []
        │   ├── module_keyword_loc: (7,0)-(7,6) = "module"
        │   ├── constant_path:
        │   │   @ ConstantPathNode (location: (7,7)-(7,14))
        │   │   ├── flags: ∅
        │   │   ├── parent:
        │   │   │   @ ConstantPathNode (location: (7,7)-(7,11))
        │   │   │   ├── flags: ∅
        │   │   │   ├── parent:
        │   │   │   │   @ ConstantReadNode (location: (7,7)-(7,8))
        │   │   │   │   ├── flags: ∅
        │   │   │   │   └── name: :A
        │   │   │   ├── name: :B
        │   │   │   ├── delimiter_loc: (7,8)-(7,10) = "::"
        │   │   │   └── name_loc: (7,10)-(7,11) = "B"
        │   │   ├── name: :C
        │   │   ├── delimiter_loc: (7,11)-(7,13) = "::"
        │   │   └── name_loc: (7,13)-(7,14) = "C"
        │   ├── body: ∅
        │   ├── end_keyword_loc: (8,0)-(8,3) = "end"
        │   └── name: :C
        └── @ ModuleNode (location: (10,0)-(16,3))
            ├── flags: ∅
            ├── locals: []
            ├── module_keyword_loc: (10,0)-(10,6) = "module"
            ├── constant_path:
            │   @ ConstantReadNode (location: (10,7)-(10,8))
            │   ├── flags: ∅
            │   └── name: :A
            ├── body:
            │   @ StatementsNode (location: (11,2)-(15,5))
            │   ├── flags: ∅
            │   └── body: (length: 2)
            │       ├── @ CallNode (location: (11,2)-(11,16))
            │       │   ├── flags: ∅, ignore_visibility
            │       │   ├── receiver: ∅
            │       │   ├── call_operator_loc: ∅
            │       │   ├── name: :include
            │       │   ├── message_loc: (11,2)-(11,9) = "include"
            │       │   ├── opening_loc: (11,9)-(11,10) = "("
            │       │   ├── arguments:
            │       │   │   @ ArgumentsNode (location: (11,10)-(11,15))
            │       │   │   ├── flags: ∅
            │       │   │   └── arguments: (length: 1)
            │       │   │       └── @ CallNode (location: (11,10)-(11,15))
            │       │   │           ├── flags: ∅
            │       │   │           ├── receiver:
            │       │   │           │   @ ConstantReadNode (location: (11,10)-(11,11))
            │       │   │           │   ├── flags: ∅
            │       │   │           │   └── name: :B
            │       │   │           ├── call_operator_loc: (11,11)-(11,12) = "."
            │       │   │           ├── name: :new
            │       │   │           ├── message_loc: (11,12)-(11,15) = "new"
            │       │   │           ├── opening_loc: ∅
            │       │   │           ├── arguments: ∅
            │       │   │           ├── closing_loc: ∅
            │       │   │           └── block: ∅
            │       │   ├── closing_loc: (11,15)-(11,16) = ")"
            │       │   └── block: ∅
            │       └── @ DefNode (location: (13,2)-(15,5))
            │           ├── flags: ∅
            │           ├── name: :foo
            │           ├── name_loc: (13,6)-(13,9) = "foo"
            │           ├── receiver: ∅
            │           ├── parameters: ∅
            │           ├── body:
            │           │   @ StatementsNode (location: (14,4)-(14,8))
            │           │   ├── flags: ∅
            │           │   └── body: (length: 1)
            │           │       └── @ SymbolNode (location: (14,4)-(14,8))
            │           │           ├── flags: ∅, static_literal, forced_us_ascii_encoding
            │           │           ├── opening_loc: (14,4)-(14,5) = ":"
            │           │           ├── value_loc: (14,5)-(14,8) = "bar"
            │           │           ├── closing_loc: ∅
            │           │           └── unescaped: "bar"
            │           ├── locals: []
            │           ├── def_keyword_loc: (13,2)-(13,5) = "def"
            │           ├── operator_loc: ∅
            │           ├── lparen_loc: ∅
            │           ├── rparen_loc: ∅
            │           ├── equal_loc: ∅
            │           └── end_keyword_loc: (15,2)-(15,5) = "end"
            ├── end_keyword_loc: (16,0)-(16,3) = "end"
            └── name: :A
"#]]
    .assert_eq(&doc);

    Ok(())
}
