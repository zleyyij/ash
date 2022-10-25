use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::fs::canonicalize;
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
        // every item but the first one
        let mut args = parsed_user_input.clone();
        args.remove(0);
        //this is a tad jank, only if no builtin process found do you check for system processes
        //tuple containing all useful information for the builtin commands
        let builtin_tuple = exec_builtin(parsed_user_input.clone());
        //change current dir if new dir was found with the cd command
        if builtin_tuple.1 != String::new() {
            // if relative path, append to current path
            // if absolute change entire path
            if Path::new(&builtin_tuple.1).is_absolute() {
                current_directory = prettify_path(&builtin_tuple.1);
            } else if Path::new(format!("{}/{}", current_directory, builtin_tuple.1).as_str()).exists() {
                current_directory = prettify_path(&format!("{}/{}", current_directory, builtin_tuple.1).to_string());
            } else {
                println!("Directory not found, looked for: {}", builtin_tuple.1);
            }
        }
        if builtin_tuple.0 == false {
            exec_process(parsed_user_input[0], args, &current_directory);
        }
    }
    //assume nothing went wrong if we reach this point
}

// Commands built into the shell (cd, exit)
// we assume that there are no system bianaries by the same name, should probably be resolved at
// some point
fn exec_builtin(inpt: Vec<&str>) -> (bool, String) {
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

fn exec_process(process: &str, args: Vec<&str>, current_dir: &String) {
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
        let _new_process: std::process::ExitStatus = new_process.unwrap();
        println!();
    }
}

//take a "messy" path and clean it up all nice and neat

fn prettify_path(path_to_clean: &String) -> String {
    if Path::new(path_to_clean).exists() {
    //we can unrap because we checked and made sure the path is valid
    return canonicalize(path_to_clean)
                                .unwrap()
                                .into_os_string()
                                .into_string()
                                //I'll fix this if someone actually submits a bug report because
                                //someone has a path that's not valid utf 8
                                .unwrap()
        

    }
    //if it's not a valid path, huck it back and let someone else deal with it

    println!("Directory not found, looked for: {}", path_to_clean);
    path_to_clean.clone()
        
}
