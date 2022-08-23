use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut problem_file: &str = "src/ProblemFiles/def_probset.csv";
    let mut time_limit: i32 = 30;

    for (i, arg) in args.iter().enumerate() {
        if arg == "--csv" {
            problem_file = &args[i+1];
        } else if arg == "--help" {
            print_help();
            return;
        } else if arg == "--limit" {
            time_limit = args[i+1].parse::<i32>().unwrap();
        }
    }
    
    // Check if file exists
    let problem_file_path = Path::new(problem_file);
    if !problem_file_path.exists() {
        println!("Problem file does not exist!");
        return;
    }
    
    // Check if time limit is valid
    if time_limit <= 0 || time_limit > 10000 {
        println!("Invalid time limit");
    }

    run_quiz(problem_file);
}

fn run_quiz(problem_file: &str) {
    println!("Loading problems from {}", problem_file);

    let mut score: i32 = 0;
    let mut max_possible: i32 = 0;
    
    if let Ok(lines) = read_lines(problem_file) {
        for line in lines {
            if let Ok(line) = line {
                score += ask_ques(&line);
                max_possible += 1;
            }
        }
    }

    println!("Score: {}/{}", score, max_possible);
}

fn ask_ques(line: &str) -> i32 {
    let line_split: Vec<&str> = line.split(",").collect();

    let ques = &line_split[0];
    let ans = line_split[1].parse::<i32>().unwrap();
    
    print!("{}: ", ques);

    io::stdout().flush().unwrap();

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read from stdin");

    let input = input_text.trim().parse::<i32>().unwrap();

    if ans == input {
        return 1 as i32;
    } else {
        return 0 as i32;
    }
}

fn print_help() {
    println!("Usage:\n\
    \t--csv: string\n\
    \t\ta csv file in the format of 'question, answer' (default \"ProblemFiles/def_probset.csv\")\n\
    \t--limit: int (0, 10000]\n\
    \t\tthe time limit for the quiz in seconds (default 30)");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
