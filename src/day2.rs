use std::error;
use std::fs;
use std::fmt;
use std::vec::Vec;
use itertools::{iproduct};

pub fn part_1() -> Result<usize, Box<dyn error::Error>> {
    let init_memory = get_init_memory()?;
    execute_program(init_memory.clone(), 12, 2)
}

pub fn part_2() -> Result<usize, Box<dyn error::Error>> {
    let init_memory = get_init_memory()?;
    let mut solution = (0, 0);

    for (noun, verb) in iproduct!(0..99, 0..99) {
        let result = execute_program(init_memory.clone(), noun, verb)?;
        if result == 19690720 {
            solution = (noun, verb);
            break;
        }
    }

    Ok(100 * solution.0 + solution.1)
}

// Internals

fn get_init_memory() -> Result<Vec<usize>, Box<dyn error::Error>> {
    let buffer = fs::read_to_string("inputs/day2.txt")?;

    let init_memory: Vec<usize> = buffer
        .trim()
        .split(",")
        .map(|character| {
            character.parse().unwrap()
        })
        .collect();

    Ok(init_memory)
}

fn execute_program(mut memory: Vec<usize>, noun: usize, verb: usize) -> Result<usize, Box<dyn error::Error>> {
    // To do this, before running the program:
    // - replace position 1 with the noun and
    // - replace position 2 with the verb
    memory[1] = noun;
    memory[2] = verb;

    let processed = process(memory)?;

    // What value is left at position 0 after the program halts?
    let solution = processed[0];

    Ok(solution)
}

enum Instruction {
    Add {a: usize, b: usize, pos: usize},
    Multiply {a: usize, b: usize, pos: usize},
    Halt,
}

enum OpCode {
    Add,
    Multiply,
    Halt,
    Unknown
}

// Define our error types. These may be customized for our error handling cases.
// Now jwe will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
struct ProcessingError;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "there was a processing error")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for ProcessingError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

fn process(mut memory: Vec<usize>) -> Result<Vec<usize>, ProcessingError> {
    // The index of the current slice
    let mut cursor: usize = 0;

    loop {
        // Take a slice of 4
        let to_process = &memory[cursor..cursor + 4];

        // Convert to instruction
        let instruction = instruction_from_slice(&to_process)?;

        // Evaluate instruction
        memory = evaluate_instruction(instruction, memory);

        // Peek one instruction ahead, to see if we should halt
        match opcode_from_number(memory[cursor + 4]) {
            // Exit loop if we're halting
            // We could probably make this more generic if we had
            // a map from instructions to expected instructions.
            // But anyway, in this case Add and Multiply are 1+3 args,
            // while Halt is 1+0 args. So it does not matter :)
            OpCode::Halt => break Ok(memory),
            // Otherwise, loop
            _other => {
                cursor = cursor + 4;
            }
        }
    }
}

fn opcode_from_number(number: usize) -> OpCode {
    match number {
        1 => OpCode::Add,
        2 => OpCode::Multiply,
        3 => OpCode::Halt,
        _ => OpCode::Unknown
    }
}

fn instruction_from_slice(memory: &[usize]) -> Result<Instruction, ProcessingError> {
    match opcode_from_number(memory[0]) {
        OpCode::Add => Ok(Instruction::Add {a: memory[1], b: memory[2], pos: memory[3]}),
        OpCode::Multiply => Ok(Instruction::Multiply {a: memory[1], b: memory[2], pos: memory[3]}),
        OpCode::Halt => Ok(Instruction::Halt),
        OpCode::Unknown => Err(ProcessingError)
    }
}

fn evaluate_instruction(instruction: Instruction, mut memory: Vec<usize>) -> Vec<usize> {
    match instruction {
        Instruction::Add {a, b, pos} => {
            memory[pos] = memory[a] + memory[b];
        },
        Instruction::Multiply {a, b, pos} => {
            memory[pos] = memory[a] * memory[b];
        },
        Instruction::Halt => ()
    }

    memory
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(process(vec![1,0,0,0,99]).unwrap(), vec![2,0,0,0,99]);
        assert_eq!(process(vec![2,3,0,3,99]).unwrap(), vec![2,3,0,6,99]);
        assert_eq!(process(vec![2,4,4,5,99,0]).unwrap(), vec![2,4,4,5,99,9801]);
        assert_eq!(process(vec![1,1,1,4,99,5,6,0,99]).unwrap(), vec![30,1,1,4,2,5,6,0,99]);
    }
}
