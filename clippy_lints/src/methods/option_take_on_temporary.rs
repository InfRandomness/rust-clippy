use clippy_utils::diagnostics::span_lint;
use clippy_utils::ty::is_type_diagnostic_item;
use rustc_hir::Expr;
use rustc_lint::LateContext;
use rustc_span::sym::Result as sym_result;

use super::OPTION_TAKE_ON_TEMPORARY;

pub(super) fn check<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>, recv: &'tcx Expr<'_>) {
    if_chain! {
        if is_type_diagnostic_item(cx, cx.typeck_results().expr_ty(recv), sym_result);
        let result_type = cx.typeck_results().expr_ty(recv);

        then {
            span_lint(
                cx,
                OPTION_TAKE_ON_TEMPORARY,
                expr.span,
                "Format test"
            );
        }
    };
}
