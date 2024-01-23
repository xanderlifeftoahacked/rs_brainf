use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::process;

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Lexeme {
    IncrPtr,   // Alexander
    DecrPtr,   // sanya
    Incr,      // ALEX
    Decr,      // SAN
    Read,      // sanyok
    Write,     // sasha
    LoopBegin, //saa
    LoopEnd,   //sha
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Instruction {
    IncrPtr,
    DecrPtr,
    Incr,
    Decr,
    Write,
    Read,
    Loop(Vec<Instruction>),
}

fn translator(source: String) -> String {
    let mut translated = String::new();
    for ch in source.chars() {
        let word = match ch {
            '+' => Some("ALEX"),
            '-' => Some("SAN"),
            '>' => Some("Alexander"),
            '<' => Some("sanya"),
            ',' => Some("sanyok"),
            '.' => Some("sasha"),
            '[' => Some("saa"),
            ']' => Some("sha"),
            _ => None,
        };

        match word {
            Some(word) => translated += &(word.to_owned() + " "),
            None => (),
        }
    }
    translated
}

fn lexer(source: String) -> Vec<Lexeme> {
    let mut operations: Vec<Lexeme> = Vec::new();

    for word in source.split(" ") {
        let lexeme = match word {
            "Alexander" => Some(Lexeme::IncrPtr),
            "sanya" => Some(Lexeme::DecrPtr),
            "ALEX" => Some(Lexeme::Incr),
            "SAN" => Some(Lexeme::Decr),
            "sanyok" => Some(Lexeme::Read),
            "sasha" => Some(Lexeme::Write),
            "saa" => Some(Lexeme::LoopBegin),
            "sha" => Some(Lexeme::LoopEnd),
            _ => None,
        };

        match lexeme {
            Some(lexeme) => operations.push(lexeme),
            None => (),
        }
    }
    operations
}

fn parser(lexemes: Vec<Lexeme>) -> Result<Vec<Instruction>, String> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut loop_stack: usize = 0;
    let mut loop_start: usize = 0;

    for (i, op) in lexemes.iter().enumerate() {
        if loop_stack == 0 {
            let instr = match op {
                Lexeme::IncrPtr => Some(Instruction::IncrPtr),
                Lexeme::DecrPtr => Some(Instruction::DecrPtr),
                Lexeme::Incr => Some(Instruction::Incr),
                Lexeme::Decr => Some(Instruction::Decr),
                Lexeme::Read => Some(Instruction::Read),
                Lexeme::Write => Some(Instruction::Write),

                Lexeme::LoopBegin => {
                    loop_start = i;
                    loop_stack += 1;
                    None
                }

                Lexeme::LoopEnd => {
                    return Err(format!("Loop at {} never starts", i.to_string()));
                }
            };

            match instr {
                Some(instr) => instructions.push(instr),
                None => (),
            };
        } else {
            match op {
                Lexeme::LoopBegin => loop_stack += 1,
                Lexeme::LoopEnd => {
                    loop_stack -= 1;
                    if loop_stack == 0 {
                        instructions.push(Instruction::Loop(
                            parser(lexemes[loop_start + 1..i].to_vec()).unwrap(),
                        ))
                    }
                }
                _ => (),
            }
        }
    }

    if loop_stack != 0 {
        return Err(format!("Loop  at {} never ends", loop_start));
    }

    Ok(instructions)
}

#[allow(arithmetic_overflow)]
fn run(instuctions: &Vec<Instruction>, tape: &mut Vec<u8>, ptr: &mut usize) {
    for instr in instuctions {
        match instr {
            Instruction::IncrPtr => *ptr += 1,
            Instruction::DecrPtr => *ptr -= 1,
            Instruction::Incr => tape[*ptr] += 1,
            Instruction::Decr => tape[*ptr] -= 1,
            Instruction::Write => {
                print!("{}", tape[*ptr] as char);
                io::stdout().flush().unwrap();
            }
            Instruction::Read => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin()
                    .read_exact(&mut input)
                    .unwrap_or_else(|err| {
                        eprintln!("Problem parsing arguments: {err}");
                        process::exit(1);
                    });

                tape[*ptr] = input[0];
            }
            Instruction::Loop(loop_instructions) => {
                while tape[*ptr] != 0 {
                    run(&loop_instructions, tape, ptr)
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("To run bf code: bf <file.bf>");
        std::process::exit(1);
    }
    let filename = &args[1];

    let mut file = File::open(filename).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let mut source = String::new();
    file.read_to_string(&mut source).unwrap_or_else(|err| {
        eprintln!("Problem readingg text: {err}");
        process::exit(1);
    });

    let source = translator(source);
    println!("The source code is: \n {}", &source);

    let ops = lexer(source);
    let instructions = parser(ops).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let mut tape: Vec<u8> = vec![0; 1024];
    let mut data_pointer = 512;

    println!("The output is:");
    run(&instructions, &mut tape, &mut data_pointer);
}
