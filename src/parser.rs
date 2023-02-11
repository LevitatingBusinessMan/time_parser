//http://craftinginterpreters.com/parsing-expressions.html

use crate::lexer::Token;

/*

We have 3 types
duration: "3 hours" or "2 minutes"
date: "23 januari" or "tuesday"
datetime: "friday at 02:00" or "7 march 2008 at 06:00"
time: "22:00" or "01:25"

We have the higher precedence date expressions (which return a date)
<duration> before <datetime>
<duration> after <datetime>

And the lower precedence time expressions (which return a time)
<duration> and <duration>
*/

/*
datetime_expression -> (duration_expression ("before" | "after"))? datetime
datetime -> date ("at" time)?
time -> hh:mm
date -> ((n day)? (month)? (year)?) | weekday
duration_expression -> addition
addition -> duration ("and" duration)*
duration -> n unit
unit -> "seconds" | "minutes" | "hours" | "days" | "weeks" | "months" | "years"
*/

type Duration = std::time::Duration;
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
	Duration(Duration),
}

impl Parser {

	fn parse(&mut self) -> Option<Date> {
		let duration = self.duration_expression();
		let now = Date::now();
		return now.checked_add(duration)
	}

	fn duration_expression(&mut self) -> Duration {
		self.addition()
	}

	fn addition(&mut self) -> Duration {
		let mut first = self.duration();

		//Reached end
		if self.current >= self.tokens.len() {
			return first;
		}

		loop {
			match current!(self) {
				Token::Keyword(keyword) => {
					if keyword == "and" {
						advance!(self);
						let second = self.duration();
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

	fn duration(&mut self) -> Duration {

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
					
					return Duration::new(seconds, 0);
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
