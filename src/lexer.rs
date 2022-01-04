use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub enum Token {
	PtrRight,
	PtrLeft,
	Inc,
	Dec,
	Out,
	In,
	JmpPast,
	JmpBack
}

pub fn lex(path: &str) -> Vec<Token> {
	let file = File::open(Path::new(path)).expect(format!("failed to open file {}.", path).as_str());

	let mut buf_reader = BufReader::new(file);
	let mut content = String::new();
	buf_reader.read_to_string(&mut content).expect("Failed to read file data to a string.");

	let mut tokens: Vec<Token> = Vec::new();

	for char in content.chars() {
		match char {
			'>' => {
				tokens.push(Token::PtrRight);
			}
			'<' => {
				tokens.push(Token::PtrLeft);
			}
			'+' => {
				tokens.push(Token::Inc);
			}
			'-' => {
				tokens.push(Token::Dec);
			}
			'.' => {
				tokens.push(Token::Out);
			}
			',' => {
				tokens.push(Token::In);
			}
			'[' => {
				tokens.push(Token::JmpPast);
			}
			']' => {
				tokens.push(Token::JmpBack);
			}
			_ => continue
		}
	}

	return tokens;
}