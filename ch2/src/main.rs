use std::fs::File;
use std::io::BufRead;
use std::iter::Peekable;
use std::str::Chars;

/// Trying to build a lexer and parser here for the input.
///
/// The first pass is just lexing a line into tokens that we care about, and then
/// the parser is run on that line of tokens to figure out what happened in the game.

fn main() {
    // Read input
    let f = File::open("./ch2/input.txt").unwrap();

    // Line by line, lex tokens -> parser -> Create a Game
    let total = std::io::BufReader::new(f)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| lex(&line))
        .map(|tokens| parse(tokens))
        // Part 1:
        //.filter(|game| game.within_bounds(12, 13, 14))
        //.fold(0, |acc, e| acc + e.id);
        .map(|g| g.power())
        .reduce(|acc, e| acc + e);

    println!("total: {}", total.unwrap());
}

// GameInstance is a line from the input and has a certain set of draws in it.
#[derive(Debug, PartialEq)]
struct GameInstance {
    id: u32,
    draws: Vec<Draw>,
}

// A Draw is a number of cubes that were pulled out at a time.
#[derive(Debug, PartialEq)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl GameInstance {
    // Part 1: Checks if the game is within the bounds of colored balls.
    fn within_bounds(&self, red: u32, green: u32, blue: u32) -> bool {
        for d in &self.draws {
            if d.red > red || d.green > green || d.blue > blue {
                return false;
            }
        }

        true
    }

    // Part 2: The power of a game is the max of each color from the draws
    // then multiplied together.
    fn power(&self) -> u32 {
        let (mut r_max, mut b_max, mut g_max) = (0, 0, 0);

        for d in &self.draws {
            if d.red > r_max {
                r_max = d.red
            }
            if d.blue > b_max {
                b_max = d.blue
            }
            if d.green > g_max {
                g_max = d.green
            }
        }

        r_max * b_max * g_max
    }
}

