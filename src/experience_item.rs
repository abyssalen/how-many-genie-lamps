use crate::skill;
use crate::skill::Skill;
use std::fmt::Error;

// TODO other experience items?

// TODO return some sort of Option or Result
// TODO this is ugly lol
pub fn calculate_number_of_lamps(starting: &Skill, target: &Skill) -> Option<u16> {
    let mut xp: u32 = starting.xp;
    let target_xp = target.xp;
    let mut count = 0;
    loop {
        if xp >= target_xp {
            break;
        }
        let level = skill::get_xp_to_level(xp)? as u32;
        let modifier: u32 = (level * 10);
        xp += modifier;
        count += 1;
    }

    Some(count)
}

// TODO tests
#[cfg(test)]
mod test {}
