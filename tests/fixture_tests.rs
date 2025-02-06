use gemfile_rs::prism_ast::deserialize::Program;

fn test_ast_alias_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/alias.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/alias.txt").trim(), program.snapshot());
}
#[test]
fn test_program_alias_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/alias.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/alias.txt"].assert_debug_eq(&program);
}

fn test_ast_arithmetic_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/arithmetic.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/arithmetic.txt").trim(), program.snapshot());
}
#[test]
fn test_program_arithmetic_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/arithmetic.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/arithmetic.txt"].assert_debug_eq(&program);
}

fn test_ast_arrays_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/arrays.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/arrays.txt").trim(), program.snapshot());
}
#[test]
fn test_program_arrays_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/arrays.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/arrays.txt"].assert_debug_eq(&program);
}

fn test_ast_begin_ensure_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/begin_ensure.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/begin_ensure.txt").trim(), program.snapshot());
}
#[test]
fn test_program_begin_ensure_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/begin_ensure.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/begin_ensure.txt"].assert_debug_eq(&program);
}

fn test_ast_begin_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/begin_rescue.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/begin_rescue.txt").trim(), program.snapshot());
}
#[test]
fn test_program_begin_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/begin_rescue.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/begin_rescue.txt"].assert_debug_eq(&program);
}

fn test_ast_blocks_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/blocks.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/blocks.txt").trim(), program.snapshot());
}
#[test]
fn test_program_blocks_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/blocks.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/blocks.txt"].assert_debug_eq(&program);
}

fn test_ast_boolean_operators_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/boolean_operators.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/boolean_operators.txt").trim(), program.snapshot());
}
#[test]
fn test_program_boolean_operators_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/boolean_operators.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/boolean_operators.txt"].assert_debug_eq(&program);
}

fn test_ast_booleans_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/booleans.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/booleans.txt").trim(), program.snapshot());
}
#[test]
fn test_program_booleans_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/booleans.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/booleans.txt"].assert_debug_eq(&program);
}

fn test_ast_break_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/break.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/break.txt").trim(), program.snapshot());
}
#[test]
fn test_program_break_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/break.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/break.txt"].assert_debug_eq(&program);
}

fn test_ast_case_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/case.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/case.txt").trim(), program.snapshot());
}
#[test]
fn test_program_case_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/case.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/case.txt"].assert_debug_eq(&program);
}

fn test_ast_classes_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/classes.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/classes.txt").trim(), program.snapshot());
}
#[test]
fn test_program_classes_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/classes.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/classes.txt"].assert_debug_eq(&program);
}

fn test_ast_command_method_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/command_method_call.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/command_method_call.txt").trim(), program.snapshot());
}
#[test]
fn test_program_command_method_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/command_method_call.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/command_method_call.txt"].assert_debug_eq(&program);
}

fn test_ast_comment_single_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/comment_single.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/comment_single.txt").trim(), program.snapshot());
}
#[test]
fn test_program_comment_single_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/comment_single.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/comment_single.txt"].assert_debug_eq(&program);
}

fn test_ast_comments_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/comments.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/comments.txt").trim(), program.snapshot());
}
#[test]
fn test_program_comments_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/comments.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/comments.txt"].assert_debug_eq(&program);
}

fn test_ast_constants_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/constants.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/constants.txt").trim(), program.snapshot());
}
#[test]
fn test_program_constants_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/constants.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/constants.txt"].assert_debug_eq(&program);
}

fn test_ast_dash_heredocs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/dash_heredocs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/dash_heredocs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_dash_heredocs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/dash_heredocs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/dash_heredocs.txt"].assert_debug_eq(&program);
}

fn test_ast_defined_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/defined.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/defined.txt").trim(), program.snapshot());
}
#[test]
fn test_program_defined_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/defined.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/defined.txt"].assert_debug_eq(&program);
}

fn test_ast_dos_endings_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/dos_endings.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/dos_endings.txt").trim(), program.snapshot());
}
#[test]
fn test_program_dos_endings_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/dos_endings.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/dos_endings.txt"].assert_debug_eq(&program);
}

fn test_ast_dstring_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/dstring.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/dstring.txt").trim(), program.snapshot());
}
#[test]
fn test_program_dstring_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/dstring.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/dstring.txt"].assert_debug_eq(&program);
}

fn test_ast_dsym_str_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/dsym_str.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/dsym_str.txt").trim(), program.snapshot());
}
#[test]
fn test_program_dsym_str_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/dsym_str.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/dsym_str.txt"].assert_debug_eq(&program);
}

fn test_ast_embdoc_no_newline_at_end_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/embdoc_no_newline_at_end.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/embdoc_no_newline_at_end.txt").trim(), program.snapshot());
}
#[test]
fn test_program_embdoc_no_newline_at_end_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/embdoc_no_newline_at_end.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/embdoc_no_newline_at_end.txt"].assert_debug_eq(&program);
}

fn test_ast_emoji_method_calls_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/emoji_method_calls.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/emoji_method_calls.txt").trim(), program.snapshot());
}
#[test]
fn test_program_emoji_method_calls_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/emoji_method_calls.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/emoji_method_calls.txt"].assert_debug_eq(&program);
}

fn test_ast_encoding_binary_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/encoding_binary.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/encoding_binary.txt").trim(), program.snapshot());
}
#[test]
fn test_program_encoding_binary_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/encoding_binary.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/encoding_binary.txt"].assert_debug_eq(&program);
}

fn test_ast_encoding_euc_jp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/encoding_euc_jp.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/encoding_euc_jp.txt").trim(), program.snapshot());
}
#[test]
fn test_program_encoding_euc_jp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/encoding_euc_jp.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/encoding_euc_jp.txt"].assert_debug_eq(&program);
}

fn test_ast_endless_methods_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/endless_methods.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/endless_methods.txt").trim(), program.snapshot());
}
#[test]
fn test_program_endless_methods_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/endless_methods.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/endless_methods.txt"].assert_debug_eq(&program);
}

fn test_ast_endless_range_in_conditional_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/endless_range_in_conditional.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/endless_range_in_conditional.txt").trim(), program.snapshot());
}
#[test]
fn test_program_endless_range_in_conditional_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/endless_range_in_conditional.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/endless_range_in_conditional.txt"].assert_debug_eq(&program);
}

fn test_ast_for_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/for.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/for.txt").trim(), program.snapshot());
}
#[test]
fn test_program_for_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/for.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/for.txt"].assert_debug_eq(&program);
}

fn test_ast_global_variables_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/global_variables.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/global_variables.txt").trim(), program.snapshot());
}
#[test]
fn test_program_global_variables_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/global_variables.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/global_variables.txt"].assert_debug_eq(&program);
}

fn test_ast_hashes_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/hashes.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/hashes.txt").trim(), program.snapshot());
}
#[test]
fn test_program_hashes_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/hashes.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/hashes.txt"].assert_debug_eq(&program);
}

fn test_ast_heredoc_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/heredoc.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/heredoc.txt").trim(), program.snapshot());
}
#[test]
fn test_program_heredoc_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/heredoc.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/heredoc.txt"].assert_debug_eq(&program);
}

fn test_ast_heredoc_with_carriage_returns_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/heredoc_with_carriage_returns.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/heredoc_with_carriage_returns.txt").trim(), program.snapshot());
}
#[test]
fn test_program_heredoc_with_carriage_returns_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/heredoc_with_carriage_returns.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/heredoc_with_carriage_returns.txt"].assert_debug_eq(&program);
}

fn test_ast_heredoc_with_comment_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/heredoc_with_comment.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/heredoc_with_comment.txt").trim(), program.snapshot());
}
#[test]
fn test_program_heredoc_with_comment_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/heredoc_with_comment.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/heredoc_with_comment.txt"].assert_debug_eq(&program);
}

fn test_ast_heredoc_with_escaped_newline_at_start_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/heredoc_with_escaped_newline_at_start.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/heredoc_with_escaped_newline_at_start.txt").trim(), program.snapshot());
}
#[test]
fn test_program_heredoc_with_escaped_newline_at_start_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/heredoc_with_escaped_newline_at_start.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/heredoc_with_escaped_newline_at_start.txt"].assert_debug_eq(&program);
}

fn test_ast_heredoc_with_trailing_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/heredoc_with_trailing_newline.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/heredoc_with_trailing_newline.txt").trim(), program.snapshot());
}
#[test]
fn test_program_heredoc_with_trailing_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/heredoc_with_trailing_newline.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/heredoc_with_trailing_newline.txt"].assert_debug_eq(&program);
}

fn test_ast_heredocs_leading_whitespace_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/heredocs_leading_whitespace.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/heredocs_leading_whitespace.txt").trim(), program.snapshot());
}
#[test]
fn test_program_heredocs_leading_whitespace_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/heredocs_leading_whitespace.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/heredocs_leading_whitespace.txt"].assert_debug_eq(&program);
}

fn test_ast_heredocs_nested_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/heredocs_nested.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/heredocs_nested.txt").trim(), program.snapshot());
}
#[test]
fn test_program_heredocs_nested_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/heredocs_nested.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/heredocs_nested.txt"].assert_debug_eq(&program);
}

fn test_ast_heredocs_with_fake_newlines_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/heredocs_with_fake_newlines.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/heredocs_with_fake_newlines.txt").trim(), program.snapshot());
}
#[test]
fn test_program_heredocs_with_fake_newlines_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/heredocs_with_fake_newlines.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/heredocs_with_fake_newlines.txt"].assert_debug_eq(&program);
}

fn test_ast_heredocs_with_ignored_newlines_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/heredocs_with_ignored_newlines.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/heredocs_with_ignored_newlines.txt").trim(), program.snapshot());
}
#[test]
fn test_program_heredocs_with_ignored_newlines_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/heredocs_with_ignored_newlines.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/heredocs_with_ignored_newlines.txt"].assert_debug_eq(&program);
}

fn test_ast_heredocs_with_ignored_newlines_and_non_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/heredocs_with_ignored_newlines_and_non_empty.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/heredocs_with_ignored_newlines_and_non_empty.txt").trim(), program.snapshot());
}
#[test]
fn test_program_heredocs_with_ignored_newlines_and_non_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/heredocs_with_ignored_newlines_and_non_empty.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/heredocs_with_ignored_newlines_and_non_empty.txt"].assert_debug_eq(&program);
}

fn test_ast_if_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/if.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/if.txt").trim(), program.snapshot());
}
#[test]
fn test_program_if_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/if.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/if.txt"].assert_debug_eq(&program);
}

fn test_ast_indented_file_end_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/indented_file_end.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/indented_file_end.txt").trim(), program.snapshot());
}
#[test]
fn test_program_indented_file_end_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/indented_file_end.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/indented_file_end.txt"].assert_debug_eq(&program);
}

fn test_ast_integer_operations_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/integer_operations.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/integer_operations.txt").trim(), program.snapshot());
}
#[test]
fn test_program_integer_operations_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/integer_operations.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/integer_operations.txt"].assert_debug_eq(&program);
}

fn test_ast_it_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/it.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/it.txt").trim(), program.snapshot());
}
#[test]
fn test_program_it_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/it.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/it.txt"].assert_debug_eq(&program);
}

fn test_ast_keyword_method_names_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/keyword_method_names.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/keyword_method_names.txt").trim(), program.snapshot());
}
#[test]
fn test_program_keyword_method_names_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/keyword_method_names.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/keyword_method_names.txt"].assert_debug_eq(&program);
}

fn test_ast_keywords_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/keywords.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/keywords.txt").trim(), program.snapshot());
}
#[test]
fn test_program_keywords_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/keywords.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/keywords.txt"].assert_debug_eq(&program);
}

fn test_ast_lambda_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/lambda.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/lambda.txt").trim(), program.snapshot());
}
#[test]
fn test_program_lambda_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/lambda.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/lambda.txt"].assert_debug_eq(&program);
}

fn test_ast_method_calls_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/method_calls.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/method_calls.txt").trim(), program.snapshot());
}
#[test]
fn test_program_method_calls_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/method_calls.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/method_calls.txt"].assert_debug_eq(&program);
}

fn test_ast_methods_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/methods.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/methods.txt").trim(), program.snapshot());
}
#[test]
fn test_program_methods_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/methods.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/methods.txt"].assert_debug_eq(&program);
}

fn test_ast_modules_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/modules.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/modules.txt").trim(), program.snapshot());
}
#[test]
fn test_program_modules_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/modules.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/modules.txt"].assert_debug_eq(&program);
}

fn test_ast_multi_write_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/multi_write.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/multi_write.txt").trim(), program.snapshot());
}
#[test]
fn test_program_multi_write_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/multi_write.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/multi_write.txt"].assert_debug_eq(&program);
}

fn test_ast_newline_terminated_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/newline_terminated.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/newline_terminated.txt").trim(), program.snapshot());
}
#[test]
fn test_program_newline_terminated_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/newline_terminated.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/newline_terminated.txt"].assert_debug_eq(&program);
}

fn test_ast_next_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/next.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/next.txt").trim(), program.snapshot());
}
#[test]
fn test_program_next_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/next.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/next.txt"].assert_debug_eq(&program);
}

fn test_ast_nils_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/nils.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/nils.txt").trim(), program.snapshot());
}
#[test]
fn test_program_nils_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/nils.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/nils.txt"].assert_debug_eq(&program);
}

fn test_ast_non_alphanumeric_methods_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/non_alphanumeric_methods.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/non_alphanumeric_methods.txt").trim(), program.snapshot());
}
#[test]
fn test_program_non_alphanumeric_methods_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/non_alphanumeric_methods.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/non_alphanumeric_methods.txt"].assert_debug_eq(&program);
}

fn test_ast_not_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/not.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/not.txt").trim(), program.snapshot());
}
#[test]
fn test_program_not_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/not.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/not.txt"].assert_debug_eq(&program);
}

fn test_ast_numbers_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/numbers.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/numbers.txt").trim(), program.snapshot());
}
#[test]
fn test_program_numbers_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/numbers.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/numbers.txt"].assert_debug_eq(&program);
}

fn test_ast_patterns_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/patterns.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/patterns.txt").trim(), program.snapshot());
}
#[test]
fn test_program_patterns_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/patterns.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/patterns.txt"].assert_debug_eq(&program);
}

fn test_ast_procs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/procs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/procs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_procs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/procs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/procs.txt"].assert_debug_eq(&program);
}

fn test_ast_range_begin_open_exclusive_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/range_begin_open_exclusive.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/range_begin_open_exclusive.txt").trim(), program.snapshot());
}
#[test]
fn test_program_range_begin_open_exclusive_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/range_begin_open_exclusive.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/range_begin_open_exclusive.txt"].assert_debug_eq(&program);
}

fn test_ast_range_begin_open_inclusive_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/range_begin_open_inclusive.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/range_begin_open_inclusive.txt").trim(), program.snapshot());
}
#[test]
fn test_program_range_begin_open_inclusive_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/range_begin_open_inclusive.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/range_begin_open_inclusive.txt"].assert_debug_eq(&program);
}

fn test_ast_range_beginless_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/range_beginless.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/range_beginless.txt").trim(), program.snapshot());
}
#[test]
fn test_program_range_beginless_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/range_beginless.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/range_beginless.txt"].assert_debug_eq(&program);
}

fn test_ast_range_end_open_exclusive_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/range_end_open_exclusive.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/range_end_open_exclusive.txt").trim(), program.snapshot());
}
#[test]
fn test_program_range_end_open_exclusive_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/range_end_open_exclusive.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/range_end_open_exclusive.txt"].assert_debug_eq(&program);
}

fn test_ast_range_end_open_inclusive_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/range_end_open_inclusive.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/range_end_open_inclusive.txt").trim(), program.snapshot());
}
#[test]
fn test_program_range_end_open_inclusive_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/range_end_open_inclusive.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/range_end_open_inclusive.txt"].assert_debug_eq(&program);
}

fn test_ast_ranges_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/ranges.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/ranges.txt").trim(), program.snapshot());
}
#[test]
fn test_program_ranges_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/ranges.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/ranges.txt"].assert_debug_eq(&program);
}

fn test_ast_regex_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/regex.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/regex.txt").trim(), program.snapshot());
}
#[test]
fn test_program_regex_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/regex.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/regex.txt"].assert_debug_eq(&program);
}

fn test_ast_regex_char_width_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/regex_char_width.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/regex_char_width.txt").trim(), program.snapshot());
}
#[test]
fn test_program_regex_char_width_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/regex_char_width.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/regex_char_width.txt"].assert_debug_eq(&program);
}

fn test_ast_regex_escape_encoding_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/regex_escape_encoding.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/regex_escape_encoding.txt").trim(), program.snapshot());
}
#[test]
fn test_program_regex_escape_encoding_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/regex_escape_encoding.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/regex_escape_encoding.txt"].assert_debug_eq(&program);
}

fn test_ast_regex_with_fake_newlines_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/regex_with_fake_newlines.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/regex_with_fake_newlines.txt").trim(), program.snapshot());
}
#[test]
fn test_program_regex_with_fake_newlines_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/regex_with_fake_newlines.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/regex_with_fake_newlines.txt"].assert_debug_eq(&program);
}

fn test_ast_repeat_parameters_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/repeat_parameters.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/repeat_parameters.txt").trim(), program.snapshot());
}
#[test]
fn test_program_repeat_parameters_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/repeat_parameters.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/repeat_parameters.txt"].assert_debug_eq(&program);
}

fn test_ast_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/rescue.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/rescue.txt").trim(), program.snapshot());
}
#[test]
fn test_program_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/rescue.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/rescue.txt"].assert_debug_eq(&program);
}

fn test_ast_rescue_modifier_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/rescue_modifier.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/rescue_modifier.txt").trim(), program.snapshot());
}
#[test]
fn test_program_rescue_modifier_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/rescue_modifier.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/rescue_modifier.txt"].assert_debug_eq(&program);
}

