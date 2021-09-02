use std::fs::*;
use std::{env, io::Write};
mod context;

struct Dictionary {
    chars: Vec<char>,
    word_length: usize,
    counter: u64,
    owned: u8,
    word: Vec<usize>,
}

impl Dictionary {
    fn get_allowed_chars() -> Vec<char> {
        (32..=127).map(char::from_u32).flatten().collect()
    }

    fn new(word_lenth: usize, start_with: usize) -> Dictionary {
        Dictionary {
            word_length: word_lenth,
            chars: Dictionary::get_allowed_chars(),
            word: (0..start_with+1).map(|_|0).collect(),
            counter: 0,
            owned: 0,
        }
    }

    fn get_current_word(&self) -> String {
        self.word.iter().map(|c| self.chars[*c]).collect()
    }

    fn next_word(&mut self) {
        self.counter += 1;
        let mut current = self.word.pop().unwrap();

        if current == self.chars.len() - 1 {
            current += 1;
            self.word.push(current);
        } else if self.word.is_empty() {
            self.word.push(0);
            self.word.push(0);
        } else {
            self.owned += 1;
            let mut last = *self.word.last().unwrap();
            while !self.word.is_empty() && last == self.chars.len() - 1 {
                self.word.pop();
                self.owned += 1;
                if !self.word.is_empty() {
                    last = *self.word.last().unwrap();
                }
            }

            let value = match self.word.pop() {
                Some(last) => last + 1,
                _ => 0,
            };

            self.word.push(value);

            if self.owned > 0 {
                for _ in 0..self.owned {
                    self.word.push(0);
                }
                self.owned = 0;
            }
        }
    }
}

fn main() {
    let params: Vec<String> = env::args().skip(1).collect();
    let mut app_context = context::execution::AppContext::new();

    match app_context.get_parameters(&params) {
        Ok((file, max_length, start_with, size)) => 
            execute(max_length, start_with, file, size),
        Err(e) => {
            println!("{}", e);
            app_context.print_help();
        }
    };
}

fn execute(max_length: usize, start_with: usize, filename: String, file_size: u32) {
    let mut dictionary = Dictionary::new(max_length, start_with);
    let has_space =
        |f: &File| -> bool { f.metadata().unwrap().len() < file_size as u64 * 1000000_u64 };
    let has_not_reached_max_length =
        |d: &mut Dictionary| -> bool { d.word.len() < d.word_length + 2 };
    let mut file_counter: usize = 1;

    println!("Lets go!");

    loop {
        let mut file_name = filename.to_owned();
        if file_counter > 1 {
            file_name.push_str(format!(".{:0>3}", file_counter).as_str());
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_name.to_owned())
            .expect(format!("Impossible to open file {}", file_name).as_str());

        while has_space(&file) && has_not_reached_max_length(&mut dictionary) {
            let word = dictionary.get_current_word();
            let _ = file.write(word.as_bytes());
            let _ = file.write(&[0x0a]);

            dictionary.next_word();

            print!("\r");
            print!("{} -> {}", dictionary.counter, word);
        }

        if !has_not_reached_max_length(&mut dictionary) {
            break;
        }

        file_counter += 1;
    }
}
