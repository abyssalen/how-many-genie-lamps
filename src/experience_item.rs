use crate::skill::Skill;

pub enum ExperienceItem {
    GenieLamp,
    BookOfKnowledge,
}

impl ExperienceItem {
    fn get_experience_modifier(&self) -> u32 {
        match self {
            ExperienceItem::GenieLamp => 10,
            ExperienceItem::BookOfKnowledge => 15,
        }
    }
}

pub fn calculate_number_of_items_needed(
    starting: &Skill,
    target: &Skill,
    item: ExperienceItem,
) -> u32 {
    let mut starting = starting.clone();
    let mut count = 0;
    while starting.get_current_xp() < target.get_current_xp() {
        let gain = starting.get_current_level() * item.get_experience_modifier();
        starting.gain_xp(gain);
        count += 1;
    }
    count
}

#[cfg(test)]
mod test {
    use crate::experience_item::{calculate_number_of_items_needed, ExperienceItem};
    use crate::skill::Skill;

    #[test]
    fn test_lamps_from_1_to_99() {
        let starting = Skill::from_level(1).unwrap();
        let target = Skill::from_level(99).unwrap();
        let amount =
            calculate_number_of_items_needed(&starting, &target, ExperienceItem::GenieLamp);
        assert_eq!(amount, 15_057);
    }

    #[test]
    fn test_lamps_from_to_xp() {
        let starting = Skill::from_xp(12_465).unwrap();
        let target = Skill::from_xp(32_085).unwrap();
        let amount =
            calculate_number_of_items_needed(&starting, &target, ExperienceItem::GenieLamp);
        assert_eq!(amount, 58);
    }

    #[test]
    fn test_books_from_1_to_99() {
        let starting = Skill::from_level(1).unwrap();
        let target = Skill::from_level(99).unwrap();
        let amount =
            calculate_number_of_items_needed(&starting, &target, ExperienceItem::BookOfKnowledge);
        assert_eq!(amount, 10_039);
    }

    #[test]
    fn test_books_from_to_xp() {
        let starting = Skill::from_xp(12_465).unwrap();
        let target = Skill::from_xp(32_085).unwrap();
        let amount =
            calculate_number_of_items_needed(&starting, &target, ExperienceItem::BookOfKnowledge);
        assert_eq!(amount, 39);
    }
}