fn test_ast_return_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/return.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/return.txt").trim(), program.snapshot());
}
#[test]
fn test_program_return_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/return.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/return.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_BEGIN_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/BEGIN.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/BEGIN.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_BEGIN_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/BEGIN.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/BEGIN.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_TestRubyParserShared_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/TestRubyParserShared.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/TestRubyParserShared.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_TestRubyParserShared_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/TestRubyParserShared.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/TestRubyParserShared.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb___ENCODING___txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/__ENCODING__.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/__ENCODING__.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb___ENCODING___txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/__ENCODING__.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/__ENCODING__.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_alias_gvar_backref_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/alias_gvar_backref.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/alias_gvar_backref.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_alias_gvar_backref_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/alias_gvar_backref.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/alias_gvar_backref.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_alias_resword_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/alias_resword.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/alias_resword.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_alias_resword_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/alias_resword.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/alias_resword.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_and_multi_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/and_multi.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/and_multi.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_and_multi_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/and_multi.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/and_multi.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_aref_args_assocs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/aref_args_assocs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/aref_args_assocs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_aref_args_assocs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/aref_args_assocs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/aref_args_assocs.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_aref_args_lit_assocs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/aref_args_lit_assocs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/aref_args_lit_assocs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_aref_args_lit_assocs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/aref_args_lit_assocs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/aref_args_lit_assocs.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_args_kw_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/args_kw_block.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/args_kw_block.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_args_kw_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/args_kw_block.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/args_kw_block.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_array_line_breaks_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/array_line_breaks.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/array_line_breaks.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_array_line_breaks_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/array_line_breaks.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/array_line_breaks.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_array_lits_trailing_calls_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/array_lits_trailing_calls.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/array_lits_trailing_calls.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_array_lits_trailing_calls_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/array_lits_trailing_calls.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/array_lits_trailing_calls.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_assoc__bare_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/assoc__bare.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/assoc__bare.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_assoc__bare_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/assoc__bare.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/assoc__bare.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_assoc_label_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/assoc_label.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/assoc_label.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_assoc_label_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/assoc_label.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/assoc_label.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_attr_asgn_colon_id_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/attr_asgn_colon_id.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/attr_asgn_colon_id.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_attr_asgn_colon_id_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/attr_asgn_colon_id.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/attr_asgn_colon_id.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_attrasgn_array_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/attrasgn_array_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/attrasgn_array_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_attrasgn_array_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/attrasgn_array_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/attrasgn_array_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_attrasgn_array_lhs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/attrasgn_array_lhs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/attrasgn_array_lhs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_attrasgn_array_lhs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/attrasgn_array_lhs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/attrasgn_array_lhs.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_attrasgn_primary_dot_constant_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/attrasgn_primary_dot_constant.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/attrasgn_primary_dot_constant.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_attrasgn_primary_dot_constant_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/attrasgn_primary_dot_constant.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/attrasgn_primary_dot_constant.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_backticks_interpolation_line_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/backticks_interpolation_line.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/backticks_interpolation_line.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_backticks_interpolation_line_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/backticks_interpolation_line.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/backticks_interpolation_line.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bang_eq_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bang_eq.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bang_eq.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bang_eq_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bang_eq.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bang_eq.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bdot2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bdot2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bdot2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bdot2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bdot2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bdot2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bdot3_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bdot3.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bdot3.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bdot3_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bdot3.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bdot3.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_begin_ensure_no_bodies_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/begin_ensure_no_bodies.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/begin_ensure_no_bodies.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_begin_ensure_no_bodies_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/begin_ensure_no_bodies.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/begin_ensure_no_bodies.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_begin_rescue_else_ensure_bodies_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/begin_rescue_else_ensure_bodies.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/begin_rescue_else_ensure_bodies.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_begin_rescue_else_ensure_bodies_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/begin_rescue_else_ensure_bodies.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/begin_rescue_else_ensure_bodies.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_begin_rescue_else_ensure_no_bodies_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/begin_rescue_else_ensure_no_bodies.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/begin_rescue_else_ensure_no_bodies.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_begin_rescue_else_ensure_no_bodies_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/begin_rescue_else_ensure_no_bodies.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/begin_rescue_else_ensure_no_bodies.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_begin_rescue_ensure_no_bodies_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/begin_rescue_ensure_no_bodies.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/begin_rescue_ensure_no_bodies.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_begin_rescue_ensure_no_bodies_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/begin_rescue_ensure_no_bodies.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/begin_rescue_ensure_no_bodies.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_arg__bare_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_arg__bare.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_arg__bare.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_arg__bare_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_arg__bare.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_arg__bare.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_arg_kwsplat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_arg_kwsplat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_arg_kwsplat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_arg_kwsplat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_arg_kwsplat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_arg_kwsplat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_arg_opt_arg_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_arg_opt_arg_block.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_arg_opt_arg_block.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_arg_opt_arg_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_arg_opt_arg_block.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_arg_opt_arg_block.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_arg_opt_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_arg_opt_splat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_arg_opt_splat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_arg_opt_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_arg_opt_splat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_arg_opt_splat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_arg_opt_splat_arg_block_omfg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_arg_opt_splat_arg_block_omfg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_arg_opt_splat_arg_block_omfg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_arg_opt_splat_arg_block_omfg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_arg_opt_splat_arg_block_omfg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_arg_opt_splat_arg_block_omfg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_arg_optional_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_arg_optional.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_arg_optional.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_arg_optional_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_arg_optional.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_arg_optional.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_arg_scope_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_arg_scope.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_arg_scope.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_arg_scope_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_arg_scope.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_arg_scope.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_arg_scope2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_arg_scope2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_arg_scope2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_arg_scope2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_arg_scope2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_arg_scope2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_arg_splat_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_arg_splat_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_arg_splat_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_arg_splat_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_arg_splat_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_arg_splat_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_args_kwargs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_args_kwargs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_args_kwargs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_args_kwargs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_args_kwargs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_args_kwargs.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_args_no_kwargs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_args_no_kwargs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_args_no_kwargs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_args_no_kwargs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_args_no_kwargs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_args_no_kwargs.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_args_opt1_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_args_opt1.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_args_opt1.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_args_opt1_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_args_opt1.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_args_opt1.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_args_opt2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_args_opt2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_args_opt2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_args_opt2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_args_opt2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_args_opt2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_args_opt2_2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_args_opt2_2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_args_opt2_2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_args_opt2_2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_args_opt2_2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_args_opt2_2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_args_opt3_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_args_opt3.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_args_opt3.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_args_opt3_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_args_opt3.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_args_opt3.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_call_defn_call_block_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_call_defn_call_block_call.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_call_defn_call_block_call.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_call_defn_call_block_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_call_defn_call_block_call.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_call_defn_call_block_call.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_call_dot_op2_brace_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_call_dot_op2_brace_block.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_call_dot_op2_brace_block.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_call_dot_op2_brace_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_call_dot_op2_brace_block.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_call_dot_op2_brace_block.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_call_dot_op2_cmd_args_do_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_call_dot_op2_cmd_args_do_block.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_call_dot_op2_cmd_args_do_block.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_call_dot_op2_cmd_args_do_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_call_dot_op2_cmd_args_do_block.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_call_dot_op2_cmd_args_do_block.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_call_operation_colon_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_call_operation_colon.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_call_operation_colon.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_call_operation_colon_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_call_operation_colon.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_call_operation_colon.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_call_operation_dot_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_call_operation_dot.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_call_operation_dot.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_call_operation_dot_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_call_operation_dot.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_call_operation_dot.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_call_paren_call_block_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_call_paren_call_block_call.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_call_paren_call_block_call.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_call_paren_call_block_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_call_paren_call_block_call.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_call_paren_call_block_call.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_command_operation_colon_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_command_operation_colon.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_command_operation_colon.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_command_operation_colon_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_command_operation_colon.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_command_operation_colon.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_command_operation_dot_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_command_operation_dot.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_command_operation_dot.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_command_operation_dot_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_command_operation_dot.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_command_operation_dot.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_decomp_anon_splat_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_decomp_anon_splat_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_decomp_anon_splat_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_decomp_anon_splat_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_decomp_anon_splat_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_decomp_anon_splat_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_decomp_arg_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_decomp_arg_splat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_decomp_arg_splat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_decomp_arg_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_decomp_arg_splat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_decomp_arg_splat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_decomp_arg_splat_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_decomp_arg_splat_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_decomp_arg_splat_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_decomp_arg_splat_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_decomp_arg_splat_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_decomp_arg_splat_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_decomp_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_decomp_splat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_decomp_splat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_decomp_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_decomp_splat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_decomp_splat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_kw_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_kw.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_kw.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_kw_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_kw.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_kw.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_kw__required_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_kw__required.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_kw__required.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_kw__required_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_kw__required.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_kw__required.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_kwarg_lvar_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_kwarg_lvar.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_kwarg_lvar.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_kwarg_lvar_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_kwarg_lvar.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_kwarg_lvar.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_kwarg_lvar_multiple_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_kwarg_lvar_multiple.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_kwarg_lvar_multiple.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_kwarg_lvar_multiple_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_kwarg_lvar_multiple.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_kwarg_lvar_multiple.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_opt_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_opt_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_opt_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_opt_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_opt_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_opt_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_opt_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_opt_splat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_opt_splat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_opt_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_opt_splat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_opt_splat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_opt_splat_arg_block_omfg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_opt_splat_arg_block_omfg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_opt_splat_arg_block_omfg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_opt_splat_arg_block_omfg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_opt_splat_arg_block_omfg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_opt_splat_arg_block_omfg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_optarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_optarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_optarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_optarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_optarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_optarg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_paren_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_paren_splat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_paren_splat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_paren_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_paren_splat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_paren_splat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_reg_optarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_reg_optarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_reg_optarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_reg_optarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_reg_optarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_reg_optarg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_return_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_return.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_return.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_return_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_return.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_return.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_scope_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_scope.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_scope.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_scope_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_scope.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_scope.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_block_splat_reg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_splat_reg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/block_splat_reg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_block_splat_reg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/block_splat_reg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/block_splat_reg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug169_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug169.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug169.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug169_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug169.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug169.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug179_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug179.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug179.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug179_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug179.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug179.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug190_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug190.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug190.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug190_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug190.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug190.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug191_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug191.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug191.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug191_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug191.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug191.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug202_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug202.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug202.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug202_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug202.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug202.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug236_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug236.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug236.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug236_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug236.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug236.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug290_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug290.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug290.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug290_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug290.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug290.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug_187_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_187.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug_187.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug_187_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_187.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug_187.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug_215_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_215.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug_215.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug_215_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_215.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug_215.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug_249_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_249.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug_249.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug_249_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_249.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug_249.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug_and_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_and.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug_and.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug_and_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_and.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug_and.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug_args__19_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_args__19.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug_args__19.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug_args__19_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_args__19.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug_args__19.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug_args_masgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_args_masgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug_args_masgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug_args_masgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_args_masgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug_args_masgn.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug_args_masgn2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_args_masgn2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug_args_masgn2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug_args_masgn2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_args_masgn2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug_args_masgn2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug_args_masgn_outer_parens__19_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_args_masgn_outer_parens__19.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug_args_masgn_outer_parens__19.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug_args_masgn_outer_parens__19_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_args_masgn_outer_parens__19.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug_args_masgn_outer_parens__19.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug_call_arglist_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_call_arglist_parens.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug_call_arglist_parens.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug_call_arglist_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_call_arglist_parens.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug_call_arglist_parens.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug_case_when_regexp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_case_when_regexp.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug_case_when_regexp.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug_case_when_regexp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_case_when_regexp.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug_case_when_regexp.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_comma.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug_comma.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_comma.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug_comma.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug_cond_pct_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_cond_pct.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug_cond_pct.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug_cond_pct_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_cond_pct.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug_cond_pct.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug_hash_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_hash_args.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug_hash_args.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug_hash_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_hash_args.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug_hash_args.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug_hash_args_trailing_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_hash_args_trailing_comma.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug_hash_args_trailing_comma.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug_hash_args_trailing_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_hash_args_trailing_comma.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug_hash_args_trailing_comma.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug_hash_interp_array_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_hash_interp_array.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug_hash_interp_array.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug_hash_interp_array_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_hash_interp_array.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug_hash_interp_array.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug_masgn_right_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_masgn_right.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug_masgn_right.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug_masgn_right_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_masgn_right.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug_masgn_right.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug_not_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_not_parens.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug_not_parens.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug_not_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_not_parens.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug_not_parens.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_bug_op_asgn_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_op_asgn_rescue.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/bug_op_asgn_rescue.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_bug_op_asgn_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/bug_op_asgn_rescue.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/bug_op_asgn_rescue.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_and_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_and.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_and.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_and_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_and.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_and.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_arg_assoc_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_arg_assoc.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_arg_assoc.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_arg_assoc_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_arg_assoc.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_arg_assoc.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_arg_assoc_kwsplat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_arg_assoc_kwsplat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_arg_assoc_kwsplat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_arg_assoc_kwsplat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_arg_assoc_kwsplat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_arg_assoc_kwsplat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_arg_kwsplat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_arg_kwsplat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_arg_kwsplat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_arg_kwsplat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_arg_kwsplat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_arg_kwsplat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_args_assoc_quoted_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_args_assoc_quoted.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_args_assoc_quoted.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_args_assoc_quoted_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_args_assoc_quoted.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_args_assoc_quoted.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_args_assoc_trailing_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_args_assoc_trailing_comma.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_args_assoc_trailing_comma.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_args_assoc_trailing_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_args_assoc_trailing_comma.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_args_assoc_trailing_comma.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_args_command_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_args_command.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_args_command.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_args_command_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_args_command.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_args_command.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_array_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_array_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_array_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_array_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_array_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_array_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_array_block_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_array_block_call.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_array_block_call.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_array_block_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_array_block_call.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_array_block_call.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_array_lambda_block_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_array_lambda_block_call.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_array_lambda_block_call.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_array_lambda_block_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_array_lambda_block_call.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_array_lambda_block_call.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_array_lit_inline_hash_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_array_lit_inline_hash.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_array_lit_inline_hash.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_array_lit_inline_hash_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_array_lit_inline_hash.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_array_lit_inline_hash.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_assoc_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_assoc.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_assoc.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_assoc_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_assoc.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_assoc.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_assoc_new_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_assoc_new.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_assoc_new.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_assoc_new_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_assoc_new.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_assoc_new.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_assoc_new_if_multiline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_assoc_new_if_multiline.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_assoc_new_if_multiline.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_assoc_new_if_multiline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_assoc_new_if_multiline.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_assoc_new_if_multiline.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_assoc_trailing_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_assoc_trailing_comma.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_assoc_trailing_comma.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_assoc_trailing_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_assoc_trailing_comma.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_assoc_trailing_comma.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_bang_command_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_bang_command_call.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_bang_command_call.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_bang_command_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_bang_command_call.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_bang_command_call.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_bang_squiggle_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_bang_squiggle.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_bang_squiggle.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_bang_squiggle_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_bang_squiggle.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_bang_squiggle.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_begin_call_block_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_begin_call_block_call.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_begin_call_block_call.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_begin_call_block_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_begin_call_block_call.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_begin_call_block_call.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_block_arg_named_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_block_arg_named.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_block_arg_named.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_block_arg_named_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_block_arg_named.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_block_arg_named.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_carat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_carat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_carat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_carat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_carat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_carat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_colon2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_colon2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_colon2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_colon2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_colon2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_colon2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_colon_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_colon_parens.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_colon_parens.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_colon_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_colon_parens.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_colon_parens.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_div_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_div.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_div.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_div_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_div.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_div.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_dot_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_dot_parens.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_dot_parens.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_dot_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_dot_parens.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_dot_parens.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_env_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_env.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_env.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_env_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_env.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_env.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_eq3_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_eq3.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_eq3.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_eq3_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_eq3.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_eq3.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_gt_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_gt.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_gt.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_gt_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_gt.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_gt.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_kwsplat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_kwsplat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_kwsplat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_kwsplat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_kwsplat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_kwsplat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_leading_dots_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_leading_dots.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_leading_dots.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_leading_dots_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_leading_dots.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_leading_dots.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_leading_dots_comment_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_leading_dots_comment.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_leading_dots_comment.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_leading_dots_comment_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_leading_dots_comment.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_leading_dots_comment.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_lt_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_lt.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_lt.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_lt_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_lt.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_lt.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_lte_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_lte.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_lte.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_lte_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_lte.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_lte.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_not_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_not.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_not.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_not_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_not.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_not.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_pipe_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_pipe.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_pipe.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_pipe_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_pipe.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_pipe.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_rshift_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_rshift.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_rshift.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_rshift_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_rshift.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_rshift.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_self_brackets_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_self_brackets.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_self_brackets.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_self_brackets_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_self_brackets.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_self_brackets.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_spaceship_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_spaceship.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_spaceship.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_spaceship_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_spaceship.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_spaceship.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_stabby_do_end_with_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_stabby_do_end_with_block.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_stabby_do_end_with_block.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_stabby_do_end_with_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_stabby_do_end_with_block.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_stabby_do_end_with_block.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_stabby_with_braces_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_stabby_with_braces_block.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_stabby_with_braces_block.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_stabby_with_braces_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_stabby_with_braces_block.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_stabby_with_braces_block.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_star_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_star.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_star.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_star_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_star.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_star.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_star2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_star2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_star2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_star2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_star2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_star2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_trailing_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_trailing_comma.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_trailing_comma.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_trailing_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_trailing_comma.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_trailing_comma.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_trailing_dots_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_trailing_dots.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_trailing_dots.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_trailing_dots_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_trailing_dots.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_trailing_dots.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_call_unary_bang_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_unary_bang.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/call_unary_bang.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_call_unary_bang_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/call_unary_bang.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/call_unary_bang.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_31_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_31.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_31.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_31_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_31.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_31.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_37_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_37.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_37.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_37_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_37.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_37.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_42_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_42.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_42.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_42_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_42.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_42.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_42_2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_42_2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_42_2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_42_2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_42_2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_42_2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_47_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_47.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_47.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_47_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_47.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_47.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_67_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_67.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_67.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_67_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_67.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_67.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_86_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_86.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_86.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_86_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_86.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_86.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_86_2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_86_2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_86_2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_86_2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_86_2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_86_2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_array_pat_const_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_array_pat_const.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_array_pat_const.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_array_pat_const_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_array_pat_const.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_array_pat_const.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_array_pat_const2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_array_pat_const2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_array_pat_const2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_array_pat_const2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_array_pat_const2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_array_pat_const2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_array_pat_paren_assign_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_array_pat_paren_assign.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_array_pat_paren_assign.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_array_pat_paren_assign_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_array_pat_paren_assign.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_array_pat_paren_assign.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_const_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_const.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_const.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_const_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_const.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_const.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_else_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_else.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_else.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_else_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_else.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_else.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_find_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_find.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_find.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_find_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_find.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_find.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_find_array_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_find_array.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_find_array.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_find_array_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_find_array.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_find_array.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_hash_pat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_hash_pat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_hash_pat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_hash_pat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_hash_pat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_hash_pat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_hash_pat_assign_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_hash_pat_assign.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_hash_pat_assign.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_hash_pat_assign_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_hash_pat_assign.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_hash_pat_assign.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_hash_pat_paren_assign_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_hash_pat_paren_assign.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_hash_pat_paren_assign.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_hash_pat_paren_assign_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_hash_pat_paren_assign.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_hash_pat_paren_assign.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_hash_pat_paren_true_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_hash_pat_paren_true.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_hash_pat_paren_true.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_hash_pat_paren_true_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_hash_pat_paren_true.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_hash_pat_paren_true.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_hash_pat_rest_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_hash_pat_rest.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_hash_pat_rest.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_hash_pat_rest_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_hash_pat_rest.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_hash_pat_rest.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_hash_pat_rest_solo_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_hash_pat_rest_solo.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_hash_pat_rest_solo.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_hash_pat_rest_solo_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_hash_pat_rest_solo.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_hash_pat_rest_solo.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_if_unless_post_mod_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_if_unless_post_mod.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_if_unless_post_mod.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_if_unless_post_mod_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_if_unless_post_mod.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_if_unless_post_mod.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_multiple_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_multiple.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_multiple.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_multiple_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_multiple.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_multiple.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_case_in_or_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_or.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/case_in_or.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_case_in_or_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/case_in_or.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/case_in_or.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_class_comments_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/class_comments.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/class_comments.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_class_comments_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/class_comments.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/class_comments.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_cond_unary_minus_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/cond_unary_minus.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/cond_unary_minus.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_cond_unary_minus_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/cond_unary_minus.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/cond_unary_minus.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_const_2_op_asgn_or2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/const_2_op_asgn_or2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/const_2_op_asgn_or2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_const_2_op_asgn_or2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/const_2_op_asgn_or2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/const_2_op_asgn_or2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_const_3_op_asgn_or_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/const_3_op_asgn_or.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/const_3_op_asgn_or.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_const_3_op_asgn_or_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/const_3_op_asgn_or.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/const_3_op_asgn_or.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_const_op_asgn_and1_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/const_op_asgn_and1.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/const_op_asgn_and1.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_const_op_asgn_and1_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/const_op_asgn_and1.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/const_op_asgn_and1.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_const_op_asgn_and2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/const_op_asgn_and2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/const_op_asgn_and2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_const_op_asgn_and2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/const_op_asgn_and2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/const_op_asgn_and2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_const_op_asgn_or_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/const_op_asgn_or.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/const_op_asgn_or.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_const_op_asgn_or_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/const_op_asgn_or.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/const_op_asgn_or.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defined_eh_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defined_eh_parens.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defined_eh_parens.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defined_eh_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defined_eh_parens.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defined_eh_parens.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_arg_asplat_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_arg_asplat_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_arg_asplat_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_arg_asplat_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_arg_asplat_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_arg_asplat_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_arg_forward_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_arg_forward_args.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_arg_forward_args.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_arg_forward_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_arg_forward_args.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_arg_forward_args.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_args_forward_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_args_forward_args.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_args_forward_args.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_args_forward_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_args_forward_args.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_args_forward_args.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_comments_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_comments.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_comments.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_comments_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_comments.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_comments.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_endless_command_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_endless_command.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_endless_command.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_endless_command_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_endless_command.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_endless_command.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_endless_command_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_endless_command_rescue.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_endless_command_rescue.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_endless_command_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_endless_command_rescue.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_endless_command_rescue.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_forward_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_forward_args.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_forward_args.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_forward_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_forward_args.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_forward_args.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_forward_args__no_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_forward_args__no_parens.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_forward_args__no_parens.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_forward_args__no_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_forward_args__no_parens.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_forward_args__no_parens.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_kwarg_env_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_kwarg_env.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_kwarg_env.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_kwarg_env_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_kwarg_env.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_kwarg_env.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_kwarg_kwarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_kwarg_kwarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_kwarg_kwarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_kwarg_kwarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_kwarg_kwarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_kwarg_kwarg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_kwarg_kwsplat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_kwarg_kwsplat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_kwarg_kwsplat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_kwarg_kwsplat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_kwarg_kwsplat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_kwarg_kwsplat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_kwarg_kwsplat_anon_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_kwarg_kwsplat_anon.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_kwarg_kwsplat_anon.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_kwarg_kwsplat_anon_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_kwarg_kwsplat_anon.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_kwarg_kwsplat_anon.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_kwarg_lvar_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_kwarg_lvar.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_kwarg_lvar.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_kwarg_lvar_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_kwarg_lvar.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_kwarg_lvar.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_kwarg_no_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_kwarg_no_parens.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_kwarg_no_parens.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_kwarg_no_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_kwarg_no_parens.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_kwarg_no_parens.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_kwarg_val_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_kwarg_val.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_kwarg_val.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_kwarg_val_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_kwarg_val.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_kwarg_val.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_no_kwargs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_no_kwargs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_no_kwargs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_no_kwargs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_no_kwargs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_no_kwargs.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_oneliner_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_oneliner.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_oneliner.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_oneliner_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_oneliner.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_oneliner.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_oneliner_eq2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_oneliner_eq2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_oneliner_eq2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_oneliner_eq2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_oneliner_eq2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_oneliner_eq2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_oneliner_noargs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_oneliner_noargs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_oneliner_noargs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_oneliner_noargs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_oneliner_noargs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_oneliner_noargs.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_oneliner_noargs_parentheses_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_oneliner_noargs_parentheses.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_oneliner_noargs_parentheses.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_oneliner_noargs_parentheses_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_oneliner_noargs_parentheses.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_oneliner_noargs_parentheses.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_oneliner_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_oneliner_rescue.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_oneliner_rescue.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_oneliner_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_oneliner_rescue.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_oneliner_rescue.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_opt_last_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_opt_last_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_opt_last_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_opt_last_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_opt_last_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_opt_last_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_opt_reg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_opt_reg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_opt_reg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_opt_reg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_opt_reg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_opt_reg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_opt_splat_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_opt_splat_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_opt_splat_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_opt_splat_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_opt_splat_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_opt_splat_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_powarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_powarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_powarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_powarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_powarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_powarg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_reg_opt_reg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_reg_opt_reg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_reg_opt_reg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_reg_opt_reg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_reg_opt_reg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_reg_opt_reg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_splat_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_splat_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_splat_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_splat_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_splat_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_splat_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defn_unary_not_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_unary_not.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defn_unary_not.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defn_unary_not_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defn_unary_not.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defn_unary_not.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defns_reserved_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defns_reserved.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defns_reserved.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defns_reserved_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defns_reserved.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defns_reserved.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defs_as_arg_with_do_block_inside_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defs_as_arg_with_do_block_inside.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defs_as_arg_with_do_block_inside.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defs_as_arg_with_do_block_inside_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defs_as_arg_with_do_block_inside.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defs_as_arg_with_do_block_inside.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defs_comments_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defs_comments.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defs_comments.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defs_comments_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defs_comments.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defs_comments.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defs_endless_command_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defs_endless_command.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defs_endless_command.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defs_endless_command_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defs_endless_command.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defs_endless_command.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defs_endless_command_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defs_endless_command_rescue.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defs_endless_command_rescue.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defs_endless_command_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defs_endless_command_rescue.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defs_endless_command_rescue.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defs_kwarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defs_kwarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defs_kwarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defs_kwarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defs_kwarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defs_kwarg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defs_oneliner_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defs_oneliner.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defs_oneliner.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defs_oneliner_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defs_oneliner.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defs_oneliner.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defs_oneliner_eq2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defs_oneliner_eq2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defs_oneliner_eq2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defs_oneliner_eq2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defs_oneliner_eq2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defs_oneliner_eq2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_defs_oneliner_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defs_oneliner_rescue.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/defs_oneliner_rescue.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_defs_oneliner_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/defs_oneliner_rescue.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/defs_oneliner_rescue.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult0__txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult0_.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult0_.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult0__txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult0_.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult0_.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult1_line_numbers_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult1_line_numbers.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult1_line_numbers.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult1_line_numbers_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult1_line_numbers.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult1_line_numbers.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult1_line_numbers2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult1_line_numbers2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult1_line_numbers2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult1_line_numbers2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult1_line_numbers2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult1_line_numbers2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult2__txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult2_.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult2_.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult2__txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult2_.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult2_.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult3__txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3_.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult3_.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult3__txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3_.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult3_.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult3_2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3_2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult3_2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult3_2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3_2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult3_2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult3_3_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3_3.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult3_3.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult3_3_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3_3.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult3_3.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult3_4_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3_4.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult3_4.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult3_4_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3_4.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult3_4.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult3_5_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3_5.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult3_5.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult3_5_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3_5.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult3_5.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult3__10_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3__10.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult3__10.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult3__10_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3__10.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult3__10.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult3__11_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3__11.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult3__11.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult3__11_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3__11.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult3__11.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult3__12_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3__12.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult3__12.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult3__12_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3__12.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult3__12.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult3__6_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3__6.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult3__6.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult3__6_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3__6.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult3__6.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult3__7_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3__7.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult3__7.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult3__7_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3__7.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult3__7.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult3__8_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3__8.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult3__8.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult3__8_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3__8.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult3__8.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult3__9_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3__9.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult3__9.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult3__9_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult3__9.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult3__9.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult4__leading_dots_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult4__leading_dots.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult4__leading_dots.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult4__leading_dots_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult4__leading_dots.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult4__leading_dots.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult4__leading_dots2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult4__leading_dots2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult4__leading_dots2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult4__leading_dots2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult4__leading_dots2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult4__leading_dots2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult6__txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult6_.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult6_.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult6__txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult6_.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult6_.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult6__7_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult6__7.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult6__7.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult6__7_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult6__7.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult6__7.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult6__8_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult6__8.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult6__8.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult6__8_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult6__8.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult6__8.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_difficult7__txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult7_.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/difficult7_.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_difficult7__txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/difficult7_.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/difficult7_.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_do_bug_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/do_bug.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/do_bug.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_do_bug_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/do_bug.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/do_bug.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_do_lambda_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/do_lambda.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/do_lambda.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_do_lambda_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/do_lambda.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/do_lambda.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_dot2_nil__26_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/dot2_nil__26.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/dot2_nil__26.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_dot2_nil__26_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/dot2_nil__26.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/dot2_nil__26.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_dot3_nil__26_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/dot3_nil__26.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/dot3_nil__26.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_dot3_nil__26_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/dot3_nil__26.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/dot3_nil__26.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_dstr_evstr_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/dstr_evstr.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/dstr_evstr.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_dstr_evstr_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/dstr_evstr.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/dstr_evstr.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_dstr_evstr_empty_end_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/dstr_evstr_empty_end.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/dstr_evstr_empty_end.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_dstr_evstr_empty_end_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/dstr_evstr_empty_end.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/dstr_evstr_empty_end.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_dstr_lex_state_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/dstr_lex_state.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/dstr_lex_state.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_dstr_lex_state_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/dstr_lex_state.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/dstr_lex_state.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_dstr_str_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/dstr_str.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/dstr_str.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_dstr_str_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/dstr_str.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/dstr_str.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_dsym_esc_to_sym_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/dsym_esc_to_sym.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/dsym_esc_to_sym.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_dsym_esc_to_sym_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/dsym_esc_to_sym.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/dsym_esc_to_sym.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_dsym_to_sym_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/dsym_to_sym.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/dsym_to_sym.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_dsym_to_sym_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/dsym_to_sym.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/dsym_to_sym.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_eq_begin_line_numbers_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/eq_begin_line_numbers.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/eq_begin_line_numbers.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_eq_begin_line_numbers_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/eq_begin_line_numbers.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/eq_begin_line_numbers.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_eq_begin_why_wont_people_use_their_spacebar_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/eq_begin_why_wont_people_use_their_spacebar.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/eq_begin_why_wont_people_use_their_spacebar.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_eq_begin_why_wont_people_use_their_spacebar_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/eq_begin_why_wont_people_use_their_spacebar.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/eq_begin_why_wont_people_use_their_spacebar.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_evstr_evstr_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/evstr_evstr.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/evstr_evstr.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_evstr_evstr_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/evstr_evstr.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/evstr_evstr.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_evstr_str_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/evstr_str.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/evstr_str.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_evstr_str_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/evstr_str.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/evstr_str.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_expr_not_bang_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/expr_not_bang.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/expr_not_bang.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_expr_not_bang_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/expr_not_bang.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/expr_not_bang.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_f_kw_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/f_kw.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/f_kw.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_f_kw_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/f_kw.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/f_kw.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_f_kw__required_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/f_kw__required.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/f_kw__required.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_f_kw__required_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/f_kw__required.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/f_kw__required.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_flip2_env_lvar_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/flip2_env_lvar.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/flip2_env_lvar.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_flip2_env_lvar_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/flip2_env_lvar.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/flip2_env_lvar.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_float_with_if_modifier_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/float_with_if_modifier.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/float_with_if_modifier.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_float_with_if_modifier_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/float_with_if_modifier.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/float_with_if_modifier.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc__backslash_dos_format_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc__backslash_dos_format.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc__backslash_dos_format.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc__backslash_dos_format_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc__backslash_dos_format.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc__backslash_dos_format.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_backslash_nl_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_backslash_nl.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_backslash_nl.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_backslash_nl_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_backslash_nl.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_backslash_nl.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_bad_hex_escape_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_bad_hex_escape.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_bad_hex_escape.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_bad_hex_escape_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_bad_hex_escape.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_bad_hex_escape.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_bad_oct_escape_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_bad_oct_escape.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_bad_oct_escape.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_bad_oct_escape_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_bad_oct_escape.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_bad_oct_escape.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_comma_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_comma_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_comma_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_comma_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_comma_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_comma_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_lineno_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_lineno.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_lineno.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_lineno_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_lineno.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_lineno.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_nested_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_nested.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_nested.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_nested_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_nested.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_nested.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_squiggly_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_squiggly.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_squiggly.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_squiggly_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_squiggly.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_squiggly.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_squiggly_blank_line_plus_interpolation_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_squiggly_blank_line_plus_interpolation.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_squiggly_blank_line_plus_interpolation.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_squiggly_blank_line_plus_interpolation_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_squiggly_blank_line_plus_interpolation.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_squiggly_blank_line_plus_interpolation.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_squiggly_blank_lines_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_squiggly_blank_lines.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_squiggly_blank_lines.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_squiggly_blank_lines_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_squiggly_blank_lines.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_squiggly_blank_lines.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_squiggly_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_squiggly_empty.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_squiggly_empty.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_squiggly_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_squiggly_empty.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_squiggly_empty.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_squiggly_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_squiggly_interp.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_squiggly_interp.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_squiggly_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_squiggly_interp.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_squiggly_interp.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_squiggly_no_indent_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_squiggly_no_indent.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_squiggly_no_indent.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_squiggly_no_indent_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_squiggly_no_indent.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_squiggly_no_indent.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_squiggly_tabs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_squiggly_tabs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_squiggly_tabs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_squiggly_tabs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_squiggly_tabs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_squiggly_tabs.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_squiggly_tabs_extra_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_squiggly_tabs_extra.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_squiggly_tabs_extra.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_squiggly_tabs_extra_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_squiggly_tabs_extra.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_squiggly_tabs_extra.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_squiggly_visually_blank_lines_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_squiggly_visually_blank_lines.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_squiggly_visually_blank_lines.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_squiggly_visually_blank_lines_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_squiggly_visually_blank_lines.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_squiggly_visually_blank_lines.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_trailing_slash_continued_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_trailing_slash_continued_call.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_trailing_slash_continued_call.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_trailing_slash_continued_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_trailing_slash_continued_call.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_trailing_slash_continued_call.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_unicode_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_unicode.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_unicode.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_unicode_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_unicode.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_unicode.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_with_carriage_return_escapes_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_with_carriage_return_escapes.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_with_carriage_return_escapes.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_with_carriage_return_escapes_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_with_carriage_return_escapes.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_with_carriage_return_escapes.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_with_carriage_return_escapes_windows_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_with_carriage_return_escapes_windows.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_with_carriage_return_escapes_windows.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_with_carriage_return_escapes_windows_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_with_carriage_return_escapes_windows.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_with_carriage_return_escapes_windows.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_with_extra_carriage_horrible_mix_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_with_extra_carriage_horrible_mix.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_with_extra_carriage_horrible_mix.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_with_extra_carriage_horrible_mix_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_with_extra_carriage_horrible_mix.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_with_extra_carriage_horrible_mix.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_with_extra_carriage_returns_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_with_extra_carriage_returns.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_with_extra_carriage_returns.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_with_extra_carriage_returns_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_with_extra_carriage_returns.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_with_extra_carriage_returns.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_with_extra_carriage_returns_windows_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_with_extra_carriage_returns_windows.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_with_extra_carriage_returns_windows.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_with_extra_carriage_returns_windows_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_with_extra_carriage_returns_windows.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_with_extra_carriage_returns_windows.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_with_interpolation_and_carriage_return_escapes_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_with_interpolation_and_carriage_return_escapes.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_with_interpolation_and_carriage_return_escapes.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_with_interpolation_and_carriage_return_escapes_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_with_interpolation_and_carriage_return_escapes.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_with_interpolation_and_carriage_return_escapes.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_with_interpolation_and_carriage_return_escapes_windows_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_with_interpolation_and_carriage_return_escapes_windows.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_with_interpolation_and_carriage_return_escapes_windows.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_with_interpolation_and_carriage_return_escapes_windows_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_with_interpolation_and_carriage_return_escapes_windows.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_with_interpolation_and_carriage_return_escapes_windows.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_with_not_global_interpolation_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_with_not_global_interpolation.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_with_not_global_interpolation.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_with_not_global_interpolation_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_with_not_global_interpolation.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_with_not_global_interpolation.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_with_only_carriage_returns_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_with_only_carriage_returns.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_with_only_carriage_returns.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_with_only_carriage_returns_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_with_only_carriage_returns.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_with_only_carriage_returns.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_heredoc_with_only_carriage_returns_windows_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_with_only_carriage_returns_windows.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/heredoc_with_only_carriage_returns_windows.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_heredoc_with_only_carriage_returns_windows_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/heredoc_with_only_carriage_returns_windows.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/heredoc_with_only_carriage_returns_windows.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_if_elsif_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/if_elsif.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/if_elsif.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_if_elsif_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/if_elsif.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/if_elsif.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_if_symbol_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/if_symbol.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/if_symbol.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_if_symbol_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/if_symbol.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/if_symbol.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_in_expr_no_case_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/in_expr_no_case.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/in_expr_no_case.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_in_expr_no_case_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/in_expr_no_case.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/in_expr_no_case.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_index_0_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/index_0.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/index_0.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_index_0_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/index_0.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/index_0.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_index_0_opasgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/index_0_opasgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/index_0_opasgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_index_0_opasgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/index_0_opasgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/index_0_opasgn.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_integer_with_if_modifier_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/integer_with_if_modifier.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/integer_with_if_modifier.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_integer_with_if_modifier_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/integer_with_if_modifier.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/integer_with_if_modifier.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_interpolated_symbol_array_line_breaks_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/interpolated_symbol_array_line_breaks.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/interpolated_symbol_array_line_breaks.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_interpolated_symbol_array_line_breaks_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/interpolated_symbol_array_line_breaks.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/interpolated_symbol_array_line_breaks.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_interpolated_word_array_line_breaks_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/interpolated_word_array_line_breaks.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/interpolated_word_array_line_breaks.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_interpolated_word_array_line_breaks_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/interpolated_word_array_line_breaks.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/interpolated_word_array_line_breaks.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_iter_args_1_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_1.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/iter_args_1.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_iter_args_1_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_1.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/iter_args_1.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_iter_args_10_1_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_10_1.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/iter_args_10_1.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_iter_args_10_1_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_10_1.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/iter_args_10_1.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_iter_args_10_2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_10_2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/iter_args_10_2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_iter_args_10_2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_10_2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/iter_args_10_2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_iter_args_11_1_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_11_1.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/iter_args_11_1.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_iter_args_11_1_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_11_1.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/iter_args_11_1.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_iter_args_11_2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_11_2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/iter_args_11_2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_iter_args_11_2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_11_2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/iter_args_11_2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_iter_args_2__19_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_2__19.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/iter_args_2__19.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_iter_args_2__19_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_2__19.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/iter_args_2__19.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_iter_args_3_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_3.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/iter_args_3.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_iter_args_3_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_3.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/iter_args_3.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_iter_args_4_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_4.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/iter_args_4.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_iter_args_4_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_4.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/iter_args_4.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_iter_args_5_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_5.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/iter_args_5.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_iter_args_5_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_5.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/iter_args_5.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_iter_args_6_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_6.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/iter_args_6.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_iter_args_6_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_6.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/iter_args_6.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_iter_args_7_1_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_7_1.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/iter_args_7_1.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_iter_args_7_1_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_7_1.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/iter_args_7_1.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_iter_args_7_2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_7_2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/iter_args_7_2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_iter_args_7_2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_7_2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/iter_args_7_2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_iter_args_8_1_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_8_1.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/iter_args_8_1.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_iter_args_8_1_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_8_1.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/iter_args_8_1.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_iter_args_8_2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_8_2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/iter_args_8_2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_iter_args_8_2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_8_2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/iter_args_8_2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_iter_args_9_1_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_9_1.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/iter_args_9_1.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_iter_args_9_1_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_9_1.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/iter_args_9_1.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_iter_args_9_2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_9_2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/iter_args_9_2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_iter_args_9_2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_args_9_2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/iter_args_9_2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_iter_kwarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_kwarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/iter_kwarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_iter_kwarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_kwarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/iter_kwarg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_iter_kwarg_kwsplat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_kwarg_kwsplat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/iter_kwarg_kwsplat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_iter_kwarg_kwsplat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/iter_kwarg_kwsplat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/iter_kwarg_kwsplat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_label_vs_string_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/label_vs_string.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/label_vs_string.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_label_vs_string_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/label_vs_string.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/label_vs_string.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_lambda_do_vs_brace_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/lambda_do_vs_brace.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/lambda_do_vs_brace.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_lambda_do_vs_brace_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/lambda_do_vs_brace.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/lambda_do_vs_brace.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_lasgn_arg_rescue_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/lasgn_arg_rescue_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/lasgn_arg_rescue_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_lasgn_arg_rescue_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/lasgn_arg_rescue_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/lasgn_arg_rescue_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_lasgn_call_bracket_rescue_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/lasgn_call_bracket_rescue_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/lasgn_call_bracket_rescue_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_lasgn_call_bracket_rescue_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/lasgn_call_bracket_rescue_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/lasgn_call_bracket_rescue_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_lasgn_call_nobracket_rescue_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/lasgn_call_nobracket_rescue_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/lasgn_call_nobracket_rescue_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_lasgn_call_nobracket_rescue_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/lasgn_call_nobracket_rescue_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/lasgn_call_nobracket_rescue_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_lasgn_command_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/lasgn_command.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/lasgn_command.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_lasgn_command_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/lasgn_command.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/lasgn_command.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_lasgn_env_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/lasgn_env.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/lasgn_env.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_lasgn_env_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/lasgn_env.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/lasgn_env.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_lasgn_ivar_env_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/lasgn_ivar_env.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/lasgn_ivar_env.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_lasgn_ivar_env_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/lasgn_ivar_env.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/lasgn_ivar_env.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_lasgn_lasgn_command_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/lasgn_lasgn_command_call.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/lasgn_lasgn_command_call.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_lasgn_lasgn_command_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/lasgn_lasgn_command_call.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/lasgn_lasgn_command_call.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_lasgn_middle_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/lasgn_middle_splat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/lasgn_middle_splat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_lasgn_middle_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/lasgn_middle_splat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/lasgn_middle_splat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_magic_encoding_comment_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/magic_encoding_comment.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/magic_encoding_comment.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_magic_encoding_comment_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/magic_encoding_comment.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/magic_encoding_comment.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_masgn_anon_splat_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_anon_splat_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/masgn_anon_splat_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_masgn_anon_splat_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_anon_splat_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/masgn_anon_splat_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_masgn_arg_colon_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_arg_colon_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/masgn_arg_colon_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_masgn_arg_colon_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_arg_colon_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/masgn_arg_colon_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_masgn_arg_ident_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_arg_ident.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/masgn_arg_ident.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_masgn_arg_ident_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_arg_ident.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/masgn_arg_ident.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_masgn_arg_splat_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_arg_splat_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/masgn_arg_splat_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_masgn_arg_splat_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_arg_splat_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/masgn_arg_splat_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_masgn_colon2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_colon2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/masgn_colon2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_masgn_colon2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_colon2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/masgn_colon2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_masgn_colon3_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_colon3.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/masgn_colon3.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_masgn_colon3_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_colon3.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/masgn_colon3.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_masgn_command_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_command_call.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/masgn_command_call.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_masgn_command_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_command_call.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/masgn_command_call.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_masgn_double_paren_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_double_paren.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/masgn_double_paren.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_masgn_double_paren_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_double_paren.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/masgn_double_paren.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_masgn_lhs_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_lhs_splat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/masgn_lhs_splat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_masgn_lhs_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_lhs_splat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/masgn_lhs_splat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_masgn_paren_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_paren.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/masgn_paren.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_masgn_paren_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_paren.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/masgn_paren.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_masgn_splat_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_splat_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/masgn_splat_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_masgn_splat_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_splat_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/masgn_splat_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_masgn_splat_arg_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_splat_arg_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/masgn_splat_arg_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_masgn_splat_arg_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_splat_arg_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/masgn_splat_arg_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_masgn_star_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_star.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/masgn_star.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_masgn_star_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_star.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/masgn_star.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_masgn_var_star_var_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_var_star_var.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/masgn_var_star_var.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_masgn_var_star_var_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/masgn_var_star_var.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/masgn_var_star_var.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_messy_op_asgn_lineno_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/messy_op_asgn_lineno.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/messy_op_asgn_lineno.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_messy_op_asgn_lineno_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/messy_op_asgn_lineno.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/messy_op_asgn_lineno.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_method_call_assoc_trailing_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/method_call_assoc_trailing_comma.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/method_call_assoc_trailing_comma.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_method_call_assoc_trailing_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/method_call_assoc_trailing_comma.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/method_call_assoc_trailing_comma.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_method_call_trailing_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/method_call_trailing_comma.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/method_call_trailing_comma.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_method_call_trailing_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/method_call_trailing_comma.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/method_call_trailing_comma.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_mlhs_back_anonsplat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/mlhs_back_anonsplat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/mlhs_back_anonsplat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_mlhs_back_anonsplat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/mlhs_back_anonsplat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/mlhs_back_anonsplat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_mlhs_back_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/mlhs_back_splat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/mlhs_back_splat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_mlhs_back_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/mlhs_back_splat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/mlhs_back_splat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_mlhs_front_anonsplat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/mlhs_front_anonsplat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/mlhs_front_anonsplat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_mlhs_front_anonsplat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/mlhs_front_anonsplat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/mlhs_front_anonsplat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_mlhs_front_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/mlhs_front_splat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/mlhs_front_splat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_mlhs_front_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/mlhs_front_splat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/mlhs_front_splat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_mlhs_keyword_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/mlhs_keyword.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/mlhs_keyword.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_mlhs_keyword_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/mlhs_keyword.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/mlhs_keyword.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_mlhs_mid_anonsplat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/mlhs_mid_anonsplat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/mlhs_mid_anonsplat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_mlhs_mid_anonsplat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/mlhs_mid_anonsplat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/mlhs_mid_anonsplat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_mlhs_mid_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/mlhs_mid_splat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/mlhs_mid_splat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_mlhs_mid_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/mlhs_mid_splat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/mlhs_mid_splat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_mlhs_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/mlhs_rescue.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/mlhs_rescue.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_mlhs_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/mlhs_rescue.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/mlhs_rescue.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_module_comments_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/module_comments.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/module_comments.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_module_comments_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/module_comments.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/module_comments.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_multiline_hash_declaration_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/multiline_hash_declaration.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/multiline_hash_declaration.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_multiline_hash_declaration_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/multiline_hash_declaration.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/multiline_hash_declaration.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_non_interpolated_symbol_array_line_breaks_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/non_interpolated_symbol_array_line_breaks.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/non_interpolated_symbol_array_line_breaks.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_non_interpolated_symbol_array_line_breaks_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/non_interpolated_symbol_array_line_breaks.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/non_interpolated_symbol_array_line_breaks.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_non_interpolated_word_array_line_breaks_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/non_interpolated_word_array_line_breaks.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/non_interpolated_word_array_line_breaks.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_non_interpolated_word_array_line_breaks_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/non_interpolated_word_array_line_breaks.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/non_interpolated_word_array_line_breaks.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_op_asgn_command_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/op_asgn_command_call.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/op_asgn_command_call.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_op_asgn_command_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/op_asgn_command_call.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/op_asgn_command_call.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_op_asgn_dot_ident_command_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/op_asgn_dot_ident_command_call.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/op_asgn_dot_ident_command_call.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_op_asgn_dot_ident_command_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/op_asgn_dot_ident_command_call.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/op_asgn_dot_ident_command_call.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_op_asgn_index_command_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/op_asgn_index_command_call.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/op_asgn_index_command_call.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_op_asgn_index_command_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/op_asgn_index_command_call.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/op_asgn_index_command_call.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_op_asgn_primary_colon_const_command_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/op_asgn_primary_colon_const_command_call.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/op_asgn_primary_colon_const_command_call.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_op_asgn_primary_colon_const_command_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/op_asgn_primary_colon_const_command_call.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/op_asgn_primary_colon_const_command_call.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_op_asgn_primary_colon_identifier1_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/op_asgn_primary_colon_identifier1.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/op_asgn_primary_colon_identifier1.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_op_asgn_primary_colon_identifier1_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/op_asgn_primary_colon_identifier1.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/op_asgn_primary_colon_identifier1.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_op_asgn_primary_colon_identifier_command_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/op_asgn_primary_colon_identifier_command_call.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/op_asgn_primary_colon_identifier_command_call.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_op_asgn_primary_colon_identifier_command_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/op_asgn_primary_colon_identifier_command_call.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/op_asgn_primary_colon_identifier_command_call.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_op_asgn_val_dot_ident_command_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/op_asgn_val_dot_ident_command_call.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/op_asgn_val_dot_ident_command_call.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_op_asgn_val_dot_ident_command_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/op_asgn_val_dot_ident_command_call.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/op_asgn_val_dot_ident_command_call.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_def_special_name_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_def_special_name.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_def_special_name.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_def_special_name_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_def_special_name.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_def_special_name.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_if_not_canonical_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_if_not_canonical.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_if_not_canonical.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_if_not_canonical_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_if_not_canonical.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_if_not_canonical.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_if_not_noncanonical_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_if_not_noncanonical.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_if_not_noncanonical.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_if_not_noncanonical_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_if_not_noncanonical.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_if_not_noncanonical.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_block.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_block.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_block.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_block.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_block_inline_comment_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_block_inline_comment.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_block_inline_comment.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_block_inline_comment_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_block_inline_comment.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_block_inline_comment.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_block_inline_comment_leading_newlines_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_block_inline_comment_leading_newlines.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_block_inline_comment_leading_newlines.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_block_inline_comment_leading_newlines_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_block_inline_comment_leading_newlines.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_block_inline_comment_leading_newlines.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_block_inline_multiline_comment_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_block_inline_multiline_comment.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_block_inline_multiline_comment.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_block_inline_multiline_comment_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_block_inline_multiline_comment.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_block_inline_multiline_comment.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_call_ivar_arg_no_parens_line_break_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_call_ivar_arg_no_parens_line_break.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_call_ivar_arg_no_parens_line_break.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_call_ivar_arg_no_parens_line_break_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_call_ivar_arg_no_parens_line_break.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_call_ivar_arg_no_parens_line_break.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_call_ivar_line_break_paren_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_call_ivar_line_break_paren.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_call_ivar_line_break_paren.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_call_ivar_line_break_paren_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_call_ivar_line_break_paren.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_call_ivar_line_break_paren.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_call_no_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_call_no_args.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_call_no_args.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_call_no_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_call_no_args.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_call_no_args.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_defn_complex_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_defn_complex.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_defn_complex.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_defn_complex_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_defn_complex.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_defn_complex.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_defn_no_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_defn_no_parens.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_defn_no_parens.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_defn_no_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_defn_no_parens.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_defn_no_parens.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_defn_no_parens_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_defn_no_parens_args.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_defn_no_parens_args.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_defn_no_parens_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_defn_no_parens_args.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_defn_no_parens_args.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_dot2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_dot2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_dot2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_dot2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_dot2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_dot2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_dot2_open_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_dot2_open.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_dot2_open.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_dot2_open_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_dot2_open.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_dot2_open.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_dot3_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_dot3.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_dot3.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_dot3_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_dot3.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_dot3.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_dot3_open_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_dot3_open.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_dot3_open.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_dot3_open_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_dot3_open.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_dot3_open.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_dstr_escaped_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_dstr_escaped_newline.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_dstr_escaped_newline.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_dstr_escaped_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_dstr_escaped_newline.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_dstr_escaped_newline.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_dstr_soft_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_dstr_soft_newline.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_dstr_soft_newline.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_dstr_soft_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_dstr_soft_newline.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_dstr_soft_newline.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_evstr_after_break_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_evstr_after_break.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_evstr_after_break.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_evstr_after_break_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_evstr_after_break.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_evstr_after_break.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_hash_lit_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_hash_lit.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_hash_lit.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_hash_lit_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_hash_lit.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_hash_lit.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_heredoc_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_heredoc.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_heredoc.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_heredoc_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_heredoc.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_heredoc.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_heredoc_evstr_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_heredoc_evstr.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_heredoc_evstr.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_heredoc_evstr_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_heredoc_evstr.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_heredoc_evstr.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_heredoc_hardnewline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_heredoc_hardnewline.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_heredoc_hardnewline.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_heredoc_hardnewline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_heredoc_hardnewline.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_heredoc_hardnewline.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_heredoc_regexp_chars_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_heredoc_regexp_chars.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_heredoc_regexp_chars.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_heredoc_regexp_chars_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_heredoc_regexp_chars.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_heredoc_regexp_chars.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_iter_call_no_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_iter_call_no_parens.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_iter_call_no_parens.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_iter_call_no_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_iter_call_no_parens.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_iter_call_no_parens.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_iter_call_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_iter_call_parens.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_iter_call_parens.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_iter_call_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_iter_call_parens.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_iter_call_parens.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_multiline_str_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_multiline_str.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_multiline_str.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_multiline_str_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_multiline_str.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_multiline_str.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_multiline_str_literal_n_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_multiline_str_literal_n.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_multiline_str_literal_n.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_multiline_str_literal_n_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_multiline_str_literal_n.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_multiline_str_literal_n.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_newlines_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_newlines.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_newlines.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_newlines_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_newlines.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_newlines.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_op_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_op_asgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_op_asgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_op_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_op_asgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_op_asgn.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_postexe_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_postexe.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_postexe.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_postexe_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_postexe.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_postexe.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_preexe_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_preexe.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_preexe.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_preexe_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_preexe.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_preexe.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_rescue.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_rescue.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_rescue.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_rescue.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_return_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_return.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_return.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_return_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_return.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_return.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_str_with_newline_escape_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_str_with_newline_escape.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_str_with_newline_escape.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_str_with_newline_escape_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_str_with_newline_escape.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_str_with_newline_escape.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_to_ary_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_to_ary.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_to_ary.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_to_ary_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_to_ary.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_to_ary.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_line_trailing_newlines_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_trailing_newlines.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_line_trailing_newlines.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_line_trailing_newlines_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_line_trailing_newlines.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_line_trailing_newlines.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_opt_call_args_assocs_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_opt_call_args_assocs_comma.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_opt_call_args_assocs_comma.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_opt_call_args_assocs_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_opt_call_args_assocs_comma.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_opt_call_args_assocs_comma.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_opt_call_args_lit_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_opt_call_args_lit_comma.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_opt_call_args_lit_comma.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_opt_call_args_lit_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_opt_call_args_lit_comma.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_opt_call_args_lit_comma.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_pattern_019_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_pattern_019.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_pattern_019.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_pattern_019_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_pattern_019.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_pattern_019.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_pattern_044_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_pattern_044.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_pattern_044.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_pattern_044_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_pattern_044.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_pattern_044.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_pattern_051_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_pattern_051.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_pattern_051.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_pattern_051_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_pattern_051.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_pattern_051.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_pattern_058_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_pattern_058.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_pattern_058.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_pattern_058_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_pattern_058.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_pattern_058.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_pattern_058_2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_pattern_058_2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_pattern_058_2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_pattern_058_2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_pattern_058_2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_pattern_058_2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_pattern_069_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_pattern_069.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_pattern_069.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_pattern_069_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_pattern_069.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_pattern_069.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_pattern_076_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_pattern_076.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_pattern_076.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_pattern_076_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_pattern_076.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_pattern_076.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_until_not_canonical_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_until_not_canonical.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_until_not_canonical.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_until_not_canonical_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_until_not_canonical.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_until_not_canonical.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_until_not_noncanonical_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_until_not_noncanonical.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_until_not_noncanonical.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_until_not_noncanonical_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_until_not_noncanonical.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_until_not_noncanonical.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_while_not_canonical_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_while_not_canonical.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_while_not_canonical.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_while_not_canonical_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_while_not_canonical.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_while_not_canonical.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_parse_while_not_noncanonical_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_while_not_noncanonical.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/parse_while_not_noncanonical.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_parse_while_not_noncanonical_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/parse_while_not_noncanonical.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/parse_while_not_noncanonical.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_pctW_lineno_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/pctW_lineno.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/pctW_lineno.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_pctW_lineno_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/pctW_lineno.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/pctW_lineno.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_pct_Q_backslash_nl_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/pct_Q_backslash_nl.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/pct_Q_backslash_nl.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_pct_Q_backslash_nl_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/pct_Q_backslash_nl.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/pct_Q_backslash_nl.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_pct_nl_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/pct_nl.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/pct_nl.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_pct_nl_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/pct_nl.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/pct_nl.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_pct_w_heredoc_interp_nested_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/pct_w_heredoc_interp_nested.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/pct_w_heredoc_interp_nested.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_pct_w_heredoc_interp_nested_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/pct_w_heredoc_interp_nested.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/pct_w_heredoc_interp_nested.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_pipe_semicolon_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/pipe_semicolon.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/pipe_semicolon.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_pipe_semicolon_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/pipe_semicolon.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/pipe_semicolon.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_pipe_space_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/pipe_space.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/pipe_space.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_pipe_space_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/pipe_space.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/pipe_space.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_qWords_space_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/qWords_space.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/qWords_space.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_qWords_space_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/qWords_space.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/qWords_space.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_qsymbols_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/qsymbols.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/qsymbols.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_qsymbols_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/qsymbols.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/qsymbols.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_qsymbols_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/qsymbols_empty.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/qsymbols_empty.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_qsymbols_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/qsymbols_empty.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/qsymbols_empty.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_qsymbols_empty_space_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/qsymbols_empty_space.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/qsymbols_empty_space.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_qsymbols_empty_space_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/qsymbols_empty_space.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/qsymbols_empty_space.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_qsymbols_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/qsymbols_interp.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/qsymbols_interp.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_qsymbols_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/qsymbols_interp.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/qsymbols_interp.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_quoted_symbol_hash_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/quoted_symbol_hash_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/quoted_symbol_hash_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_quoted_symbol_hash_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/quoted_symbol_hash_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/quoted_symbol_hash_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_quoted_symbol_keys_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/quoted_symbol_keys.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/quoted_symbol_keys.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_quoted_symbol_keys_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/quoted_symbol_keys.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/quoted_symbol_keys.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_qw_escape_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/qw_escape.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/qw_escape.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_qw_escape_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/qw_escape.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/qw_escape.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_qw_escape_term_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/qw_escape_term.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/qw_escape_term.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_qw_escape_term_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/qw_escape_term.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/qw_escape_term.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_qwords_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/qwords_empty.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/qwords_empty.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_qwords_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/qwords_empty.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/qwords_empty.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_read_escape_unicode_curlies_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/read_escape_unicode_curlies.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/read_escape_unicode_curlies.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_read_escape_unicode_curlies_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/read_escape_unicode_curlies.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/read_escape_unicode_curlies.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_read_escape_unicode_h4_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/read_escape_unicode_h4.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/read_escape_unicode_h4.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_read_escape_unicode_h4_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/read_escape_unicode_h4.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/read_escape_unicode_h4.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_regexp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/regexp.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/regexp.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_regexp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/regexp.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/regexp.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_regexp_esc_C_slash_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/regexp_esc_C_slash.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/regexp_esc_C_slash.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_regexp_esc_C_slash_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/regexp_esc_C_slash.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/regexp_esc_C_slash.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_regexp_esc_u_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/regexp_esc_u.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/regexp_esc_u.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_regexp_esc_u_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/regexp_esc_u.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/regexp_esc_u.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_regexp_escape_extended_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/regexp_escape_extended.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/regexp_escape_extended.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_regexp_escape_extended_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/regexp_escape_extended.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/regexp_escape_extended.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_regexp_unicode_curlies_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/regexp_unicode_curlies.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/regexp_unicode_curlies.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_regexp_unicode_curlies_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/regexp_unicode_curlies.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/regexp_unicode_curlies.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_required_kwarg_no_value_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/required_kwarg_no_value.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/required_kwarg_no_value.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_required_kwarg_no_value_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/required_kwarg_no_value.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/required_kwarg_no_value.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_rescue_do_end_ensure_result_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/rescue_do_end_ensure_result.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/rescue_do_end_ensure_result.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_rescue_do_end_ensure_result_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/rescue_do_end_ensure_result.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/rescue_do_end_ensure_result.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_rescue_do_end_no_raise_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/rescue_do_end_no_raise.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/rescue_do_end_no_raise.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_rescue_do_end_no_raise_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/rescue_do_end_no_raise.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/rescue_do_end_no_raise.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_rescue_do_end_raised_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/rescue_do_end_raised.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/rescue_do_end_raised.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_rescue_do_end_raised_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/rescue_do_end_raised.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/rescue_do_end_raised.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_rescue_do_end_rescued_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/rescue_do_end_rescued.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/rescue_do_end_rescued.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_rescue_do_end_rescued_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/rescue_do_end_rescued.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/rescue_do_end_rescued.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_rescue_in_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/rescue_in_block.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/rescue_in_block.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_rescue_in_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/rescue_in_block.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/rescue_in_block.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_rescue_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/rescue_parens.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/rescue_parens.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_rescue_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/rescue_parens.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/rescue_parens.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_return_call_assocs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/return_call_assocs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/return_call_assocs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_return_call_assocs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/return_call_assocs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/return_call_assocs.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_rhs_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/rhs_asgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/rhs_asgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_rhs_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/rhs_asgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/rhs_asgn.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_ruby21_numbers_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/ruby21_numbers.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/ruby21_numbers.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_ruby21_numbers_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/ruby21_numbers.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/ruby21_numbers.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_safe_attrasgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_attrasgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/safe_attrasgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_safe_attrasgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_attrasgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/safe_attrasgn.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_safe_attrasgn_constant_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_attrasgn_constant.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/safe_attrasgn_constant.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_safe_attrasgn_constant_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_attrasgn_constant.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/safe_attrasgn_constant.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_safe_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_call.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/safe_call.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_safe_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_call.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/safe_call.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_safe_call_after_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_call_after_newline.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/safe_call_after_newline.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_safe_call_after_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_call_after_newline.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/safe_call_after_newline.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_safe_call_dot_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_call_dot_parens.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/safe_call_dot_parens.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_safe_call_dot_parens_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_call_dot_parens.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/safe_call_dot_parens.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_safe_call_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_call_newline.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/safe_call_newline.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_safe_call_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_call_newline.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/safe_call_newline.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_safe_call_operator_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_call_operator.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/safe_call_operator.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_safe_call_operator_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_call_operator.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/safe_call_operator.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_safe_call_rhs_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_call_rhs_newline.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/safe_call_rhs_newline.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_safe_call_rhs_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_call_rhs_newline.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/safe_call_rhs_newline.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_safe_calls_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_calls.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/safe_calls.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_safe_calls_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_calls.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/safe_calls.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_safe_op_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_op_asgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/safe_op_asgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_safe_op_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_op_asgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/safe_op_asgn.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_safe_op_asgn2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_op_asgn2.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/safe_op_asgn2.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_safe_op_asgn2_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/safe_op_asgn2.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/safe_op_asgn2.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_slashy_newlines_within_string_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/slashy_newlines_within_string.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/slashy_newlines_within_string.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_slashy_newlines_within_string_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/slashy_newlines_within_string.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/slashy_newlines_within_string.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_stabby_arg_no_paren_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/stabby_arg_no_paren.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/stabby_arg_no_paren.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_stabby_arg_no_paren_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/stabby_arg_no_paren.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/stabby_arg_no_paren.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_stabby_arg_opt_splat_arg_block_omfg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/stabby_arg_opt_splat_arg_block_omfg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/stabby_arg_opt_splat_arg_block_omfg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_stabby_arg_opt_splat_arg_block_omfg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/stabby_arg_opt_splat_arg_block_omfg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/stabby_arg_opt_splat_arg_block_omfg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_stabby_block_iter_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/stabby_block_iter_call.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/stabby_block_iter_call.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_stabby_block_iter_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/stabby_block_iter_call.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/stabby_block_iter_call.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_stabby_block_iter_call_no_target_with_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/stabby_block_iter_call_no_target_with_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/stabby_block_iter_call_no_target_with_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_stabby_block_iter_call_no_target_with_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/stabby_block_iter_call_no_target_with_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/stabby_block_iter_call_no_target_with_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_stabby_block_kw_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/stabby_block_kw.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/stabby_block_kw.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_stabby_block_kw_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/stabby_block_kw.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/stabby_block_kw.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_stabby_block_kw__required_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/stabby_block_kw__required.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/stabby_block_kw__required.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_stabby_block_kw__required_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/stabby_block_kw__required.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/stabby_block_kw__required.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_stabby_proc_scope_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/stabby_proc_scope.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/stabby_proc_scope.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_stabby_proc_scope_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/stabby_proc_scope.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/stabby_proc_scope.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_str_backslashes_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_backslashes.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/str_backslashes.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_str_backslashes_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_backslashes.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/str_backslashes.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_str_double_double_escaped_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_double_double_escaped_newline.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/str_double_double_escaped_newline.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_str_double_double_escaped_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_double_double_escaped_newline.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/str_double_double_escaped_newline.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_str_double_escaped_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_double_escaped_newline.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/str_double_escaped_newline.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_str_double_escaped_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_double_escaped_newline.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/str_double_escaped_newline.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_str_double_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_double_newline.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/str_double_newline.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_str_double_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_double_newline.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/str_double_newline.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_str_evstr_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_evstr.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/str_evstr.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_str_evstr_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_evstr.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/str_evstr.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_str_evstr_escape_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_evstr_escape.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/str_evstr_escape.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_str_evstr_escape_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_evstr_escape.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/str_evstr_escape.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_str_heredoc_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_heredoc_interp.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/str_heredoc_interp.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_str_heredoc_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_heredoc_interp.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/str_heredoc_interp.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_str_interp_ternary_or_label_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_interp_ternary_or_label.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/str_interp_ternary_or_label.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_str_interp_ternary_or_label_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_interp_ternary_or_label.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/str_interp_ternary_or_label.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_str_lit_concat_bad_encodings_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_lit_concat_bad_encodings.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/str_lit_concat_bad_encodings.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_str_lit_concat_bad_encodings_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_lit_concat_bad_encodings.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/str_lit_concat_bad_encodings.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_str_newline_hash_line_number_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_newline_hash_line_number.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/str_newline_hash_line_number.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_str_newline_hash_line_number_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_newline_hash_line_number.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/str_newline_hash_line_number.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_str_pct_Q_nested_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_pct_Q_nested.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/str_pct_Q_nested.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_str_pct_Q_nested_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_pct_Q_nested.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/str_pct_Q_nested.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_str_pct_nested_nested_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_pct_nested_nested.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/str_pct_nested_nested.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_str_pct_nested_nested_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_pct_nested_nested.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/str_pct_nested_nested.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_str_pct_q_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_pct_q.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/str_pct_q.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_str_pct_q_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_pct_q.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/str_pct_q.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_str_single_double_escaped_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_single_double_escaped_newline.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/str_single_double_escaped_newline.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_str_single_double_escaped_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_single_double_escaped_newline.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/str_single_double_escaped_newline.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_str_single_escaped_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_single_escaped_newline.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/str_single_escaped_newline.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_str_single_escaped_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_single_escaped_newline.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/str_single_escaped_newline.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_str_single_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_single_newline.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/str_single_newline.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_str_single_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_single_newline.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/str_single_newline.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_str_str_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_str.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/str_str.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_str_str_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_str.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/str_str.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_str_str_str_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_str_str.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/str_str_str.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_str_str_str_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/str_str_str.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/str_str_str.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_super_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/super_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/super_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_super_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/super_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/super_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_symbol_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/symbol_empty.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/symbol_empty.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_symbol_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/symbol_empty.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/symbol_empty.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_symbol_list_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/symbol_list.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/symbol_list.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_symbol_list_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/symbol_list.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/symbol_list.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_symbols_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/symbols.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/symbols.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_symbols_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/symbols.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/symbols.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_symbols_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/symbols_empty.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/symbols_empty.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_symbols_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/symbols_empty.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/symbols_empty.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_symbols_empty_space_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/symbols_empty_space.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/symbols_empty_space.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_symbols_empty_space_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/symbols_empty_space.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/symbols_empty_space.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_symbols_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/symbols_interp.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/symbols_interp.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_symbols_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/symbols_interp.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/symbols_interp.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_thingy_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/thingy.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/thingy.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_thingy_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/thingy.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/thingy.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_uminus_float_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/uminus_float.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/uminus_float.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_uminus_float_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/uminus_float.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/uminus_float.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_unary_minus_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/unary_minus.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/unary_minus.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_unary_minus_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/unary_minus.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/unary_minus.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_unary_plus_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/unary_plus.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/unary_plus.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_unary_plus_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/unary_plus.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/unary_plus.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_unary_plus_on_literal_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/unary_plus_on_literal.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/unary_plus_on_literal.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_unary_plus_on_literal_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/unary_plus_on_literal.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/unary_plus_on_literal.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_unary_tilde_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/unary_tilde.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/unary_tilde.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_unary_tilde_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/unary_tilde.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/unary_tilde.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_utf8_bom_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/utf8_bom.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/utf8_bom.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_utf8_bom_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/utf8_bom.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/utf8_bom.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_when_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/when_splat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/when_splat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_when_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/when_splat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/when_splat.txt"].assert_debug_eq(&program);
}

