use std::error;
use std::fs;
use std::fmt;
use std::vec::Vec;
use itertools::{iproduct};

pub fn part_1() -> Result<usize, Box<dyn error::Error>> {
    let buffer = fs::read_to_string("inputs/day2.txt")?;

    let init_memory: Vec<usize> = buffer
        .trim()
        .split(",")
        .map(|character| {
            character.parse().unwrap()
        })
        .collect();

    execute_program(init_memory.clone(), 12, 2)
}

pub fn part_2() -> Result<usize, Box<dyn error::Error>> {
    let buffer = fs::read_to_string("inputs/day2.txt")?;

    let init_memory: Vec<usize> = buffer
        .trim()
        .split(",")
        .map(|character| {
            character.parse().unwrap()
        })
        .collect();

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

fn execute_program(mut memory: Vec<usize>, noun: usize, verb: usize) -> Result<usize, Box<dyn error::Error>> {
    // "To do this, before running the program,
    // replace position 1 with the noune and replace position 2 with the verb."
    memory[1] = noun;
    memory[2] = verb;

    let processed = process(memory)?;

    // What value is left at position 0 after the program halts?
    let solution = processed[0];

    Ok(solution)
}

enum OpCode {
    Add {a: usize, b: usize, pos: usize},
    Multiply {a: usize, b: usize, pos: usize},
    Halt,
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

fn process(codes: Vec<usize>) -> Result<Vec<usize>, ProcessingError> {
    // The index of the current slice
    let mut cursor: usize = 0;
    let mut result_codes = codes;

    loop {
        // Take a slice of 4
        let to_process = &result_codes[cursor..cursor + 4];

        // Convert to opcode
        let opcode = opcode_from_slice(&to_process)?;

        // Evaluate opcode
        result_codes = evaluate_opcode(opcode, result_codes);

        // Peek one instruction ahead, to see if we should halt
        match result_codes[cursor + 4] {
            // Exit loop if we're halting
            // TODO: I'd like this to be cleaner, why are we suddenly dealing with numbers?
            // I wonder if opcode_from_slice is a bad assumption
            99 => break Ok(result_codes),
            // Otherwise, loop
            _other => {
                cursor = cursor + 4;
            }
        }
    }
}

fn opcode_from_slice(code: &[usize]) -> Result<OpCode, ProcessingError> {
    match code[0] {
        1 => Ok(OpCode::Add {a: code[1], b: code[2], pos: code[3]}),
        2 => Ok(OpCode::Multiply {a: code[1], b: code[2], pos: code[3]}),
        99 => Ok(OpCode::Halt),
        _unknown => Err(ProcessingError)
    }
}

fn evaluate_opcode(code: OpCode, mut codes: Vec<usize>) -> Vec<usize> {
    match code {
        OpCode::Add {a, b, pos} => {
            codes[pos] = codes[a] + codes[b];
        },
        OpCode::Multiply {a, b, pos} => {
            codes[pos] = codes[a] * codes[b];
        },
        OpCode::Halt => ()
    }

    codes
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
