pub mod execution {

    use self::args::Argument;

    pub struct AppContext {
        start_with: Argument<usize>,
        start_with_char: Argument<char>,
        word_length: Argument<usize>,
        file: Argument<String>,
        size: Argument<u32>,
        verbose: Argument<bool>,
        available_chars: Vec<char>,
    }

    pub struct AppParameters {
        pub start_with: usize,
        pub start_with_char: usize,
        pub word_length: usize,
        pub file: String,
        pub size: u32,
        pub verbose: bool,
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
                verbose: context.verbose.value.unwrap(),
                start_with_char: context
                    .available_chars
                    .iter()
                    .position(|c| *c == context.start_with_char.value.unwrap())
                    .unwrap_or_default(),
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
                verbose: Argument {
                    keyword: "--verbose",
                    description: "Indicates whether it has to show output info or not. Optional. By default is set to false.",
                    value: None,
                    default: Some(false)
                },
                available_chars: (32..=127).map(char::from_u32).flatten().collect()
            }
        }

        pub fn get_parameters(&mut self, params: &Vec<String>) -> Result<AppParameters, String> {
            self.word_length.try_set(&params)?;
            self.file.try_set(&params)?;
            self.size.try_set(&params)?;
            self.start_with.try_set(&params)?;
            self.start_with_char.try_set(&params)?;
            self.verbose.try_set(&params)?;

            Ok(AppParameters::from(self))
        }

        pub fn print_help(&self) {
            let arguments: Vec<(&str, &str)> = vec![
                (self.word_length.keyword, self.word_length.description),
                (self.file.keyword, self.file.description),
                (self.size.keyword, self.size.description),
                (self.start_with.keyword, self.start_with.description),
                (
                    self.start_with_char.keyword,
                    self.start_with_char.description,
                ),
                (self.verbose.keyword, self.verbose.description),
            ];

            println!("");
            println!("Welcome to brute-force !!");
            println!("");
            println!("Use the following arguments:");
            println!("");
            for (keyword, description) in arguments {
                println!("\t{: <20}{}", keyword, description);
            }
            println!("");
            println!(
                "Example: # brute-force --length 5 --file /home/bruteforce/dictionary --size 100"
            );
            println!("");
        }
    }

    mod args {
        use std::ops::Deref;

        pub struct Argument<T> {
            pub keyword: &'static str,
            pub description: &'static str,
            pub value: Option<T>,
            pub default: Option<T>,
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
                    }
                    _ => self.parse(params)?,
                };

                self.value = Some(result.clone());
                Ok(result)
            }

            fn parse_core(
                &mut self,
                params: &Vec<String>,
                inner_func: Box<dyn Fn(&String) -> Result<T, String>>,
            ) -> Result<T, String> {
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
                let inner: Box<dyn Fn(&String) -> Result<usize, String>> =
                    Box::new(|value| match value.parse::<usize>() {
                        Ok(l) => Ok(l),
                        _ => Err(format!("Impossible to parse {} to usize", value)),
                    });

                self.parse_core(params, inner)
            }
        }

        impl Parse<String> for Argument<String> {
            fn parse(&mut self, params: &Vec<String>) -> Result<String, String> {
                let inner: Box<dyn Fn(&String) -> Result<String, String>> =
                    Box::new(|value| match std::fs::File::create(value) {
                        Ok(_) => Ok(value.to_owned()),
                        _ => Err(format!("Impossible to create file {}", value)),
                    });

                self.parse_core(params, inner)
            }
        }

        impl Parse<u32> for Argument<u32> {
            fn parse(&mut self, params: &Vec<String>) -> Result<u32, String> {
                let inner: Box<dyn Fn(&String) -> Result<u32, String>> =
                    Box::new(|value| match value.parse::<u32>() {
                        Ok(l) => Ok(l),
                        _ => Err(format!("Impossible to parse {} to u32", value)),
                    });

                self.parse_core(params, inner)
            }
        }

        impl Parse<char> for Argument<char> {
            fn parse(&mut self, params: &Vec<String>) -> Result<char, String> {
                let inner: Box<dyn Fn(&String) -> Result<char, String>> =
                    Box::new(|value| match value.parse::<char>() {
                        Ok(l) => Ok(l),
                        _ => Err(format!("Impossible to parse {} to char", value)),
                    });

                self.parse_core(params, inner)
            }
        }

        impl Parse<bool> for Argument<bool> {
            fn parse(&mut self, params: &Vec<String>) -> Result<bool, String> {
                let inner: Box<dyn Fn(&String) -> Result<bool, String>> =
                    Box::new(|value| match value.parse::<bool>() {
                        Ok(l) => Ok(l),
                        _ => Err(format!("Impossible to parse {} to boolean", value)),
                    });

                self.parse_core(params, inner)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use spectral::prelude::*;
        use super::*;

        #[test]
        fn given_boolean_argument_is_present_when_parsed_then_gets_proper_value() {
            let params = vec![String::from("--arg1"), String::from("true")];
            let mut argument: Argument<bool> = Argument {
                keyword: "--arg1",
                description: "",
                default: Some(false),
                value: None,
            };

            let actual = argument.try_set(&params);

            assert_that(&actual).is_ok();
            assert_that(&actual.unwrap()).is_equal_to(true);
        }

        #[test]
        fn given_boolean_argument_is_present_with_only_keyword_when_parsed_then_gets_default_value() {
            let params = vec![String::from("--arg1")];
            let mut argument: Argument<bool> = Argument {
                keyword: "--arg1",
                description: "",
                default: Some(false),
                value: None,
            };

            let actual = argument.try_set(&params);

            assert_that(&actual).is_ok();
            assert_that(&actual.unwrap()).is_equal_to(false);
        }

        #[test]
        fn given_boolean_argument_is_not_present_when_parsed_then_gets_default_value() {
            let params = vec![];
            let mut argument: Argument<bool> = Argument {
                keyword: "--arg1",
                description: "",
                default: Some(false),
                value: None,
            };

            let actual = argument.try_set(&params);

            assert_that(&actual).is_ok();
            assert_that(&actual.unwrap()).is_equal_to(false);
        }

        #[test]
        fn given_boolean_argument_is_incorrect_with_non_default_value_when_parsed_then_gets_error() {
            let params = vec![String::from("--arg1"), String::from("123")];
            let mut argument: Argument<bool> = Argument {
                keyword: "--arg1",
                description: "",
                default: None,
                value: None,
            };

            let actual = argument.try_set(&params);

            assert_that(&actual).is_err();
        }

        #[test]
        fn given_boolean_argument_is_incorrect_with_default_value_when_parsed_then_gets_default_value() {
            let params = vec![String::from("--arg1"), String::from("123")];
            let mut argument: Argument<bool> = Argument {
                keyword: "--arg1",
                description: "",
                default: Some(true),
                value: None,
            };

            let actual = argument.try_set(&params);

            assert_that(&actual).is_ok();
            assert_that(&actual.unwrap()).is_equal_to(true);
        }

        #[test]
        fn given_u32_argument_is_present_when_parsed_then_gets_proper_value() {
            let params = vec![String::from("--arg1"), String::from("64")];
            let mut argument: Argument<u32> = Argument {
                keyword: "--arg1",
                description: "",
                default: None,
                value: None,
            };

            let actual = argument.try_set(&params);

            assert_that(&actual).is_ok();
            assert_that(&actual.unwrap()).is_equal_to(64);
        }

        #[test]
        fn given_u32_argument_is_present_with_only_keyword_when_parsed_then_gets_default_value() {
            let params = vec![String::from("--arg1")];
            let mut argument: Argument<u32> = Argument {
                keyword: "--arg1",
                description: "",
                default: Some(32),
                value: None,
            };

            let actual = argument.try_set(&params);

            assert_that(&actual).is_ok();
            assert_that(&actual.unwrap()).is_equal_to(32);
        }

        #[test]
        fn given_u32_argument_is_not_present_when_parsed_then_gets_default_value() {
            let params = vec![];
            let mut argument: Argument<u32> = Argument {
                keyword: "--arg1",
                description: "",
                default: Some(16),
                value: None,
            };

            let actual = argument.try_set(&params);

            assert_that(&actual).is_ok();
            assert_that(&actual.unwrap()).is_equal_to(16);
        }

        #[test]
        fn given_u32_argument_is_incorrect_with_non_default_value_when_parsed_then_gets_error() {
            let params = vec![String::from("--arg1"), String::from("something")];
            let mut argument: Argument<bool> = Argument {
                keyword: "--arg1",
                description: "",
                default: None,
                value: None,
            };

            let actual = argument.try_set(&params);

            assert_that(&actual).is_err();
        }

        #[test]
        fn given_u32_argument_is_incorrect_with_default_value_when_parsed_then_gets_default_value() {
            let params = vec![String::from("--arg1"), String::from("something")];
            let mut argument: Argument<u32> = Argument {
                keyword: "--arg1",
                description: "",
                default: Some(8),
                value: None,
            };

            let actual = argument.try_set(&params);

            assert_that(&actual).is_ok();
            assert_that(&actual.unwrap()).is_equal_to(8);
        }

        #[test]
        fn given_char_argument_is_present_when_parsed_then_gets_proper_value() {
            let params = vec![String::from("--arg1"), String::from("a")];
            let mut argument: Argument<char> = Argument {
                keyword: "--arg1",
                description: "",
                default: None,
                value: None,
            };

            let actual = argument.try_set(&params);

            assert_that(&actual).is_ok();
            assert_that(&actual.unwrap()).is_equal_to('a');
        }

        #[test]
        fn given_char_argument_is_present_with_only_keyword_when_parsed_then_gets_default_value() {
            let params = vec![String::from("--arg1")];
            let mut argument: Argument<char> = Argument {
                keyword: "--arg1",
                description: "",
                default: Some('b'),
                value: None,
            };

            let actual = argument.try_set(&params);

            assert_that(&actual).is_ok();
            assert_that(&actual.unwrap()).is_equal_to('b');
        }

        #[test]
        fn given_char_argument_is_not_present_when_parsed_then_gets_default_value() {
            let params = vec![];
            let mut argument: Argument<char> = Argument {
                keyword: "--arg1",
                description: "",
                default: Some('c'),
                value: None,
            };

            let actual = argument.try_set(&params);

            assert_that(&actual).is_ok();
            assert_that(&actual.unwrap()).is_equal_to('c');
        }

        #[test]
        fn given_char_argument_is_incorrect_with_non_default_value_when_parsed_then_gets_error() {
            let params = vec![String::from("--arg1"), String::from("something")];
            let mut argument: Argument<char> = Argument {
                keyword: "--arg1",
                description: "",
                default: None,
                value: None,
            };

            let actual = argument.try_set(&params);

            assert_that(&actual).is_err();
        }

        #[test]
        fn given_char_argument_is_incorrect_with_default_value_when_parsed_then_gets_default_value() {
            let params = vec![String::from("--arg1"), String::from("something")];
            let mut argument: Argument<char> = Argument {
                keyword: "--arg1",
                description: "",
                default: Some('d'),
                value: None,
            };

            let actual = argument.try_set(&params);

            assert_that(&actual).is_ok();
            assert_that(&actual.unwrap()).is_equal_to('d');
        }

        #[test]
        fn given_usize_argument_is_present_when_parsed_then_gets_proper_value() {
            let params = vec![String::from("--arg1"), String::from("64")];
            let mut argument: Argument<usize> = Argument {
                keyword: "--arg1",
                description: "",
                default: None,
                value: None,
            };

            let actual = argument.try_set(&params);


            assert_that(&actual).is_ok();
            assert_that(&actual.unwrap()).is_equal_to(64);
        }

        #[test]
        fn given_usize_argument_is_present_with_only_keyword_when_parsed_then_gets_default_value() {
            let params = vec![String::from("--arg1")];
            let mut argument: Argument<usize> = Argument {
                keyword: "--arg1",
                description: "",
                default: Some(32),
                value: None,
            };

            let actual = argument.try_set(&params);


            assert_that(&actual).is_ok();
            assert_that(&actual.unwrap()).is_equal_to(32);
        }

        #[test]
        fn given_usize_argument_is_not_present_when_parsed_then_gets_default_value() {
            let params = vec![];
            let mut argument: Argument<usize> = Argument {
                keyword: "--arg1",
                description: "",
                default: Some(16),
                value: None,
            };

            let actual = argument.try_set(&params);

            assert_that(&actual).is_ok();
            assert_that(&actual.unwrap()).is_equal_to(16);
        }

        #[test]
        fn given_usize_argument_is_incorrect_with_non_default_value_when_parsed_then_gets_error() {
            let params = vec![String::from("--arg1"), String::from("something")];
            let mut argument: Argument<usize> = Argument {
                keyword: "--arg1",
                description: "",
                default: None,
                value: None,
            };

            let actual = argument.try_set(&params);

            assert_that(&actual).is_err();
        }

        #[test]
        fn given_usize_argument_is_incorrect_with_default_value_when_parsed_then_gets_default_value() {
            let params = vec![String::from("--arg1"), String::from("something")];
            let mut argument: Argument<usize> = Argument {
                keyword: "--arg1",
                description: "",
                default: Some(8),
                value: None,
            };

            let actual = argument.try_set(&params);

            assert_that(&actual).is_ok();
            assert_that(&actual.unwrap()).is_equal_to(8);
        }

        #[test]
        fn when_new_then_gets_appcontext_instance() {
            let actual = AppContext::new();
                        
            assert_that(&actual.available_chars).has_length(96);
            assert_that(&actual.start_with.keyword).is_equal_to("--start-with");
            assert_that(&actual.start_with_char.keyword).is_equal_to("--start-with-char");
            assert_that(&actual.file.keyword).is_equal_to("--file");
            assert_that(&actual.size.keyword).is_equal_to("--size");
            assert_that(&actual.word_length.keyword).is_equal_to("--length");
            assert_that(&actual.verbose.keyword).is_equal_to("--verbose");
        }

        #[test]
        fn given_app_context_when_from_then_gets_parameters() {

            const WORD_LENGTH: usize = 8;
            const FILE: &'static str = "/home/brute-force/output";
            const SIZE: u32 = 10;
            const START_WITH: usize = 2;
            const START_WITH_CHAR: char = 'b';
            const VERBOSE: bool = true;

            let mut app_context = AppContext::new();
            app_context.word_length.value = Some(WORD_LENGTH);
            app_context.file.value = Some(FILE.to_string());
            app_context.size.value = Some(SIZE);
            app_context.start_with.value = Some(START_WITH);
            app_context.start_with_char.value = Some(START_WITH_CHAR);
            app_context.verbose.value = Some(true);

         
            let actual = AppParameters::from(&app_context);
            
            assert_that(&actual.word_length).is_equal_to(WORD_LENGTH);
            assert_that(&actual.file).is_equal_to(FILE.to_string());
            assert_that(&actual.size).is_equal_to(SIZE);
            assert_that(&actual.start_with).is_equal_to(START_WITH);
            assert_that(&actual.start_with_char).is_equal_to(66);
            assert_that(&actual.verbose).is_equal_to(VERBOSE);
        }

    }
}
