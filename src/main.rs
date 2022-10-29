use std::fs::canonicalize;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};
fn main() {
    println!("Loaded");
    // the string that stores stdin
    let mut user_input = String::new();
    let mut current_directory = String::from("/");
    //load config files here if ever used

    //loop here
    loop {
        //get user input as a string and strip the newline character

        let prompt = String::from(format!("[{}]> ", current_directory));
        user_input = read_input(prompt);
        user_input.truncate(user_input.len() - 1);
        let parsed_user_input: Vec<&str> = user_input.split(" ").collect();
        // // every item but the first one
        // let mut args = parsed_user_input.clone();
        // args.remove(0);
        //this is a tad jank, only if no builtin process found do you check for system processes
        //tuple containing all useful information for the builtin commands
        //         let builtin_tuple = exec_builtin(parsed_user_input.clone());
        //         //change current dir if new dir was found with the cd command
        //         if builtin_tuple.1 != String::new() {
        //             // if relative path, append to current path
        //             // if absolute change entire path
        //             if Path::new(&builtin_tuple.1).is_absolute() {
        //                 current_directory = prettify_path(&builtin_tuple.1);
        //             } else if Path::new(format!("{}/{}", current_directory, builtin_tuple.1).as_str())
        //                 .exists()
        //             {
        //                 current_directory = prettify_path(

        //             &format!("{}/{}", current_directory, builtin_tuple.1).to_string(),
        //                 );
        //             } else {
        //                 println!("Directory not found, looked for: {}", builtin_tuple.1);
        //             }
        //         }
        //         if builtin_tuple.0 == false {
        //             exec_process(parsed_user_input[0], args, &current_directory);
        // }
        //     }
        handle_input(parsed_user_input, &current_directory);
        //assume nothing went wrong if we reach this point
    }

    // Commands built into the shell (cd, exit)
    // we assume that there are no system bianaries by the same name, should probably be resolved at
    // some point
    pub fn exec_builtin(inpt: Vec<&str>) -> (bool, String) {
        let builtin: Vec<&str> = vec!["cd", "exit", "gnowo"];
        //return tuple
        //first value is if found a builtin command, second value is the path for the cd command
        let mut return_tuple: (bool, String) = (false, String::new());
        // get the command specified by taking the first "word"
        let command: &str = inpt[0];
        for i in builtin {
            if command == i {
                return_tuple.0 = true;
            }
        }
        //if no builtin commands are found, return
        if !return_tuple.0 {
            return return_tuple;
        }

        match command {
            "test" => println!("test"),
            "cd" => {
                // if Path::new(inpt[1]).exists() {
                //          //set the new path
                //          return_tuple.1 = inpt[1].to_owned();
                //      } else {
                //          println!("Directory not found, looked for: {}", inpt[1]);
                //      }
                return_tuple.1 = inpt[1].to_owned();
            }
            _ => return return_tuple,
        };
        return return_tuple;
    }

    // obtain standard input as a string
    fn read_input(prompt: String) -> String {
        let mut input = String::new();
        print!("{}", prompt);
        //ensure output is printed immediately
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        stdin.read_line(&mut input).unwrap();
        input
    }

    //start system processes
    pub fn exec_process(
        process: &str,
        args: Vec<&str>,
        current_dir: &String,
    ) -> Option<std::process::ExitStatus> {
        let new_process = Command::new(process)
            .args(args)
            .current_dir(current_dir)
            .status();
        let new_process: Option<std::process::ExitStatus> = match new_process {
            Ok(prcs) => Some(prcs),
            Err(_error) => {
                println!("Command not found");
                None
            }
        };

        if new_process != None {
            let new_process: std::process::ExitStatus = new_process.unwrap();
            //println!();
            return Some(new_process);
        }
        None
    }

    // Spltting the user input up, running necessary commands, redirecting and piping stdio, general
    // processing
    pub fn handle_input(input_as_vec: Vec<&str>, current_dir: &String) {
        // every item but the first one
        //let mut args = parsed_user_input.clone();
        //args.remove(0);

        // we want to mutate this at some point
        let mut input_vec: Vec<&str> = input_as_vec.clone();

        //check the first vec for a command to run, and the second vec to see what to do with the
        //stdio
        // A vector of valid commmands
        let mut commands: Vec<Vec<&str>> = vec![];

        // a vector of control characters found in the input, EG(; | > &&)
        let mut control_chars: Vec<Option<&str>> = vec![];

        //list of every implemented control char to search for
        let control_chars_list = ["|", ">", ";"];

        //search through for valid control chars,
        //take everything before it as a command
        //then put each into the arrays and remove from the input vec

        for i in control_chars_list {
            while input_vec.len() > 0 {
                let index_of_control_char = input_vec.iter().position(|&x| x == i.to_string());
                //if a control char exists, take everything before it as a command
                //take the char itself and dump it in an array
                match index_of_control_char {
                    Some(index) => {
                        control_chars.push(Some(input_vec[index]));
                        let mut command_vec: Vec<&str> = vec![];
                        for i in input_vec[..index].iter() {
                            command_vec.push(i);
                        }
                        commands.push(command_vec);
                        //remove the items already processed
                        input_vec.drain(..index + 1);
                    }
                    None => {
                        //we assume there are no control chars here,
                        //and dump the entire vec into the command vector
                        commands.push(input_vec.clone());
                        control_chars.push(None);
                        input_vec = vec![];
                    }
                }
            }
        }

        //loop through the arrays of processed input, and handle accordingly
        for i in 0..control_chars.len() {
            match control_chars[i] {
                None => {
                    let mut args = commands[i].clone();
                    args.remove(0);
                    exec_process(commands[i][0], args, &current_dir);
                }
                Some("|") => {}
                Some(_) => {
                    panic!("wtf happened");
                }
            }
        }

        //this is a tad jank, only if no builtin process found do you check for system processes
        //tuple containing all useful information for the builtin commands
        let builtin_tuple = exec_builtin(input_as_vec.clone());

        //change current dir if new dir was found with the cd command
        let mut current_dir = current_dir.clone();

        if builtin_tuple.1 != String::new() {
            // if relative path, append to current path
            // if absolute change entire path
            if Path::new(&builtin_tuple.1).is_absolute() {
                current_dir = prettify_path(&builtin_tuple.1);
            } else if Path::new(format!("{}/{}", current_dir, builtin_tuple.1).as_str()).exists() {
                current_dir =
                    prettify_path(&format!("{}/{}", current_dir, builtin_tuple.1).to_string());
            } else {
                println!("Directory not found, looked for: {}", builtin_tuple.1);
            }
        }
        if builtin_tuple.0 == false {
            //   exec_process(parsed_user_input[0], args, &current_directory);
        }
    }
}

//take a "messy" path and clean it up all nice and neat
//eg: //./home/../home/./. is technically valid

pub fn prettify_path(path_to_clean: &String) -> String {
    if Path::new(path_to_clean).exists() {
        //we can unrap because we checked and made sure the path is valid
        return canonicalize(path_to_clean)
            .unwrap()
            .into_os_string()
            .into_string()
            //I'll fix this if someone actually submits a bug report because
            //someone has a path that's not valid utf 8
            .unwrap();
    }
    //if it's not a valid path, huck it back and let someone else deal with it

    println!("Directory not found, looked for: {}", path_to_clean);
    path_to_clean.clone()
}