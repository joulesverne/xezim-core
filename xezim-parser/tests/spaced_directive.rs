//! IEEE 1800-2017 §22.1: whitespace between the grave accent and a compiler
//! directive name (`` ` ifdef X``) is not legal, but other simulators take it
//! as the directive. The preprocessor does the same, with a warning, rather
//! than pass the text to the parser, which then failed on it.

use sv_parser::preprocess;

fn norm(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn a_spaced_ifdef_block_is_skipped_when_undefined() {
    let out = preprocess(concat!(
        "module m;\n",
        "` ifdef NOT_DEFINED\n",
        "  logic gone;\n",
        "` endif\n",
        "  logic kept;\n",
        "endmodule\n",
    ));
    let n = norm(&out);
    assert!(
        !n.contains("gone"),
        "undefined branch must vanish; got: {n:?}"
    );
    assert!(
        !n.contains('`'),
        "no directive text may reach the parser; got: {n:?}"
    );
    assert!(
        n.contains("logic kept"),
        "code after `endif survives; got: {n:?}"
    );
}

#[test]
fn a_spaced_define_and_ifdef_take_effect() {
    let out = preprocess(concat!(
        "`\tdefine ON\n",
        "` ifdef ON\n",
        "  logic taken;\n",
        "` else\n",
        "  logic not_taken;\n",
        "` endif\n",
    ));
    let n = norm(&out);
    assert!(n.contains("logic taken"), "defined branch kept; got: {n:?}");
    assert!(!n.contains("not_taken"), "else branch dropped; got: {n:?}");
}

#[test]
fn only_directive_names_are_accepted_after_the_space() {
    // `ifdef_like` merely starts with a directive name: not a directive.
    let out = preprocess("module m;\n` ifdef_like\nendmodule\n");
    let n = norm(&out);
    assert!(
        n.contains("ifdef_like"),
        "text must be left alone; got: {n:?}"
    );
    assert!(n.contains("endmodule"), "got: {n:?}");
}
