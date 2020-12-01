use std::io::BufReader;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    amain().unwrap();
}

fn amain() -> Result<(), Box<dyn Error>> {
    let file = File::open("../input.txt")?;
    let buf_read = BufReader::new(file);
    let numbers: Vec<_> = buf_read.lines()
        .map(|line| line.unwrap().parse::<isize>().unwrap()).collect();
    'out: for (i, n) in numbers.iter().enumerate() {
        for m in numbers.iter().skip(i+1) {
            if (n+m) == 2020 {
                println!("1: {} * {}  = {}", n, m, n*m);
                break 'out;
            }
        }
    }
    'out2: for (i, n) in numbers.iter().enumerate() {
        for (j, m) in numbers.iter().skip(i+1).enumerate() {
            for o in numbers.iter().skip(j+1) {
                if (n+m+o) == 2020 {
                    println!("2: {} * {} * {} = {}", n, m, o, n*m*o);
                    break 'out2;
                }
            }
        }
    }
    Result::Ok(())
}
