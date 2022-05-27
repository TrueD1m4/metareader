#[derive(Debug)]
pub enum OutputType {
    Console,
    File
}

#[derive(Debug)]
pub struct Arguments {
    pub output_type: OutputType,
    pub filepath: String,
    pub output_file: Option<String>
}

impl Arguments {
    pub fn new(arguments: &Vec<String>) -> Result<Arguments, String> {
        let mut args: Arguments = Arguments {
            output_type: OutputType::Console,
            filepath: "".to_string(),
            output_file: None
        };

        if arguments.len() < 2 {
            Err(String::from("Not enough arguments!"))
        } else if arguments.len() > 4 {
            Err(String::from("To much arguments!"))
        } else {

            let normal_length = arguments.len() == 3;
            match arguments[1].as_str() {
                "-h" => return Err(String::from("Help")),
                "help" => return Err(String::from("Help")),
                "-c" => {
                    if normal_length {
                        args.output_type = OutputType::Console;
                        args.output_file = None;
                    } else {
                        return Err(String::from("Invalid syntax!"));
                    }
                },
                "console" => {
                    if normal_length {
                        args.output_type = OutputType::Console;
                        args.output_file = None;
                    } else {
                        return Err(String::from("Invalid syntax!"));
                    }
                },
                "-f" => {
                    Self::check_file_argument(arguments, &mut args, normal_length);
                },
                "file" => {
                    Self::check_file_argument(arguments, &mut args, normal_length);
                }
                _ => {
                    args.output_type = OutputType::Console;
                    args.filepath = arguments[1].clone();
                    args.output_file = None;
                }
            };

            Ok(args)
        }

    }

    fn check_file_argument(arguments: &Vec<String>, args: &mut Arguments, normal_length: bool) {
        if normal_length {
            args.output_type = OutputType::File;
            args.output_file = Some(String::from("Output.txt"));
        } else if arguments.len() == 4 {
            args.output_type = OutputType::File;
            args.output_file = Some(arguments[3].clone());
        }
    }
}