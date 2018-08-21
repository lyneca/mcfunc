use std::fs::{File};
use std::io::{BufReader, BufRead};

const PREFIX: &str = "/summon minecraft:command_block_minecart ~ ~3 ~ {Passengers:[";
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[0];
    let file = File::open("test.mcfunction").expect("Couldn't open file");
    let lines = BufReader::new(&file).lines();

    let mut commands: Vec<String> = Vec::new();

    let mut hanging = false;
    for line in lines {
        let command_line = line.expect("File is empty");
        let mut command_line = command_line.as_str();
        hanging = command_line.starts_with(' ');
        if hanging {
            let mut last_line = commands.pop().expect("Invalid indentation");
            last_line.push_str(" ");
            last_line.push_str(command_line.trim());
            commands.push(last_line);
        } else {
            commands.push(String::from(command_line.trim()));
        }
    }
    println!("Input Commands:");
    for i in commands.iter() {
        println!(":: {}", i);
    }
    let mut final_command: String = String::from(PREFIX);
    let mut i = 1;
    for command in commands {
        if i == 1 {
            final_command.push_str(&to_minecart(repeating(command, i)));
        } else {
            final_command.push_str(&to_minecart(chain(command, i)));
        }
        i += 1;
    }
    final_command.push_str("]}");
    println!("Final Command:");
    println!("{}", final_command);
}

fn repeating(command: String, block: i32) -> String {
    format!("setblock ~ ~-{} ~ minecraft:repeating_command_block{{Command:\"{}\"}}", block, escape(command))
}

fn chain(command: String, block: i32) -> String {
    format!("setblock ~ ~-{} ~ minecraft:chain_command_block{{Command:\"{}\"}}", block, escape(command))
}

fn to_minecart(command: String) -> String {
    // println!(" - Adding minecart with command: {}", escape(command));
    format!("{{id:command_block_minecart,Command:\"{}\"}},", escape(command))
    // String::from("hello")
}

fn escape(string: String) -> String {
    string.replace("\\", "\\\\").replace("\"", "\\\"")
}