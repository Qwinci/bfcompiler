mod lexer;
mod codegen;

fn main() {
	// let tokens = lexer::lex("tests/five.bf");
	let tokens = lexer::lex("tests/main.bf");
	codegen::codegen(tokens);
    println!("Hello, world!");
}
