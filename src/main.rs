// Authored in full by Steven Anmar Aziz
// Last Modified 10/15/2023

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum TokenTypes {
    DATA,
    INPUT,
    PROCESS,
    OUTPUT,
    END,
    ID,
    NUM,
    TRUE,
    FALSE,
    READ,
    COLON,
    COMMA,
    PERIOD,
    LPAREN,
    RPAREN,
    ASSIGN,
    VECTOR,
    NUMBER,
    REGRESSIONA,
    REGRESSIONB,
    MEAN,
    STDDEV,
    CORRELATION,
    STRING,
}

#[derive(Clone)]
struct Token {
    token: TokenTypes,
    lexeme: String,
}

// Takes String input and produces vector of Tokens (see Token struct)
fn lexer(input: &String) -> Vec<Token> {
    let mut i: usize = 0;
    let input_length: usize = input.len();
    let mut curr_char: char;
    let mut output: Vec<Token> = Vec::new();
    let special_lexeme: HashMap<char, TokenTypes> = HashMap::from([
        (':', TokenTypes::COLON),
        (',', TokenTypes::COMMA),
        ('.', TokenTypes::PERIOD),
        ('(', TokenTypes::LPAREN),
        (')', TokenTypes::RPAREN),
        ('=', TokenTypes::ASSIGN),
    ]);
    let reserved_lexeme: HashMap<&str, TokenTypes> = HashMap::from([
        ("data", TokenTypes::DATA),
        ("input", TokenTypes::INPUT),
        ("process", TokenTypes::PROCESS),
        ("output", TokenTypes::OUTPUT),
        ("end", TokenTypes::END),
        ("true", TokenTypes::TRUE),
        ("false", TokenTypes::FALSE),
        ("read", TokenTypes::READ),
        ("vector", TokenTypes::VECTOR),
        ("number", TokenTypes::NUMBER),
        ("regressiona", TokenTypes::REGRESSIONA),
        ("regressionb", TokenTypes::REGRESSIONB),
        ("mean", TokenTypes::MEAN),
        ("stddev", TokenTypes::STDDEV),
        ("correlation", TokenTypes::CORRELATION),
    ]);

    while i < input_length {
        curr_char = input.chars().nth(i).unwrap();

        if special_lexeme.contains_key(&curr_char) {
            output.push(Token {
                token: special_lexeme[&curr_char],
                lexeme: String::from(curr_char),
            });
            i += 1;
        } else if curr_char.is_ascii_lowercase() {
            let mut lexeme = String::new();
            lexeme.push(curr_char);
            i += 1;
            while i < input_length {
                curr_char = input.chars().nth(i).unwrap();
                if curr_char.is_ascii_lowercase() {
                    lexeme.push(curr_char);
                    i += 1;
                } else {
                    break;
                }
            }
            if reserved_lexeme.contains_key(lexeme.as_str()) {
                output.push(Token {
                    token: reserved_lexeme[lexeme.as_str()],
                    lexeme: lexeme,
                });
            } else {
                output.push(Token {
                    token: TokenTypes::ID,
                    lexeme: lexeme,
                });
            }
        } else if curr_char.is_ascii_digit() {
            let mut lexeme = String::new();
            lexeme.push(curr_char);
            i += 1;
            while i < input_length {
                curr_char = input.chars().nth(i).unwrap();
                if curr_char.is_ascii_digit() {
                    lexeme.push(curr_char);
                    i += 1;
                } else {
                    break;
                }
            }
            output.push(Token {
                token: TokenTypes::NUM,
                lexeme: lexeme,
            });
        } else if curr_char == '\"' {
            let mut lexeme = String::new();
            lexeme.push(curr_char);
            i += 1;
            while i < input_length {
                curr_char = input.chars().nth(i).unwrap();
                if curr_char.is_ascii_lowercase()
                    || curr_char.is_ascii_whitespace()
                    || curr_char.is_ascii_digit()
                    || curr_char == '.'
                    || curr_char == '='
                {
                    lexeme.push(curr_char);
                    i += 1;
                } else if curr_char == '\"' {
                    lexeme.push(curr_char);
                    i += 1;
                    break;
                } else {
                    panic!("\n\n; SYNTAX ERROR!\n; Expected '\"' after '{}'.\n\n", lexeme);
                }
            }
            output.push(Token {
                token: TokenTypes::STRING,
                lexeme: lexeme,
            });
        } else if curr_char.is_ascii_whitespace() {
            i += 1;
        } else {
            panic!(
                "\n\n; LEXICAL ERROR!\n; Unrecognized character '{}'.\n\n",
                curr_char
            );
        }
    }

    return output;
}

