use std::{env, io::{self, Write}, path::PathBuf, thread::sleep, time::Duration};
use colored::Colorize;

use crate::StopType::{Long, Medium, VeryLong};

mod file;

enum StopType {
    Short,
    Medium,
    Long,
    VeryLong,
}

fn next_stop_type(word: &str) -> StopType {
    let last_char = match word.chars().last() {
        None => return StopType::Short,
        Some(s) => s,
    };

    let first_char = match word.chars().nth(0) {
        None => return StopType::Short,
        Some(s) => s,
    };

    // TODO: Into switch case
    if last_char == ',' || last_char == ';' {
        return Medium;
    }

    if first_char == '"' || last_char == '"' || last_char == '.' || last_char == ':' {
        return Long;
    }

    if first_char.is_uppercase() {
        return VeryLong;
    }

    StopType::Short
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Provide the filename");
        std::process::exit(1);
    }
    
    let file_path = PathBuf::from(&args[1]);
    let word_itr = file::get_word_iter(file_path.as_path());
    
    if let Err(e) = word_itr {
        eprintln!("{e}");
        std::process::exit(1);
    }

    const FIXED_PAD: u32 = 30; // Adjustable

    // TODO: Use terminal-size

    let itr = word_itr.unwrap();
    for word in itr {
        print!("\x1B[2K");

        for _ in 0..FIXED_PAD {
            print!(" ")
        }

        let anchor = match word.len() {
            a if a <= 3 => 0,
            a if a > 3 && a <= 5 => 1,
            a if a > 5 && a <= 9 => 2,
            a if a < 15 => 3,
            _ => 4,
        };
        
        // Min 1 space for the cursor
        for _ in 0..=(4 - anchor) {
            print!(" ");
        }

        if anchor != 0 {
            print!("{}", &word[..anchor]);
        }

        print!("{}", &word[anchor..=anchor].red().bold());
        print!("{}\r", &word[(anchor + 1)..]);

        io::stdout().flush().unwrap();

        match next_stop_type(&word) {
            StopType::Short => sleep(Duration::from_millis(250)),
            Medium => sleep(Duration::from_millis(550)),
            Long => sleep(Duration::from_millis(750)),
            VeryLong => sleep(Duration::from_millis(950)),
        }
    }
}
