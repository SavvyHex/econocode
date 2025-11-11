# EconoCode

A toy compiler that transforms a small language (arithmetic + control flow + input) into intermediate representation (IR) code and can interpret it.

## Overview

EconoCode is a simple compiler frontend that demonstrates the basic stages of compilation:
1. **Lexical Analysis** - Tokenizes input using the Logos crate
2. **Parsing** - Converts tokens to Abstract Syntax Tree (AST) using LALRPOP
3. **Lowering** - Transforms AST into intermediate representation with energy estimation
4. **Execution** - Interprets the IR to run the code

## Supported Language Features

The language supports arithmetic, control flow, and input:
- Integers with optional type suffix: `42i32`, `123i64` (default is `i64`)
- Variables with optional type: `x: i32`, `myVar: i64` (default is `i64`)
- Binary ops: `+`, `-`, `*`, `/`
- Comparisons: `==`, `!=`, `<`, `<=`, `>`, `>=` (booleans as 0/1)
- Assignment: `x = 5`, `y: i64 = x + 1`
- Blocks: `{ stmt* }`
- Conditionals: `if (cond) { ... } else { ... }`
- Loops: `while (cond) { ... }`
- Input: `read x;` or `read y: i32;` (reads one line from stdin, parses integer)

## Examples

```bash
# 32-bit operations (lower energy cost)
./target/debug/econocode - <<< "2i32 + 3i32"
# Output: Energy ~3 units

# 64-bit operations (higher energy cost)  
./target/debug/econocode - <<< "2i64 * 3i64"
# Output: Energy ~7 units

# Variables with types
./target/debug/econocode - <<< "x: i32 + 5i32"
```

## Examples

Arithmetic only (32-bit):
```
2i32 * 3i32
```

If + while:
```
{
	x: i64 = 0;
	y: i64 = 1;
	if (2i64 * 3i64 == 6i64) {
		x = 10i64;
	} else {
		x = 20i64;
	}
	while (x < 13i64) {
		x = x + y;
	}
	x;
}
```

Read input and add:
```
{
	read n: i64;
	read m;      // defaults to i64
	s: i64 = n + m;
	s;
}
```

## Architecture

- **`src/lexer.rs`** - Token definitions and lexical analysis
- **`src/parser.lalrpop`** - Grammar specification for LALRPOP parser
- **`src/ast.rs`** - Abstract Syntax Tree node definitions
- **`src/lower.rs`** - AST to IR transformation logic
- **`src/ir.rs`** - Intermediate representation definitions
- **`src/main.rs`** - Command-line interface and compilation pipeline
- New since control flow/input:
	- `IfElse`, `While`, `Assign`, `Block`, `CmpOp`, and `Read` in AST
	- IR adds `Cmp`, `Label`, `BrIf`, `Jmp`, and `Read`
	- Interpreter executes with a program counter and label table; `read` consumes one stdin line per statement

## Run it

- Build and run a file:
	- cargo run path/to/file.txt

- Execute the IR and print the result:
	- cargo run -- --run path/to/file.txt

- Provide stdin for `read` statements (example adds two numbers):
	- printf "7\n8\n" | cargo run -- --run test/test9.txt

### Interactive input

When a program uses `read`, you can run interactively and type values when prompted:
- cargo run -- --run test/test9.txt
	- Input n: 7
	- Input m: 8
	- Execution result: 15

Notes:
- Each `read` consumes one line from stdin (numbers like `-23` are accepted).
- Prompts appear as `Input <var>:`.

## Dependencies

- **LALRPOP** - LR(1) parser generator
- **Logos** - Fast lexer generator
- **Clap** - Command-line argument parsing

## License

See [LICENSE](LICENSE) file for details.
