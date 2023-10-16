// Authored in full by Steven Anmar Aziz
// Last Modified 10/15/2023

// This is a first attempt at building a parser in Rust and there are many possible improvements (especially with the output generation)
// This code is far from perfect

use core::cmp::PartialEq;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Eq)]
enum Flag {
    Scheme,
    Prolog,
    None,
}

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

// Lexer
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
                    panic!(
                        "\n\n; SYNTAX ERROR!\n; Expected '\"' after '{}'.\n\n",
                        lexeme
                    );
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

// Helper function
// Takes in vector of tokens and an index of the vector to return the Token at that index
fn get_next_token(index: usize, tokens: &Vec<Token>) -> Token {
    return tokens[index].clone();
}

// Helper function
// Takes two integers and panicks if one is smaller than the other, otherwise increments and returns smaller one
fn increment_i(i: usize, i_max: usize) -> usize {
    assert!(
        i < i_max - 1,
        "\n\n; SYNTAX ERROR!\n; Program incomplete!\n\n"
    );
    return i + 1;
}

// Parses RHS for the datadef rule of the grammar
// Takes in an integer, a vector of Tokens, and a flag and returns an integer corresponding to the index where datadef ends
fn datadef_parser(start_index: usize, tokens: &Vec<Token>, flag: &Flag) -> usize {
    let mut i: usize = start_index;
    let num_tokens: usize = tokens.len();
    let mut curr_token: Token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::ID,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );

    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::COLON,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );

    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::VECTOR || curr_token.token == TokenTypes::NUMBER,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    return i;
}

// Parses RHS for the inputop rule of the grammar
// Takes in an integer, a vector of Tokens, and a flag and returns an integer corresponding to the index where inputop ends
// Also prints Prolog/Scheme output depending on the flag
fn inputop_parser(start_index: usize, tokens: &Vec<Token>, flag: &Flag) -> usize {
    let mut i: usize = start_index;
    let num_tokens: usize = tokens.len();
    let mut curr_token: Token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::ID,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    let id: String = curr_token.lexeme;
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::ASSIGN,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::READ,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::LPAREN,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::STRING,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    let str: String = curr_token.lexeme;
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::COMMA,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::TRUE || curr_token.token == TokenTypes::FALSE,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    let bool: String = curr_token.lexeme;
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::COMMA,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::NUM,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    let num: String = curr_token.lexeme;
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::RPAREN,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);

    if flag == &Flag::Prolog {
        println!("   load_data_column({}, {}, {}, {}),", str, bool, num, id);
    } else if flag == &Flag::Scheme {
        println!(
            "(define {} (read-csv {} #{} {}))",
            id,
            str,
            bool.chars().nth(0).unwrap(),
            num
        );
    }

    return i;
}

// Parses RHS for the processop rule of the grammar
// Takes in an integer, a vector of Tokens, and a flag and returns an integer corresponding to the index where processop ends
// Also prints Prolog/Scheme output depending on the flag
fn processop_parser(start_index: usize, tokens: &Vec<Token>, flag: &Flag) -> usize {
    let mut i: usize = start_index;
    let num_tokens: usize = tokens.len();
    let mut curr_token: Token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::ID,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    let id: String = curr_token.lexeme;
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::ASSIGN,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, &tokens);

    if curr_token.token == TokenTypes::REGRESSIONA
        || curr_token.token == TokenTypes::REGRESSIONB
        || curr_token.token == TokenTypes::CORRELATION
    {
        let func: String = curr_token.lexeme;
        i = increment_i(i, num_tokens);
        curr_token = get_next_token(i, &tokens);
        assert!(
            curr_token.token == TokenTypes::LPAREN,
            "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
            curr_token.lexeme
        );

        i = increment_i(i, num_tokens);
        curr_token = get_next_token(i, &tokens);
        assert!(
            curr_token.token == TokenTypes::ID,
            "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
            curr_token.lexeme
        );
        let param1: String = curr_token.lexeme;

        i = increment_i(i, num_tokens);
        curr_token = get_next_token(i, &tokens);
        assert!(
            curr_token.token == TokenTypes::COMMA,
            "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
            curr_token.lexeme
        );

        i = increment_i(i, num_tokens);
        curr_token = get_next_token(i, &tokens);
        assert!(
            curr_token.token == TokenTypes::ID,
            "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
            curr_token.lexeme
        );
        let param2: String = curr_token.lexeme;

        i = increment_i(i, num_tokens);
        curr_token = get_next_token(i, &tokens);
        assert!(
            curr_token.token == TokenTypes::RPAREN,
            "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
            curr_token.lexeme
        );

        if flag == &Flag::Prolog {
            println!("   {}({}, {}, {}),", func, param1, param2, id);
        } else if flag == &Flag::Scheme {
            println!("(define {} ({} {} {}))", id, func, param1, param2);
        }
    } else if curr_token.token == TokenTypes::MEAN || curr_token.token == TokenTypes::STDDEV {
        let func: String = curr_token.lexeme;
        i = increment_i(i, num_tokens);
        curr_token = get_next_token(i, &tokens);
        assert!(
            curr_token.token == TokenTypes::LPAREN,
            "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
            curr_token.lexeme
        );

        i = increment_i(i, num_tokens);
        curr_token = get_next_token(i, &tokens);
        assert!(
            curr_token.token == TokenTypes::ID,
            "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
            curr_token.lexeme
        );
        let param1: String = curr_token.lexeme;

        i = increment_i(i, num_tokens);
        curr_token = get_next_token(i, &tokens);
        assert!(
            curr_token.token == TokenTypes::RPAREN,
            "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
            curr_token.lexeme
        );

        if flag == &Flag::Prolog {
            println!("   {}({}, {}),", func, param1, id);
        } else if flag == &Flag::Scheme {
            println!("(define {} ({} {}))", id, func, param1);
        }
    } else {
        panic!(
            "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
            curr_token.lexeme
        );
    }

    return i + 1;
}

