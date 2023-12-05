use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::os::raw::c_char;

pub struct BF {
    pub mem: Vec<u8>,
    pub cmds_str: String,
    pub jumps: HashMap<usize, usize>,
    pub ptr: usize,
    pub pc: usize,
}

impl BF {
    pub fn new() -> Self {
        Self {
            mem: vec![0],
            cmds_str: String::new(),
            jumps: HashMap::new(),
            ptr: 0,
            pc: 0,
        }
    }

    pub fn left(&mut self) -> bool {
        if self.ptr == 0 {
            true
        } else {
            self.ptr -= 1;
            false
        }
    }

    pub fn right(&mut self) {
        if self.mem.len() - 1 == self.ptr {
            self.mem.push(0);
        }
        self.ptr += 1;
    }

    pub fn add(&mut self) {
        self.mem[self.ptr] += 1;
    }

    pub fn sub(&mut self) {
        self.mem[self.ptr] -= 1;
    }

    pub fn get_ptr(&self) -> usize {
        self.ptr
    }

    pub fn get_pointed_val(&self) -> u8 {
        self.mem[self.ptr]
    }

    pub fn output(&self) -> c_char {
        self.mem[self.ptr] as c_char
    }

    pub fn print(&self) {
        let c = self.output();
        print!("{}", c);
        if c == '\n' as c_char {
            io::stdout().flush().unwrap();
        }
    }

    pub fn input(&mut self) {
        let mut buf = [0; 1];
        let mut stdin = std::io::stdin().lock();
        stdin.read_exact(&mut buf).unwrap();
        self.mem[self.ptr] = buf[0];
    }

    pub fn point_zero(&self) -> bool {
        self.mem[self.ptr] == 0
    }

    pub fn run_cmd(&mut self, cmd: String) -> Result<(), String> {
        self.cmds_str = cmd;
        self.parse_cmd()?;
        // dbg!(&self.jumps);
        self.run();
        Ok(())
    }

    pub fn parse_cmd(&mut self) -> Result<(), String> {
        let mut stack = vec![];
        for (i, c) in self.cmds_str.as_bytes().iter().enumerate() {
            match c {
                b'[' => stack.push(i),
                b']' => match stack.pop() {
                    Some(start) => {
                        self.jumps.insert(start, i);
                        self.jumps.insert(i, start);
                        // dbg!(&self.jumps);
                    }
                    None => {
                        return Err(format!(
                            "Syntax error: There is no '[' corresponding to ']' at {}",
                            i
                        ))
                    }
                },
                _ => {}
            }
        }
        if stack.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "Syntax error: There is no ']' corresponding to '[' as the end"
            ))
        }
    }

    pub fn run(&mut self) {
        let mut c;
        loop {
            if self.pc == self.cmds_str.len() {
                break;
            }
            c = self.cmds_str.as_bytes()[self.pc];
            // dbg!(&self.mem);
            match c {
                b'>' => {
                    if self.mem.len() - 1 == self.ptr {
                        self.mem.push(0);
                    }
                    self.ptr += 1;
                    self.pc += 1;
                }
                b'<' => {
                    if self.ptr == 0 {
                        break;
                    } else {
                        self.ptr -= 1;
                        self.pc += 1;
                    }
                }
                b'+' => {
                    self.mem[self.ptr] += 1;
                    self.pc += 1
                }
                b'-' => {
                    self.mem[self.ptr] -= 1;
                    self.pc += 1;
                }
                b'.' => {
                    print!("{}", self.output() as u8 as char);
                    io::stdout().flush().unwrap();
                    self.pc += 1;
                }
                b',' => {
                    let mut buf = [0; 1];
                    let mut stdin = std::io::stdin().lock();
                    stdin.read_exact(&mut buf).unwrap();
                    if buf[0] == b'\n' {
                        buf[0] = 0;
                    }
                    self.mem[self.ptr] = buf[0];
                    self.pc += 1;
                }
                b'[' => {
                    if self.mem[self.ptr] == 0 {
                        self.pc = self.jumps[&self.pc];
                    } else {
                        self.pc += 1;
                    }
                }
                b']' => {
                    if self.mem[self.ptr] != 0 {
                        self.pc = self.jumps[&self.pc];
                    } else {
                        self.pc += 1;
                    }
                }
                _ => {
                    self.pc += 1;
                }
            }
        }
    }
}
