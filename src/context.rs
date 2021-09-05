pub mod execution {

    use self::args::Argument;

    pub struct AppContext {
        start_with: Argument<usize>,
        start_with_char: Argument<char>,
        word_length: Argument<usize>,
        file: Argument<String>,
        size: Argument<u32>,
        available_chars: Vec<char>,
    }

    pub struct AppParameters {
        pub start_with: usize,
        pub start_with_char: usize,
        pub word_length: usize,
        pub file: String,
        pub size: u32,
        pub available_chars: Vec<char>,
    }

    impl AppParameters {
        fn from(context: &AppContext) -> Self {
            Self {
                available_chars: context.available_chars.to_owned(),
                file: context.file.value.to_owned().unwrap(),
                size: context.size.value.unwrap(),
                start_with: context.start_with.value.unwrap(),
                word_length: context.word_length.value.unwrap(),
                start_with_char: context.available_chars
                    .iter()
                    .position(|c|*c == context.start_with_char.value.unwrap())
                    .unwrap_or_default()
            }
        }
    }

    impl AppContext {
        pub fn new() -> Self {
            Self {
                start_with: Argument {
                    keyword: "--start-with",
                    description: "Starts composing words from certain length. Optional.",
                    value: None,
                    default: Some(0)
                },
                start_with_char: Argument {
                    keyword: "--start-with-char",
                    description: "Starting character when composing words. Can be used in combination of --start-with. Optional.",
                    value: None,
                    default: Some(' ')
                },
                word_length: Argument {
                    keyword: "--length",
                    description: "Executes until reaching the word length.",
                    value: None,
                    default: None
                },
                file: Argument {
                    keyword: "--file",
                    description: "The output file name path.",
                    value: None,
                    default: None
                },
                size: Argument {
                    keyword: "--size",
                    description: "Sets the maximum file size in Mb.",
                    value: None,
                    default: None
                },
                available_chars: (32..=127).map(char::from_u32).flatten().collect()
            }
        }

        pub fn get_parameters(
            &mut self,
            params: &Vec<String>,
        ) -> Result<AppParameters, String> {
            self.word_length.try_set(&params)?;
            self.file.try_set(&params)?;
            self.size.try_set(&params)?;
            self.start_with.try_set(&params)?;
            self.start_with_char.try_set(&params)?;
            
            Ok(AppParameters::from(self))
        }

        pub fn print_help(&self) {
            let arguments: Vec<(&str, &str)> = vec![
                (self.word_length.keyword, self.word_length.description),
                (self.file.keyword, self.file.description),
                (self.size.keyword, self.size.description),
                (self.start_with.keyword, self.start_with.description),
            ];

            println!("");
            println!("Welcome to brute-force !!");
            println!("");
            println!("Use the following arguments:");
            println!("");
            for (keyword, description) in arguments {
                println!("\t{: <15}{}", keyword, description);
            }
            println!("");
            println!(
                "Example: # brute-force --length 5 --file /home/bruteforce/dictionary --size 100"
            );
            println!("");
        }
    }

    mod args {
        use std::{ops::Deref};


        pub struct Argument<T> {
            pub keyword: &'static str,
            pub description: &'static str,
            pub value: Option<T>,
            pub default: Option<T>
        }

        pub trait Parse<T> {
            fn parse(&mut self, params: &Vec<String>) -> Result<T, String>;
        }

        impl<T: Clone> Argument<T>
        where
            Argument<T>: Parse<T>,
        {
            pub fn try_set(&mut self, params: &Vec<String>) -> Result<T, String> {
                let result: T = match &self.default {
                    Some(val) => {
                        let def: Result<T, String> = Ok(val.clone());
                        self.parse(params).or(def)?
                    },
                    _ =>  self.parse(params)?
                };
                
                self.value = Some(result.clone());
                Ok(result)
            }

            fn parse_core(&mut self, params: &Vec<String>, inner_func: Box<dyn Fn(&String) ->  Result<T, String>>) -> Result<T, String> {
                match params.iter().position(|arg| arg.as_str() == self.keyword) {
                    Some(position) => match params.get(position + 1) {
                        Some(length) => inner_func.deref()(length),
                        _ => Err(format!("{} value not provided", self.keyword)),
                    },
                    _ => Err(format!("{} not provided", self.keyword)),
                }
            }
        }

        impl Parse<usize> for Argument<usize> {
            fn parse(&mut self, params: &Vec<String>) -> Result<usize, String> {               
                let inner: Box<dyn Fn(&String) ->  Result<usize, String>> = Box::new(|value|match value.parse::<usize>() {
                    Ok(l) => Ok(l),
                    _ => Err(format!("Impossible to parse {} to usize", value)),
                });

                self.parse_core(params, inner)
            }
        }

        impl Parse<String> for Argument<String> {
            fn parse(&mut self, params: &Vec<String>) -> Result<String, String> {
                let inner: Box<dyn Fn(&String) ->  Result<String, String>> = Box::new(|value|match std::fs::File::create(value) {
                    Ok(_) => Ok(value.to_owned()),
                    _ => Err(format!("Impossible to create file {}", value)),
                });

                self.parse_core(params, inner)
            }
        }

        impl Parse<u32> for Argument<u32> {
            fn parse(&mut self, params: &Vec<String>) -> Result<u32, String> {
                let inner: Box<dyn Fn(&String) ->  Result<u32, String>> = Box::new(|value|match value.parse::<u32>() {
                    Ok(l) => Ok(l),
                    _ => Err(format!("Impossible to parse {} to u32", value)),
                });

                self.parse_core(params, inner)
            }
        }

        impl Parse<char> for Argument<char> {
            fn parse(&mut self, params: &Vec<String>) -> Result<char, String> {
                let inner: Box<dyn Fn(&String) ->  Result<char, String>> = Box::new(|value|match value.parse::<char>() {
                    Ok(l) => Ok(l),
                    _ => Err(format!("Impossible to parse {} to char", value)),
                });

                self.parse_core(params, inner)
            }
        }

    }
}
