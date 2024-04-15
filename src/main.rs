use std::fs;

fn main() {
    if let Ok(lines) = fs::read_to_string("c:/focus-logs.csv") {
        print!("{}", lines);
    }
}
