use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Based on https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

fn main() {
    // File hosts must exist in current path before this produces output
    let mut cal_vec: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut calories = 0;
        for line in lines {
            let cal: String = line.expect("exists");
            if cal.eq(""){
                cal_vec.push(calories);
                calories = 0;
            } else {
                calories = calories + cal.parse::<i32>().unwrap();
            }
        }

        let max_cal = cal_vec.iter().max().unwrap();
        // part 1
        println!("{}", max_cal);
        cal_vec.sort();
        // part 2
        println!("{}", cal_vec.iter().rev().take(3).sum::<i32>());
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