fn get_next_token(index: usize, tokens: &Vec<Token>) -> Token {
    return tokens[index].clone();
}

fn check_complete() {} // TODO

fn datadef_parser(start_index: usize, tokens: &Vec<Token>) -> usize {
    let mut i: usize = start_index;
    let mut curr_token: Token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::ID,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n",
        curr_token.lexeme
    );

    i += 1;
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::COLON,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );

    i += 1;
    curr_token = get_next_token(i, &tokens);

    if curr_token.token == TokenTypes::VECTOR || curr_token.token == TokenTypes::NUMBER {
        i += 1;
        return i;
    } else {
        panic!(
            "; SYNTAX ERROR!\n; Syntax error at '{}'.",
            curr_token.lexeme
        );
    }
}

fn inputop_parser(start_index: usize, tokens: &Vec<Token>) -> usize {
    let mut i: usize = start_index;
    let mut curr_token: Token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::ID,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );
    i += 1;
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::ASSIGN,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );
    i += 1;
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::READ,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );
    i += 1;
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::LPAREN,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );
    i += 1;
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::STRING,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );
    i += 1;
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::COMMA,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );
    i += 1;
    curr_token = get_next_token(i, &tokens);

    if curr_token.token == TokenTypes::TRUE || curr_token.token == TokenTypes::FALSE {
        i += 1;
        curr_token = get_next_token(i, &tokens);
    } else {
        panic!(
            "; SYNTAX ERROR!\n; Syntax error at '{}'.",
            curr_token.lexeme
        );
    }

    assert!(
        curr_token.token == TokenTypes::COMMA,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );
    i += 1;
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::NUM,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );
    i += 1;
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::RPAREN,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );
    i += 1;

    return i;
}

fn processop_parser(start_index: usize, tokens: &Vec<Token>) -> usize {
    let mut i: usize = start_index;
    let mut curr_token: Token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::ID,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );
    i += 1;
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::ASSIGN,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );
    i += 1;
    curr_token = get_next_token(i, &tokens);

    if curr_token.token == TokenTypes::REGRESSIONA
        || curr_token.token == TokenTypes::REGRESSIONB
        || curr_token.token == TokenTypes::CORRELATION
    {
        i += 1;
        curr_token = get_next_token(i, &tokens);
        assert!(
            curr_token.token == TokenTypes::LPAREN,
            "; SYNTAX ERROR!\n; Syntax error at '{}'.",
            curr_token.lexeme
        );

        i += 1;
        curr_token = get_next_token(i, &tokens);
        assert!(
            curr_token.token == TokenTypes::ID,
            "; SYNTAX ERROR!\n; Syntax error at '{}'.",
            curr_token.lexeme
        );

        i += 1;
        curr_token = get_next_token(i, &tokens);
        assert!(
            curr_token.token == TokenTypes::COMMA,
            "; SYNTAX ERROR!\n; Syntax error at '{}'.",
            curr_token.lexeme
        );

        i += 1;
        curr_token = get_next_token(i, &tokens);
        assert!(
            curr_token.token == TokenTypes::ID,
            "; SYNTAX ERROR!\n; Syntax error at '{}'.",
            curr_token.lexeme
        );

        i += 1;
        curr_token = get_next_token(i, &tokens);
        assert!(
            curr_token.token == TokenTypes::RPAREN,
            "; SYNTAX ERROR!\n; Syntax error at '{}'.",
            curr_token.lexeme
        );
    } else if curr_token.token == TokenTypes::MEAN || curr_token.token == TokenTypes::STDDEV {
        i += 1;
        curr_token = get_next_token(i, &tokens);
        assert!(
            curr_token.token == TokenTypes::LPAREN,
            "; SYNTAX ERROR!\n; Syntax error at '{}'.",
            curr_token.lexeme
        );

        i += 1;
        curr_token = get_next_token(i, &tokens);
        assert!(
            curr_token.token == TokenTypes::ID,
            "; SYNTAX ERROR!\n; Syntax error at '{}'.",
            curr_token.lexeme
        );

        i += 1;
        curr_token = get_next_token(i, &tokens);
        assert!(
            curr_token.token == TokenTypes::RPAREN,
            "; SYNTAX ERROR!\n; Syntax error at '{}'.",
            curr_token.lexeme
        );
    } else {
        panic!(
            "; SYNTAX ERROR!\n; Syntax error at '{}'.",
            curr_token.lexeme
        );
    }

    return i + 1;
}

