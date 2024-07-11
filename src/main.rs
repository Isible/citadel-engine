use std::{env, fs, path::PathBuf};

use bumpalo::Bump;
use citadel_api::{
    backend::asm::{AsmBackend, TargetX86_64},
    compile,
};
use citadel_irparser::{IRLexer, IRParser};

fn main() {
    run()
}

fn run() {
    let file_content =
        fs::read(path_from_arg()).expect("User needs to specify a path to the file containing the IR");
    let lexer = IRLexer::new(std::str::from_utf8(&file_content).unwrap());
    let arena = Bump::new();
    let mut parser = IRParser::new(&lexer, &arena);
    let ir_stream = parser.parse_program();
    compile!(AsmBackend::new(TargetX86_64), ir_stream)
        .to_file(PathBuf::from("out.asm"))
        .expect("Failed to create compiled asm file");
}

fn path_from_arg() -> PathBuf {
    env::args()
        .into_iter()
        .nth(1)
        .map(|arg| arg.into())
        .unwrap_or(PathBuf::from("main.chir"))
}
