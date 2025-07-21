# 🚀 YA-RLOX: Yet Another Rust Lox Interpreter

**The project is still in development**

**A fast, elegantly simple, yet powerful tree-walking interpreter for the Lox programming language.**

## TODO
- [ ] Implement OOP
- [ ] Add STD
- - [ ] `clock` functionality

---

## What is YA-RLOX?

**Simple enough interpreter to understand in an afternoon. Powerful enough to run real programs.**

YA-RLOX is a complete, Turing-complete implementation of the Lox language from Robert Nystrom's book "Crafting Interpreters"

### Simple, Powerful

Capabilities:
- **Full Expression Evaluation**: Binary operations, unary operators, grouping, and literal values
- **Dynamic Variables**: Declare, assign, and manipulate variables 
- **Control Flow**: Conditional statements (`if/else`) and loops (`for`, `while`) with `break` support
- **Functions**: First-class functions with closures and proper lexical scoping
- **Error Handling**: Error reporting with meaningful messages

### Architecture

```
Scanner → Parser → Interpreter
   ↓        ↓         ↓
Tokens  →  AST   → Execution
```

**Clean separation of concerns**:

- **Scanner** `scanner.rs`: Tokenizes source code into array of Token 
- **Parser** `parser.rs`: Builds Abstract Syntax Trees using recursive descent
- **Interpreter** `interpreter.rs`: Executes code with a tree-walking evaluator
- **Environment** `env.rs`: Manages variable scoping and closures

### Performance 

- **Zero-copy tokenization**: Efficient string handling throughout the pipeline
- **Interactive REPL**: Instant feedback for rapid development

## Try It Out

### Run a Lox Program
```bash
cargo run --bin=rlox test.lox
```

### REPL
```bash
cargo run --bin=rlox
> print "Hello, Lox!";
Hello, Lox!
> var x = 42;
> print x * 2;
84
```

### Sample Lox Code
```lox
// Functions and closures
fun fibonacci(n) {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

// Loops and control flow
for (var i = 0; i < 10; i = i + 1) {
    if (i >= 2) break;
    print "fib(" + i + ") = " + fibonacci(i);
}
```

## Technical Highlights

| Feature | Implementation |
|---------|---------------|
| **Lexical Analysis** | Hand-written scanner with peek-ahead |
| **Parsing** | Recursive descent parser |
| **Evaluation** | Tree-walking interpreter with visitor pattern |
| **Environment** | Lexical scoping with environment chains |
| **Functions** | First-class functions with closures |
| **Error Handling** | Comprehensive error types and propagation |

## Getting Started

### Prerequisites
- Rust 1.70+ (Edition 2024)

### Installation
```bash
git clone https://github.com/alexniemtsov/ya-rlox.git
cd ya-rlox
cargo build --release
```

### Usage
```bash
# Run a file
./target/release/rlox script.lox

# REPL
./target/release/rlox
```

## Why?
**For Learning**: Always wanted to try to implement my own interpreter
**For Fun**: A complete programming language in your pocket
