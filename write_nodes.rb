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

Field = Struct.new(:name, :type, :kind, :comment)

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
    "StringField"
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
  f.puts "  NEWLINE = 1 << 0,"
  f.puts "  STATIC_LITERAL = 1 << 1,"
  flag["values"].each_with_index do |value, index|
    f.puts "  /* #{flag["comment"]} */"
    f.puts "  #{value["name"]} = 1 << #{index+2},"
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
    Field.new(fname, type, kind, field["comment"])
  end
  flags = node["flags"]
  comment = node["comment"]
  node = Node.new(name, fields)

  f.puts "/**\n#{comment.gsub(/^/, "  ")}  */"
  f.puts "#[derive(Debug, Clone, PartialEq)]"
  f.print "pub struct #{name}"
    f.puts " {"
    f.puts "  pub flags: enumflags2::BitFlags<#{flags || :DefaultNodeFlags}>,"
    node.fields&.each do |field|
      f.puts "  /**\n#{field.comment.gsub(/^/, "    ")}    */" if field.comment
      f.puts "  pub #{field.name}: #{rust_type field.type},"
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
  f.puts "    winnow::combinator::seq![#{name}{"
  if name == "DefNode"
    f.puts "        _: winnow::binary::u32(winnow::binary::Endianness::Native),"
  end
  f.puts "        flags: parse_varuint.map(#{flags || :DefaultNodeFlags}::from_bits_truncate).context(winnow::error::StrContext::Label(\"#{name}.flags\")),"
  node["fields"]&.each do |field|
    f.puts "        #{field["name"]}: #{parse_method(field["type"])}.context(winnow::error::StrContext::Label(\"#{name}.#{field['name']}\")),"
  end
  f.puts "    }].parse_next(input)"
  f.puts "}"
  f.puts
  f.puts "}"
  f.puts
end

f.puts <<~RS

#[derive(Debug, Clone, PartialEq)]
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
RS

# puts "Types:\n  #{types.sort.join("\n  ")}"
# puts "Types with kinds:\n  #{types_with_kinds.sort.join("\n  ")}"
# puts "Kinds:\n  #{kinds.map(&:to_s).sort.join("\n  ")}"

f.close

require "fileutils"

f = File.open("tests/fixture_tests.rs", "w")
f.puts "use gemfile_rs::prism_ast::deserialize::Program;"
f.puts "use gemfile_rs::prism_ast::snapshot::snapshot;"
base = "/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures"
Dir["{**/,}*.txt", base:].sort.uniq.each do |path|
    f.puts
    f.puts "#[test]"
    f.puts "fn test_ast_#{path.gsub(/[\/.]/, "_")}() {"
    f.puts "  let program = Program::parse(include_str!(\"#{File.join(base, path)}\").to_string()).unwrap();"
    f.puts "  similar_asserts::assert_eq!(include_str!(\"#{File.join(File.dirname(base), "snapshots", path)}\").trim(), snapshot(&program));"
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

snapshot = File.read("src/prism_ast/snapshot.rs")



snapshot_body = +""

snapshot_body << <<~RUST
  impl Snapshot for NodeKind {
    fn snapshot<'a>(
        &'a self,
        program: &'a Program,
        hardline: RcDoc<'a, ()>,
    ) -> pretty::RcDoc<'a, ()> {
        match self {
            #{config['nodes'].map { |node| "NodeKind::#{node['name']}(node) => node.snapshot(program, hardline)," }.join("\n            ")}
        }
    }
  }
RUST

config['nodes'].each do |node|
  args = %w[self program hardline flags]
  args.concat(node['fields'].map { |field| field['name'] }) if node['fields']
  snapshot_body << <<~RUST
    impl Snapshot for #{node['name']} {
        fn snapshot<'a>(
            &'a self,
            program: &'a Program,
            hardline: RcDoc<'a, ()>,
        ) -> pretty::RcDoc<'a, ()> {
            docs![#{args.join(', ')}]
        }
    }
  RUST
end

raise unless snapshot.sub!(%r{(// BEGIN GENERATED CODE. DO NOT EDIT\n).*(// END GENERATED CODE. DO NOT EDIT\n)}m, "\\1#{snapshot_body}\\2")


File.write("src/prism_ast/snapshot.rs", snapshot)

`rustfmt src/prism_ast/snapshot.rs`