// Parses RHS for the outputop rule of the grammar
// Takes in an integer, a vector of Tokens, and a flag and returns an integer corresponding to the index where output ends
// Also prints Prolog/Scheme output depending on the flag
fn outputop_parser(start_index: usize, tokens: &Vec<Token>, flag: &Flag) -> usize {
    let i: usize = start_index;
    let curr_token: Token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::STRING || curr_token.token == TokenTypes::ID,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    if flag == &Flag::Prolog {
        println!("   writeIn({}),", curr_token.lexeme);
    } else if flag == &Flag::Scheme {
        println!("(display {})\n(newline)", curr_token.lexeme);
    }
    return i + 1;
}

// Helper function
// This acts as parser for datadefs, inputops, processops, and outputops
// Takes in an integer, a vector of Tokens, a function, and a flag and returns an integer corresponding to the index where output ends
// This parser calls the appropriate subprograms and makes sure datadef, inputop, processop, and outputop nonterminals are comma-separated
fn special_parser(
    start_index: usize,
    tokens: &Vec<Token>,
    function: fn(usize, &Vec<Token>, &Flag) -> usize,
    flag: &Flag,
) -> usize {
    let mut i: usize = start_index;
    let num_tokens: usize = tokens.len();
    let mut curr_token: Token;

    i = function(i, &tokens, flag);

    curr_token = get_next_token(i, &tokens);

    while curr_token.token == TokenTypes::COMMA {
        i = increment_i(i, num_tokens);
        i = function(i, &tokens, flag);
        curr_token = get_next_token(i, &tokens);
    }
    return i;
}

// Parses RHS for the program rule of the grammar (start symbol, top of the parse tree)
// Takes in an integer, a vector of Tokens, and a flag and returns an integer corresponding to the index where inputop ends
// Also prints Prolog output depending on the flag
fn program_parser(start_index: usize, tokens: &Vec<Token>, flag: &Flag) {
    let mut i: usize = start_index;
    let num_tokens: usize = tokens.len();
    let mut curr_token: Token = get_next_token(i, &tokens);

    assert!(
        curr_token.token == TokenTypes::DATA,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, &tokens);
    assert!(
        curr_token.token == TokenTypes::COLON,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    i = special_parser(i, &tokens, datadef_parser, flag);

    curr_token = get_next_token(i, &tokens);
    assert!(
        curr_token.token == TokenTypes::INPUT,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, &tokens);
    assert!(
        curr_token.token == TokenTypes::COLON,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    if flag == &Flag::Prolog {
        println!("main :-");
    }
    i = special_parser(i, &tokens, inputop_parser, flag);

    curr_token = get_next_token(i, &tokens);
    assert!(
        curr_token.token == TokenTypes::PROCESS,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, &tokens);
    assert!(
        curr_token.token == TokenTypes::COLON,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    i = special_parser(i, &tokens, processop_parser, flag);

    curr_token = get_next_token(i, &tokens);
    assert!(
        curr_token.token == TokenTypes::OUTPUT,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, &tokens);
    assert!(
        curr_token.token == TokenTypes::COLON,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    i = special_parser(i, &tokens, outputop_parser, flag);

    curr_token = get_next_token(i, &tokens);
    assert!(
        curr_token.token == TokenTypes::END,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, &tokens);
    assert!(
        curr_token.token == TokenTypes::PERIOD,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );

    assert!(
        i == num_tokens - 1,
        "\n\n; SYNTAX ERROR!\n; Unexpected characters after 'end.'"
    );
}

// The main function receives/checks program parameters, opens and reads the input file, calls the lexer, calls the program parser, and prints messages for errors/completion
fn main() {
    let params: Vec<String> = env::args().collect();
    let mut flag: Flag = Flag::None;

    if params.len() == 1 {
        panic!("\n\n; No input file provided!\n\n");
    } else if params.len() == 2 {
        println!("\n; Processing input file '{}'.\n\n", params[1]);
    } else if params.len() == 3 {
        assert!(
            params[2] == "-p" || params[2] == "-s",
            "\n\n; Unrecognized input parameter '{}'!\n\n",
            params[2]
        );
        if params[2] == "-p" {
            flag = Flag::Prolog;
        } else {
            flag = Flag::Scheme;
        }
        println!("\n; Processing input file '{}'.\n\n", params[1]);
    } else {
        panic!("\n\n; Unrecognized input parameters!\n\n");
    }

    let mut input: File =
        File::open(&params[1]).expect("\n\n; FILE ERROR!\n; Could not open the file!\n\n");

    let mut contents: String = String::new();

    input
        .read_to_string(&mut contents)
        .expect("\n\n; FILE ERROR!\n; The contents of the file could not be read!\n\n");

    let tokens: Vec<Token> = lexer(&contents);
    program_parser(0, &tokens, &flag);
    println!("\n; Lexical and Syntax analysis passed.\n\n");
}