# 🚀 YA-RLOX: Yet Another Rust Lox Interpreter

**A fast, elegantly simple, yet powerful tree-walking interpreter for the Lox programming language.**

---

## What is YA-RLOX?

**Simple enough interpreter to understand in an afternoon. Powerful enough to run real programs.**

YA-RLOX is a complete, Turing-complete implementation of the Lox language from Robert Nystrom's book "Crafting Interpreters" – built with the performance and safety of Rust. **~1,500 lines of code**, this interpreter delivers a surprisingly rich programming experience.

### Simple, Powerful

Don't let the clean codebase fool you. YA-RLOX packs serious capabilities:
- **Full Expression Evaluation**: Binary operations, unary operators, grouping, and literal values
- **Dynamic Variables**: Declare, assign, and manipulate variables with ease
- **Control Flow**: Conditional statements (`if/else`) and loops (`for`, `while`) with `break` support
- **Functions**: First-class functions with closures and proper lexical scoping
- **Built-in Functions**: Native functions like `clock()` for real-world utility
- **Error Handling**: Comprehensive error reporting with meaningful messages

### Architecture

```
Scanner → Parser → Interpreter
   ↓        ↓         ↓
Tokens → AST → Execution
```

**Clean separation of concerns**:

- **Scanner**: Tokenizes source code with precision
- **Parser**: Builds Abstract Syntax Trees using recursive descent
- **Interpreter**: Executes code with a tree-walking evaluator
- **Environment**: Manages variable scoping and closures

### Performance 

- **Zero-copy tokenization**: Efficient string handling throughout the pipeline
- **Interactive REPL**: Instant feedback for rapid development

## Try It Out

### Run a Lox Program
```bash
cargo run -- examples/fibonacci.lox
```

### Interactive Mode
```bash
cargo run
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

# Interactive mode
./target/release/rlox
```

## Why?
**For Learning**: Perfect for understanding interpreter design and implementation
**For Teaching**: Clean, well-structured code that's easy to follow and extend
**For Fun**: A complete programming language in your pocket

**Built with ❤️ and Rust 🦀**