fn test_ast_seattlerb_words_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/words_interp.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/seattlerb/words_interp.txt").trim(), program.snapshot());
}
#[test]
fn test_program_seattlerb_words_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/seattlerb/words_interp.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/seattlerb/words_interp.txt"].assert_debug_eq(&program);
}

fn test_ast_single_method_call_with_bang_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/single_method_call_with_bang.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/single_method_call_with_bang.txt").trim(), program.snapshot());
}
#[test]
fn test_program_single_method_call_with_bang_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/single_method_call_with_bang.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/single_method_call_with_bang.txt"].assert_debug_eq(&program);
}

fn test_ast_single_quote_heredocs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/single_quote_heredocs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/single_quote_heredocs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_single_quote_heredocs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/single_quote_heredocs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/single_quote_heredocs.txt"].assert_debug_eq(&program);
}

fn test_ast_spanning_heredoc_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/spanning_heredoc.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/spanning_heredoc.txt").trim(), program.snapshot());
}
#[test]
fn test_program_spanning_heredoc_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/spanning_heredoc.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/spanning_heredoc.txt"].assert_debug_eq(&program);
}

fn test_ast_spanning_heredoc_newlines_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/spanning_heredoc_newlines.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/spanning_heredoc_newlines.txt").trim(), program.snapshot());
}
#[test]
fn test_program_spanning_heredoc_newlines_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/spanning_heredoc_newlines.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/spanning_heredoc_newlines.txt"].assert_debug_eq(&program);
}

