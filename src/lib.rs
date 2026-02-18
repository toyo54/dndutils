use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::Stylize,
    terminal::{self, ClearType},
};
use rand::{RngExt, rng};

use std::{
    env,
    fs::OpenOptions,
    io::{self, Error as IoError, Result as IoResult, Write},
};

// import project internals
pub mod faerun;
use faerun::{Character, Class, Cycle, ProficiencyLevel, Race, Skill, Skills, Stats};

#[derive(Debug)]
pub enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

fn ask_stats() -> IoResult<Stats> {
    println!("Insert the stats of the character");

    // before loop
    let _ = terminal::enable_raw_mode();
    let mut stdout = io::stdout();

    let _ = execute!(stdout, cursor::Hide); // hides the cursor

    let stats = Stats::new();
    let mut statlist: [(&str, u8); 6] = stats.list();
    let mut selected_index = 0;

    // loop that updates statlist
    loop {
        let _ = execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        );
        println!("Use Up/Down to select, Left/Right to change, Esc when you are done\r");

        // prints the stats of the actual player
        for (i, (name, value)) in statlist.iter().enumerate() {
            if i == selected_index {
                // highlight the selected stat
                println!(
                    "    {}: < {} > \r", // Visual indicator
                    name.to_uppercase().bold().cyan(),
                    value.to_string().bold().black().on_cyan()
                );
            } else {
                // render it normally
                println!("    {}:   {}   \r", name.to_uppercase(), value);
            }
        }
        stdout.flush()?;

        // await for an event
        match event::read()? {
            // if a key was pressed
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Up if selected_index > 0 => selected_index -= 1,
                KeyCode::Down if selected_index < statlist.len() - 1 => selected_index += 1,
                KeyCode::Left if statlist[selected_index].1 > 1 => statlist[selected_index].1 -= 1,
                KeyCode::Right if statlist[selected_index].1 < 20 => {
                    statlist[selected_index].1 += 1
                }
                KeyCode::Enter | KeyCode::Esc => break,
                _ => (),
            },
            _ => (),
        }
    }

    // go back to canonical mode
    let _ = terminal::disable_raw_mode();
    let _ = execute!(stdout, cursor::Show);

    let stats = Stats::from(statlist);
    Ok(stats)
}

fn ask_skills() -> IoResult<Skills> {
    let mut stdout = io::stdout();
    let mut selected_index = 0;

    let skill_list = Skills::ALL_NAMES;
    let mut skills = [Skill::new(ProficiencyLevel::None); 18];

    let _ = terminal::enable_raw_mode();
    let _ = execute!(stdout, cursor::Hide);

    loop {
        let _ = execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        );

        println!("Select Skills\r");
        println!("- Use Up/Down to navigate, Left/Right to toggle.\r");
        println!("- Key: [ ]=None, [P]=Proficient, [E]=Expertise, [h]=Half\r\n");
        println!("  • +/- :       Adjust Misc Bonus (items, feats)\r\n");

        for (i, name) in skill_list.iter().enumerate() {
            let current_level = skills[i];
            let symbol = current_level.proficiency.symbol();
            let misc_bonus = current_level.misc_bonus;

            if i == selected_index {
                // highlight the selected skill
                println!(
                    "    {}: < {} > {:+}\r", // Visual indicator
                    symbol.bold().black().on_cyan(),
                    name.bold().black().on_cyan(),
                    misc_bonus
                );
            } else {
                let symbol_styled = if current_level.proficiency != ProficiencyLevel::None {
                    symbol.green()
                } else {
                    symbol.dark_grey()
                };
                // render it normally
                println!("    {}:   {}  {:+}  \r", symbol_styled, name, misc_bonus);
            }
        }

        stdout.flush()?;

        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Up if selected_index > 1 => selected_index -= 1,
                KeyCode::Down if selected_index < 17 => selected_index += 1,
                KeyCode::Left => skills[selected_index].proficiency.prev(),
                KeyCode::Right => skills[selected_index].proficiency.next(),
                KeyCode::Char('+') if skills[selected_index].misc_bonus < 10 => {
                    skills[selected_index].misc_bonus += 1
                }
                KeyCode::Char('-') if skills[selected_index].misc_bonus > -10 => {
                    skills[selected_index].misc_bonus -= 1
                }
                KeyCode::Esc | KeyCode::Enter => break,
                _ => (),
            },
            _ => (),
        }
    }

    // go back to canonical mode
    let _ = terminal::disable_raw_mode();
    let _ = execute!(stdout, cursor::Show);

    Ok(Skills::from(skills))
}

