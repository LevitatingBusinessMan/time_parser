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
date_expression -> (time_expression ("before" | "after"))? date
date -> ((nth day)? (month)? (year)?) | weekday
time_expression -> addition
addition -> time ("and" time)*
time -> number unit
*/

type Time = std::time::Duration;
type Date = std::time::SystemTime;

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

	let date = parser.parse();
	match date {
		Some(date) => println!("{:?}", date.duration_since(Date::UNIX_EPOCH).unwrap().as_secs()),
		_ => println!("error")
	}
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

	fn parse(&mut self) -> Option<Date> {
		let duration = self.time_expression();
		let now = Date::now();
		return now.checked_add(duration)
	}

	fn time_expression(&mut self) -> Time {
		self.addition()
	}

	fn addition(&mut self) -> Time {
		let mut first = self.time();

		//Reached end
		if self.current >= self.tokens.len() {
			return first;
		}

		loop {
			match current!(self) {
				Token::Keyword(keyword) => {
					if keyword == "and" {
						advance!(self);
						let second = self.time();
						if let Some(total) = first.checked_add(second) {
							first = total;
							if next!(self).is_none() {
								break;
							}
						} else {
							panic!();
						}
					} else {
						break;
					}
				},
				_ => break
			}
		}

		return first;
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