fn test_ast_strings_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/strings.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/strings.txt").trim(), program.snapshot());
}
#[test]
fn test_program_strings_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/strings.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/strings.txt"].assert_debug_eq(&program);
}

fn test_ast_super_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/super.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/super.txt").trim(), program.snapshot());
}
#[test]
fn test_program_super_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/super.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/super.txt"].assert_debug_eq(&program);
}

fn test_ast_symbols_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/symbols.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/symbols.txt").trim(), program.snapshot());
}
#[test]
fn test_program_symbols_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/symbols.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/symbols.txt"].assert_debug_eq(&program);
}

fn test_ast_ternary_operator_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/ternary_operator.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/ternary_operator.txt").trim(), program.snapshot());
}
#[test]
fn test_program_ternary_operator_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/ternary_operator.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/ternary_operator.txt"].assert_debug_eq(&program);
}

fn test_ast_tilde_heredocs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/tilde_heredocs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/tilde_heredocs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_tilde_heredocs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/tilde_heredocs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/tilde_heredocs.txt"].assert_debug_eq(&program);
}

fn test_ast_undef_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/undef.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/undef.txt").trim(), program.snapshot());
}
#[test]
fn test_program_undef_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/undef.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/undef.txt"].assert_debug_eq(&program);
}

fn test_ast_unescaping_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unescaping.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unescaping.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unescaping_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unescaping.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unescaping.txt"].assert_debug_eq(&program);
}

