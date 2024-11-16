use std::{
    fmt::{Display, Write as _},
    fs::read,
    io::{stdout, Read, Write as _},
    path::PathBuf,
    str::FromStr,
};

use clap::Parser;

/// Simple brainfuck interpreter
#[derive(Debug, Parser)]
#[command(name = "brainfuck-int", author = "de_grote", version, long_about = None)]
struct Cli {
    /// Code to interpret
    #[arg(
        short,
        long,
        conflicts_with = "file",
        required_unless_present = "file",
        value_name = "PROGRAM"
    )]
    interpret: Option<String>,

    /// Path to brainfuck file
    #[arg(
        short,
        long,
        conflicts_with = "interpret",
        required_unless_present = "interpret"
    )]
    file: Option<PathBuf>,

    /// Number of cells in tape
    #[arg(short, long, default_value_t = 30000)]
    cells: usize,

    /// Value when out of input
    #[arg(long, default_value = "0")]
    eof: EofIndicator,
}

#[derive(Clone, Copy, Debug)]
enum EofIndicator {
    Char(u8),
    Unchanged,
}

impl Default for EofIndicator {
    fn default() -> Self {
        Self::Char(0)
    }
}

impl FromStr for EofIndicator {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.eq_ignore_ascii_case("unchanged") {
            return Ok(Self::Unchanged);
        }
        if let Ok(byte) = input.parse() {
            return Ok(EofIndicator::Char(byte));
        }
        if let Ok(byte) = input.parse::<i8>() {
            return Ok(EofIndicator::Char(byte as u8));
        }
        if input.starts_with('\'') && input.ends_with('\'') {
            let i = &input[1..input.len() - 1];
            if let Some(first_char) = i.chars().next() {
                if i.len() == 1 {
                    return Ok(EofIndicator::Char(first_char as u8));
                }
                if first_char == '\\' && i.len() == 2 {
                    let i = match i.chars().nth(1) {
                        Some('0') => Some(0),
                        Some('a') => Some(7),
                        Some('b') => Some(10),
                        Some('t') => Some(11),
                        Some('n') => Some(12),
                        Some('v') => Some(13),
                        Some('r') => Some(14),
                        Some('f') => Some(15),
                        Some('e') => Some(33),
                        _ => None,
                    };
                    if let Some(v) = i {
                        return Ok(EofIndicator::Char(v));
                    }
                }
            };
        }
        Err("Must be 'unchanged', a number under 255 or an (escaped) ascii character in 'single quotes'".into())
    }
}

impl Display for EofIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EofIndicator::Char(c) => f.write_char(char::from(*c)),
            EofIndicator::Unchanged => f.write_str("Unchanged"),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let program = match (cli.interpret, cli.file) {
        (Some(program), None) => program
            .chars()
            .filter_map(|c| c.is_ascii().then_some(c as u8))
            .collect(),
        (None, Some(path)) => read(path).expect("couldn't get file"),
        _ => unreachable!(),
    };
    interpret(&program, cli.cells, cli.eof);
}

fn interpret(input: &[u8], cells: usize, eof: EofIndicator) {
    let mut program_index = 0;
    let mut pointer = 0;
    let mut tape = vec![0u8; cells];
    while program_index < input.len() {
        match input[program_index] {
            // +
            43 => tape[pointer] = tape[pointer].wrapping_add(1),
            // -
            45 => tape[pointer] = tape[pointer].wrapping_sub(1),
            // >
            62 => pointer = (pointer + 1) % cells,
            // <
            60 => pointer = if pointer == 0 { cells - 1 } else { pointer - 1 },
            // [
            91 => {
                if tape[pointer] == 0 {
                    let mut opens = 0;
                    let Some(end_of_loop) =
                        input
                            .iter()
                            .enumerate()
                            .skip(program_index + 1)
                            .find_map(|(idx, &c)| {
                                if c == 91 {
                                    opens += 1;
                                }
                                if c == 93 {
                                    if opens == 0 {
                                        Some(idx)
                                    } else {
                                        opens -= 1;
                                        None
                                    }
                                } else {
                                    None
                                }
                            })
                    else {
                        panic!("Invalid loop start at {}", program_index);
                    };
                    program_index = end_of_loop;
                }
            }
            // ]
            93 => {
                if tape[pointer] != 0 {
                    let mut closes = 0;
                    let Some(start_of_loop) = input
                        .iter()
                        .enumerate()
                        .take(program_index)
                        .rev()
                        .find_map(|(idx, &c)| {
                            if c == 93 {
                                closes += 1;
                            }
                            if c == 91 {
                                if closes == 0 {
                                    Some(idx)
                                } else {
                                    closes -= 1;
                                    None
                                }
                            } else {
                                None
                            }
                        })
                    else {
                        panic!("Invalid loop end at {}", program_index);
                    };

                    program_index = start_of_loop;
                }
            }
            // ,
            44 => {
                let mut buf = [0; 1];
                tape[pointer] = if std::io::stdin().read_exact(&mut buf).is_ok() {
                    buf[0]
                } else {
                    match eof {
                        EofIndicator::Char(c) => c,
                        EofIndicator::Unchanged => tape[pointer],
                    }
                };
            }
            // .
            46 => stdout()
                .write_all(&[tape[pointer]])
                .expect("error writing to stdout"),
            _ => (),
        }
        program_index += 1;
    }
}
