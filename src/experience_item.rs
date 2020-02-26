use crate::skill::Skill;

pub enum ExperienceItem {
    GenieLamp,
    BookOfKnowledge,
}

impl ExperienceItem {
    fn get_experience_modifier(&self) -> u8 {
        match self {
            ExperienceItem::GenieLamp => 10,
            ExperienceItem::BookOfKnowledge => 15,
        }
    }
}

pub fn calculate_number_of_experience_item(
    starting: &Skill,
    target: &Skill,
    item: ExperienceItem,
) -> Option<u32> {
    let mut starting = starting.clone();
    let mut count = 0;
    while starting.get_current_xp() < target.get_current_xp() {
        let gain = starting.get_current_level() as u32 * item.get_experience_modifier() as u32;
        starting.gain_xp(gain);
        count += 1;
    }
    Some(count)
}

#[cfg(test)]
mod test {
    use crate::experience_item::{calculate_number_of_experience_item, ExperienceItem};
    use crate::skill::Skill;

    #[test]
    fn test_lamps_from_1_to_99() {
        let starting = Skill::from_level(1).unwrap();
        let target = Skill::from_level(99).unwrap();
        let amount =
            calculate_number_of_experience_item(&starting, &target, ExperienceItem::GenieLamp)
                .unwrap();
        assert_eq!(amount, 15_057);
    }

    #[test]
    fn test_lamps_from_to_xp() {
        let starting = Skill::from_xp(12_465).unwrap();
        let target = Skill::from_xp(32_085).unwrap();
        let amount =
            calculate_number_of_experience_item(&starting, &target, ExperienceItem::GenieLamp)
                .unwrap();
        assert_eq!(amount, 58);
    }

    #[test]
    fn test_book_from_1_to_99() {
        let starting = Skill::from_level(1).unwrap();
        let target = Skill::from_level(99).unwrap();
        let amount = calculate_number_of_experience_item(
            &starting,
            &target,
            ExperienceItem::BookOfKnowledge,
        )
        .unwrap();
        assert_eq!(amount, 10_039);
    }

    #[test]
    fn test_book_from_to_xp() {
        let starting = Skill::from_xp(12_465).unwrap();
        let target = Skill::from_xp(32_085).unwrap();
        let amount = calculate_number_of_experience_item(
            &starting,
            &target,
            ExperienceItem::BookOfKnowledge,
        )
        .unwrap();
        assert_eq!(amount, 39);
    }
}
