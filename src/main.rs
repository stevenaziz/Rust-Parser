// Authored by Steven Anmar Aziz
// Last Modified 10/17/2023

// The variable 'i' will be used throughout this source code as an integer iterator

use core::cmp::PartialEq;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

// Flag enum will be used to indicate whether Prolog or Scheme output is requested
#[derive(PartialEq, Eq)]
enum Flag {
    Scheme,
    Prolog,
    None,
}

// TokenTypes enum will be used to store a token type and to compare tokens
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

// Token struct will be used to store each token in a vector
#[derive(Clone)]
struct Token {
    token: TokenTypes,
    lexeme: String,
}

// Lexer function
// Takes String input and produces vector of Tokens
// If lexical or syntax errors are found, function panics
fn lexer(input: String) -> Vec<Token> {
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
// Takes an unsigened integer and a vector of Tokens and returns the Token in the vector at the index of the integer
fn get_next_token(index: usize, tokens: &Vec<Token>) -> Token {
    return tokens[index].clone();
}

// Helper function
// Takes two integers, i and i_max, and returns i incremented by 1
// Function panics if i is greater than i_max - 1
fn increment_i(i: usize, i_max: usize) -> usize {
    assert!(
        i < i_max - 1,
        "\n\n; SYNTAX ERROR!\n; Program incomplete!\n\n"
    );
    return i + 1;
}

// DataDef Parser
// Parses RHS for the datadef rule of the grammar
// Takes an unsigned integer, a vector of Tokens, a Flag (which is thrown away), and a String
// Returns a tuple with an integer and a String
// Function panics if syntax errors are found
fn datadef_parser(
    start_index: usize,
    tokens: &Vec<Token>,
    _: &Flag,
    prog_output: String,
) -> (usize, String) {
    let mut i: usize = start_index;
    let num_tokens: usize = tokens.len();
    let mut curr_token: Token = get_next_token(i, tokens);

    assert!(
        curr_token.token == TokenTypes::ID,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );

    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, tokens);

    assert!(
        curr_token.token == TokenTypes::COLON,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );

    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, tokens);

    assert!(
        curr_token.token == TokenTypes::VECTOR || curr_token.token == TokenTypes::NUMBER,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    return (i, prog_output);
}

// InputOp Parser
// Parses RHS for the inputop rule of the grammar
// Takes an unsigned integer, a vector of Tokens, a Flag, and a String
// Returns a tuple with an integer and a String
// Function panics if syntax errors are found
fn inputop_parser(
    start_index: usize,
    tokens: &Vec<Token>,
    flag: &Flag,
    mut prog_output: String,
) -> (usize, String) {
    let mut i: usize = start_index;
    let num_tokens: usize = tokens.len();
    let mut curr_token: Token = get_next_token(i, tokens);

    assert!(
        curr_token.token == TokenTypes::ID,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    let id: String = curr_token.lexeme;
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, tokens);

    assert!(
        curr_token.token == TokenTypes::ASSIGN,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, tokens);

    assert!(
        curr_token.token == TokenTypes::READ,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, tokens);

    assert!(
        curr_token.token == TokenTypes::LPAREN,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, tokens);

    assert!(
        curr_token.token == TokenTypes::STRING,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    let str: String = curr_token.lexeme;
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, tokens);

    assert!(
        curr_token.token == TokenTypes::COMMA,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, tokens);

    assert!(
        curr_token.token == TokenTypes::TRUE || curr_token.token == TokenTypes::FALSE,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    let mut bool: String = curr_token.lexeme;
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, tokens);

    assert!(
        curr_token.token == TokenTypes::COMMA,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, tokens);

    assert!(
        curr_token.token == TokenTypes::NUM,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    let num: String = curr_token.lexeme;
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, tokens);

    assert!(
        curr_token.token == TokenTypes::RPAREN,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);

    if flag == &Flag::Prolog {
        prog_output
            .push_str(format!("\n   load_data_column({str}, {bool}, {num}, {id}),").as_str());
    } else if flag == &Flag::Scheme {
        bool = String::from(bool.chars().nth(0).unwrap());
        prog_output.push_str(format!("(define {id} (read-csv {str} #{bool} {num}))\n").as_str());
    }

    return (i, prog_output);
}