fn ask_race() -> IoResult<Race> {
    // Drow < Seldarine > === press right arrow key ==> Drow < Lolth Sworn >
    let mut stdout = io::stdout();

    let mut races = Race::default_list();
    let mut selected_index = 0;

    let _ = terminal::enable_raw_mode();
    let _ = execute!(stdout, cursor::Hide);

    loop {
        let _ = execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        );

        println!("Select Race\r");
        println!("- Up/Down navigate | Left/Right: Change Subrace\r\n");

        for (i, race) in races.iter().enumerate() {
            let text = race.to_string();

            if i == selected_index {
                if race.has_subrace() {
                    println!(
                        "{} < {} > \r",
                        " ".on_red(),
                        text.bold().white().on_dark_yellow()
                    );
                } else {
                    println!("{}  \r", text.bold().white().on_dark_yellow());
                }
            } else {
                println!("{}  \r", text.dark_grey());
            }
        }

        stdout.flush()?;

        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Up if selected_index > 0 => selected_index -= 1,
                KeyCode::Down if selected_index < 10 => selected_index += 1,
                KeyCode::Right => races[selected_index].cycle_sub_next(),
                KeyCode::Left => races[selected_index].cycle_sub_prev(),
                KeyCode::Enter | KeyCode::Esc => break,
                _ => (),
            },
            _ => (),
        }
    }

    let _ = terminal::disable_raw_mode();
    let _ = execute!(stdout, cursor::Show);
    Ok(races[selected_index])
}

fn ask_class() -> IoResult<Class> {
    // Warlock < Fiend > === press right arrow key ==> Warlock < HexBlade >
    let mut stdout = io::stdout();

    let mut classes = Class::default_list();
    let mut selected_index = 0;

    let _ = terminal::enable_raw_mode();
    let _ = execute!(stdout, cursor::Hide);

    loop {
        let _ = execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        );

        println!("Select Class\r");
        println!("- Up/Down navigate | Left/Right: Change Subrace\r\n");

        for (i, class) in classes.iter().enumerate() {
            let text = class.to_string();

            if i == selected_index {
                println!(
                    "{} < {} > \r",
                    " ".on_red(),
                    text.bold().white().on_dark_yellow()
                );
            } else {
                println!("{}  \r", text.dark_grey());
            }
        }

        stdout.flush()?;

        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Up if selected_index > 0 => selected_index -= 1,
                KeyCode::Down if selected_index < 10 => selected_index += 1,
                KeyCode::Right => classes[selected_index].cycle_sub_next(),
                KeyCode::Left => classes[selected_index].cycle_sub_prev(),
                KeyCode::Enter | KeyCode::Esc => break,
                _ => (),
            },
            _ => (),
        }
    }

    let _ = terminal::disable_raw_mode();
    let _ = execute!(stdout, cursor::Show);
    Ok(classes[selected_index])
}

/*  ========================
 *  General purpose prompts
 *  ========================
 */

/// Simple line reading function, stops with '\n' (Enter)
fn prompt(prompt: &str) -> IoResult<String> {
    println!("{prompt}");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    let trimmed_len = buf.trim_end().len();
    buf.truncate(trimmed_len);
    Ok(buf)
}

