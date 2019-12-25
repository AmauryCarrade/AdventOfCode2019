use itertools::Itertools;
use std::io::{self, Read};
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub message: &'static str,
}

/// An instruction of the program, containing the opcode and
/// the parameters, alongside their modes.
struct Instruction {
    opcode: OpCode,
    parameters: Vec<Parameter>,
}

/// A parameter, i.e. a piece of data and a ParameterMode to
/// know how to interpret it.
/// See `ParameterMode`.
#[derive(Debug, Copy, Clone)]
struct Parameter {
    data: i64,
    mode: ParameterMode,
}

#[derive(Debug, Copy, Clone)]
enum ParameterMode {
    /// The parameter's value is the value stored at it's data
    /// interpreted as a pointer.
    Position,

    /// The parameter's value is its data, directly.
    Immediate,

    /// The parameter's value is the value stored at it's
    /// data interpreted as a relative pointer from the
    /// current relative base.
    Relative,
}

/// OpCodes specify the purpose of each instruction in the program.
enum OpCode {
    /// Calculates the result of an arithmetic operation between
    /// the two first parameters, and stores it at the address stored
    /// in the first one.
    Arithmetic(Operation),

    /// Takes an input (default from stdin) and stores it.
    Input,

    /// Outputs the value pointed by its parameter.
    Output,

    /// Jump to the second parameter if the first one passes
    /// the test specified in the closure.
    Jump(Box<dyn Fn(i64) -> bool>),

    /// Stores 1 in the address stored in the third parameter if
    /// the two first parameters validate the test specified in
    /// the closure; 0 else.
    Test(Box<dyn Fn(i64, i64) -> bool>),

    AdjustRelativeBase,

    /// Halts the program.
    Halt,
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    /// In Arithmetic opcode, adds the two parameters.
    Add,

    /// In Arithmetic opcode, multiply the two parameters.
    Multiply,
}

/// The Intcode program interpreter.
///
/// For references, see [days two](https://adventofcode.com/2019/day/2),
/// [five](https://adventofcode.com/2019/day/5) and
/// [nine](https://adventofcode.com/2019/day/9) of 2019's Advent of Code.
pub struct Program {
    /// The program's memory. It stores both the instructions
    /// (source code) to execute, and the data (“variables”)
    /// in one unique self-modifiable chain.
    memory: Vec<i64>,

    /// The current pointer in the program's execution.
    pointer: usize,

    /// The current relative base for relative mode.
    relative_base: usize,

    /// An input source for the Input opcode. It's a closure
    /// receiving a number, incremented each time an input is
    /// required (starts at 0), and returning a value (i64).
    input_source: Box<dyn Fn(usize) -> Result<i64>>,

    /// The number of times an input was requested.
    /// (See `input_source`.)
    input_count: usize,

    /// The outputs from the Output opcode.
    output: Vec<i64>,

    /// True if the program is running (stays true if the program
    /// is executed until next output).
    running: bool,
}

impl FromStr for Program {
    type Err = Error;

    fn from_str(source_code: &str) -> Result<Self> {
        match source_code
            .split(',')
            .filter(|number_str| !number_str.is_empty())
            .map(|number_str| number_str.parse::<i64>())
            .collect()
        {
            Ok(memory) => Ok(Program {
                memory,
                pointer: 0,
                relative_base: 0,
                input_source: Box::new(|_| {
                    let mut buffer = String::new();
                    match io::stdin().read_to_string(&mut buffer) {
                        Ok(_) => match buffer.trim().parse() {
                            Ok(i) => Ok(i),
                            Err(_) => Err(Error {
                                message: "Invalid input: not a number",
                            }),
                        },
                        Err(_) => Err(Error {
                            message: "Invalid input: unable to read from stdin",
                        }),
                    }
                }),
                input_count: 0,
                output: vec![],
                running: false,
            }),
            Err(_) => Err(Error {
                message: "Invalid source code: invalid numbers.",
            }),
        }
    }
}

impl Program {
    /// Patches the program, replacing the value at
    /// the given address by the given new value.
    pub fn patch(&mut self, address: usize, value: i64) {
        self.set(address, value);
    }

    /// Returns the value stored into the program's
    /// memory at the given index. If the address is out
    /// of the current memory, returns 0.
    pub fn get(&self, address: usize) -> Option<i64> {
        Some(self.memory.get(address).cloned().unwrap_or(0))
    }

    /// Sets the value at the address, expanding the
    /// memory if needed.
    fn set(&mut self, address: usize, value: i64) {
        // If the address is out of the current allocated memory, we
        // have to expand it.
        if self.memory.len() <= address {
            self.memory.reserve(address - self.memory.len());
            (self.memory.len()..address).for_each(|_| self.memory.push(0));
            self.memory.push(value);
        } else {
            self.memory[address] = value;
        }
    }

