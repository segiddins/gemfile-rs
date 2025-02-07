#!/usr/bin/env ruby

require "yaml"

config = YAML.load_file("/Users/segiddins/Development/github.com/ruby/prism/config.yml")

Node = Struct.new(:name, :fields) do
  def fields?
    fields&.any?
  end
end

Optional = Struct.new(:type)
Repeated = Struct.new(:type)

Field = Struct.new(:name, :type, :kind)

types = Set.new
types_with_kinds = Set.new
kinds = Set.new

f = File.open("src/prism_ast/generated.rs", "w")

def rust_type(type)
  case type
  when "constant"
    "ConstantRef"
  when "constant[]"
    "Vec<ConstantRef>"
  when "constant?"
    "Option<ConstantRef>"
  when "double"
    "f64"
  when "integer"
    "Integer"
  when "location"
    "Location"
  when "location?"
    "Option<Location>"
  when "node"
    "NodeRef"
  when "node[]"
    "Vec<NodeRef>"
  when "node?"
    "Option<NodeRef>"
  when "string"
    "String"
  when "uint32"
    "u32"
  when "uint8"
    "u8"
  else
    raise "Unknown type: #{type}"
  end
end

def parse_method(type)
  case type
  when "constant"
    "parse_constant"
  when "constant[]"
    "length_repeat(parse_varuint, parse_constant)"
  when "constant?"
    "parse_optional_constant"
  when "double"
    "parse_double"
  when "integer"
    "parse_integer"
  when "location"
    "parse_location"
  when "location?"
    "parse_optional_location"
  when "node"
    "parse_node"
  when "node[]"
    "length_repeat(parse_varuint, parse_node)"
  when "node?"
    "parse_optional_node"
  when "string"
    "parse_string_field"
  when "uint32"
    "parse_varuint"
  when "uint8"
    "winnow::binary::u8"
  else
    raise "Unknown type: #{type}"
  end
end

f.puts "use std::fmt::Write;"
f.puts
f.puts "use super::deserialize::*;"
f.puts "use enumflags2::BitFlag;"
f.puts "use winnow::binary::length_repeat;"
f.puts

config["flags"].each do |flag|
  f.puts "/* #{flag["comment"]} */"
  f.puts "#[enumflags2::bitflags]"
  f.puts "#[repr(u32)]"
  f.puts "#[derive(Debug, Clone, Copy, PartialEq, Eq)]"
  f.puts "pub enum #{flag["name"]} {"
  flag["values"].each_with_index do |value, index|
    f.puts "  /* #{flag["comment"]} */"
    f.puts "  #{value["name"]} = 1 << #{index},"
  end
  f.puts "}"
  f.puts
end

config["nodes"].each_with_index do |node, index|
  name = node["name"]
  fields = node["fields"]&.map do |field|
    fname = field["name"]
    type = field["type"]
    kind = field["kind"]
    types << type
    types_with_kinds << type if kind
    kinds.merge Array(kind).flatten
    Field.new(fname, type, kind)
  end
  flags = node["flags"]
  node = Node.new(name, fields)

  f.puts "// #{index}"
  f.puts "#[derive(Debug, Clone)]"
  f.print "pub struct #{name}"
    f.puts " {"
    if flags
      f.puts "  pub flags: enumflags2::BitFlags<#{flags}>,"
    end
    node.fields&.each do |field|
      f.puts "  #{field.name}: #{rust_type field.type},"
    end
    f.puts "}"

  f.puts "impl #{name} {"
  node.fields&.each do |field|
    next
    case field.type
    when "constant"
      f.puts "  pub fn #{field.name}(&self, pool: ()) -> ConstantRef { pool.get(self.#{field.name}) }"
    when "constant[]"
      f.puts "  pub fn iter_#{field.name}(&self, pool: ()) -> impl Iterator<Item = ConstantRef> { pool.get(self.#{field.name}).iter().map(|&id| pool.get(id)) }"
    end
  end
  f.puts "pub fn into_node_kind(self) -> NodeKind { NodeKind::#{name}(self) }"

  f.puts "pub fn parser(input: &mut Stream) -> winnow::ModalResult<Self> {"
  f.puts "    use winnow::Parser;"
  if node.fields&.any?
    f.puts "    winnow::combinator::seq![#{name}{"
    if name == "DefNode"
      f.puts "        _: winnow::binary::u32(winnow::binary::Endianness::Native),"
    end
    if flags
      f.puts "        flags: parse_varuint.map(#{flags}::from_bits_truncate).context(winnow::error::StrContext::Label(\"#{name}.flags\")),"
    else
      f.puts "        _: zero_flags.context(winnow::error::StrContext::Label(\"#{name}.flags\")),"
    end
    node["fields"]&.each do |field|
      f.puts "        #{field["name"]}: #{parse_method(field["type"])}.context(winnow::error::StrContext::Label(\"#{name}.#{field['name']}\")),"
    end
    f.puts "    }].parse_next(input)"
  else
    f.puts "    zero_flags.context(winnow::error::StrContext::Label(\"#{name}.flags\")).value(Self {}).parse_next(input)"
  end
  f.puts "}"
  f.puts
  f.puts "}"
  f.puts
