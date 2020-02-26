use crate::skill::Skill;

// TODO other experience items such as book of knowledge

pub fn calculate_number_of_lamps(starting: &Skill, target: &Skill) -> Option<u32> {
    let mut starting = starting.clone();
    let mut count = 0;
    while starting.get_current_xp() < target.get_current_xp() {
        let gain = starting.get_current_level() as u32 * 10;
        starting.gain_xp(gain);
        count += 1;
    }
    Some(count)
}

#[cfg(test)]
mod test {
    use crate::experience_item::calculate_number_of_lamps;
    use crate::skill::Skill;

    #[test]
    fn test_lamps_from_1_to_99() {
        let starting = Skill::from_level(1).unwrap();
        let target = Skill::from_level(99).unwrap();
        let lamps = calculate_number_of_lamps(&starting, &target).unwrap();
        assert_eq!(lamps, 15_057);
    }

    #[test]
    fn test_lamps_from_to_xp() {
        let starting = Skill::from_xp(12_465).unwrap();
        let target = Skill::from_xp(32_085).unwrap();
        let lamps = calculate_number_of_lamps(&starting, &target).unwrap();
        assert_eq!(lamps, 58);
    }
}