    /// Retrieves the value of a parameter, according to its mode.
    ///
    /// instruction: the instruction where the parameter is.
    /// parameter: the parameter index in the instruction (starts at zero).
    fn get_parameter(&self, instruction: &Instruction, parameter: usize) -> Option<i64> {
        match instruction.parameters.get(parameter) {
            Some(parameter) => match parameter.mode {
                ParameterMode::Position => self.get(parameter.data as usize),
                ParameterMode::Relative => {
                    self.get((self.relative_base as isize + parameter.data as isize) as usize)
                }
                ParameterMode::Immediate => Some(parameter.data),
            },
            None => None,
        }
    }

    /// Interprets the parameter as an address, taking into account the
    /// relative mode.
    fn get_address(&self, parameter: &Parameter) -> usize {
        match parameter.mode {
            ParameterMode::Relative => {
                (self.relative_base as isize + parameter.data as isize) as usize
            }
            _ => parameter.data as usize,
        }
    }

    /// Sets the input source of the program. It's a closure receiving
    /// a number: the nth time an input is asked by the program (starts at
    /// zero) and returning a i64.
    /// If not set, stdin is used.
    pub fn set_input(&mut self, input: impl Fn(usize) -> Result<i64> + 'static) {
        self.input_source = Box::new(input);
    }

    /// Requests an input from the input source set.
    fn request_input(&mut self) -> Result<i64> {
        let input = (self.input_source)(self.input_count);
        self.input_count += 1;
        input
    }

    /// Returns the values outputted by the program.
    pub fn output(&self) -> Vec<i64> {
        self.output.clone()
    }

    /// Same as output, but concatenates all output into a String.
    pub fn output_str(&self) -> String {
        self.output.iter().map(|o| o.to_string()).collect()
    }