impl Default for Draw {
    fn default() -> Self {
        Draw {
            red: 0,
            blue: 0,
            green: 0,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Ident(Identifier),
    Number(u32),
    Semicolon,
    Colon,
    Space,
    Comma,
}

#[derive(Debug, PartialEq)]
enum Identifier {
    Game,
    Red,
    Blue,
    Green,
}

// Consumes the lexer into its tokens.
fn lex(s: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.peek() {
        if let Some(token) = match c {
            ' ' => Some(Token::Space),
            ',' => Some(Token::Comma),
            ':' => Some(Token::Colon),
            ';' => Some(Token::Semicolon),
            _ => None,
        } {
            tokens.push(token);
            chars.next(); // Advance the iterator
            continue;
        }

        if c.is_alphabetic() {
            // Lex an identifier
            let ident = read_ident(&mut chars);
            tokens.push(Token::Ident(ident));
            continue;
        }
        if c.is_numeric() {
            // Lex a number
            let num = read_number(&mut chars);
            tokens.push(Token::Number(num));
            continue;
        }

        unreachable!("got an unexpected: {}", c)
    }

    tokens
}

// Reads from the current position until the next non-alphabetical character.
fn read_ident(cs: &mut Peekable<Chars>) -> Identifier {
    let mut buffer = String::new();

    // Read until the next non-alphabetical character, filling the buffer.
    while let Some(c) = cs.peek() {
        if !c.is_alphabetic() {
            break;
        }

        buffer.push(cs.next().unwrap());
    }

    match buffer.as_str() {
        "Game" => Identifier::Game,
        "red" => Identifier::Red,
        "blue" => Identifier::Blue,
        "green" => Identifier::Green,
        _ => panic!("unknown identifier '{}'", buffer),
    }
}

// Reads from the current position until the next non-numeric character.
fn read_number(cs: &mut Peekable<Chars>) -> u32 {
    let mut buffer = String::new();

    // Read until the next non-numeric character, filling the buffer.
    while let Some(c) = cs.peek() {
        if !c.is_digit(10) {
            break;
        }

        buffer.push(cs.next().unwrap());
    }

    // Convert the buffer to an ident
    buffer.parse().unwrap()
}

fn parse(ts: Vec<Token>) -> GameInstance {
    let mut tokens = ts.into_iter().filter(|token| match token {
        Token::Comma | Token::Colon | Token::Space => false,
        _ => true,
    });
    let mut g = GameInstance {
        id: 0,
        draws: vec![],
    };

    let mut draw: Draw = Default::default();
    let mut processed = vec![];
    while let Some(token) = tokens.next() {
        match token {
            Token::Ident(Identifier::Game) => {
                // The next number is the game id
                let id = match tokens.next().unwrap() {
                    Token::Number(num) => num,
                    anything => panic!(
                        "expected number, instead got: {:?}. Processed: {:?}",
                        anything, processed
                    ),
                };
                g.id = id;
                continue;
            }
            Token::Number(num) => {
                // The next token should be a color
                match tokens.next().unwrap() {
                    Token::Ident(ident) => match ident {
                        Identifier::Red => draw.red += num,
                        Identifier::Blue => draw.blue += num,
                        Identifier::Green => draw.green += num,
                        _ => panic!(
                            "expected a color, instead got: {:?}. Processed: {:?}",
                            ident, processed
                        ),
                    },
                    anything => {
                        panic!(
                            "was expecting a identifier, instead got: {:?}. Processed: {:?}",
                            anything, processed
                        )
                    }
                };
            }
            Token::Semicolon => {
                // Semicolon means that the draw is done and the next one begins.
                g.draws.push(draw);
                draw = Default::default();
            }
            _ => panic!("unexpected token: {:?}. Processed: {:?}", token, processed),
        }

        processed.push(token);
    }
    g.draws.push(draw);

    g
}

#[cfg(test)]
mod test {
    use super::Identifier::*;
    use super::Token::*;
    use super::*;

    #[test]
    fn read_ident_test() {
        let mut cs = "Game ".chars().peekable();
        let got = read_ident(&mut cs);
        assert_eq!(got, Game);

        let mut cs = "Game".chars().peekable();
        let got = read_ident(&mut cs);
        assert_eq!(got, Game);
    }

    #[test]
    #[should_panic]
    fn read_bad_ident() {
        let mut cs = "game".chars().peekable();
        read_ident(&mut cs);
    }

    #[test]
    fn read_number_test() {
        let mut cs = "7".chars().peekable();
        let got = read_number(&mut cs);
        assert_eq!(got, 7);

        let mut cs = "8 9".chars().peekable();
        let got = read_number(&mut cs);
        assert_eq!(got, 8);

        let mut cs = "89".chars().peekable();
        let got = read_number(&mut cs);
        assert_eq!(got, 89);
    }

    #[test]
    fn lex_test() {
        let want: Vec<Token> = vec![
            Ident(Game),
            Number(1),
            Colon,
            Number(3),
            Ident(Blue),
            Number(4),
            Ident(Red),
            Semicolon,
            Number(1),
            Ident(Red),
            Number(2),
            Ident(Green),
            Number(6),
            Ident(Blue),
            Semicolon,
            Number(2),
            Ident(Green),
        ];
        let got = lex("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let useful: Vec<_> = got
            .into_iter() // Gets an iterator of T, not just &T
            .filter(|i| match i {
                Space | Comma => false,
                _ => true,
            })
            .collect();

        assert_eq!(want, useful);
    }

    #[test]
    fn parse_test() {
        let input = vec![
            Ident(Game),
            Number(1),
            Colon,
            Number(3),
            Ident(Blue),
            Number(4),
            Ident(Red),
            Semicolon,
            Number(1),
            Ident(Red),
            Number(2),
            Ident(Green),
            Number(6),
            Ident(Blue),
            Semicolon,
            Number(2),
            Ident(Green),
        ];
        let want = GameInstance {
            id: 1,
            draws: vec![
                Draw {
                    red: 4,
                    blue: 3,
                    green: 0,
                },
                Draw {
                    red: 1,
                    blue: 6,
                    green: 2,
                },
                Draw {
                    red: 0,
                    blue: 0,
                    green: 2,
                },
            ],
        };

        let got = parse(input);
        assert_eq!(want, got)
    }
}
