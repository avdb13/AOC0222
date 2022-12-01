use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::path::Path;

fn main() {
    dbg!(calorie_counter("./input"));
}

fn calorie_counter<P: AsRef<Path>>(filename: P) -> u32 {
    let mut cnt: u32 = 0;
    let mut maximum: u32 = 0;

    if let Ok(lines) = read_lines(filename) {
        lines.into_iter().for_each(|l| {
            if let Ok(s) = l {
                match s.is_empty() {
                    true => {
                        if cnt > maximum {
                            maximum = cnt;
                        }
                        cnt = 0;
                    }
                    false => cnt += s.parse::<u32>().unwrap(),
                }
            }
        });
    }

    maximum
}

fn read_lines<P: AsRef<Path>>(filename: P) -> Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
