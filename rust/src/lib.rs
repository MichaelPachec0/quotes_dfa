use conversions_rust_lib::ToUnicodeVec;
use std::str;

#[derive(Debug)]
struct CollectQoutes<'a> {
    state: State,
    quotes: Vec<String>,

    raw_str: Vec<&'a str>,
}

#[derive(Debug)]
enum State {
    None,
    RightHandQuote,
    LeftHandQuote,
}

impl<'a> CollectQoutes<'a> {
    fn new(raw_str: &'a str) -> Self {
        Self {
            state: State::None,
            quotes: vec![],
            raw_str: raw_str.to_characters().collect::<Vec<&'a str>>(),
        }
    }
    fn process(&mut self) -> Result<(), liberr::Err> {
        let mut quote_index = 0;
        for (i, &str_char) in self.raw_str.iter().enumerate() {
            if str_char == "\"" {
                self.state = match &self.state {
                    State::None | State::RightHandQuote => {
                        quote_index = i;
                        State::LeftHandQuote
                    }
                    State::LeftHandQuote => {
                        self.quotes
                            .push(String::from(&self.raw_str[quote_index..=i].join("")));
                        State::RightHandQuote
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_left_quote() -> Result<(), liberr::Err> {
        let test_str = "Marcus said, \"Yo! Have you eaten? \". I replied, \"Not yet. I am currently looking for food now. How about you?\"";
        let mut dfa = CollectQoutes::new(test_str);
        dfa.process()?;
        println!("{:?}", dfa);
        Ok(())
    }
}
