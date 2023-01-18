#![allow(clippy::missing_docs_in_private_items)]
use conversions_rust_lib::ToUnicodeVec;
use std::str;

#[derive(Debug)]
struct CollectQoutes<'a> {
    state: State,
    slices: Vec<(usize, usize)>,
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
            slices: vec![],
            raw_str: raw_str.to_characters().collect::<Vec<&'a str>>(),
        }
    }
    fn process(&mut self) {
        let mut quote_index = 0;
        for (i, &str_char) in self.raw_str.iter().enumerate() {
            if str_char == "\"" {
                self.state = match &self.state {
                    State::None | State::RightHandQuote => {
                        quote_index = i;
                        State::LeftHandQuote
                    }
                    State::LeftHandQuote => {
                        self.slices.push((quote_index, i));
                        State::RightHandQuote
                    }
                }
            }
        }
    }
    fn print_results(&self) {
        for (i, &(start, end)) in self.slices.iter().enumerate() {
            let slice = &self.raw_str[start..=end].join("");
            println!("QUOTE #{} of {}: {slice}", i+1, self.slices.len());
        }
    }
    fn return_results(&self) -> impl Iterator<Item = String> + '_ {
        self.slices.iter().map(|&(start, end)|String::from(&self.raw_str[start..=end].join("")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_left_quote() {
        let test_str = "Marcus said, \"Yo! Have you eaten? \". I replied, \"Not yet. I am currently looking for food now. How about you?\"";
        let mut quote_collector = CollectQoutes::new(test_str);
        quote_collector.process();
        println!("{quote_collector:?}" );
        quote_collector.print_results();
        let results = quote_collector.return_results().collect::<Vec<String>>();
        println!("{results:?}");
    }
}
