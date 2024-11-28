use std::fs;

mod util;

static DATA_PATH: &str = "../assets";

fn main() {
    
    if let Ok(lines) = util::read_lines(DATA_PATH.to_owned() + "/example/test") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            for token in line.split(",") {
                println!("{}", token);
            }
        }
    }
}
