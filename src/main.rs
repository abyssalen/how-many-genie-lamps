use clap::{value_t, App, Arg};

use hmgl::experience_item::{BOOK_OF_KNOWLEDGE, GENIE_LAMP};
use hmgl::skill::{Level, Skill, Xp};
use num_format::{Locale, ToFormattedString};

fn main() {
    let matches = App::new("How Many Genie Lamps?")
        .version("0.1.0")
        .author("Ronnie T. <ronnie.tran2@gmail.com>")
        .about("Calculates how many genie lamps and books of knowledge are required to meet a certain level or experience")
        .arg(
            Arg::with_name("starting-level")
                .long("start-lvl")
                .value_name("LEVEL")
                .help("Sets the starting level")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("starting-xp")
                .long("start-xp")
                .value_name("STARTING-XP")
                .help("Sets the starting xp")
                .conflicts_with("starting-level")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("target-level")
                .long("end-lvl")
                .value_name("LEVEL")
                .help("Sets the target level")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("target-xp")
                .long("end-xp")
                .value_name("TARGET-XP")
                .help("Sets the target xp")
                .conflicts_with("target-level")
                .takes_value(true),
        )
        .get_matches();

    let starting = match (
        value_t!(matches, "starting-level", u8),
        value_t!(matches, "starting-xp", u32),
    ) {
        (Ok(v), Err(_)) => Skill::from_level(Level(v)),
        (Err(_), Ok(v)) => Skill::from_xp(Xp(v)),
        _ => None,
    };

    // TODO better error handling for command line arguments

    if starting.is_none() {
        print!("Could not parse skill from command line arguments.");
        return;
    }

    let target = match (
        value_t!(matches, "target-level", u8),
        value_t!(matches, "target-xp", u32),
    ) {
        (Ok(v), Err(_)) => Skill::from_level(Level(v)),
        (Err(_), Ok(v)) => Skill::from_xp(Xp(v)),
        _ => None,
    };

    if target.is_none() {
        print!("Could not parse target skill from command line arguments.");
        return;
    }

    let starting = starting.unwrap();
    let target = target.unwrap();

    println!(
        "Starting = [level: {}, xp: {}]",
        starting
            .get_current_level()
            .0
            .to_formatted_string(&Locale::en),
        starting.get_current_xp().0.to_formatted_string(&Locale::en)
    );
    println!(
        "Target = [level: {}, xp: {}]",
        target
            .get_current_level()
            .0
            .to_formatted_string(&Locale::en),
        target.get_current_xp().0.to_formatted_string(&Locale::en)
    );

    let number_of_items_needed =
        hmgl::experience_item::calculate_number_of_items_needed(&starting, &target, GENIE_LAMP)
            .to_formatted_string(&Locale::en);

    let number_of_items_needed_book = hmgl::experience_item::calculate_number_of_items_needed(
        &starting,
        &target,
        BOOK_OF_KNOWLEDGE,
    )
    .to_formatted_string(&Locale::en);
    println!(
        "You need {} lamps or {} books to reach the target.",
        number_of_items_needed, number_of_items_needed_book,
    );
}
