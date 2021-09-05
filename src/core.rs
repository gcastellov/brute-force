pub struct Dictionary {
    pub chars: Vec<char>,
    pub word_length: usize,
    pub counter: u64,
    pub owned: u8,
    pub word: Vec<usize>,
}

impl Dictionary {
    pub fn new(word_lenth: usize, start_with: usize, start_with_char: usize, available_chars: &Vec<char>) -> Dictionary {
        Dictionary {
            word_length: word_lenth,
            chars: available_chars.to_owned(),
            word: (0..start_with+1).map(|_|start_with_char).collect(),
            counter: 0,
            owned: 0,
        }
    }

    pub fn get_current_word(&self) -> String {
        self.word.iter().map(|c| self.chars[*c]).collect()
    }

    pub fn next_word(&mut self) {
        self.counter += 1;
        let mut current = self.word.pop().unwrap();

        if current < self.chars.len() - 1 {
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

#[cfg(test)]
mod tests {
    use super::Dictionary;

    fn get_instance(word_length: usize, start_with: usize, start_with_char: usize) -> Dictionary {
        let available_chars: Vec<char> = (32..128).filter_map(char::from_u32).collect();
        Dictionary::new(word_length, start_with, start_with_char, &available_chars)
    }


    #[test]
    fn given_default_values_when_new_dictionary_then_gets_instance() {
        let available_chars: Vec<char> = (32..128).filter_map(char::from_u32).collect();
        let word_length: usize = 0;
        let start_with: usize = 0;
        let start_with_char: usize = 0;

        let dictionary = Dictionary::new(word_length, start_with, start_with_char, &available_chars);
        
        assert_eq!(available_chars, dictionary.chars);
        assert_eq!(word_length, dictionary.word_length);
        assert_eq!(vec![0], dictionary.word);
        assert_eq!(0, dictionary.counter);
        assert_eq!(0, dictionary.owned);
    }

    #[test]
    fn given_start_with_length_when_new_dictionary_then_gets_instance() {
        let word_length: usize = 0;
        let start_with: usize = 2;
        let start_with_char: usize = 0;

        let dictionary = get_instance(word_length, start_with, start_with_char);

        assert_eq!(word_length, dictionary.word_length);
        assert_eq!(vec![0, 0, 0], dictionary.word);
    }

    #[test]
    fn given_start_with_char_when_new_dictionary_then_gets_instance() {
        let word_length: usize = 0;
        let start_with: usize = 0;
        let start_with_char: usize = 1;

        let dictionary = get_instance(word_length, start_with, start_with_char);

        assert_eq!(word_length, dictionary.word_length);
        assert_eq!(vec![start_with_char], dictionary.word);
    }

    #[test]
    fn given_start_with_length_and_char_when_new_dictionary_then_gets_instance() {
        let word_length: usize = 0;
        let start_with: usize = 2;
        let start_with_char: usize = 2;

        let dictionary = get_instance(word_length, start_with, start_with_char);

        assert_eq!(word_length, dictionary.word_length);
        assert_eq!(vec![start_with_char, start_with_char, start_with_char], dictionary.word);
    }

    #[test]
    fn given_initial_values_when_getting_current_word_then_translates_to_string() {
        let word_length: usize = 0;
        let start_with: usize = 0;
        let start_with_char: usize = 0;

        let dictionary = get_instance(word_length, start_with, start_with_char);
        let actual = dictionary.get_current_word();

        assert_eq!(actual, String::from(" "));
    }

    #[test]
    fn given_start_with_char_values_when_getting_current_word_then_translates_to_string() {
        let word_length: usize = 0;
        let start_with: usize = 0;
        let start_with_char: usize = 1;

        let dictionary = get_instance(word_length, start_with, start_with_char);
        let actual = dictionary.get_current_word();

        assert_eq!(actual, String::from("!"));
    }

    #[test]
    fn given_start_with_length_and_char_values_when_getting_current_word_then_translates_to_string() {
        let word_length: usize = 0;
        let start_with: usize = 2;
        let start_with_char: usize = 1;

        let dictionary = get_instance(word_length, start_with, start_with_char);
        let actual = dictionary.get_current_word();

        assert_eq!(actual, String::from("!!!"));
    }

    #[test]
    fn given_start_with_length_and_char_values_when_getting_next_word_then_sets_representation() {
        let word_length: usize = 0;
        let start_with: usize = 2;
        let start_with_char: usize = 1;

        let mut dictionary = get_instance(word_length, start_with, start_with_char);
        dictionary.next_word();

        assert_eq!(vec![start_with_char, start_with_char, 2], dictionary.word);
    }
}
