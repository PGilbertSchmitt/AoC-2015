use std::{collections::HashSet, fmt::Display};
use logos::{Lexer, Logos};

use crate::util::Pair;

#[derive(Logos)]
#[logos(skip r"[ ,\n]", skip r"through")]
enum Token {
    // Actions

    #[token("turn off")]
    TurnOff,

    #[token("turn on")]
    TurnOn,

    #[token("toggle")]
    Toggle,

    // Other

    #[regex("[0-9]+", |lex| lex.slice().parse::<isize>().unwrap())]
    Integer(isize),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TurnOff => f.write_str("Turn Off"),
            Self::TurnOn => f.write_str("Turn On"),
            Self::Toggle => f.write_str("Toggle"),
            Self::Integer(value) => f.write_fmt(format_args!("Integer({value})")),
        }
    }
}

#[derive(Debug)]
enum Action {
    TurnOff,
    TurnOn,
    Toggle,
}

impl From<Token> for Action {
    fn from(value: Token) -> Self {
        match value {
            Token::TurnOff => Action::TurnOff,
            Token::TurnOn => Action::TurnOn,
            Token::Toggle => Action::Toggle,
            _ => panic!("Attempted to read action from number, parser was desynced"),
        }
    }
}

impl From<Token> for isize {
    fn from(value: Token) -> Self {
        match value {
            Token::Integer(value) => value,
            _ => panic!("Attempted to read number from action, parser was desynced"),
        }
    }
}

#[derive(Debug)]
struct Command {
    action: Action,
    start: Pair,
    end: Pair,
}

fn read_line(lexer: &mut Lexer<'_, Token>) -> Option<Command> {
    if let Some(action) = lexer.next() {
        Some(Command {
            action: action.unwrap().into(),
            start: Pair (
                lexer.next().unwrap().unwrap().into(),
                lexer.next().unwrap().unwrap().into(),
            ),
            end: Pair (
                lexer.next().unwrap().unwrap().into(),
                lexer.next().unwrap().unwrap().into(),
            ),
        })
    } else {
        None
    }
}

fn process_instructions(input: &'static str) -> Vec<Command> {
    let mut lex = Token::lexer(input);
    let mut commands = Vec::<Command>::new();
    while let Some(command) = read_line(&mut lex) {
        commands.push(command);
    }
    commands
}

// Iterate through the commands (which should have be in reverse order)
// until we either reach a terminal action (turn-on or turn-off) or the
// end of the list (which means off by default). For every toggle we reach,
// we invert the toggle state, and when we return, we invert if the toggle
// is active. 
fn is_lit<'a, I>(commands: I, x: isize, y: isize) -> bool
where
    I: Iterator<Item = &'a Command>
{
    let mut toggled = false;
    for Command { action, start, end } in commands {
        if start.0 <= x && x <= end.0 && start.1 <= y && y <= end.1 {
            match action {
                Action::TurnOn => return true != toggled,
                Action::TurnOff => return false != toggled,
                Action::Toggle => {
                    toggled = !toggled;
                }
            }
        }
    }
    false != toggled
}

pub fn get_lit_lights(input: &'static str) -> isize {
    let commands = process_instructions(input);
    let mut x_boundaries = HashSet::<isize>::new();
    let mut y_boundaries = HashSet::<isize>::new();
    x_boundaries.insert(0);
    y_boundaries.insert(0);

    for command in commands.iter() {
        x_boundaries.insert(command.start.0);
        x_boundaries.insert(command.end.0 + 1);
        y_boundaries.insert(command.start.1);
        y_boundaries.insert(command.end.1 + 1);
    };

    let mut x_boundaries: Vec<isize> = x_boundaries.into_iter().collect();
    let mut y_boundaries: Vec<isize> = y_boundaries.into_iter().collect();
    x_boundaries.sort();
    y_boundaries.sort();

    let mut on_count: isize = 0;
    
    let mut xs = x_boundaries.iter().peekable();
    while let Some(cur_x) = xs.next() {
        let next_x = xs.peek().unwrap_or(&&1000);
        let x_len = **next_x - *cur_x;

        let mut ys = y_boundaries.iter().peekable();
        while let Some(cur_y) = ys.next() {
            let next_y = ys.peek().unwrap_or(&&1000);
            let y_len = **next_y - *cur_y;
            let block_size = x_len * y_len;

            let cs = commands.iter().rev();
            if is_lit(cs, *cur_x, *cur_y) {
                on_count += block_size
            }
        }
    }

    on_count
}

#[cfg(test)]
mod tests {
    use core::str;

    use super::*;

    static SAMPLE: &'static str = "turn on 2,2 through 3,3\ntoggle 2,3 through 4,5";
    static INPUT: &'static str = include_str!("./inputs/day6.txt");

    #[test]
    fn try_this() {
        assert_eq!(9, get_lit_lights(SAMPLE));
        assert_eq!(400_410, get_lit_lights(INPUT));
    }
}
