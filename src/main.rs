mod lexer;
mod codegen;

fn main() {
	let tokens = lexer::lex("tests/five.bf");
	codegen::codegen(tokens);
    println!("Hello, world!");
}