// ProcessOp Parser
// Parses RHS for the processop rule of the grammar
// Takes an unsigned integer, a vector of Tokens, a Flag, and a String
// Returns a tuple with an integer and a String
// Function panics if syntax errors are found
fn processop_parser(
    start_index: usize,
    tokens: &Vec<Token>,
    flag: &Flag,
    mut prog_output: String,
) -> (usize, String) {
    let mut i: usize = start_index;
    let num_tokens: usize = tokens.len();
    let mut curr_token: Token = get_next_token(i, tokens);

    assert!(
        curr_token.token == TokenTypes::ID,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    let id: String = curr_token.lexeme;
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, tokens);

    assert!(
        curr_token.token == TokenTypes::ASSIGN,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    i = increment_i(i, num_tokens);
    curr_token = get_next_token(i, tokens);

    let func: String = curr_token.lexeme;
    if curr_token.token == TokenTypes::REGRESSIONA
        || curr_token.token == TokenTypes::REGRESSIONB
        || curr_token.token == TokenTypes::CORRELATION
    {
        i = increment_i(i, num_tokens);
        curr_token = get_next_token(i, tokens);
        assert!(
            curr_token.token == TokenTypes::LPAREN,
            "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
            curr_token.lexeme
        );

        i = increment_i(i, num_tokens);
        curr_token = get_next_token(i, tokens);
        assert!(
            curr_token.token == TokenTypes::ID,
            "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
            curr_token.lexeme
        );
        let param1: String = curr_token.lexeme;

        i = increment_i(i, num_tokens);
        curr_token = get_next_token(i, tokens);
        assert!(
            curr_token.token == TokenTypes::COMMA,
            "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
            curr_token.lexeme
        );

        i = increment_i(i, num_tokens);
        curr_token = get_next_token(i, tokens);
        assert!(
            curr_token.token == TokenTypes::ID,
            "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
            curr_token.lexeme
        );
        let param2: String = curr_token.lexeme;

        i = increment_i(i, num_tokens);
        curr_token = get_next_token(i, tokens);
        assert!(
            curr_token.token == TokenTypes::RPAREN,
            "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
            curr_token.lexeme
        );

        if flag == &Flag::Prolog {
            prog_output.push_str(format!("\n   {func}({param1}, {param2}, {id}),").as_str());
        } else if flag == &Flag::Scheme {
            prog_output.push_str(format!("(define {id} ({func} {param1} {param2}))\n").as_str());
        }
    } else if curr_token.token == TokenTypes::MEAN || curr_token.token == TokenTypes::STDDEV {
        i = increment_i(i, num_tokens);
        curr_token = get_next_token(i, tokens);
        assert!(
            curr_token.token == TokenTypes::LPAREN,
            "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
            curr_token.lexeme
        );

        i = increment_i(i, num_tokens);
        curr_token = get_next_token(i, tokens);
        assert!(
            curr_token.token == TokenTypes::ID,
            "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
            curr_token.lexeme
        );
        let param1: String = curr_token.lexeme;

        i = increment_i(i, num_tokens);
        curr_token = get_next_token(i, tokens);
        assert!(
            curr_token.token == TokenTypes::RPAREN,
            "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
            curr_token.lexeme
        );

        if flag == &Flag::Prolog {
            prog_output.push_str(format!("\n   {func}({param1}, {id}),").as_str());
        } else if flag == &Flag::Scheme {
            prog_output.push_str(format!("(define {id} ({func} {param1}))\n").as_str());
        }
    } else {
        panic!(
            "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
            func // This is 'curr_token.lexeme,' renamed
        );
    }

    return (i + 1, prog_output);
}

// OutputOp Parser
// Parses RHS for the outputop rule of the grammar
// Takes an unsigned integer, a vector of Tokens, a Flag, and a String
// Returns a tuple with an integer and a String
// Function panics if syntax errors are found
fn outputop_parser(
    start_index: usize,
    tokens: &Vec<Token>,
    flag: &Flag,
    mut prog_output: String,
) -> (usize, String) {
    let i: usize = start_index;
    let curr_token: Token = get_next_token(i, tokens);

    assert!(
        curr_token.token == TokenTypes::STRING || curr_token.token == TokenTypes::ID,
        "\n\n; SYNTAX ERROR!\n; Syntax error at '{}'.\n\n",
        curr_token.lexeme
    );
    let str_or_id = curr_token.lexeme;
    if flag == &Flag::Prolog {
        prog_output.push_str(format!("\n   writeIn({str_or_id}),").as_str());
    } else if flag == &Flag::Scheme {
        prog_output.push_str(format!("(display {str_or_id})\n(newline)\n").as_str());
    }
    return (i + 1, prog_output);
}

// Special helper function
// Parses comma-spearated datadef,inputop, processop, or outputop nonterminals of the grammar
// Takes an unsigned integer, a vector of Tokens, a function (datadef, inputop, processop, or outputop), a Flag, and a String
// Returns a tuple with an integer and a String
fn special_parser(
    start_index: usize,
    tokens: &Vec<Token>,
    function: fn(usize, &Vec<Token>, &Flag, String) -> (usize, String),
    flag: &Flag,
    mut prog_output: String,
) -> (usize, String) {
    let mut i: usize = start_index;
    let num_tokens: usize = tokens.len();
    let mut curr_token: Token;

    (i, prog_output) = function(i, tokens, flag, prog_output);

    curr_token = get_next_token(i, tokens);

    while curr_token.token == TokenTypes::COMMA {
        i = increment_i(i, num_tokens);
        (i, prog_output) = function(i, tokens, flag, prog_output);
        curr_token = get_next_token(i, tokens);
    }
    return (i, prog_output);
}

// Program Parser
// Parses RHS for the program rule of the grammar
// Takes a vector of Tokens, a Flag, and a String
// Returns a String corresponding to the requested Prolog/Scheme output
// Function panics if syntax errors are found
fn program_parser(tokens: Vec<Token>, flag: Flag, mut prog_output: String) -> String {
    let mut i: usize = 0;
    let num_tokens: usize = tokens.len();
    let mut curr_token: Token = get_next_token(i, &tokens);
    if flag == Flag::Prolog {
        prog_output.push_str("main :-");
    }
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
    (i, prog_output) = special_parser(i, &tokens, datadef_parser, &flag, prog_output);

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
    (i, prog_output) = special_parser(i, &tokens, inputop_parser, &flag, prog_output);

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
    (i, prog_output) = special_parser(i, &tokens, processop_parser, &flag, prog_output);

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
    (i, prog_output) = special_parser(i, &tokens, outputop_parser, &flag, prog_output);

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
    if flag == Flag::Prolog {
        prog_output.pop();
        prog_output.push('.');
    }

    assert!(
        i == num_tokens - 1,
        "\n\n; SYNTAX ERROR!\n; Unexpected characters after 'end.'"
    );
    return prog_output;
}

// Main
// Receives and checks program parameters, opens and reads the input file, calls the lexer, calls the program parser, and prints the requested output (if any)
// Function panics if any errors are found
fn main() {
    let prog_params: Vec<String> = env::args().collect();
    let mut flag: Flag = Flag::None;
    let mut prog_output: String = String::new();

    if prog_params.len() == 1 {
        panic!("\n\n; No input file provided!\n\n");
    } else if prog_params.len() == 2 {
        println!("\n; Processing input file '{}'.\n", prog_params[1]);
    } else if prog_params.len() == 3 {
        assert!(
            prog_params[2] == "-p" || prog_params[2] == "-s",
            "\n\n; Unrecognized input parameter '{}'!\n\n",
            prog_params[2]
        );
        if prog_params[2] == "-p" {
            flag = Flag::Prolog;
        } else {
            flag = Flag::Scheme;
        }
        println!("\n; Processing input file '{}'.\n", prog_params[1]);
    } else {
        panic!("\n\n; Unrecognized input parameters!\n\n");
    }

    let mut input_file: File =
        File::open(&prog_params[1]).expect("\n\n; FILE ERROR!\n; Could not open the file!\n\n");

    let mut contents: String = String::new();

    input_file
        .read_to_string(&mut contents)
        .expect("\n\n; FILE ERROR!\n; The contents of the file could not be read!\n\n");

    let tokens: Vec<Token> = lexer(contents);

    prog_output = program_parser(tokens, flag, prog_output);

    println!("\n; Lexical and Syntax analysis passed.\n");

    println!("{prog_output}\n");
}
