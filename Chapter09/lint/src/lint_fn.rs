#![feature(plugin_registrar)]
#![feature(box_syntax, rustc_private)]

extern crate syntax;
extern crate syntax_pos;

#[macro_use]
extern crate rustc;
extern crate rustc_plugin;

use rustc::lint::{EarlyContext, LintContext, LintPass, EarlyLintPass,
                  LintArray};
use rustc_plugin::Registry;
use syntax::ast::{NodeId, FnDecl, Block, FunctionRetTy};
use syntax::visit::FnKind;
use syntax_pos::Span;

declare_lint!(TEST_FN_RETURN, Warn, "Warn about functions that have no return parameters");

struct Pass;

impl LintPass for Pass {
    fn get_lints(&self) -> LintArray {
        lint_array!(TEST_FN_RETURN)
    }
}

impl EarlyLintPass for Pass {
    fn check_fn(&mut self, cx: &EarlyContext, _: FnKind, fndecl: &FnDecl, span: Span, _: NodeId) {
        match fndecl.output {
            FunctionRetTy::Default(_) =>
                cx.span_lint(TEST_FN_RETURN, span, "function has no return parameters"),
            _ => {}
        }
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    let pass = Box::new(Pass);
    reg.register_early_lint_pass(pass);
}
