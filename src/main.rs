use std::process;

use crossterm::style::Stylize;
use dndutils::{Dice, parse_dice, prompt_character, roll, save};

enum Operation {
    Create,
    Roll(Dice, i8),
}

/// Prompts the user on how to use the program
fn show_info() -> ! {
    eprintln!("{}", "Usage:".cyan());
    eprintln!("{}", "  dndutils create".cyan());
    eprintln!("{}", "  dndutils roll <Dn> [modifier]".cyan());
    eprintln!("{}", "  Example: dndutils roll D20 5".cyan());
    process::exit(2);
}

/// Parses the CLI arguments
fn parse_args() -> Operation {
    let mut args = std::env::args().peekable();
    let Some(op) = args.nth(1) else { show_info() };

    let op = match op.as_str() {
        "create" if args.peek().is_none() => Operation::Create,
        "roll" if args.peek().is_some() => {
            // must be safe becauese we peeked before
            // in the match guard
            let dice = args.next().unwrap();

            // get dice
            let dice = parse_dice(&dice).unwrap_or_else(|| {
                eprintln!("Invalid dice");
                show_info()
            });

            // get modifier
            let modifier = args
                .next()
                .and_then(|arg| arg.parse::<i8>().ok())
                .unwrap_or(0);

            // check for extra argument
            if args.peek().is_some() {
                show_info()
            }

            Operation::Roll(dice, modifier)
        }
        _ => show_info(),
    };

    op
}

fn run(op: Operation) -> Result<(), String> {
    match op {
        Operation::Create => {
            let character = prompt_character().map_err(|err| err.to_string())?;
            save(&character).map_err(|err| err.to_string())?;
        }
        Operation::Roll(dice, modifier) => {
            let result = roll(&dice, modifier);

            println!("Rolling {:?} {:+}...", dice, modifier);
            println!("Result: {}", result);
        }
    }

    Ok(())
}

fn main() {
    let op = parse_args();
    if let Err(err) = run(op) {
        eprintln!("An error occured: {err}")
    }
    parse_args();
}