end

f.puts <<~RS

#[derive(Debug, Clone)]
pub enum NodeKind {
RS
config["nodes"].each_with_index do |node, index|
  name = node["name"]
  f.puts "  #{name}(#{name}),"
end

f.puts <<~RS
}

impl NodeKind {
    pub fn name(&self) -> &str {
        match self {
RS
config["nodes"].each_with_index do |node, index|
  name = node["name"]
  f.puts "            NodeKind::#{name}(_) => \"#{name}\","
end

f.puts <<~RS
        }
    }
}


pub fn parse_node(input: &mut Stream) -> winnow::ModalResult<NodeRef> {
    use winnow::Parser;

    let (kind, identifier, location) = winnow::combinator::seq![(
          winnow::binary::u8,
          parse_varuint,
          parse_location,
      )].parse_next(input)?;

    let node_kind = match kind {
RS

  config["nodes"].each_with_index do |node, index|
  name = node["name"]
  f.puts "    #{index+1} => #{name}::parser.map(#{name}::into_node_kind).parse_next(input),"
end

f.puts <<~RS
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
RS

def snapshot_field(f, field, prefix)
  f.puts "            write!(f, \"#{prefix} #{field['name']}:\")?;"

  case field["type"]
  when "node"
    f.puts "writeln!(f)?;"
    f.puts "let mut pad = PadWriter::new(f, \"    \", true);"
    f.puts "writeln!(&mut pad, \"{:?}\", NodeSnapshot { program: self.program, node: node.#{field['name']} })?;"
    f.puts "drop(pad);"
  when "node[]"
    f.puts "writeln!(f, \" (length: {})\", node.#{field['name']}.len())?;"
    f.puts "let mut pad = PadWriter::new(f, \"|   \", true);"
    f.puts <<~RS
            for node in &node.#{field['name']} {
                writeln!(&mut pad, "├── {:?}", NodeSnapshot { program: self.program, node: *node })?;
            }
    RS
    f.puts "drop(pad);"
  when "location"
    f.puts "writeln!(f, \"{:?}\", LocationSnapshot { program: self.program, location: &node.#{field['name']} })?;"
  when "location?"
    f.puts <<~RS
    match &node.#{field['name']} {
      Some(loc) => writeln!(f, " {:?}", LocationSnapshot { program: self.program, location: loc }),
      None => writeln!(f, " ∅"),
    }?;
    RS
  when "string"
    f.puts "writeln!(f, \" {:?}\", node.#{field['name']})?;"
  when "constant"
    f.puts "writeln!(f, \" {:?}\", self.program.constant(&node.#{field['name']}))?;"
  when "constant[]"
    f.puts "writeln!(f, \" {:?}\", node.#{field['name']}.iter().map(|r| self.program.constant(r)).collect::<Vec<_>>())?;"
  else
    f.puts "writeln!(f, \"# {:?}\", node.#{field['name']})?;"
    # raise "Implement #{field['type']}"
  end
end

config["nodes"].each do |node|
  f.puts "        NodeKind::#{node["name"]}(node) => {"

  node["fields"]&.[](..-2)&.each do |field|
    snapshot_field(f, field, "├──")
  end
  if field = node["fields"]&.last
    snapshot_field(f, field, "└──")
  end
  f.puts "        },"
end

f.puts <<~RS
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

impl<T: std::fmt::Write> std::fmt::Write for PadWriter<'_, T>
{
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
      let mut buf = s;
              loop {
        let l = buf.find('\\n');
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
            self.writer.write_str("\\n")?;
            buf = &buf[l+1..];
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
RS

puts "Types:\n  #{types.sort.join("\n  ")}"
puts "Types with kinds:\n  #{types_with_kinds.sort.join("\n  ")}"
puts "Kinds:\n  #{kinds.map(&:to_s).sort.join("\n  ")}"

f.close

require "fileutils"

f = File.open("tests/fixture_tests.rs", "w")
f.puts "use gemfile_rs::prism_ast::deserialize::Program;"
base = "/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures"
Dir["{**/,}*.txt", base:].sort.uniq.each do |path|
    f.puts
    f.puts "#[test]"
    f.puts "fn test_ast_#{path.gsub(/[\/.]/, "_")}() {"
    f.puts "  let program = Program::parse(include_str!(\"#{File.join(base, path)}\").to_string()).unwrap();"
    f.puts "  similar_asserts::assert_eq!(include_str!(\"#{File.join(File.dirname(base), "snapshots", path)}\").trim(), program.snapshot());"
    f.puts "}"
    f.puts "#[test]"
    f.puts "fn test_program_#{path.gsub(/[\/.]/, "_")}() {"
    f.puts "  let program = Program::parse(include_str!(\"#{File.join(base, path)}\").to_string()).unwrap();"
    f.puts "  expect_test::expect_file![\"#{File.expand_path("tests/snapshots/#{path}")}\"].assert_debug_eq(&program);"
    f.puts "}"

    FileUtils.mkdir_p "tests/snapshots/#{File.dirname(path)}"
end

f.close

`rustfmt src/prism_ast/generated.rs`
