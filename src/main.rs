use std::cell::RefCell;
use std::fs::canonicalize;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};
//third party crates

fn main() {
    println!("Loaded");
    // the string that stores stdin
    let mut user_input: String;
    let mut current_directory = String::from("/");
    //load config files here if ever used

    //loop here
    loop {
        //get user input as a string and strip the newline character

        let prompt = String::from(format!("[{}]> ", current_directory));
        user_input = read_input(prompt);
        // remove newline character so that it doesn't get passed as an argument
        user_input.truncate(user_input.len() - 1);
        let parsed_user_input: Vec<String> = parse_user_input(user_input);
        //rust is quirky about the ownership of &str, so it's hard to pass it from a function, so we convert to and from a str
        let parsed_user_input: Vec<&str> = parsed_user_input.iter().map(|s| s as &str).collect();

        current_directory = handle_input(parsed_user_input, &current_directory);
        //assume nothing went wrong if we reach this point
    }
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
            if inpt.len() > 1 {
                //if Path::new(inpt[1]).exists() {
                //set the new path
                return_tuple.1 = inpt[1].to_owned();
                //}
            }
        }
        //catchall to satisfy the compiler
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
pub fn exec_process(process: &str, args: Vec<&str>, current_dir: &String) {
    //don't touch stdio if there isn't anything to do

    let new_process = Command::new(process)
        .args(args)
        .current_dir(current_dir)
        .status();
    //nuke the below line if everything still works
    //let new_process: Option<std::process::ExitStatus> =
    match new_process {
        Ok(prcs) => Some(prcs),
        Err(_error) => {
            println!("Command not found");
            None
        }
    };
}

//handle stuff like `ls | grep blah`
pub fn exec_processes_with_pipes(processes_to_handle: Vec<Vec<&str>>, current_dir: String) {
    //https://stackoverflow.com/questions/63935315/how-to-send-input-to-stdin-of-a-process-created-with-command-and-then-capture-ou

    let mut destination_process = Command::new(processes_to_handle[1][0])
        .current_dir(&current_dir)
        .args(processes_to_handle[1][1..].to_vec())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        //should be removed later, assumes the process exists and executes correctly
        .unwrap();

    let _source_process = Command::new(processes_to_handle[0][0])
        .current_dir(current_dir)
        .args(processes_to_handle[0][1..].to_vec())
        .stdout(destination_process.stdin.take().unwrap())
        .spawn()
        .unwrap();
    //destination process output
    let destination_process_output = destination_process.wait_with_output().unwrap();

    //print stdout
    match destination_process_output.status.code() {
        Some(0) => println!(
            "{}",
            String::from_utf8_lossy(&destination_process_output.stdout)
        ),
        Some(code) => println!("Error: {}", code),
        None => {}
    }
    // should also be removed\
}

fn parse_user_input(user_input: String) -> Vec<String> {
    let mut parsed_user_input: Vec<String> = Vec::new();
    let mut input_string = String::from(user_input);

    //we have one to write to that gets synced to the other one to keep things safe and sound
    let ref mut writable_user_input_buf: Vec<char> = Vec::new();
    let ref mut user_input_buf: Vec<char> = Vec::new();
    while input_string.len() > 0 {
        match input_string.chars().next().unwrap() {
            '\"' => {
                let index_of_quotes: Vec<_> = input_string.match_indices("\"").collect();
                //make sure there's two quotes worth quoting
                if index_of_quotes.len() > 1 {
                    //assuming everything in the buffer is intended as a seperate command
                    //convert the vector of chars into a string
                    //generate a nonmutable vec to please the borrow checker

                    let user_input_string: String =
                        user_input_buf.clone().into_iter().collect::<String>();
                    //I don't know why format compiles but not as_str() but whatever
                    parsed_user_input.push(user_input_string);

                    parsed_user_input.push(
                        input_string
                            .clone()
                            .get(index_of_quotes[0].0 + 1..index_of_quotes[1].0)
                            .unwrap()
                            .to_string(),
                    );
                    //appparently this is an issue but it still compiles so /shrug
                    input_string.drain(..index_of_quotes[1].0 + 1);
                }
            }
            ' ' => {
                //because this is a seperating char, everything before it is meant as a distinct block and should be treated as a single character block
                parsed_user_input.push(user_input_buf.clone().into_iter().collect::<String>());
                //wipe the buffer
                user_input_buf.drain(..);
                input_string.remove(0);
            },
            //same thing as spaces but we add the character instead of dropping it
            '|' | ';' | '>' => {
                parsed_user_input.push(user_input_buf.clone().into_iter().collect::<String>());
                parsed_user_input.push(String::from(input_string.remove(0)));
                user_input_buf.drain(..);
                
            },
            _ => {
                user_input_buf.push(input_string.chars().next().unwrap());
                input_string.remove(0);
            }
        }
    }
    //assume that if there's no formatting characters (" ", ") than it's a whole commanderino
    parsed_user_input.push(user_input_buf.clone().into_iter().collect::<String>());
    

    //cleanup all the blank strings created
    let mut fully_polished_user_input = parsed_user_input.clone();
    for i in (0..fully_polished_user_input.len() - 1).rev() {
        if fully_polished_user_input[i] == "" {
            fully_polished_user_input.remove(i);
        }
    }

    fully_polished_user_input
}

// Spltting the user input up, running necessary commands, redirecting and piping stdio, general
// processing
pub fn handle_input(input_as_vec: Vec<&str>, current_dir: &String) -> String {
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

    while input_vec.len() > 0 {
        for i in control_chars_list {
            let index_of_control_char = input_vec.iter().position(|&x| x == i);
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
                    //if we've reached the end of the list
                    if i == control_chars_list[control_chars_list.len() - 1] {
                        commands.push(input_vec.clone());
                        control_chars.push(None);
                        input_vec = vec![];
                    }
                }
            }
        }
    }

    //change current dir if new dir was found with the cd command
    let mut current_dir = current_dir.clone();
    //loop through the arrays of processed input, and handle accordingly
    //I don't know why this works or why it's an issue in the first place, leave me alone
    if control_chars.len() > 1 {
        control_chars.pop();
    }
    for i in 0..control_chars.len() {
        let mut args = commands[i].clone();
        args.remove(0);
        match control_chars[i] {
            None => {
                //this is a tad jank, only if no builtin process found do you check for system processes
                //tuple containing all useful information for the builtin commands
                let builtin_tuple = exec_builtin(commands[i].clone());

                //make sure it's not a blank string
                if builtin_tuple.1 != String::new() {
                    // if relative path, append to current path
                    // if absolute change entire path
                    if Path::new(&builtin_tuple.1).is_absolute() {
                        current_dir = prettify_path(&builtin_tuple.1);
                    } else if Path::new(format!("{}/{}", current_dir, builtin_tuple.1).as_str())
                        .exists()
                    {
                        current_dir = prettify_path(
                            &format!("{}/{}", current_dir, builtin_tuple.1).to_string(),
                        );
                    } else {
                        println!("Directory not found, looked for: {}", builtin_tuple.1);
                    }
                }
                if builtin_tuple.0 == false {
                    exec_process(commands[i][0], args, &current_dir);
                }
            }
            Some("|") => {
                //pipe the current process's stdout into the next process's stdin
                exec_processes_with_pipes(
                    vec![commands[i].clone(), commands[i + 1].clone()],
                    current_dir.clone(),
                );
            }
            Some(";") => {
                exec_process(commands[i][0], args, &current_dir);
            }
            Some(">") => {
                println!("Found >");
            }
            Some(_) => {
                panic!("wtf happened");
            }
        }
    }
    return current_dir;
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
