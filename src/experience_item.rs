use crate::skill::{Skill, Xp};

pub struct ExperienceItem(u32);

pub const GENIE_LAMP: ExperienceItem = ExperienceItem(10);
pub const BOOK_OF_KNOWLEDGE: ExperienceItem = ExperienceItem(15);

pub fn calculate_number_of_items_needed(
    starting: &Skill,
    target: &Skill,
    item: ExperienceItem,
) -> u32 {
    let ExperienceItem(exp_modifier) = item;
    let mut starting = Skill::from_xp(Xp(starting.get_current_xp().0)).unwrap();
    let mut count = 0;
    while starting.get_current_xp().0 < target.get_current_xp().0 {
        let gain = Xp(u32::from(starting.get_current_level().0) * exp_modifier);
        starting.gain_xp(gain);
        count += 1;
    }
    count
}

#[cfg(test)]
mod test {
    use crate::experience_item::{calculate_number_of_items_needed, BOOK_OF_KNOWLEDGE, GENIE_LAMP};
    use crate::skill::{Level, Skill, Xp};

    #[test]
    fn test_lamps_from_1_to_99() {
        let starting = Skill::from_level(Level(1)).unwrap();
        let target = Skill::from_level(Level(99)).unwrap();
        let amount = calculate_number_of_items_needed(&starting, &target, GENIE_LAMP);
        assert_eq!(amount, 15_057);
    }

    #[test]
    fn test_lamps_from_to_xp() {
        let starting = Skill::from_xp(Xp(12_465)).unwrap();
        let target = Skill::from_xp(Xp(32_085)).unwrap();
        let amount = calculate_number_of_items_needed(&starting, &target, GENIE_LAMP);
        assert_eq!(amount, 58);
    }

    #[test]
    fn test_books_from_1_to_99() {
        let starting = Skill::from_level(Level(1)).unwrap();
        let target = Skill::from_level(Level(99)).unwrap();
        let amount = calculate_number_of_items_needed(&starting, &target, BOOK_OF_KNOWLEDGE);
        assert_eq!(amount, 10_039);
    }

    #[test]
    fn test_books_from_to_xp() {
        let starting = Skill::from_xp(Xp(12_465)).unwrap();
        let target = Skill::from_xp(Xp(32_085)).unwrap();
        let amount = calculate_number_of_items_needed(&starting, &target, BOOK_OF_KNOWLEDGE);
        assert_eq!(amount, 39);
    }
}
