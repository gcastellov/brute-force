pub mod execution {

    use self::args::Argument;

    pub struct AppContext {
        word_lenght: Argument<usize>,
        file: Argument<String>,
        size: Argument<u32>,
    }

    impl AppContext {
        pub fn new() -> Self {
            Self {
                word_lenght: Argument {
                    keyword: "--length",
                    description: "Executes until reaching the word length",
                    value: None,
                },
                file: Argument {
                    keyword: "--file",
                    description: "File name",
                    value: None,
                },
                size: Argument {
                    keyword: "--size",
                    description: "Sets the maximum file size in Mb",
                    value: None,
                },
            }
        }

        pub fn get_parameters(
            &mut self,
            params: &Vec<String>,
        ) -> Result<(String, usize, u32), String> {
            self.word_lenght.try_set(&params)?;
            self.file.try_set(&params)?;
            self.size.try_set(&params)?;

            Ok((
                self.file.value.to_owned().unwrap(),
                self.word_lenght.value.unwrap(),
                self.size.value.unwrap(),
            ))
        }

        pub fn print_help(&self) {
            let arguments: Vec<(&str, &str)> = vec![
                (self.word_lenght.keyword, self.word_lenght.description),
                (self.file.keyword, self.file.description),
                (self.size.keyword, self.size.description),
            ];

            println!("");
            println!("¡¡ Welcome to bruteforce !!");
            println!("");
            println!("Use the following arguments:");
            println!("");
            for (keyword, description) in arguments {
                println!("\t{: <15}{}", keyword, description);
            }
            println!("");
            println!(
                "Example: # PwdNextGen --length 5 --file /home/bruteforce/dictionary --size 100"
            );
            println!("");
        }
    }

    mod args {

        pub struct Argument<T> {
            pub keyword: &'static str,
            pub description: &'static str,
            pub value: Option<T>,
        }

        pub trait Parse<T> {
            fn parse(&mut self, params: &Vec<String>) -> Result<T, String>;
        }

        impl<T: Clone> Argument<T>
        where
            Argument<T>: Parse<T>,
        {
            pub fn try_set(&mut self, params: &Vec<String>) -> Result<T, String> {
                let result = self.parse(params)?;
                self.value = Some(result.clone());
                Ok(result)
            }
        }

        impl Parse<usize> for Argument<usize> {
            fn parse(&mut self, params: &Vec<String>) -> Result<usize, String> {
                match params.iter().position(|arg| arg.as_str() == self.keyword) {
                    Some(position) => match params.get(position + 1) {
                        Some(length) => match length.parse::<usize>() {
                            Ok(l) => Ok(l),
                            _ => Err(format!("Impossible to parse {} to usize", length)),
                        },
                        _ => Err(format!("{} value not provided", self.keyword)),
                    },
                    _ => Err(format!("{} not provided", self.keyword)),
                }
            }
        }

        impl Parse<String> for Argument<String> {
            fn parse(&mut self, params: &Vec<String>) -> Result<String, String> {
                match params.iter().position(|arg| arg.as_str() == self.keyword) {
                    Some(position) => match params.get(position + 1) {
                        Some(filename) => match std::fs::File::create(filename) {
                            Ok(_) => {
                                return Ok(filename.to_owned());
                            }
                            _ => Err(format!("Impossible to create file {}", filename)),
                        },
                        _ => Err(format!("{} value not provided", self.keyword)),
                    },
                    _ => Err(format!("{} not provided", self.keyword)),
                }
            }
        }

        impl Parse<u32> for Argument<u32> {
            fn parse(&mut self, params: &Vec<String>) -> Result<u32, String> {
                match params.iter().position(|arg| arg.as_str() == self.keyword) {
                    Some(position) => match params.get(position + 1) {
                        Some(length) => match length.parse::<u32>() {
                            Ok(l) => Ok(l),
                            _ => Err(format!("Impossible to parse {} to u32", length)),
                        },
                        _ => Err(format!("{} value not provided", self.keyword)),
                    },
                    _ => Err(format!("{} not provided", self.keyword)),
                }
            }
        }
    }
}
