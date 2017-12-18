use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

fn lines_from_file<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


mod inverse_captcha;

fn main() {
    use inverse_captcha::*;

    for line in lines_from_file("input.txt") {
        println!("The answer is {}", inverse_captcha_part_2(&line))
    }
}