fn outputop_parser(start_index: usize, tokens: &Vec<Token>) -> usize {
    let i: usize = start_index;
    let curr_token: Token = get_next_token(i, &tokens);

    if curr_token.token == TokenTypes::STRING || curr_token.token == TokenTypes::ID {
        return i + 1;
    } else {
        panic!(
            "; SYNTAX ERROR!\n; Syntax error at '{}'.",
            curr_token.lexeme
        );
    }
}

fn special_parser(
    start_index: usize,
    tokens: &Vec<Token>,
    function: fn(usize, &Vec<Token>) -> usize,
) -> usize {
    let mut i: usize = start_index;
    let mut curr_token: Token;

    i = function(i, &tokens);

    curr_token = get_next_token(i, &tokens);

    while curr_token.token == TokenTypes::COMMA {
        i += 1;
        i = function(i, &tokens);
        curr_token = get_next_token(i, &tokens);
    }
    return i;
}

fn program_parser(start_index: usize, tokens: &Vec<Token>) -> bool {
    let mut i: usize = start_index;
    let mut curr_token: Token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::DATA,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );
    i += 1;
    curr_token = get_next_token(i, &tokens);
    assert!(
        curr_token.token == TokenTypes::COLON,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );
    i += 1;
    i = special_parser(i, &tokens, datadef_parser);

    curr_token = get_next_token(i, &tokens);
    assert!(
        curr_token.token == TokenTypes::INPUT,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );
    i += 1;
    curr_token = get_next_token(i, &tokens);
    assert!(
        curr_token.token == TokenTypes::COLON,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );
    i += 1;
    i = special_parser(i, &tokens, inputop_parser);

    curr_token = get_next_token(i, &tokens);
    assert!(
        curr_token.token == TokenTypes::PROCESS,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );
    i += 1;
    curr_token = get_next_token(i, &tokens);
    assert!(
        curr_token.token == TokenTypes::COLON,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );
    i += 1;
    i = special_parser(i, &tokens, processop_parser);

    curr_token = get_next_token(i, &tokens);
    assert!(
        curr_token.token == TokenTypes::OUTPUT,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );
    i += 1;
    curr_token = get_next_token(i, &tokens);
    assert!(
        curr_token.token == TokenTypes::COLON,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );
    i += 1;
    i = special_parser(i, &tokens, outputop_parser);

    curr_token = get_next_token(i, &tokens);
    assert!(
        curr_token.token == TokenTypes::END,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );
    i += 1;
    curr_token = get_next_token(i, &tokens);
    assert!(
        curr_token.token == TokenTypes::PERIOD,
        "; SYNTAX ERROR!\n; Syntax error at '{}'.",
        curr_token.lexeme
    );

    return true;
}

fn main() {
    let params: Vec<String> = env::args().collect();

    if params.len() == 1 {
        panic!("; No input file provided!");
    } else if params.len() == 2 {
        println!("; Processing input file '{}'.", params[1]);
    } else if params.len() == 3 {
        if params[2] != "-p" && params[2] != "-s" {
            panic!("; Unrecognized input parameter '{}'!", params[2]);
        }
        println!("; Processing input file '{}'.", params[1]);
    } else {
        panic!("; Unrecognized program parameters!");
    }

    let mut input = File::open(&params[1]).expect("; FILE ERROR!\n; Could not open the file!");

    let mut contents = String::new();

    input
        .read_to_string(&mut contents)
        .expect("; FILE ERROR!\n; The contents of the file could not be read!");

    let tokens: Vec<Token> = lexer(&contents);

    if program_parser(0, &tokens) {
        println!("; Lexical and Syntax analysis passed.");
    } else {
        println!("; Lexical and Syntax analysis failed.");
    }

    // don't forget to check bounds of i
}

// fn inputops_parser(start_index : usize, tokens : &Vec<Token>) -> usize {
//     let mut i : usize = start_index;
//     let mut curr_token : Token;

//     i = inputop_parser(i, &tokens);

//     curr_token = get_next_token(i, tokens);

//     while curr_token.token == TokenTypes::COMMA {
//         i+=1;
//         i = inputop_parser(i, tokens);
//         curr_token = get_next_token(i, tokens);
//     }
//     return i;
// }

// fn datadefs_parser(start_index : usize, tokens : &Vec<Token>) -> usize {
//     let mut i : usize = start_index;
//     let mut curr_token : Token;

//     i = datadef_parser(i, &tokens);

//     curr_token = get_next_token(i, tokens);

//     while curr_token.token == TokenTypes::COMMA {
//         i+=1;
//         i = datadef_parser(i, tokens);
//         curr_token = get_next_token(i, tokens);
//     }
//     return i;
// }
