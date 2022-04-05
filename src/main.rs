#![feature(rustc_private)]

// NOTE: For the example to compile, you will need to first run the following:
//     rustup component add rustc-dev llvm-tools-preview

// version: 1.53.0-nightly (9b0edb7fd 2021-03-27)

extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use rustc_driver::Compilation;
use rustc_interface::interface;
use rustc_interface::Config;
use rustc_interface::Queries;
use rustc_session::config;
use rustc_span::source_map;
use std::path;
use std::process;
use std::str;
struct MyCallback {
    input: String,
    sys_root: Option<path::PathBuf>,
}

impl rustc_driver::Callbacks for MyCallback {
    fn config(&mut self, config: &mut Config) {
        config.input = config::Input::Str {
            name: source_map::FileName::Custom("main.rs".to_string()),
            input: self.input.clone(),
        };
        config.opts.maybe_sysroot = self.sys_root.clone()
    }

    fn after_analysis<'tcx>(
        &mut self,
        compiler: &interface::Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        // let session = compiler.session();
        // session.abort_if_errors();
        
        queries.global_ctxt().unwrap().peek_mut().enter(|tcx| {
            tcx.mir_keys(()).iter().for_each(|&local_def_id| {
                // Skip items that are not functions or methods.
                let hir_id = tcx.hir().local_def_id_to_hir_id(local_def_id);
                let hir_node = tcx.hir().get(hir_id);
            });
        });

        Compilation::Continue
    }
}

fn main() -> Result<(), rustc_errors::ErrorReported>
{
    let input = "fn main() {}";
    let out = process::Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        .unwrap();
    let sysroot = str::from_utf8(&out.stdout).unwrap().trim();
   
    let mut callback = MyCallback {
        sys_root: Some(path::PathBuf::from(sysroot)),
        input: input.to_string()
    };
    let args : Vec<String> = vec![
        "-v".into(),
        "main.rs".into(),
    ];
    rustc_driver::RunCompiler::new(&args, &mut callback).run()
}