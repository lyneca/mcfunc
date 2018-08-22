extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

use std::thread::sleep;
use std::time::Duration;

use std::fs::{File};
use std::io::{BufReader, BufRead};

const PREFIX: &str = "/summon minecraft:command_block_minecart ~ ~3 ~ {Passengers:[";


#[allow(dead_code)]
enum CommandBlockType {
    Impulse,
    Chain,
    Repeat
}

#[derive(Debug)]
struct Command {
    command: String
}

impl Command {
    fn new(command: String) -> Command {
        Command { command }
    }
    fn from_str(command: &str) -> Command {
        Command { command: String::from(command) }
    }
    fn to_minecart(&self) -> String {
        format!("{{id:command_block_minecart,Command:\"{}\"}},", self.escape().command)
    }
    fn escape(&self) -> Command {
        Command { command: self.command.replace("\\", "\\\\").replace("\"", "\\\"") }
    }
}

struct CommandBlock {
    variant: CommandBlockType,
    command: Command,
    conditional: bool,
    redstone: bool
}

use CommandBlockType::*;

impl CommandBlock {
    fn repeat(command: Command, redstone: bool) -> CommandBlock {
        CommandBlock {
            variant: CommandBlockType::Repeat,
            command,
            conditional: false,
            redstone
        }
    }
    fn chain(command: Command, conditional: bool, redstone: bool) -> CommandBlock {
        CommandBlock {
            variant: CommandBlockType::Chain,
            command, conditional, redstone
        }
    }
    fn setblock(&self, block_pos: i32) -> Command {
        let string_prefix = format!("setblock ~ ~{} ~ minecraft:{}[facing=down,conditional={}]", block_pos, match self.variant {
            Impulse => "command_block",
            Chain => "chain_command_block",
            Repeat => "repeating_command_block"
        }, self.conditional);
        let mut command_string = String::from(string_prefix);
        let command_tag = format!("{{Command:\"{}\",auto:{}}}", self.command.escape().command, self.redstone);
        command_string.push_str(command_tag.as_str());
        Command::new(command_string)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Requires a path to a .mcfunction file.");
        return;
    }
    let filename = &args[1];
    let file = File::open(filename).expect("Couldn't open file");
    let lines = BufReader::new(&file).lines();

    let mut commands: Vec<Command> = Vec::new();

    let mut hanging;
    for line in lines {
        let command_line = line.expect("File is empty");
        let mut command_line = command_line.as_str();
        if command_line.trim().starts_with('#') { continue };
        hanging = command_line.starts_with(' ');
        if hanging {
            let mut last_line = commands.pop().expect("Invalid indentation");
            last_line.command.push_str(" ");
            last_line.command.push_str(command_line.trim());
            commands.push(last_line);
        } else {
            commands.push(Command::new(String::from(command_line.trim())));
        }
    }
    println!("Input Commands:");
    for i in commands.iter() {
        println!(":: {:?}", i);
    }
    let mut final_command: String = String::from(PREFIX);
    let mut i = 1;
    for command in commands {
        let block = match i {
            1 => CommandBlock::repeat(command, false),
            _ => CommandBlock::chain(command, true, true),
        };
        final_command.push_str(block.setblock(-i).to_minecart().as_str());
        i += 1;
    }
    let kill_command = Command::from_str("kill @e[type=command_block_minecart,distance=0..1.5]");
    final_command.push_str(kill_command.to_minecart().as_str());
    final_command.push_str("]}");
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(final_command.to_owned()).unwrap();
    println!("Put this in a command block and run it. Run the resulting minecarts over an activator rail.");
    println!("Final Command:");
    println!("{}", final_command);
    sleep(Duration::from_secs(1));
}