fn test_ast_unless_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unless.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unless.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unless_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unless.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unless.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_alias_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/alias.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/alias.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_alias_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/alias.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/alias.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_assignment_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/assignment.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/assignment.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_assignment_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/assignment.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/assignment.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/block.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/block.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/block.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/block.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_case_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/case.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/case.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_case_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/case.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/case.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_class_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/class.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/class.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_class_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/class.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/class.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_def_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/def.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/def.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_def_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/def.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/def.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_defined_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/defined.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/defined.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_defined_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/defined.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/defined.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_defs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/defs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/defs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_defs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/defs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/defs.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_dstr_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/dstr.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/dstr.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_dstr_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/dstr.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/dstr.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/empty.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/empty.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/empty.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/empty.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_empty_begin_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/empty_begin.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/empty_begin.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_empty_begin_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/empty_begin.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/empty_begin.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_flipflop_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/flipflop.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/flipflop.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_flipflop_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/flipflop.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/flipflop.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_for_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/for.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/for.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_for_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/for.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/for.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_hookexe_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/hookexe.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/hookexe.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_hookexe_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/hookexe.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/hookexe.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_if_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/if.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/if.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_if_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/if.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/if.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_kwbegin_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/kwbegin.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/kwbegin.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_kwbegin_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/kwbegin.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/kwbegin.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_lambda_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/lambda.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/lambda.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_lambda_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/lambda.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/lambda.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_literal_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/literal.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/literal.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_literal_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/literal.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/literal.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_module_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/module.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/module.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_module_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/module.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/module.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_opasgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/opasgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/opasgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_opasgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/opasgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/opasgn.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_pattern_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/pattern.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/pattern.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_pattern_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/pattern.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/pattern.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_pragma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/pragma.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/pragma.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_pragma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/pragma.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/pragma.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_range_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/range.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/range.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_range_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/range.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/range.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/rescue.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/rescue.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/rescue.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/rescue.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_send_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/send.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/send.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_send_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/send.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/send.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_since_27_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/since/27.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/since/27.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_since_27_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/since/27.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/since/27.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_since_30_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/since/30.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/since/30.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_since_30_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/since/30.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/since/30.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_since_31_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/since/31.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/since/31.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_since_31_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/since/31.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/since/31.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_since_32_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/since/32.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/since/32.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_since_32_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/since/32.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/since/32.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_singletons_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/singletons.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/singletons.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_singletons_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/singletons.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/singletons.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_super_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/super.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/super.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_super_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/super.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/super.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_unary_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/unary.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/unary.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_unary_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/unary.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/unary.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_undef_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/undef.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/undef.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_undef_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/undef.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/undef.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_variables_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/variables.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/variables.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_variables_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/variables.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/variables.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_literal_while_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/while.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/literal/while.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_literal_while_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/literal/while.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/literal/while.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_semantic_and_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/semantic/and.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/semantic/and.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_semantic_and_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/semantic/and.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/semantic/and.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_semantic_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/semantic/block.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/semantic/block.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_semantic_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/semantic/block.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/semantic/block.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_semantic_def_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/semantic/def.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/semantic/def.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_semantic_def_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/semantic/def.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/semantic/def.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_semantic_dstr_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/semantic/dstr.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/semantic/dstr.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_semantic_dstr_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/semantic/dstr.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/semantic/dstr.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_semantic_kwbegin_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/semantic/kwbegin.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/semantic/kwbegin.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_semantic_kwbegin_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/semantic/kwbegin.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/semantic/kwbegin.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_semantic_literal_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/semantic/literal.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/semantic/literal.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_semantic_literal_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/semantic/literal.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/semantic/literal.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_semantic_opasgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/semantic/opasgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/semantic/opasgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_semantic_opasgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/semantic/opasgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/semantic/opasgn.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_semantic_send_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/semantic/send.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/semantic/send.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_semantic_send_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/semantic/send.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/semantic/send.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_semantic_undef_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/semantic/undef.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/semantic/undef.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_semantic_undef_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/semantic/undef.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/semantic/undef.txt"].assert_debug_eq(&program);
}

