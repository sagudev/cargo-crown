#![feature(rustc_private)]
#![warn(rustc::internal)]
#![allow(rustc::potential_query_instability)]

// This rustc crates are private so they must be manually imported.
extern crate rustc_ast;
extern crate rustc_driver;
extern crate rustc_error_messages;
extern crate rustc_hir;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_trait_selection;
extern crate rustc_type_ir;

use std::process::ExitCode;

use rustc_driver::Callbacks;
use rustc_interface::interface::Config;

mod common;
#[cfg(feature = "unrooted_must_root_lint")]
mod unrooted_must_root;

#[cfg(feature = "trace_in_no_trace_lint")]
mod trace_in_no_trace;

struct MyCallbacks;

impl Callbacks for MyCallbacks {
    fn config(&mut self, config: &mut Config) {
        config.register_lints = Some(Box::new(move |sess, lint_store| {
            // Skip checks for proc-macro crates.
            if sess
                .crate_types()
                .contains(&rustc_session::config::CrateType::ProcMacro)
            {
                return;
            }

            #[cfg(feature = "unrooted_must_root_lint")]
            unrooted_must_root::register(lint_store);
            #[cfg(feature = "trace_in_no_trace_lint")]
            trace_in_no_trace::register(lint_store);
        }));
    }
}

fn main() -> ExitCode {
    rustc_driver::init_env_logger("CROWN_LOG");
    let args: Vec<_> = std::env::args().collect();

    match rustc_driver::RunCompiler::new(&args, &mut MyCallbacks).run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
