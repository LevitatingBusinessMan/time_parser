#[derive(Debug)]
pub enum Token {
	Number(i64),
	Unit(&'static str),
	Keyword(&'static str)
}

static UNITS: &'static [&str] = &["seconds", "minutes", "hours", "days", "weeks", "months", "years"];
static KEYWORDS: &'static [&str] = &["and"];

pub fn lex(source: &str) -> Vec<Token> {

	//TODO Should prob do a regex
	let mut split = source.split_whitespace();

	let mut tokens = Vec::<Token>::new();

	loop {
		if let Some(item) = split.next() {

			// Number
			if item.chars().all(|char| {char.is_ascii_digit()}) {
				tokens.push(Token::Number(item.parse::<i64>().unwrap()))
			}

			// Text
			else if item.chars().all(|char| {char.is_alphabetic()}) {

				//TODO add an "s" to the end if needed
				if UNITS.contains(&item) {
					let index = UNITS.iter().position(|&unit| unit == item).unwrap();
					tokens.push(Token::Unit(UNITS[index]))
				} else if KEYWORDS.contains(&item) {
					let index = KEYWORDS.iter().position(|&unit| unit == item).unwrap();
					tokens.push(Token::Keyword(KEYWORDS[index]))
				}
				else {
					panic!("Unknown text token");
				}
			}

			else {
				panic!("Some token word here")
			}


		} else {
			return tokens;
		}
	}

}