fn prompt_num(prompt: &str) -> IoResult<u8> {
    println!("{prompt}");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let num = buf
        .trim()
        .parse::<u8>()
        .map_err(|err| IoError::new(io::ErrorKind::InvalidData, err));
    num
}

// This function
fn prompt_multiline(prompt: &str) -> IoResult<String> {
    let mut buf = String::with_capacity(1024);
    let mut stdout = io::stdout();

    let _ = terminal::enable_raw_mode();

    loop {
        let _ = execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        );

        println!("{}\r", prompt.bold().cyan());

        // NOTE: "\r\n" = '\n' in raw mode
        println!("(Press 'Esc' to finish)\r\n");
        println!("{}\r", "-".repeat(20).dark_grey());

        // prints the buffer
        print!("{}", buf.replace("\n", "\r\n"));
        // flush moves the cursor at
        // the end of the printed buffer
        stdout.flush()?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(c) => buf.push(c),
                    KeyCode::Enter => buf.push('\n'),
                    KeyCode::Backspace => {
                        buf.pop();
                    }
                    KeyCode::Esc => break,
                    _ => (),
                }
            }
        }
    }

    let _ = terminal::disable_raw_mode();
    let trimmed_len = buf.trim_end().len();
    buf.truncate(trimmed_len);
    Ok(buf)
}

/// This function prompts the character creation for the user
pub fn prompt_character() -> IoResult<Character> {
    let lvl = prompt_num("Character Level: ")?;
    let name = prompt("Character name: ")?;
    let nickname = prompt("Nickname: ")?;
    let ac = prompt_num("AC: ")?;
    let race = ask_race()?;
    let class = ask_class()?;
    let stats = ask_stats()?;
    let skills = ask_skills()?;
    let items_notes = prompt_multiline("Inventory/Items Notes: ")?;
    let notes = prompt_multiline("Notes: ")?;

    Ok(Character {
        lvl,
        name,
        nickname,
        ac,
        race,
        class,
        stats,
        skills,
        items_notes,
        notes,
    })
}

/*
 *  =================
 *  File Operations
 *  =================
 */

/// This function saves the character
/// in either the desktop or
/// the current directory
pub fn save(character: &Character) -> IoResult<()> {
    // gets the desktop path otherwise the current directory
    let mut path = if let Some(mut home) = env::home_dir() {
        home.push(format!("Desktop"));
        home
    } else {
        eprintln!(
            "{}",
            "WARNING: the HOME env variable is absent, saving file in the current directory..."
                .bold()
                .white()
                .on_yellow()
        );
        env::current_dir()?
    };

    // pushes the file name to the path
    path.push(format!("{}.dndc", character.name));

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&path)?;

    file.write_all(character.to_string().as_bytes())?;

    Ok(())
}

/*
*   ====================
*   Dice rolling section
*   ====================
*/

/// This function parses a Dice from the **s** slice
pub fn parse_dice(s: &str) -> Option<Dice> {
    let sides = s
        .trim_start_matches(|c| c == 'D' || c == 'd')
        .parse::<u8>()
        .ok()?;

    match sides {
        4 => Some(Dice::D4),
        6 => Some(Dice::D6),
        8 => Some(Dice::D8),
        10 => Some(Dice::D10),
        12 => Some(Dice::D12),
        20 => Some(Dice::D20),
        100 => Some(Dice::D100),
        _ => None,
    }
}

/// This function rolls the given **dice**
/// and arithmetically adds the modifier
/// to the result
pub fn roll(dice: &Dice, modifier: i8) -> i16 {
    let result = match dice {
        Dice::D4 => rng().random_range(1..=4),
        Dice::D6 => rng().random_range(1..=6),
        Dice::D8 => rng().random_range(1..=8),
        Dice::D10 => rng().random_range(1..=10),
        Dice::D12 => rng().random_range(1..=12),
        Dice::D20 => rng().random_range(1..=20),
        Dice::D100 => rng().random_range(1..=100),
    };

    (result as i16) + (modifier as i16)
}
