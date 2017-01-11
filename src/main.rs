/**
 *  Wesam Al-Haddad
 *  Jan 2017
 *  implmentation of a shell in Rust
 *  inspired by this tutorial 
 *  https://brennan.io/2015/01/16/write-a-shell-in-c/
 *  First Stage: implement a single command line: exit, help
 *  TODO Next Stage: take in arguments seperated by white space
 *  TODO Third Stage: ... 
 *  The goal of this exercise is to learn Rust
 **/

use std::io::{self,Write};
use std::process::Command;

const BUILTIN_NUM: usize = 4;

//TODO find a way to pass &str
fn builtin_commands() -> [String; BUILTIN_NUM] {
    let commands = ["help".to_string(),
                    "exit".to_string(),
                    "echo".to_string(), //todo
                    "cd".to_string()];  //todo
    commands
}

fn rssh_exit() -> i32 {
    0
}

fn rssh_help() -> i32 {
    println!("~> Type any of the following commands:");
    for command in builtin_commands().iter() {
        println!("\t{}", command);
    }
    1
}

fn rssh_cd() -> i32 {
    println!("Coming sooon...");
    1
}

fn rssh_echo() -> i32 {
    println!("Coming sooon...");
    1
}

//programs launched by shell
fn rssh_external(s: &str) -> i32 {
    println!("...Launching an external program: {}", &s); 

    let output = Command::new(&s.trim())
                        .output()
                        .expect("..Failed to execute process");
    if output.status.success() {
        let m = String::from_utf8_lossy(&output.stdout); 
        println!("{}",m);
    } else {
        let m = String::from_utf8_lossy(&output.stderr); 
        println!("...Command failed and stderr is:\n{}",m);
    }
    1
}

//TODO make better
fn rssh_execute(s: &str) -> i32 {
    match s.trim() {
        "exit" => { return rssh_exit(); },
        "help" => { return rssh_help(); },
        "cd"   => { return rssh_cd(); },
        "echo" => { return rssh_echo(); },
         _     => { return rssh_external(&s); },
    }
}

//first word for now
//Converting a String to a &str is cheap, but converting the &str to a String involves an allocation.
fn rssh_split_line(s: &String) -> &str {
    let bytes = s.as_bytes();
    //let mut v: Vec<str> = Vec::new();

    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn rssh_read_line() -> String {
    print!("~> ");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    line
}

fn rssh_loop() {
    
    while  {
        let line   = rssh_read_line();
        let args   = rssh_split_line(&line);
        let status = rssh_execute(&args);

        status != 0
    } {}

}

fn main() {
    println!("Welcome to RSSH!");
    println!("Type help to find about builtin commands!");
    rssh_loop();
}
