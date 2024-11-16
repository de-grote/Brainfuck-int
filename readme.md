# Brainfuck Int

[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Github](https://img.shields.io/badge/github-de_grote/Brainfuck-int?style=flat-square&logo=github)](https://github.com/de-grote/Brainfuck-int)
[![Crates.io](https://img.shields.io/crates/v/brainfuck-int?style=flat-square)](https://crates.io/crates/brainfuck-int)
[![Crates.io](https://img.shields.io/crates/d/brainfuck-int?style=flat-square)](https://crates.io/crates/brainfuck-int)

Basically all variations of brainfuck interpreter were taken already so I'm stuck with this name.

This is just another Brainfuck interpreter with no special features that sets it apart.

Uses 8 bit integers as cells and 30 000 cells by default.

When getting input from a file using `< file` or `Get-Content file`, and all bytes have been consumed, the interpreter will by default return 0 on `,`, this behaviour can be changed using the --eof flag passing in 'unchanged', to leave the cell unchanged, or a number under 255 or an (escaped) ascii character in 'single quotes', which will change the cell to that value. This is 0 by default.

More information on brainfuck can be found on [esolangs.org](https://esolangs.org/wiki/Brainfuck)

This is a cli, and only ment to be used as a cli.

Use `brainfuck-int -h` to see all options:

```
Simple brainfuck interpreter

Usage: brainfuck-int.exe [OPTIONS]

Options:
  -i, --interpret <PROGRAM>  Code to interpret
  -f, --file <FILE>          Path to brainfuck file
  -c, --cells <CELLS>        Number of cells in tape [default: 30000]
      --eof <EOF>            Value when out of input [default: 0]
  -h, --help                 Print help
  -V, --version              Print version
```

## Installation

You can download the cli tool using cargo.

To install `brainfuck-int` systemwide use:

```sh
cargo install brainfuck-int
```

Alternatively, if you have the repository cloned, you can use:

```sh
cargo install --path .
```

## Examples

This program prints `HI` using interpret mode.
```sh
brainfuck-int -i "++++++++[->+++++++++<]>.+."
```

This is a cat program read from a file.
```sh
echo ",[.,]" > cat.bf
brainfuck-int -f cat.bf < file.txt
```

A [truth machine](https://esolangs.org/wiki/Truth-machine) (only works for 0 and 1). By specifying the amount of cells we want to use we can golf the program a little.
```sh
brainfuck-int           -i "++++++[>++++++++<-],[->->+<<]>>.<[>.<]"
brainfuck-int --cells=3 -i "++++++[>++++++++<-],[->->+>]<.<[>.<]"
brainfuck-int -c3 -i "++++++[>++++++++<-],[->->+>]<.<[>.<]"
```
