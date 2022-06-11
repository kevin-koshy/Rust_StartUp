use std::env;

pub fn run(){

    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = String::from("Kevin");
    let status = "100%";

    // println!("Command: {:?}", command);

    if command == "hello" {
        println!("Hi {}, how are you ? ", name);
    }
    else if command == "Status" {
        println!("Status is {}", status);
    }
    else {
        println!("Thats not a valid comment");
    }
}