fn test_ast_unparser_corpus_semantic_while_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/semantic/while.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/unparser/corpus/semantic/while.txt").trim(), program.snapshot());
}
#[test]
fn test_program_unparser_corpus_semantic_while_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/unparser/corpus/semantic/while.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/unparser/corpus/semantic/while.txt"].assert_debug_eq(&program);
}

fn test_ast_until_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/until.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/until.txt").trim(), program.snapshot());
}
#[test]
fn test_program_until_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/until.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/until.txt"].assert_debug_eq(&program);
}

fn test_ast_variables_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/variables.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/variables.txt").trim(), program.snapshot());
}
#[test]
fn test_program_variables_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/variables.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/variables.txt"].assert_debug_eq(&program);
}

fn test_ast_while_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/while.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/while.txt").trim(), program.snapshot());
}
#[test]
fn test_program_while_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/while.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/while.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark___ENCODING___txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/__ENCODING__.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/__ENCODING__.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark___ENCODING___txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/__ENCODING__.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/__ENCODING__.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark___ENCODING___legacy__txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/__ENCODING___legacy_.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/__ENCODING___legacy_.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark___ENCODING___legacy__txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/__ENCODING___legacy_.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/__ENCODING___legacy_.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_alias_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/alias.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/alias.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_alias_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/alias.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/alias.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_alias_gvar_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/alias_gvar.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/alias_gvar.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_alias_gvar_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/alias_gvar.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/alias_gvar.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ambiuous_quoted_label_in_ternary_operator_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ambiuous_quoted_label_in_ternary_operator.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ambiuous_quoted_label_in_ternary_operator.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ambiuous_quoted_label_in_ternary_operator_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ambiuous_quoted_label_in_ternary_operator.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ambiuous_quoted_label_in_ternary_operator.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_and_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/and.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/and.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_and_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/and.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/and.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_and_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/and_asgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/and_asgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_and_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/and_asgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/and_asgn.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_and_or_masgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/and_or_masgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/and_or_masgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_and_or_masgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/and_or_masgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/and_or_masgn.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_anonymous_blockarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/anonymous_blockarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/anonymous_blockarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_anonymous_blockarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/anonymous_blockarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/anonymous_blockarg.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/arg.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_arg_combinations_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/arg_combinations.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/arg_combinations.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_arg_combinations_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/arg_combinations.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/arg_combinations.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_arg_duplicate_ignored_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/arg_duplicate_ignored.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/arg_duplicate_ignored.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_arg_duplicate_ignored_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/arg_duplicate_ignored.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/arg_duplicate_ignored.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_arg_label_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/arg_label.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/arg_label.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_arg_label_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/arg_label.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/arg_label.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_arg_scope_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/arg_scope.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/arg_scope.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_arg_scope_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/arg_scope.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/arg_scope.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/args.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/args.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/args.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/args.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_args_args_assocs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/args_args_assocs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/args_args_assocs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_args_args_assocs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/args_args_assocs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/args_args_assocs.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_args_args_assocs_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/args_args_assocs_comma.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/args_args_assocs_comma.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_args_args_assocs_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/args_args_assocs_comma.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/args_args_assocs_comma.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_args_args_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/args_args_comma.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/args_args_comma.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_args_args_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/args_args_comma.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/args_args_comma.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_args_args_star_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/args_args_star.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/args_args_star.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_args_args_star_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/args_args_star.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/args_args_star.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_args_assocs_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/args_assocs_comma.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/args_assocs_comma.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_args_assocs_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/args_assocs_comma.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/args_assocs_comma.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_args_block_pass_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/args_block_pass.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/args_block_pass.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_args_block_pass_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/args_block_pass.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/args_block_pass.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_args_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/args_cmd.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/args_cmd.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_args_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/args_cmd.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/args_cmd.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_args_star_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/args_star.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/args_star.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_args_star_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/args_star.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/args_star.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_array_assocs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/array_assocs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/array_assocs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_array_assocs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/array_assocs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/array_assocs.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_array_plain_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/array_plain.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/array_plain.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_array_plain_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/array_plain.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/array_plain.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_array_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/array_splat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/array_splat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_array_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/array_splat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/array_splat.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_array_symbols_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/array_symbols.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/array_symbols.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_array_symbols_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/array_symbols.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/array_symbols.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_array_symbols_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/array_symbols_empty.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/array_symbols_empty.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_array_symbols_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/array_symbols_empty.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/array_symbols_empty.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_array_symbols_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/array_symbols_interp.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/array_symbols_interp.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_array_symbols_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/array_symbols_interp.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/array_symbols_interp.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_array_words_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/array_words.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/array_words.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_array_words_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/array_words.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/array_words.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_array_words_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/array_words_empty.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/array_words_empty.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_array_words_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/array_words_empty.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/array_words_empty.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_array_words_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/array_words_interp.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/array_words_interp.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_array_words_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/array_words_interp.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/array_words_interp.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_asgn_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/asgn_cmd.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/asgn_cmd.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_asgn_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/asgn_cmd.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/asgn_cmd.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_asgn_mrhs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/asgn_mrhs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/asgn_mrhs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_asgn_mrhs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/asgn_mrhs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/asgn_mrhs.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_back_ref_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/back_ref.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/back_ref.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_back_ref_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/back_ref.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/back_ref.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bang_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bang.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bang.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bang_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bang.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bang.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bang_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bang_cmd.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bang_cmd.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bang_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bang_cmd.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bang_cmd.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_begin_cmdarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/begin_cmdarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/begin_cmdarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_begin_cmdarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/begin_cmdarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/begin_cmdarg.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_beginless_erange_after_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/beginless_erange_after_newline.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/beginless_erange_after_newline.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_beginless_erange_after_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/beginless_erange_after_newline.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/beginless_erange_after_newline.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_beginless_irange_after_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/beginless_irange_after_newline.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/beginless_irange_after_newline.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_beginless_irange_after_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/beginless_irange_after_newline.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/beginless_irange_after_newline.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_beginless_range_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/beginless_range.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/beginless_range.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_beginless_range_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/beginless_range.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/beginless_range.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_block_arg_combinations_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/block_arg_combinations.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/block_arg_combinations.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_block_arg_combinations_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/block_arg_combinations.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/block_arg_combinations.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_block_kwarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/block_kwarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/block_kwarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_block_kwarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/block_kwarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/block_kwarg.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_block_kwarg_combinations_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/block_kwarg_combinations.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/block_kwarg_combinations.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_block_kwarg_combinations_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/block_kwarg_combinations.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/block_kwarg_combinations.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_blockarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/blockarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/blockarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_blockarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/blockarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/blockarg.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_blockargs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/blockargs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/blockargs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_blockargs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/blockargs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/blockargs.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bug_435_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_435.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bug_435.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bug_435_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_435.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bug_435.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bug_447_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_447.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bug_447.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bug_447_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_447.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bug_447.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bug_452_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_452.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bug_452.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bug_452_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_452.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bug_452.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bug_466_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_466.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bug_466.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bug_466_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_466.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bug_466.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bug_473_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_473.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bug_473.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bug_473_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_473.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bug_473.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bug_480_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_480.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bug_480.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bug_480_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_480.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bug_480.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bug_481_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_481.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bug_481.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bug_481_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_481.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bug_481.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bug_ascii_8bit_in_literal_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_ascii_8bit_in_literal.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bug_ascii_8bit_in_literal.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bug_ascii_8bit_in_literal_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_ascii_8bit_in_literal.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bug_ascii_8bit_in_literal.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bug_cmd_string_lookahead_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_cmd_string_lookahead.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bug_cmd_string_lookahead.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bug_cmd_string_lookahead_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_cmd_string_lookahead.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bug_cmd_string_lookahead.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bug_cmdarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_cmdarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bug_cmdarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bug_cmdarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_cmdarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bug_cmdarg.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bug_def_no_paren_eql_begin_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_def_no_paren_eql_begin.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bug_def_no_paren_eql_begin.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bug_def_no_paren_eql_begin_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_def_no_paren_eql_begin.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bug_def_no_paren_eql_begin.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bug_do_block_in_call_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_do_block_in_call_args.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bug_do_block_in_call_args.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bug_do_block_in_call_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_do_block_in_call_args.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bug_do_block_in_call_args.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bug_do_block_in_cmdarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_do_block_in_cmdarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bug_do_block_in_cmdarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bug_do_block_in_cmdarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_do_block_in_cmdarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bug_do_block_in_cmdarg.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bug_do_block_in_hash_brace_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_do_block_in_hash_brace.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bug_do_block_in_hash_brace.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bug_do_block_in_hash_brace_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_do_block_in_hash_brace.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bug_do_block_in_hash_brace.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bug_heredoc_do_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_heredoc_do.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bug_heredoc_do.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bug_heredoc_do_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_heredoc_do.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bug_heredoc_do.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bug_interp_single_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_interp_single.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bug_interp_single.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bug_interp_single_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_interp_single.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bug_interp_single.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bug_lambda_leakage_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_lambda_leakage.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bug_lambda_leakage.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bug_lambda_leakage_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_lambda_leakage.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bug_lambda_leakage.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bug_regex_verification_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_regex_verification.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bug_regex_verification.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bug_regex_verification_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_regex_verification.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bug_regex_verification.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bug_rescue_empty_else_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_rescue_empty_else.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bug_rescue_empty_else.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bug_rescue_empty_else_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_rescue_empty_else.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bug_rescue_empty_else.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_bug_while_not_parens_do_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_while_not_parens_do.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/bug_while_not_parens_do.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_bug_while_not_parens_do_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/bug_while_not_parens_do.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/bug_while_not_parens_do.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_case_cond_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/case_cond.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/case_cond.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_case_cond_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/case_cond.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/case_cond.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_case_cond_else_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/case_cond_else.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/case_cond_else.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_case_cond_else_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/case_cond_else.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/case_cond_else.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_case_expr_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/case_expr.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/case_expr.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_case_expr_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/case_expr.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/case_expr.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_case_expr_else_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/case_expr_else.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/case_expr_else.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_case_expr_else_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/case_expr_else.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/case_expr_else.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_casgn_scoped_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/casgn_scoped.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/casgn_scoped.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_casgn_scoped_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/casgn_scoped.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/casgn_scoped.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_casgn_toplevel_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/casgn_toplevel.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/casgn_toplevel.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_casgn_toplevel_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/casgn_toplevel.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/casgn_toplevel.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_casgn_unscoped_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/casgn_unscoped.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/casgn_unscoped.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_casgn_unscoped_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/casgn_unscoped.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/casgn_unscoped.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_character_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/character.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/character.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_character_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/character.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/character.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_class_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/class.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/class.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_class_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/class.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/class.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_class_super_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/class_super.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/class_super.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_class_super_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/class_super.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/class_super.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_class_super_label_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/class_super_label.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/class_super_label.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_class_super_label_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/class_super_label.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/class_super_label.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_comments_before_leading_dot__27_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/comments_before_leading_dot__27.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/comments_before_leading_dot__27.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_comments_before_leading_dot__27_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/comments_before_leading_dot__27.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/comments_before_leading_dot__27.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_complex_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/complex.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/complex.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_complex_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/complex.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/complex.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_cond_begin_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cond_begin.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/cond_begin.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_cond_begin_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cond_begin.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/cond_begin.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_cond_begin_masgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cond_begin_masgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/cond_begin_masgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_cond_begin_masgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cond_begin_masgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/cond_begin_masgn.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_cond_eflipflop_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cond_eflipflop.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/cond_eflipflop.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_cond_eflipflop_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cond_eflipflop.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/cond_eflipflop.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_cond_eflipflop_with_beginless_range_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cond_eflipflop_with_beginless_range.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/cond_eflipflop_with_beginless_range.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_cond_eflipflop_with_beginless_range_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cond_eflipflop_with_beginless_range.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/cond_eflipflop_with_beginless_range.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_cond_eflipflop_with_endless_range_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cond_eflipflop_with_endless_range.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/cond_eflipflop_with_endless_range.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_cond_eflipflop_with_endless_range_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cond_eflipflop_with_endless_range.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/cond_eflipflop_with_endless_range.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_cond_iflipflop_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cond_iflipflop.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/cond_iflipflop.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_cond_iflipflop_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cond_iflipflop.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/cond_iflipflop.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_cond_iflipflop_with_beginless_range_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cond_iflipflop_with_beginless_range.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/cond_iflipflop_with_beginless_range.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_cond_iflipflop_with_beginless_range_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cond_iflipflop_with_beginless_range.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/cond_iflipflop_with_beginless_range.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_cond_iflipflop_with_endless_range_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cond_iflipflop_with_endless_range.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/cond_iflipflop_with_endless_range.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_cond_iflipflop_with_endless_range_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cond_iflipflop_with_endless_range.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/cond_iflipflop_with_endless_range.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_cond_match_current_line_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cond_match_current_line.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/cond_match_current_line.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_cond_match_current_line_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cond_match_current_line.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/cond_match_current_line.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_const_op_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/const_op_asgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/const_op_asgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_const_op_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/const_op_asgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/const_op_asgn.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_const_scoped_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/const_scoped.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/const_scoped.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_const_scoped_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/const_scoped.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/const_scoped.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_const_toplevel_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/const_toplevel.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/const_toplevel.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_const_toplevel_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/const_toplevel.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/const_toplevel.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_const_unscoped_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/const_unscoped.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/const_unscoped.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_const_unscoped_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/const_unscoped.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/const_unscoped.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_cpath_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cpath.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/cpath.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_cpath_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cpath.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/cpath.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_cvar_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cvar.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/cvar.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_cvar_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cvar.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/cvar.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_cvasgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cvasgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/cvasgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_cvasgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/cvasgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/cvasgn.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_dedenting_heredoc_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/dedenting_heredoc.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/dedenting_heredoc.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_dedenting_heredoc_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/dedenting_heredoc.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/dedenting_heredoc.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_dedenting_interpolating_heredoc_fake_line_continuation_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/dedenting_interpolating_heredoc_fake_line_continuation.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/dedenting_interpolating_heredoc_fake_line_continuation.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_dedenting_interpolating_heredoc_fake_line_continuation_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/dedenting_interpolating_heredoc_fake_line_continuation.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/dedenting_interpolating_heredoc_fake_line_continuation.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_dedenting_non_interpolating_heredoc_line_continuation_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/dedenting_non_interpolating_heredoc_line_continuation.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/dedenting_non_interpolating_heredoc_line_continuation.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_dedenting_non_interpolating_heredoc_line_continuation_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/dedenting_non_interpolating_heredoc_line_continuation.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/dedenting_non_interpolating_heredoc_line_continuation.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_def_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/def.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/def.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_def_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/def.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/def.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_defined_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/defined.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/defined.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_defined_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/defined.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/defined.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_defs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/defs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/defs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_defs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/defs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/defs.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_emit_arg_inside_procarg0_legacy_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/emit_arg_inside_procarg0_legacy.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/emit_arg_inside_procarg0_legacy.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_emit_arg_inside_procarg0_legacy_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/emit_arg_inside_procarg0_legacy.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/emit_arg_inside_procarg0_legacy.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_empty_stmt_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/empty_stmt.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/empty_stmt.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_empty_stmt_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/empty_stmt.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/empty_stmt.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_endless_comparison_method_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/endless_comparison_method.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/endless_comparison_method.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_endless_comparison_method_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/endless_comparison_method.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/endless_comparison_method.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_endless_method_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/endless_method.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/endless_method.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_endless_method_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/endless_method.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/endless_method.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_endless_method_command_syntax_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/endless_method_command_syntax.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/endless_method_command_syntax.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_endless_method_command_syntax_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/endless_method_command_syntax.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/endless_method_command_syntax.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_endless_method_forwarded_args_legacy_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/endless_method_forwarded_args_legacy.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/endless_method_forwarded_args_legacy.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_endless_method_forwarded_args_legacy_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/endless_method_forwarded_args_legacy.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/endless_method_forwarded_args_legacy.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_endless_method_with_rescue_mod_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/endless_method_with_rescue_mod.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/endless_method_with_rescue_mod.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_endless_method_with_rescue_mod_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/endless_method_with_rescue_mod.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/endless_method_with_rescue_mod.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_endless_method_without_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/endless_method_without_args.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/endless_method_without_args.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_endless_method_without_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/endless_method_without_args.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/endless_method_without_args.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ensure_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ensure.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ensure.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ensure_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ensure.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ensure.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ensure_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ensure_empty.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ensure_empty.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ensure_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ensure_empty.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ensure_empty.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_false_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/false.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/false.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_false_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/false.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/false.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_find_pattern_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/find_pattern.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/find_pattern.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_find_pattern_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/find_pattern.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/find_pattern.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_float_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/float.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/float.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_float_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/float.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/float.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_for_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/for.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/for.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_for_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/for.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/for.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_for_mlhs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/for_mlhs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/for_mlhs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_for_mlhs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/for_mlhs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/for_mlhs.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_forward_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/forward_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/forward_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_forward_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/forward_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/forward_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_forward_arg_with_open_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/forward_arg_with_open_args.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/forward_arg_with_open_args.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_forward_arg_with_open_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/forward_arg_with_open_args.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/forward_arg_with_open_args.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_forward_args_legacy_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/forward_args_legacy.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/forward_args_legacy.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_forward_args_legacy_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/forward_args_legacy.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/forward_args_legacy.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_forwarded_argument_with_kwrestarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/forwarded_argument_with_kwrestarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/forwarded_argument_with_kwrestarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_forwarded_argument_with_kwrestarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/forwarded_argument_with_kwrestarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/forwarded_argument_with_kwrestarg.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_forwarded_argument_with_restarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/forwarded_argument_with_restarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/forwarded_argument_with_restarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_forwarded_argument_with_restarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/forwarded_argument_with_restarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/forwarded_argument_with_restarg.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_forwarded_kwrestarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/forwarded_kwrestarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/forwarded_kwrestarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_forwarded_kwrestarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/forwarded_kwrestarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/forwarded_kwrestarg.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_forwarded_kwrestarg_with_additional_kwarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/forwarded_kwrestarg_with_additional_kwarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/forwarded_kwrestarg_with_additional_kwarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_forwarded_kwrestarg_with_additional_kwarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/forwarded_kwrestarg_with_additional_kwarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/forwarded_kwrestarg_with_additional_kwarg.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_forwarded_restarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/forwarded_restarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/forwarded_restarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_forwarded_restarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/forwarded_restarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/forwarded_restarg.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_gvar_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/gvar.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/gvar.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_gvar_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/gvar.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/gvar.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_gvasgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/gvasgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/gvasgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_gvasgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/gvasgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/gvasgn.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_hash_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/hash_empty.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/hash_empty.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_hash_empty_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/hash_empty.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/hash_empty.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_hash_hashrocket_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/hash_hashrocket.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/hash_hashrocket.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_hash_hashrocket_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/hash_hashrocket.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/hash_hashrocket.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_hash_kwsplat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/hash_kwsplat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/hash_kwsplat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_hash_kwsplat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/hash_kwsplat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/hash_kwsplat.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_hash_label_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/hash_label.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/hash_label.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_hash_label_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/hash_label.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/hash_label.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_hash_label_end_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/hash_label_end.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/hash_label_end.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_hash_label_end_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/hash_label_end.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/hash_label_end.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_hash_pair_value_omission_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/hash_pair_value_omission.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/hash_pair_value_omission.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_hash_pair_value_omission_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/hash_pair_value_omission.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/hash_pair_value_omission.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_heredoc_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/heredoc.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/heredoc.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_heredoc_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/heredoc.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/heredoc.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_if_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/if.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/if.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_if_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/if.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/if.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_if_else_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/if_else.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/if_else.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_if_else_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/if_else.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/if_else.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_if_elsif_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/if_elsif.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/if_elsif.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_if_elsif_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/if_elsif.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/if_elsif.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_if_masgn__24_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/if_masgn__24.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/if_masgn__24.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_if_masgn__24_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/if_masgn__24.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/if_masgn__24.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_if_mod_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/if_mod.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/if_mod.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_if_mod_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/if_mod.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/if_mod.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_if_nl_then_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/if_nl_then.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/if_nl_then.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_if_nl_then_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/if_nl_then.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/if_nl_then.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_int_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/int.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/int.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_int_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/int.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/int.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_int___LINE___txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/int___LINE__.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/int___LINE__.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_int___LINE___txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/int___LINE__.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/int___LINE__.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_interp_digit_var_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/interp_digit_var.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/interp_digit_var.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_interp_digit_var_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/interp_digit_var.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/interp_digit_var.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ivar_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ivar.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ivar.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ivar_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ivar.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ivar.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ivasgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ivasgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ivasgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ivasgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ivasgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ivasgn.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_keyword_argument_omission_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/keyword_argument_omission.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/keyword_argument_omission.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_keyword_argument_omission_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/keyword_argument_omission.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/keyword_argument_omission.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_kwarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/kwarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/kwarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_kwarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/kwarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/kwarg.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_kwarg_combinations_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/kwarg_combinations.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/kwarg_combinations.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_kwarg_combinations_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/kwarg_combinations.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/kwarg_combinations.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_kwarg_no_paren_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/kwarg_no_paren.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/kwarg_no_paren.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_kwarg_no_paren_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/kwarg_no_paren.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/kwarg_no_paren.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_kwbegin_compstmt_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/kwbegin_compstmt.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/kwbegin_compstmt.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_kwbegin_compstmt_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/kwbegin_compstmt.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/kwbegin_compstmt.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_kwnilarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/kwnilarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/kwnilarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_kwnilarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/kwnilarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/kwnilarg.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_kwoptarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/kwoptarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/kwoptarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_kwoptarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/kwoptarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/kwoptarg.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_kwoptarg_with_kwrestarg_and_forwarded_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/kwoptarg_with_kwrestarg_and_forwarded_args.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/kwoptarg_with_kwrestarg_and_forwarded_args.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_kwoptarg_with_kwrestarg_and_forwarded_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/kwoptarg_with_kwrestarg_and_forwarded_args.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/kwoptarg_with_kwrestarg_and_forwarded_args.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_kwrestarg_named_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/kwrestarg_named.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/kwrestarg_named.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_kwrestarg_named_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/kwrestarg_named.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/kwrestarg_named.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_kwrestarg_unnamed_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/kwrestarg_unnamed.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/kwrestarg_unnamed.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_kwrestarg_unnamed_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/kwrestarg_unnamed.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/kwrestarg_unnamed.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_lbrace_arg_after_command_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/lbrace_arg_after_command_args.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/lbrace_arg_after_command_args.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_lbrace_arg_after_command_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/lbrace_arg_after_command_args.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/lbrace_arg_after_command_args.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_lparenarg_after_lvar__since_25_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/lparenarg_after_lvar__since_25.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/lparenarg_after_lvar__since_25.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_lparenarg_after_lvar__since_25_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/lparenarg_after_lvar__since_25.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/lparenarg_after_lvar__since_25.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_lvar_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/lvar.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/lvar.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_lvar_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/lvar.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/lvar.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_lvar_injecting_match_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/lvar_injecting_match.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/lvar_injecting_match.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_lvar_injecting_match_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/lvar_injecting_match.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/lvar_injecting_match.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_lvasgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/lvasgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/lvasgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_lvasgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/lvasgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/lvasgn.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_marg_combinations_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/marg_combinations.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/marg_combinations.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_marg_combinations_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/marg_combinations.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/marg_combinations.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_masgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/masgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/masgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_masgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/masgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/masgn.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_masgn_attr_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/masgn_attr.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/masgn_attr.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_masgn_attr_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/masgn_attr.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/masgn_attr.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_masgn_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/masgn_cmd.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/masgn_cmd.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_masgn_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/masgn_cmd.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/masgn_cmd.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_masgn_const_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/masgn_const.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/masgn_const.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_masgn_const_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/masgn_const.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/masgn_const.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_masgn_nested_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/masgn_nested.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/masgn_nested.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_masgn_nested_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/masgn_nested.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/masgn_nested.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_masgn_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/masgn_splat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/masgn_splat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_masgn_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/masgn_splat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/masgn_splat.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_method_definition_in_while_cond_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/method_definition_in_while_cond.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/method_definition_in_while_cond.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_method_definition_in_while_cond_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/method_definition_in_while_cond.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/method_definition_in_while_cond.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_module_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/module.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/module.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_module_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/module.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/module.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_multiple_args_with_trailing_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/multiple_args_with_trailing_comma.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/multiple_args_with_trailing_comma.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_multiple_args_with_trailing_comma_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/multiple_args_with_trailing_comma.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/multiple_args_with_trailing_comma.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_multiple_pattern_matches_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/multiple_pattern_matches.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/multiple_pattern_matches.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_multiple_pattern_matches_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/multiple_pattern_matches.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/multiple_pattern_matches.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_newline_in_hash_argument_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/newline_in_hash_argument.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/newline_in_hash_argument.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_newline_in_hash_argument_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/newline_in_hash_argument.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/newline_in_hash_argument.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_nil_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/nil.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/nil.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_nil_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/nil.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/nil.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_nil_expression_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/nil_expression.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/nil_expression.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_nil_expression_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/nil_expression.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/nil_expression.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_non_lvar_injecting_match_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/non_lvar_injecting_match.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/non_lvar_injecting_match.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_non_lvar_injecting_match_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/non_lvar_injecting_match.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/non_lvar_injecting_match.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_not_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/not.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/not.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_not_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/not.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/not.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_not_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/not_cmd.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/not_cmd.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_not_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/not_cmd.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/not_cmd.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_not_masgn__24_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/not_masgn__24.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/not_masgn__24.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_not_masgn__24_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/not_masgn__24.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/not_masgn__24.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_nth_ref_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/nth_ref.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/nth_ref.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_nth_ref_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/nth_ref.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/nth_ref.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_numbered_args_after_27_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/numbered_args_after_27.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/numbered_args_after_27.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_numbered_args_after_27_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/numbered_args_after_27.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/numbered_args_after_27.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_numparam_outside_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/numparam_outside_block.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/numparam_outside_block.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_numparam_outside_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/numparam_outside_block.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/numparam_outside_block.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_numparam_ruby_bug_19025_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/numparam_ruby_bug_19025.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/numparam_ruby_bug_19025.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_numparam_ruby_bug_19025_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/numparam_ruby_bug_19025.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/numparam_ruby_bug_19025.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_op_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/op_asgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/op_asgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_op_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/op_asgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/op_asgn.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_op_asgn_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/op_asgn_cmd.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/op_asgn_cmd.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_op_asgn_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/op_asgn_cmd.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/op_asgn_cmd.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_op_asgn_index_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/op_asgn_index.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/op_asgn_index.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_op_asgn_index_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/op_asgn_index.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/op_asgn_index.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_op_asgn_index_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/op_asgn_index_cmd.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/op_asgn_index_cmd.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_op_asgn_index_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/op_asgn_index_cmd.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/op_asgn_index_cmd.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_optarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/optarg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/optarg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_optarg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/optarg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/optarg.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_or_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/or.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/or.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_or_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/or.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/or.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_or_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/or_asgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/or_asgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_or_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/or_asgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/or_asgn.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_parser_bug_272_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_bug_272.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/parser_bug_272.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_parser_bug_272_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_bug_272.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/parser_bug_272.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_parser_bug_490_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_bug_490.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/parser_bug_490.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_parser_bug_490_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_bug_490.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/parser_bug_490.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_parser_bug_507_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_bug_507.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/parser_bug_507.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_parser_bug_507_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_bug_507.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/parser_bug_507.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_parser_bug_518_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_bug_518.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/parser_bug_518.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_parser_bug_518_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_bug_518.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/parser_bug_518.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_parser_bug_525_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_bug_525.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/parser_bug_525.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_parser_bug_525_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_bug_525.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/parser_bug_525.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_parser_bug_604_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_bug_604.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/parser_bug_604.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_parser_bug_604_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_bug_604.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/parser_bug_604.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_parser_bug_640_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_bug_640.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/parser_bug_640.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_parser_bug_640_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_bug_640.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/parser_bug_640.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_parser_bug_645_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_bug_645.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/parser_bug_645.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_parser_bug_645_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_bug_645.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/parser_bug_645.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_parser_bug_830_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_bug_830.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/parser_bug_830.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_parser_bug_830_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_bug_830.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/parser_bug_830.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_parser_bug_989_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_bug_989.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/parser_bug_989.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_parser_bug_989_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_bug_989.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/parser_bug_989.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_parser_drops_truncated_parts_of_squiggly_heredoc_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_drops_truncated_parts_of_squiggly_heredoc.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/parser_drops_truncated_parts_of_squiggly_heredoc.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_parser_drops_truncated_parts_of_squiggly_heredoc_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_drops_truncated_parts_of_squiggly_heredoc.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/parser_drops_truncated_parts_of_squiggly_heredoc.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_parser_slash_slash_n_escaping_in_literals_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_slash_slash_n_escaping_in_literals.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/parser_slash_slash_n_escaping_in_literals.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_parser_slash_slash_n_escaping_in_literals_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/parser_slash_slash_n_escaping_in_literals.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/parser_slash_slash_n_escaping_in_literals.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pattern_matching__FILE__LINE_literals_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching__FILE__LINE_literals.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pattern_matching__FILE__LINE_literals.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pattern_matching__FILE__LINE_literals_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching__FILE__LINE_literals.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pattern_matching__FILE__LINE_literals.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pattern_matching_blank_else_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_blank_else.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pattern_matching_blank_else.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pattern_matching_blank_else_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_blank_else.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pattern_matching_blank_else.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pattern_matching_const_pattern_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_const_pattern.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pattern_matching_const_pattern.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pattern_matching_const_pattern_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_const_pattern.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pattern_matching_const_pattern.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pattern_matching_constants_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_constants.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pattern_matching_constants.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pattern_matching_constants_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_constants.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pattern_matching_constants.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pattern_matching_else_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_else.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pattern_matching_else.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pattern_matching_else_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_else.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pattern_matching_else.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pattern_matching_explicit_array_match_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_explicit_array_match.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pattern_matching_explicit_array_match.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pattern_matching_explicit_array_match_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_explicit_array_match.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pattern_matching_explicit_array_match.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pattern_matching_expr_in_paren_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_expr_in_paren.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pattern_matching_expr_in_paren.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pattern_matching_expr_in_paren_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_expr_in_paren.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pattern_matching_expr_in_paren.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pattern_matching_hash_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_hash.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pattern_matching_hash.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pattern_matching_hash_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_hash.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pattern_matching_hash.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pattern_matching_if_unless_modifiers_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_if_unless_modifiers.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pattern_matching_if_unless_modifiers.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pattern_matching_if_unless_modifiers_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_if_unless_modifiers.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pattern_matching_if_unless_modifiers.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pattern_matching_implicit_array_match_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_implicit_array_match.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pattern_matching_implicit_array_match.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pattern_matching_implicit_array_match_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_implicit_array_match.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pattern_matching_implicit_array_match.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pattern_matching_keyword_variable_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_keyword_variable.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pattern_matching_keyword_variable.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pattern_matching_keyword_variable_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_keyword_variable.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pattern_matching_keyword_variable.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pattern_matching_lambda_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_lambda.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pattern_matching_lambda.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pattern_matching_lambda_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_lambda.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pattern_matching_lambda.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pattern_matching_match_alt_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_match_alt.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pattern_matching_match_alt.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pattern_matching_match_alt_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_match_alt.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pattern_matching_match_alt.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pattern_matching_match_as_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_match_as.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pattern_matching_match_as.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pattern_matching_match_as_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_match_as.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pattern_matching_match_as.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pattern_matching_nil_pattern_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_nil_pattern.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pattern_matching_nil_pattern.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pattern_matching_nil_pattern_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_nil_pattern.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pattern_matching_nil_pattern.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pattern_matching_no_body_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_no_body.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pattern_matching_no_body.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pattern_matching_no_body_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_no_body.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pattern_matching_no_body.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pattern_matching_ranges_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_ranges.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pattern_matching_ranges.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pattern_matching_ranges_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_ranges.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pattern_matching_ranges.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pattern_matching_single_line_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_single_line.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pattern_matching_single_line.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pattern_matching_single_line_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_single_line.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pattern_matching_single_line.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pattern_matching_single_line_allowed_omission_of_parentheses_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_single_line_allowed_omission_of_parentheses.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pattern_matching_single_line_allowed_omission_of_parentheses.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pattern_matching_single_line_allowed_omission_of_parentheses_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_single_line_allowed_omission_of_parentheses.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pattern_matching_single_line_allowed_omission_of_parentheses.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pattern_matching_single_match_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_single_match.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pattern_matching_single_match.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pattern_matching_single_match_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pattern_matching_single_match.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pattern_matching_single_match.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_pin_expr_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pin_expr.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/pin_expr.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_pin_expr_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/pin_expr.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/pin_expr.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_postexe_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/postexe.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/postexe.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_postexe_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/postexe.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/postexe.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_preexe_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/preexe.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/preexe.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_preexe_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/preexe.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/preexe.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_procarg0_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/procarg0.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/procarg0.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_procarg0_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/procarg0.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/procarg0.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_procarg0_legacy_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/procarg0_legacy.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/procarg0_legacy.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_procarg0_legacy_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/procarg0_legacy.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/procarg0_legacy.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_range_exclusive_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/range_exclusive.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/range_exclusive.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_range_exclusive_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/range_exclusive.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/range_exclusive.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_range_inclusive_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/range_inclusive.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/range_inclusive.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_range_inclusive_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/range_inclusive.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/range_inclusive.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_rational_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rational.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/rational.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_rational_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rational.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/rational.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_regex_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/regex_interp.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/regex_interp.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_regex_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/regex_interp.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/regex_interp.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_regex_plain_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/regex_plain.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/regex_plain.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_regex_plain_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/regex_plain.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/regex_plain.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_resbody_list_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/resbody_list.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/resbody_list.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_resbody_list_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/resbody_list.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/resbody_list.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_resbody_list_mrhs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/resbody_list_mrhs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/resbody_list_mrhs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_resbody_list_mrhs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/resbody_list_mrhs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/resbody_list_mrhs.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_resbody_list_var_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/resbody_list_var.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/resbody_list_var.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_resbody_list_var_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/resbody_list_var.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/resbody_list_var.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_resbody_var_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/resbody_var.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/resbody_var.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_resbody_var_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/resbody_var.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/resbody_var.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rescue.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/rescue.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_rescue_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rescue.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/rescue.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_rescue_else_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rescue_else.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/rescue_else.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_rescue_else_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rescue_else.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/rescue_else.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_rescue_else_ensure_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rescue_else_ensure.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/rescue_else_ensure.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_rescue_else_ensure_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rescue_else_ensure.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/rescue_else_ensure.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_rescue_ensure_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rescue_ensure.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/rescue_ensure.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_rescue_ensure_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rescue_ensure.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/rescue_ensure.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_rescue_in_lambda_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rescue_in_lambda_block.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/rescue_in_lambda_block.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_rescue_in_lambda_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rescue_in_lambda_block.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/rescue_in_lambda_block.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_rescue_mod_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rescue_mod.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/rescue_mod.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_rescue_mod_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rescue_mod.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/rescue_mod.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_rescue_mod_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rescue_mod_asgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/rescue_mod_asgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_rescue_mod_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rescue_mod_asgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/rescue_mod_asgn.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_rescue_mod_masgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rescue_mod_masgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/rescue_mod_masgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_rescue_mod_masgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rescue_mod_masgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/rescue_mod_masgn.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_rescue_mod_op_assign_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rescue_mod_op_assign.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/rescue_mod_op_assign.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_rescue_mod_op_assign_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rescue_mod_op_assign.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/rescue_mod_op_assign.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_rescue_without_begin_end_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rescue_without_begin_end.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/rescue_without_begin_end.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_rescue_without_begin_end_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/rescue_without_begin_end.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/rescue_without_begin_end.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_restarg_named_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/restarg_named.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/restarg_named.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_restarg_named_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/restarg_named.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/restarg_named.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_restarg_unnamed_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/restarg_unnamed.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/restarg_unnamed.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_restarg_unnamed_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/restarg_unnamed.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/restarg_unnamed.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_return_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/return.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/return.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_return_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/return.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/return.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_return_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/return_block.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/return_block.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_return_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/return_block.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/return_block.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ruby_bug_10279_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_10279.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ruby_bug_10279.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ruby_bug_10279_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_10279.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ruby_bug_10279.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ruby_bug_10653_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_10653.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ruby_bug_10653.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ruby_bug_10653_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_10653.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ruby_bug_10653.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ruby_bug_11107_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_11107.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ruby_bug_11107.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ruby_bug_11107_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_11107.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ruby_bug_11107.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ruby_bug_11380_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_11380.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ruby_bug_11380.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ruby_bug_11380_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_11380.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ruby_bug_11380.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ruby_bug_11873_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_11873.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ruby_bug_11873.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ruby_bug_11873_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_11873.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ruby_bug_11873.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ruby_bug_11873_a_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_11873_a.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ruby_bug_11873_a.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ruby_bug_11873_a_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_11873_a.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ruby_bug_11873_a.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ruby_bug_11873_b_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_11873_b.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ruby_bug_11873_b.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ruby_bug_11873_b_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_11873_b.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ruby_bug_11873_b.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ruby_bug_11989_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_11989.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ruby_bug_11989.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ruby_bug_11989_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_11989.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ruby_bug_11989.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ruby_bug_11990_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_11990.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ruby_bug_11990.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ruby_bug_11990_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_11990.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ruby_bug_11990.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ruby_bug_12073_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_12073.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ruby_bug_12073.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ruby_bug_12073_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_12073.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ruby_bug_12073.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ruby_bug_12402_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_12402.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ruby_bug_12402.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ruby_bug_12402_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_12402.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ruby_bug_12402.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ruby_bug_12669_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_12669.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ruby_bug_12669.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ruby_bug_12669_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_12669.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ruby_bug_12669.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ruby_bug_12686_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_12686.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ruby_bug_12686.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ruby_bug_12686_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_12686.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ruby_bug_12686.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ruby_bug_13547_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_13547.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ruby_bug_13547.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ruby_bug_13547_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_13547.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ruby_bug_13547.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ruby_bug_14690_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_14690.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ruby_bug_14690.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ruby_bug_14690_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_14690.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ruby_bug_14690.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ruby_bug_15789_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_15789.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ruby_bug_15789.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ruby_bug_15789_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_15789.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ruby_bug_15789.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ruby_bug_18878_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_18878.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ruby_bug_18878.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ruby_bug_18878_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_18878.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ruby_bug_18878.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ruby_bug_19281_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_19281.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ruby_bug_19281.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ruby_bug_19281_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_19281.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ruby_bug_19281.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ruby_bug_19539_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_19539.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ruby_bug_19539.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ruby_bug_19539_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_19539.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ruby_bug_19539.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ruby_bug_9669_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_9669.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ruby_bug_9669.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ruby_bug_9669_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ruby_bug_9669.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ruby_bug_9669.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_sclass_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/sclass.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/sclass.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_sclass_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/sclass.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/sclass.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_self_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/self.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/self.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_self_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/self.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/self.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_attr_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_attr_asgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_attr_asgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_attr_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_attr_asgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_attr_asgn.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_attr_asgn_conditional_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_attr_asgn_conditional.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_attr_asgn_conditional.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_attr_asgn_conditional_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_attr_asgn_conditional.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_attr_asgn_conditional.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_binary_op_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_binary_op.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_binary_op.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_binary_op_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_binary_op.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_binary_op.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_block_chain_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_block_chain_cmd.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_block_chain_cmd.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_block_chain_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_block_chain_cmd.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_block_chain_cmd.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_block_conditional_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_block_conditional.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_block_conditional.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_block_conditional_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_block_conditional.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_block_conditional.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_call.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_call.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_call.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_call.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_conditional_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_conditional.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_conditional.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_conditional_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_conditional.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_conditional.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_index_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_index.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_index.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_index_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_index.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_index.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_index_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_index_asgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_index_asgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_index_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_index_asgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_index_asgn.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_index_asgn_legacy_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_index_asgn_legacy.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_index_asgn_legacy.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_index_asgn_legacy_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_index_asgn_legacy.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_index_asgn_legacy.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_index_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_index_cmd.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_index_cmd.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_index_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_index_cmd.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_index_cmd.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_index_legacy_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_index_legacy.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_index_legacy.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_index_legacy_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_index_legacy.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_index_legacy.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_lambda_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_lambda.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_lambda.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_lambda_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_lambda.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_lambda.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_lambda_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_lambda_args.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_lambda_args.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_lambda_args_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_lambda_args.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_lambda_args.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_lambda_args_noparen_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_lambda_args_noparen.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_lambda_args_noparen.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_lambda_args_noparen_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_lambda_args_noparen.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_lambda_args_noparen.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_lambda_args_shadow_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_lambda_args_shadow.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_lambda_args_shadow.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_lambda_args_shadow_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_lambda_args_shadow.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_lambda_args_shadow.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_lambda_legacy_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_lambda_legacy.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_lambda_legacy.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_lambda_legacy_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_lambda_legacy.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_lambda_legacy.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_op_asgn_conditional_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_op_asgn_conditional.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_op_asgn_conditional.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_op_asgn_conditional_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_op_asgn_conditional.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_op_asgn_conditional.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_plain_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_plain.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_plain.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_plain_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_plain.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_plain.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_plain_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_plain_cmd.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_plain_cmd.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_plain_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_plain_cmd.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_plain_cmd.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_self_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_self.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_self.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_self_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_self.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_self.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_self_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_self_block.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_self_block.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_self_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_self_block.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_self_block.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_send_unary_op_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_unary_op.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/send_unary_op.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_send_unary_op_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/send_unary_op.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/send_unary_op.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_slash_newline_in_heredocs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/slash_newline_in_heredocs.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/slash_newline_in_heredocs.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_slash_newline_in_heredocs_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/slash_newline_in_heredocs.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/slash_newline_in_heredocs.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_space_args_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/space_args_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/space_args_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_space_args_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/space_args_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/space_args_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_space_args_arg_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/space_args_arg_block.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/space_args_arg_block.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_space_args_arg_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/space_args_arg_block.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/space_args_arg_block.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_space_args_arg_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/space_args_arg_call.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/space_args_arg_call.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_space_args_arg_call_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/space_args_arg_call.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/space_args_arg_call.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_space_args_arg_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/space_args_arg_newline.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/space_args_arg_newline.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_space_args_arg_newline_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/space_args_arg_newline.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/space_args_arg_newline.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_space_args_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/space_args_block.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/space_args_block.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_space_args_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/space_args_block.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/space_args_block.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_space_args_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/space_args_cmd.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/space_args_cmd.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_space_args_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/space_args_cmd.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/space_args_cmd.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_string___FILE___txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/string___FILE__.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/string___FILE__.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_string___FILE___txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/string___FILE__.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/string___FILE__.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_string_concat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/string_concat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/string_concat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_string_concat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/string_concat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/string_concat.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_string_dvar_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/string_dvar.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/string_dvar.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_string_dvar_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/string_dvar.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/string_dvar.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_string_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/string_interp.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/string_interp.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_string_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/string_interp.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/string_interp.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_string_plain_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/string_plain.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/string_plain.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_string_plain_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/string_plain.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/string_plain.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_super_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/super.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/super.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_super_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/super.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/super.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_super_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/super_block.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/super_block.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_super_block_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/super_block.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/super_block.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_symbol_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/symbol_interp.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/symbol_interp.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_symbol_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/symbol_interp.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/symbol_interp.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_symbol_plain_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/symbol_plain.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/symbol_plain.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_symbol_plain_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/symbol_plain.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/symbol_plain.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ternary_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ternary.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ternary.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ternary_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ternary.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ternary.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_ternary_ambiguous_symbol_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ternary_ambiguous_symbol.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/ternary_ambiguous_symbol.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_ternary_ambiguous_symbol_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/ternary_ambiguous_symbol.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/ternary_ambiguous_symbol.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_trailing_forward_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/trailing_forward_arg.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/trailing_forward_arg.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_trailing_forward_arg_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/trailing_forward_arg.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/trailing_forward_arg.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_true_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/true.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/true.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_true_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/true.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/true.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_unary_num_pow_precedence_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/unary_num_pow_precedence.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/unary_num_pow_precedence.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_unary_num_pow_precedence_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/unary_num_pow_precedence.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/unary_num_pow_precedence.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_undef_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/undef.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/undef.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_undef_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/undef.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/undef.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_unless_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/unless.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/unless.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_unless_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/unless.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/unless.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_unless_else_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/unless_else.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/unless_else.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_unless_else_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/unless_else.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/unless_else.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_unless_mod_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/unless_mod.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/unless_mod.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_unless_mod_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/unless_mod.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/unless_mod.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_until_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/until.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/until.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_until_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/until.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/until.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_until_mod_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/until_mod.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/until_mod.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_until_mod_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/until_mod.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/until_mod.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_until_post_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/until_post.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/until_post.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_until_post_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/until_post.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/until_post.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_var_and_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/var_and_asgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/var_and_asgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_var_and_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/var_and_asgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/var_and_asgn.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_var_op_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/var_op_asgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/var_op_asgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_var_op_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/var_op_asgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/var_op_asgn.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_var_op_asgn_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/var_op_asgn_cmd.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/var_op_asgn_cmd.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_var_op_asgn_cmd_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/var_op_asgn_cmd.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/var_op_asgn_cmd.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_var_or_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/var_or_asgn.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/var_or_asgn.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_var_or_asgn_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/var_or_asgn.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/var_or_asgn.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_when_multi_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/when_multi.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/when_multi.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_when_multi_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/when_multi.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/when_multi.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_when_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/when_splat.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/when_splat.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_when_splat_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/when_splat.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/when_splat.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_when_then_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/when_then.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/when_then.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_when_then_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/when_then.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/when_then.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_while_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/while.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/while.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_while_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/while.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/while.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_while_mod_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/while_mod.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/while_mod.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_while_mod_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/while_mod.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/while_mod.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_while_post_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/while_post.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/while_post.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_while_post_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/while_post.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/while_post.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_xstring_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/xstring_interp.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/xstring_interp.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_xstring_interp_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/xstring_interp.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/xstring_interp.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_xstring_plain_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/xstring_plain.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/xstring_plain.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_xstring_plain_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/xstring_plain.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/xstring_plain.txt"].assert_debug_eq(&program);
}

