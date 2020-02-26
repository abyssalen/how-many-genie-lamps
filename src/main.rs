use rs_lamps::*;

fn main() {
    // TODO the CLI

    let starting = rs_lamps::skill::Skill::new(1).expect("Should be fine");
    let target = rs_lamps::skill::Skill::new(99).expect("Should be fine");

    let lamps = rs_lamps::experience_item::calculate_number_of_lamps(&starting, &target);

    println!("Starting level: {}, xp: {}", starting.level, starting.xp);
    println!("Target level: {}, xp: {}", target.level, target.xp);
    println!(
        "You need to use {} lamps to reach level {}.",
        lamps.unwrap_or_default(),
        target.level
    );
}