    /// Resets the internal pointer to the beginning of
    /// the program.
    fn reset(&mut self) {
        self.pointer = 0;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Executes the program, and returns the output of
    /// its execution.
    pub fn execute(&mut self) -> Result<Vec<i64>> {
        self.execute0(false)
    }

    /// Executes the program until the next output, then
    /// pauses it and returns the last output.
    /// To resume the program, call this same function
    /// again until `is_running()` is false.
    pub fn execute_until_next_output(&mut self) -> Result<i64> {
        self.execute0(true)
            .map(|outputs| outputs.last().cloned())
            .map_or_else(
                |error| Err(error),
                |output| {
                    output.ok_or(Error {
                        message: "No output",
                    })
                },
            )
    }

    fn execute0(&mut self, until_next_output: bool) -> Result<Vec<i64>> {
        if !self.running {
            self.reset();
        }

        self.running = true;

        loop {
            let output_len = self.output.len();

            if !self.forward()? {
                self.running = false;
                break Ok(self.output());
            }

            if self.output.len() > output_len && until_next_output {
                break Ok(self.output());
            }
        }
    }

    /// Returns the value at the current internal pointer position.
    fn current(&self) -> Option<i64> {
        self.memory.get(self.pointer).cloned()
    }

    /// Returns the value `add` addresses after the current
    /// internal pointer position.
    fn offset(&self, add: usize) -> Option<i64> {
        self.memory.get(self.pointer + add).cloned()
    }

    /// Computes the result of an operation from its operands.
    fn compute_operation(&self, operation: Operation, a: i64, b: i64) -> i64 {
        match operation {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
        }
    }

    /// Processes one instruction in the program and move the internal
    /// pointer to the beginning of the next instruction.
    fn forward(&mut self) -> Result<bool> {
        match self.parse_instruction() {
            Ok(instruction) => match &instruction.opcode {
                OpCode::Arithmetic(operation) => match self.get_parameter(&instruction, 0) {
                    Some(operand1) => match self.get_parameter(&instruction, 1) {
                        Some(operand2) => match instruction.parameters.get(2) {
                            Some(result_address) => {
                                self.set(
                                    self.get_address(result_address),
                                    self.compute_operation(*operation, operand1, operand2),
                                );
                                Ok(true)
                            }
                            None => Err(Error {
                                message: "Invalid third parameter in operation (1|2)",
                            }),
                        },
                        None => Err(Error {
                            message: "Invalid second parameter in operation (1|2)",
                        }),
                    },
                    None => Err(Error {
                        message: "Invalid first parameter pointer in operation (1|2)",
                    }),
                },
                OpCode::Input => match instruction.parameters.get(0) {
                    Some(input_address) => match self.request_input() {
                        Ok(input) => {
                            self.set(self.get_address(input_address), input);
                            Ok(true)
                        }
                        Err(e) => Err(e),
                    },
                    None => Err(Error {
                        message: "Invalid first parameter pointer in input (3)",
                    }),
                },
                OpCode::Output => match self.get_parameter(&instruction, 0) {
                    Some(output) => {
                        self.output.push(output);
                        Ok(true)
                    }
                    None => Err(Error {
                        message: "Invalid first parameter pointer in output (4)",
                    }),
                },
                OpCode::Jump(condition) => match self.get_parameter(&instruction, 0) {
                    Some(test) if condition(test) => match self.get_parameter(&instruction, 1) {
                        Some(new_pointer) => {
                            self.pointer = new_pointer as usize;
                            Ok(true)
                        }
                        None => Err(Error {
                            message: "Invalid second parameter pointer in jump_if (5|6)",
                        }),
                    },
                    None => Err(Error {
                        message: "Invalid first parameter pointer in jump_if (5|6)",
                    }),
                    _ => Ok(true),
                },
                OpCode::Test(condition) => match self.get_parameter(&instruction, 0) {
                    Some(operand1) => match self.get_parameter(&instruction, 1) {
                        Some(operand2) => match instruction.parameters.get(2) {
                            Some(test_result_address) => {
                                self.set(
                                    self.get_address(test_result_address),
                                    if condition(operand1, operand2) { 1 } else { 0 },
                                );
                                Ok(true)
                            }
                            None => Err(Error {
                                message: "Invalid third parameter pointer in test (7|8)",
                            }),
                        },
                        None => Err(Error {
                            message: "Invalid second parameter pointer in test (7|8)",
                        }),
                    },
                    None => Err(Error {
                        message: "Invalid first parameter pointer in test (7|8)",
                    }),
                },
                OpCode::AdjustRelativeBase => match self.get_parameter(&instruction, 0) {
                    Some(relative_base) => {
                        self.relative_base =
                            (self.relative_base as isize + relative_base as isize) as usize;
                        Ok(true)
                    }
                    None => Err(Error {
                        message: "Invalid parameter in adjust_relative_base (9)",
                    }),
                },
                OpCode::Halt => Ok(false),
            },
            Err(e) => Err(e),
        }
    }

    /// Parses an OPCode and returns a tuple containing the opcode
    /// and the number of parameters for this opcode.
    fn parse_opcode(&self, opcode_code: i64) -> Result<(OpCode, usize)> {
        match opcode_code % 100 {
            1 => Ok((OpCode::Arithmetic(Operation::Add), 3)),
            2 => Ok((OpCode::Arithmetic(Operation::Multiply), 3)),
            3 => Ok((OpCode::Input, 1)),
            4 => Ok((OpCode::Output, 1)),
            5 => Ok((OpCode::Jump(Box::new(|p| p != 0)), 2)),
            6 => Ok((OpCode::Jump(Box::new(|p| p == 0)), 2)),
            7 => Ok((OpCode::Test(Box::new(|a, b| a < b)), 3)),
            8 => Ok((OpCode::Test(Box::new(|a, b| a == b)), 3)),
            9 => Ok((OpCode::AdjustRelativeBase, 1)),
            99 => Ok((OpCode::Halt, 0)),
            _ => {
                println!(
                    "Unexpected opcode {} (converted: {})",
                    opcode_code,
                    opcode_code % 100
                );
                Err(Error {
                    message: "Unexpected opcode",
                })
            }
        }
    }

    /// Pre-supposing the internal instruction pointer is at the beginning
    /// of a new instruction, parses it, advances the instruction pointer
    /// if needed, and returns the instruction.
    fn parse_instruction(&mut self) -> Result<Instruction> {
        match self.current() {
            Some(opcode_code) => match self.parse_opcode(opcode_code) {
                Ok((opcode, parameters_count)) => {
                    let instruction = Instruction {
                        opcode,
                        parameters: opcode_code
                            .to_string()
                            .chars()
                            .rev()
                            .skip(2)
                            .pad_using(parameters_count, |_| '0')
                            .enumerate()
                            .map(|(i, mode)| Parameter {
                                data: self.offset(i + 1).unwrap(),
                                mode: match mode {
                                    '0' => ParameterMode::Position,
                                    '1' => ParameterMode::Immediate,
                                    '2' => ParameterMode::Relative,
                                    _ => ParameterMode::Position,
                                },
                            })
                            .collect(),
                    };

                    self.pointer += parameters_count + 1;

                    Ok(instruction)
                }
                Err(e) => Err(e),
            },
            None => Err(Error {
                message: "Dangling internal pointer",
            }),
        }
    }
}
