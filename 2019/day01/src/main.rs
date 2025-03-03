use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut sum: usize = 0;
    let mut que = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mass: usize = line.parse().unwrap();
        if let Some(mass) = (mass / 3).checked_sub(2) {
            sum = sum.saturating_add(mass);
            que.push(mass);
        }
    }
    while ! que.is_empty() {
        let mass = que.pop().unwrap();
        if let Some(mass) = (mass / 3).checked_sub(2) {
            sum = sum.saturating_add(mass);
            que.push(mass);
        }
    }
    println!("{sum}");
    Ok(())
}
