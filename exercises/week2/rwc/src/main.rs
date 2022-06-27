use std::env;
use std::fs::File;
use std::process;
use std::io::{self, BufRead};

fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let mut vec = Vec::new();
    let file = File::open(filename)?;
    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        vec.push(line_str);
    }
    Ok(vec)
}

fn get_lines(lines: &Vec<String>) -> usize {
    lines.len() - 1
}

fn get_words(lines: &Vec<String>) -> usize {
    let mut word_counter = 0;
    for i in 0..lines.len() {
        word_counter += lines[i].split_whitespace().count();
    }
    word_counter
}

fn get_bytes(filename: &String) -> u64 {
    std::fs::metadata(filename).unwrap().len()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    
    let lines = read_file_lines(filename).expect("filenames is invalid");

    if args.len() == 2 {
        println!(" {}  {} {} {}", get_lines(&lines), get_words(&lines), get_bytes(filename), filename);
    } else {
        match args[2].as_str() {
            "-l" => {
                println!("{} {}", get_lines(&lines), filename)
            }
            "-c" => {
                println!("{} {}", get_bytes(filename), filename)
            }
            "-w" => {
                println!("{} {}", get_words(&lines), filename)
            }
            _ => {
                println!("wrong args!")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::process::Command;
    use super::*;

    #[test]
    fn test_read_file_lines() {
        let lines_result = read_file_lines(&String::from("text"));
        assert!(lines_result.is_ok());
        let lines = lines_result.unwrap();
        assert_eq!(lines.len(), 3);
        assert_eq!(
            lines[0],
            "sss"
        );
    }

    #[test]
    fn test_wc() {
        let output = Command::new("wc").arg("text").output().expect("failed run wc command!");
        // assert_eq!(res.stdout, " 2  5 22 text\n");
        let wc_res = String::from_utf8_lossy(&output.stdout);
        println!("{}", wc_res);
        // TODO get main result and compare
    }
}