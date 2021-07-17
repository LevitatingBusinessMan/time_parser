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

type Time = std::time::Duration;

static SECONDS_LOOKUP: [(&'static str, u64); 7] = [
	("seconds", 1),
	("minutes", 60),
	("hours", 60 * 60),
	("days", 24 * 60 * 60),
	("weeks", 7 * 24 * 60 * 60),
	("months", ((365.2425 * (24 * 60 * 60) as f32) / 12 as f32) as u64),
	("years", (365.2425 * (24 * 60 * 60) as f32) as u64 ),
	
	/* A Gregorian year has 365.2425 * 24 * 60 * 60 ==
         31556952 seconds on the average */
];

struct Parser {
	tokens: Vec<Token>,
	current: usize
}

pub fn parse(tokens: Vec<Token>) {
	let mut parser = Parser {
		tokens,
		current: 0
	};

	parser.parse();
}

macro_rules! current {
	($self:ident) => {
		$self.tokens[$self.current]
	};
}

macro_rules! next {
	($self:ident) => {
		if $self.tokens.len() > $self.current+1 {
			Some(&$self.tokens[$self.current+1])
		}
		else {None}
	};
}

macro_rules! advance {
	($self:ident) => {
		$self.current += 1;
	};
}

enum Node {
	Time(Time),
}

impl Parser {

	fn parse(&mut self) -> Node{
		self.expression()
	}

	fn expression(&mut self) -> Node {
		
		println!("{:?}", self.addition());
		self.current = 0;
		Node::Time(self.addition())
	}

	fn addition(&mut self) -> Time {
		let first = self.time();

		//Reached end
		if self.current >= self.tokens.len() {
			return first;
		}

		match current!(self) {
			Token::Keyword(keyword) => {
				if keyword == "and" {
					advance!(self);
					let second = self.time();
					if let Some(total) = first.checked_add(second) {
						return total;
					} else {
						panic!();
					}
				} else {
					return first;
				}
			},
			_ => return first
		}
	}

	fn time(&mut self) -> Time {

		if let Token::Number(number) = current!(self) {
			
			if next!(self).is_some() {
				advance!(self)
			} else {
				panic!("Time node should end with unit")
			}

			if let Token::Unit(unit) = current!(self) {
				advance!(self);

				if let Some(lkp) = SECONDS_LOOKUP.iter().find(|lkp| lkp.0 == unit) {
					let seconds = lkp.1 * number as u64;
					
					return Time::new(seconds, 0);
				} else {
					panic!("Invalid unit")
				}

			} else {
				panic!("Time node should end with unit")
			}
		
		} else {
			panic!("Time node should start with number")
		}

	}
}
