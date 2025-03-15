use logos::Logos;
use std::collections::{hash_map::Entry, HashMap};

#[derive(Logos)]
#[logos(skip "[ =>\n]")]
enum Token {
  #[regex("[A-Za-z]+", |lex| lex.slice().to_owned())]
  Word(String),
}

impl Token {
  fn take(self) -> String {
    // I'll be honest, this feels dumb
    match self {
      Self::Word(word) => word,
    }
  }
}

fn build_transform_map(input: &str) -> (String, HashMap<String, Vec<String>>) {
  let mut h: HashMap<String, Vec<String>> = HashMap::new();

  let mut parts = input.trim().split("\n\n");
  let mappings = parts.next().unwrap();
  let init_sequence = parts.next().unwrap();

  let mut lex = Token::lexer(mappings);
  while let Some(tok) = lex.next() {
    let key = tok.unwrap().take();
    let val = lex.next().unwrap().unwrap().take();
    match h.entry(key) {
      Entry::Occupied(mut e) => {
        e.get_mut().push(val);
      }
      Entry::Vacant(e) => {
        e.insert(vec![val]);
      }
    }
  }

  (init_sequence.to_owned(), h)
}

const INPUT: &str = include_str!("./inputs/day19.txt");

#[test]
fn part_1() {
  println!("{:?}", build_transform_map(INPUT).1);
}
