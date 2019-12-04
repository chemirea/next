use inkwell::context::Context;
use inkwell::passes::PassManager;
use next_compiler::compiler::Compiler;
use next_parser;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = get_filename();
    let mut file = File::open(&filename).expect(&format!("{} file not found", &filename));

    let mut input = String::new();
    file.read_to_string(&mut input)
        // ファイルの読み込み中に問題がありました
        .expect("something went wrong reading the file");

    match next_parser::parse(&input) {
        Ok(v) => compile(&v),
        Err(e) => println!("{:?}", e),
    };
}

fn get_filename() -> String {
    let args: Vec<String> = env::args().collect();
    return if args.len() >= 2 {
        args.get(1).unwrap().to_string()
    } else {
        "sample_nx_project/main.nx".to_string()
    };
}

fn compile(program: &next_parser::ast::Program) {
    let context = &Context::create();
    let module = &context.create_module("main");
    let builder = &context.create_builder();

    let fpm = &PassManager::create(module);

    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();
    fpm.add_gvn_pass();
    fpm.add_cfg_simplification_pass();
    fpm.add_basic_alias_analysis_pass();
    fpm.add_promote_memory_to_register_pass();
    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();

    fpm.initialize();

    let compiler = Compiler {
        context,
        builder,
        fpm,
        module,
        program,
    };

    compiler.compile();
}
