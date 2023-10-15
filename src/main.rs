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


fn lexer(input: &String) -> Vec<Token> {
    let mut i: usize = 0;
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

    while i < input.len() {
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
            curr_char = input.chars().nth(i).unwrap();
            while curr_char.is_ascii_lowercase() {
                lexeme.push(curr_char);
                i += 1;
                curr_char = input.chars().nth(i).unwrap();
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
            curr_char = input.chars().nth(i).unwrap();
            while curr_char.is_ascii_digit() {
                lexeme.push(curr_char);
                i += 1;
                curr_char = input.chars().nth(i).unwrap();
            }
            output.push(Token {
                token: TokenTypes::NUM,
                lexeme: lexeme,
            });
        } else if curr_char == '\"' {
            let mut lexeme = String::new();
            lexeme.push(curr_char);
            i += 1;
            curr_char = input.chars().nth(i).unwrap();
            while curr_char.is_ascii_lowercase()
                || curr_char.is_ascii_whitespace()
                || curr_char.is_ascii_digit()
                || curr_char == '.'
                || curr_char == '='
            {
                lexeme.push(curr_char);
                i += 1;
                curr_char = input.chars().nth(i).unwrap();
            }
            if curr_char == '\"' {
                lexeme.push(curr_char);
                i += 1;
            } else {
                panic!("; SYNTAX ERROR!\n; Expected '\"' after {}.", lexeme);
            }
            output.push(Token {
                token: TokenTypes::STRING,
                lexeme: lexeme,
            });
        } else if curr_char.is_ascii_whitespace() {
            i += 1;
        } else {
            panic!("; LEXICAL ERROR!\n; Unrecognized character '{}'.", curr_char);
        }
    }

    return output;
}


fn get_next_token(index : usize, tokens : &Vec<Token>) -> Token {
    return tokens[index].clone();
}


fn datadef_parser(start_index : usize, tokens : &Vec<Token>) -> usize {
    return 0; // delete when done
}


fn datadefs_parser(start_index : usize, tokens : &Vec<Token>) -> usize {
    let mut i : usize = start_index;

    i = datadef_parser(i, &tokens);
    
    return 0; // delete when done
}


fn program_parser(start_index : usize, tokens : &Vec<Token>) -> bool {
    let mut i : usize = start_index;

    let mut curr_token : Token = get_next_token(i, &tokens);
    assert!(curr_token.token == TokenTypes::DATA, "; SYNTAX ERROR!\n; Syntax error at '{}'.", curr_token.lexeme);
    i+=1;

    curr_token = get_next_token(i, &tokens);
    assert!(curr_token.token == TokenTypes::COLON, "; SYNTAX ERROR!\n; Syntax error at '{}'.", curr_token.lexeme);
    i+=1;

    i = datadefs_parser(i, &tokens);

    return true;
}


fn main() {
    let params: Vec<String> = env::args().collect();

    if params.len() == 1{
        panic!("; No input file provided!");
    }
    else {
        println!("; Processing input file {}.", params[1]);
    }

    let mut input = File::open(&params[1]).expect("; FILE ERROR!\n; Could not open the file!");

    let mut contents = String::new();

    input
        .read_to_string(&mut contents)
        .expect("; FILE ERROR!\n; The contents of the file could not be read!");

    let tokens : Vec<Token> = lexer(&contents);

    if program_parser(0, &tokens) {
        println!("; Lexical and Syntax analysis passed.");
    }
    else {
        println!("; Lexical and Syntax analysis failed.");
    }
}


// PROGRAM     -->   data:
//                      DATADEFS
//                   input:
//                      INPUTOPS
//                   process:
//                      PROCESSOPS
//                   output:
//                      OUTPUTOPS
//                   end.
// DATADEFS    -->   DATADEF |
//                   DATADEF, DATADEFS
// DATADEF     -->   ID : TYPE
// INPUTOPS    -->   INPUTOP |
//                   INPUTOP, INPUTOPS
// INPUTOP     -->   ID = read(STRING, BOOL, NUM)
// PROCESSOPS  -->   PROCESSOP |
//                   PROCESSOP, PROCESSOPS
// PROCESSOP   -->   ID = regressiona(ID, ID) |
//                   ID = regressionb(ID, ID) |
//                   ID = mean(ID) |
//                   ID = stddev(ID) |
//                   ID = correlation(ID, ID)
// OUTPUTOPS   -->   OUTPUTOP |
//                   OUTPUTOP, | OUTPUTOPS
// OUTPUTOP    -->   STRING |
//                   ID
// ID          -->   LETTER+
// TYPE        -->   vector | number
// BOOL        -->   true | false
// STRING      -->   "(LETTER|.| |=|DIGIT)+"
// NUM         -->   DIGIT+
// LETTER      -->   a | b | c | d | e | f | g | ... | z
// DIGIT       -->   0 | 1 | 2 | 3 | 4 | 5 | 6 | ... | 9