fn test_ast_whitequark_zsuper_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/zsuper.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/whitequark/zsuper.txt").trim(), program.snapshot());
}
#[test]
fn test_program_whitequark_zsuper_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/whitequark/zsuper.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/whitequark/zsuper.txt"].assert_debug_eq(&program);
}

fn test_ast_xstring_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/xstring.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/xstring.txt").trim(), program.snapshot());
}
#[test]
fn test_program_xstring_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/xstring.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/xstring.txt"].assert_debug_eq(&program);
}

fn test_ast_xstring_with_backslash_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/xstring_with_backslash.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/xstring_with_backslash.txt").trim(), program.snapshot());
}
#[test]
fn test_program_xstring_with_backslash_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/xstring_with_backslash.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/xstring_with_backslash.txt"].assert_debug_eq(&program);
}

fn test_ast_yield_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/yield.txt").to_string()).unwrap();
  similar_asserts::assert_eq!(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/snapshots/yield.txt").trim(), program.snapshot());
}
#[test]
fn test_program_yield_txt() {
  let program = Program::parse(include_str!("/Users/segiddins/Development/github.com/ruby/prism/test/prism/fixtures/yield.txt").to_string()).unwrap();
  expect_test::expect_file!["/Users/segiddins/Development/github.com/segiddins/gemfile-rs/tests/snapshots/yield.txt"].assert_debug_eq(&program);
}
