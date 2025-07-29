# Pico Data Analysis Tool — Rust Implementation

## Overview

In this project, I developed a front-end compiler component—a combined **lexical analyzer (scanner)** and **syntax analyzer (parser)**—for a domain-specific language called Data-Analysis (DA). The tool reads DA programs, performs lexical and syntactic validation, and translates the input into executable code in either Scheme or Prolog, depending on the specified output format.

This work deepened my understanding of compiler construction concepts, including tokenization, grammar parsing, and error handling, all implemented in Rust to leverage its performance and safety features.

## Key Features

* **Lexical Analysis:** Efficiently tokenizes DA source code, identifying valid tokens such as identifiers, keywords, literals, and symbols.
* **Syntax Analysis:** Parses tokens according to the DA grammar, verifying the correct program structure and syntax rules.
* **Error Handling:** Implements a fail-fast strategy to detect and report the first lexical or syntax error encountered, ensuring clear and immediate feedback.
* **Code Generation:** Converts syntactically correct DA programs into either:

  * Scheme code (via a `-s` flag) for subsequent execution in a Scheme interpreter.
  * Prolog queries (via a `-p` flag) for use in Prolog environments.

## Example

Given a DA program defining data vectors, inputs, processing operations (e.g., regression, correlation), and output formatting, my tool successfully parses the code and outputs corresponding Scheme or Prolog code.

### Sample DA Code Input

```plaintext
data:
   xvalues = vector,
   yvalues = vector,
   a = number,
   b = number,
   r = number
input:
   xvalues = read("file.csv", false, 0),
   yvalues = read("file.csv", false, 1)
process:
   a = regressiona(xvalues, yvalues),
   b = regressionb(xvalues, yvalues),
   r = correlation(xvalues, yvalues)
output:
   "value of a = ",
   a,
   "value of b = ",
   b,
   "value of r = ",
   r
end.
```

### Corresponding Scheme Output (with `-s` flag)

```scheme
(define xvalues (read-csv "./file.csv" #f 0))
(define yvalues (read-csv "./file.csv" #f 1))
(define a (regressiona xvalues yvalues))
(define b (regressionb xvalues yvalues))
(define r (correlation xvalues yvalues))
(display "value of a = ")
(newline)
(display a)
(newline)
(display "value of b = ")
(newline)
(display b)
(newline)
(display "value of r = ")
(newline)
(display r)
(newline)
```

### Corresponding Prolog Output (with `-p` flag)

```prolog
main :-
   load_data_column('file.csv', false, 0, Data0),
   load_data_column('file.csv', false, 1, Data1),
   regressiona(Data0, Data1, A),
   regressionb(Data0, Data1, B),
   correlation(Data0, Data1, R),
   writeln("value of a = "),
   writeln(A),
   writeln("value of b = "),
   writeln(B),
   writeln("value of r = "),
   writeln(R).
```

## Implementation Details

* Written entirely in **Rust**, emphasizing performance, memory safety, and concurrency readiness.
* Designed the lexical analyzer to produce meaningful tokens based on the DA language grammar.
* Built a recursive-descent parser to enforce syntax rules and produce structured representations of the input.
* Incorporated modular code organization with clear function and variable naming for maintainability.
* Implemented comprehensive error reporting that halts processing on the first detected error (lexical or syntax), improving usability.
* Supported extensible output generation, allowing easy addition of new target languages or output formats.

## Running the Program

Build and run the program with Cargo, specifying the input DA file and the desired output format:

```bash
cargo run -- input.da -s    # For Scheme output
cargo run -- input.da -p    # For Prolog output
```

The program performs lexical and syntax analysis before generating code or reporting errors.

## Reflection

This project reinforced my skills in:

* Compiler theory fundamentals — lexical and syntax analysis
* Rust programming idioms and best practices
* Error detection and user-friendly messaging
* Translating high-level domain-specific languages into executable code

It also demonstrated the practical application of compiler design concepts in a real-world context, improving both my coding discipline and architectural thinking.
