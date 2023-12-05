use brainfuck::BF;
use std::env;
use std::fs;

pub mod brainfuck;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let code = fs::read_to_string(filename).expect("open file error");
    let mut bf = BF::new();
    bf.run_cmd(code)?;
    println!("\nend");
    // dbg!(&bf.mem);
    Ok(())
}
