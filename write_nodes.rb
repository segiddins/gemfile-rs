#!/usr/bin/env ruby

require "yaml"

config = YAML.load_file("/Users/segiddins/Development/github.com/ruby/prism/config.yml")

Node = Struct.new(:name, :fields)

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

f.puts "use super::deserialize::*;"
f.puts

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
  node = Node.new(name, fields)

  f.puts "// #{index}"
  f.puts "#[derive(Debug, Clone)]"
  f.print "pub struct #{name}"
  if node.fields
    f.puts " {"
    node.fields.each do |field|
      f.puts "  #{field.name}: #{rust_type field.type},"
    end
    f.puts "}"
  else
    f.puts "(());"
  end

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
RS

config["nodes"].each_with_index do |node, index|
  name = node["name"]
  f.puts "    #{index+1} => NodeKind::#{name}("
  f.puts "      #{name} {"
  node["fields"]&.each do |field|
    f.puts "        #{field["name"]}: #{parse_method(field["type"])}.context(winnow::error::StrContext::Label(\"#{name}.#{field['name']}\")).parse_next(input)?,"
  end
  if !node["fields"]&.any?
    f.puts "        0: (),"
  end
  f.puts "      }"
  f.puts "    ),"
end

f.puts <<~RS
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
RS

puts "Types:\n  #{types.sort.join("\n  ")}"
puts "Types with kinds:\n  #{types_with_kinds.sort.join("\n  ")}"
puts "Kinds:\n  #{kinds.map(&:to_s).sort.join("\n  ")}"
