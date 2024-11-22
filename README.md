# BrainRusted: Brainf*ck Interpreter in Rust

This is a simple interpreter for the Brainf*ck programming language, implemented in Rust. The project was created around 2020/2021 as a learning endeavor to practice Rust programming and to gain a better understanding of how interpreters work. While it is unlikely to receive updates in the future, it is not impossible.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Example](#example)
- [Contributing](#contributing)
- [License](#license)

## Features

- Parses Brainf*ck code from a file.
- Executes Brainf*ck instructions.
- Supports all standard Brainf*ck commands: `>`, `<`, `+`, `-`, `.`, `,`, `[`, `]`.

## Installation

To run this project, you need to have [Rust](https://www.rust-lang.org/tools/install) installed on your machine. Once you have Rust set up, you can clone the repository and build the project.

```bash
git clone https://github.com/dev-diego-fully/brain-rusted.git
cd brain-rusted
cargo build --release
```

## Usage

After building the project, you can run the interpreter by passing the path to a Brainf*ck program as an argument:

```bash
cargo run path/to/your/brainf_ck_program.bf
```

## Example

Here is a simple Brainf*ck program that prints "Hello, World!" when executed:

```brainfuck
>++++++++[<+++++++++>-]<.
>++++[<+++++++>-]<+.
+++++++..
+++.
>>++++++[<+++++++>-]<++.
------------.
>++++++[<+++++++++>-]<+.
<.
+++.
------.
--------.
>>>++++[<++++++++>-]<+.
```

Save this code in a file (e.g., `hello.bf`) and run it with the interpreter:

```bash
cargo run hello.bf
```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the GPLv3 License. See the [LICENSE](LICENSE) file for more details.
