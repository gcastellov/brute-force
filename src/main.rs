use std::fs::*;
use std::{env, io::Write};
use context::execution::AppParameters;
use crate::core::Dictionary;
mod context;
mod core;

fn main() {
    let params: Vec<String> = env::args().skip(1).collect();
    let mut app_context = context::execution::AppContext::new();

    match app_context.get_parameters(&params) {
        Ok(params) => 
            execute(&params),
        Err(e) => {
            println!("{}", e);
            app_context.print_help();
        }
    };
}

fn execute(params: &AppParameters) {
    let mut dictionary = Dictionary::new(params.word_length, params.start_with, params.start_with_char, &params.available_chars);
    let has_space =
        |f: &File| -> bool { f.metadata().unwrap().len() < params.size as u64 * 1000000_u64 };
    let has_not_reached_max_length =
        |d: &mut Dictionary| -> bool { d.word.len() < d.word_length + 1 };
    let mut file_counter: usize = 1;

    println!("Lets go!");

    loop {
        let mut file_name = params.file.to_owned();
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
