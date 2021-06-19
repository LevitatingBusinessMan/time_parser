//http://craftinginterpreters.com/parsing-expressions.html

use crate::lexer::Token;

/*

We have 2 primitives
Time: "3 hours" or "2 minutes"
Date: "23 januari" or "tuesday"

We have the higher precedence date expressions (which return a date)
<time> before <date>
<time> after <date>

And the lower precedence time expressions (which return a time)
<time> and <time>
*/

/*
expression -> addition
addition -> time ("and" time)*
time -> number unit
*/

struct Parser {
	tokens: Vec<Token>,
	current: u32
}

pub fn parse(tokens: Vec<Token>) {
	let mut parser = Parser {
		tokens,
		current: 0
	};

	parser.parse()
}

impl Parser {
	fn parse(&mut self) {
		
	}

	fn expression() {

	}
}
