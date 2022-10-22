#![forbid(unsafe_code)]

mod commands;
mod util;
mod command_parser;
mod vcs_state_manager;

use command_parser::CommandParser;
use command_parser::Command;
use commands::{init::init, commit::commit, jump::jump, new_branch::new_branch};

use clap::Parser;

fn main() {
    let args = CommandParser::parse();
    let x = args.command;
    match x {   
        Command::Status => {println!("status")},
        Command::Log => {println!("log")},
        Command::Init{path} => { init(path) },
        Command::Commit{message} => { commit(message) },
        Command::Jump{branch, commit} => { jump(branch, commit) },
        Command::NewBranch{name} => { new_branch(name) },
        Command::Merge{branch} => { println!("{:?}", branch)},
    }
}
