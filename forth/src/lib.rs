use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    mem: Vec<Value>,
    program: HashMap<String, String>,
}

macro_rules! try_pop {
    ($var:ident) => {
        $var.mem.pop().ok_or(Error::StackUnderflow)?
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            mem: vec![],
            program: Default::default(),
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.mem.clone()
    }

    fn arithmetic_operation(&mut self, op: char) -> Result<(), Error> {
        let (y, x) = (try_pop!(self), try_pop!(self));

        match op {
            '+' => self.mem.push(x + y),
            '-' => self.mem.push(x - y),
            '*' => self.mem.push(x * y),
            '/' => {
                if y == 0 {
                    return Err(Error::DivisionByZero);
                }
                self.mem.push(x / y)
            }
            _ => panic!()
        }

        Ok(())
    }

    fn program_interpreter(&self, command: String) -> String {
        let mut interpreted_command = command;
        for (word, prog) in self.program.iter() {
            interpreted_command = interpreted_command.replace(word, prog.as_str());
        }

        interpreted_command
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let input = input.to_lowercase();

        if input.chars().nth(0).unwrap() == ':' {
            let input: Vec<String> = input.split_whitespace().map(String::from).collect();

            if input.len() >= 4 && input.last() == Some(&String::from(";")) {
                let word: String = input.iter().nth(1).unwrap().to_string();

                if word.parse::<i16>().is_ok() {
                    return Err(Error::InvalidWord)
                }

                let program = input.iter()
                    .skip(2)
                    .filter(|s| **s != String::from(";"))
                    .fold(String::new(), |acc, c| {
                        format!("{} {}", acc, c)
                    });

                let program = self.program_interpreter(program);
                self.program.insert(word, program);

                return Ok(());
            } else {
                return Err(Error::InvalidWord);
            }
        }
        let input = self.program_interpreter(input);

        for arg in input.split_whitespace() {
            match arg {
                _ if arg.parse::<i16>().is_ok() => {
                    self.mem.push(arg.parse().unwrap())
                }
                "+" | "-" | "*" | "/" => self.arithmetic_operation(arg.chars().nth(0).unwrap())?,
                "dup" => {
                    let last_value = self.mem.last();
                    let last_value = match last_value {
                        Some(x) => *x,
                        None => {
                            return Err(Error::StackUnderflow);
                        }
                    };

                    self.mem.push(last_value);
                }
                "drop" => {
                    try_pop!(self);
                }
                "swap" => {
                    let (x, y) = (try_pop!(self), try_pop!(self));

                    self.mem.push(x);
                    self.mem.push(y);
                }
                "over" => {
                    let second_last_value = self.mem.iter().rev().nth(1);
                    let second_last_value = match second_last_value {
                        Some(x) => *x,
                        None => {
                            return Err(Error::StackUnderflow);
                        }
                    };

                    self.mem.push(second_last_value);
                }
                _ => {
                    return Err(Error::UnknownWord);
                }
            }
        }

        Ok(())
    }
}
