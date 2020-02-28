const MAXIMUM_LEVEL: Level = Level(127);
const MAXIMUM_XP: Xp = Xp(200_000_000);

const XP_TABLE: [u32; MAXIMUM_LEVEL.0 as usize] = [
    0,
    83,
    174,
    276,
    388,
    512,
    650,
    801,
    969,
    1_154,
    1_358,
    1_584,
    1_833,
    2_107,
    2_411,
    2_746,
    3_115,
    3_523,
    3_973,
    4_470,
    5_018,
    5_624,
    6_291,
    7_028,
    7_842,
    8_740,
    9_730,
    10_824,
    12_031,
    13_363,
    14_833,
    16_456,
    18_247,
    20_224,
    22_406,
    24_815,
    27_473,
    30_408,
    33_648,
    37_224,
    41_171,
    45_529,
    50_339,
    55_649,
    61_512,
    67_983,
    75_127,
    83_014,
    91_721,
    101_333,
    111_945,
    123_660,
    136_594,
    150_872,
    166_636,
    184_040,
    203_254,
    224_466,
    247_886,
    273_742,
    302_288,
    333_804,
    368_599,
    407_015,
    449_428,
    496_254,
    547_953,
    605_032,
    668_051,
    737_627,
    814_445,
    899_257,
    992_895,
    1_096_278,
    1_210_421,
    1_336_443,
    1_475_581,
    1_629_200,
    1_798_808,
    1_986_068,
    2_192_818,
    2_421_087,
    2_673_114,
    2_951_373,
    3_258_594,
    3_597_792,
    3_972_294,
    4_385_776,
    4_842_295,
    5_346_332,
    5_902_831,
    6_517_253,
    7_195_629,
    7_944_614,
    8_771_558,
    9_684_577,
    10_692_629,
    11_805_606,
    13_034_431,
    14_391_160,
    15_889_109,
    17_542_976,
    19_368_992,
    21_385_073,
    23_611_006,
    26_068_632,
    28_782_069,
    31_777_943,
    35_085_654,
    38_737_661,
    42_769_801,
    47_221_641,
    52_136_869,
    57_563_718,
    63_555_443,
    70_170_840,
    77_474_828,
    85_539_082,
    94_442_737,
    104_273_167,
    115_126_838,
    127_110_260,
    140_341_028,
    154_948_977,
    171_077_457,
    188_884_740,
    200_000_000,
];

#[derive(Debug)]
pub struct Skill {
    level: Level,
    xp: Xp,
}

#[derive(Debug)]
pub struct Level(pub u8);

impl Level {
    fn as_xp(&self) -> Option<Xp> {
        match self.0 {
            level if level == 0 => None,
            level if level >= MAXIMUM_LEVEL.0 => None,
            level => Some(Xp(XP_TABLE[level as usize - 1])),
        }
    }
}

#[derive(Debug)]
pub struct Xp(pub u32);

impl Xp {
    fn as_level(&self) -> Option<Level> {
        match self.0 {
            xp if xp > MAXIMUM_XP.0 => None,
            xp => Some(Level(
                (1..MAXIMUM_LEVEL.0)
                    .rev()
                    .find(|level| Level(*level).as_xp().unwrap().0 <= xp)
                    .unwrap(),
            )),
        }
    }
}

impl Skill {
    pub fn from_level(level: Level) -> Option<Skill> {
        let xp_from_level = level.as_xp();

        Some(Skill {
            level,
            xp: xp_from_level?,
        })
    }

    pub fn from_xp(xp: Xp) -> Option<Skill> {
        let level_from_xp = xp.as_level();
        Some(Skill {
            level: level_from_xp?,
            xp,
        })
    }

    pub fn gain_xp(&mut self, xp: Xp) {
        let Xp(current_xp) = self.xp;
        let Xp(xp_to_add) = xp;
        let new_xp: Xp = if current_xp + xp_to_add > MAXIMUM_XP.0 {
            MAXIMUM_XP
        } else {
            Xp(current_xp + xp_to_add)
        };
        let Level(current_level) = self.level;
        let Level(level_from_new_xp) = new_xp.as_level().expect("problem getting level from xp");

        if current_level < level_from_new_xp {
            self.level = Level(level_from_new_xp);
        }
        self.xp = new_xp;
    }

    pub fn get_current_level(&self) -> &Level {
        &self.level
    }

    pub fn get_current_xp(&self) -> &Xp {
        &self.xp
    }
}

#[cfg(test)]
mod test {
    use super::Skill;
    use crate::skill::{Level, Xp, MAXIMUM_XP};

    #[test]
    fn test_gain_xp_in_skill() {
        let mut skill = Skill::from_level(Level(99)).unwrap();
        let xp = Xp(1_000);
        skill.gain_xp(xp);
        assert_eq!(skill.level.0, 99);
        assert_eq!(skill.xp.0, 13_034_431 + 1_000);
    }

    #[test]
    fn test_gain_level_in_skill() {
        let mut skill = Skill::from_level(Level(1)).unwrap();
        skill.gain_xp(Xp(88));
        assert_eq!(skill.level.0, 2);
        assert_eq!(skill.xp.0, 88);
    }

    #[test]
    fn test_gain_over_max_xp_in_skill() {
        let mut skill = Skill::from_level(Level(65)).unwrap();
        skill.gain_xp(MAXIMUM_XP);
        assert_eq!(skill.xp.0, 200_000_000)
    }

    #[test]
    fn test_create_skill_valid() {
        let skill = Skill::from_level(Level(99)).unwrap();
        assert_eq!(skill.level.0, 99);
        assert_eq!(skill.xp.0, 13_034_431)
    }

    #[test]
    fn test_create_skill_from_xp_valid() {
        let skill = Skill::from_xp(Xp(739_639)).unwrap();
        assert_eq!(skill.level.0, 70);
        assert_eq!(skill.xp.0, 739_639)
    }

    #[test]
    fn test_create_skill_invalid() {
        assert!(Skill::from_level(Level(128)).is_none());
    }

    #[test]
    fn test_create_skill_from_xp_invalid() {
        assert!(Skill::from_xp(Xp(200_000_001)).is_none());
    }

    #[test]
    fn test_xp_to_same_level() {
        let xp_level_70 = Xp(739_639);
        assert_eq!(xp_level_70.as_level().unwrap().0, 70);

        let xp_level_70 = Xp(770_110);
        assert_eq!(xp_level_70.as_level().unwrap().0, 70);
    }

    #[test]
    fn test_level_to_xp_to_level_again() {
        // so convert level 70 to its xp equivalent
        let xp_level_70 = Level(70).as_xp().unwrap();
        // then convert the xp back into the level
        assert_eq!(xp_level_70.as_level().unwrap().0, 70);
    }

    #[test]
    fn test_xp_0_to_level_1() {
        assert_eq!(Xp(0).as_level().unwrap().0, 1);
    }

    #[test]
    fn test_xp_to_level_126() {
        let xp = Xp(199_884_740);
        assert_eq!(xp.as_level().unwrap().0, 126);
    }

    #[test]
    fn test_xp_above_level_126() {
        assert!(Xp(200_884_740).as_level().is_none());
    }

    #[test]
    fn test_level_54_to_xp() {
        assert_eq!(Level(54).as_xp().unwrap().0, 150_872);
    }

    #[test]
    fn test_level_1_to_xp() {
        assert_eq!(Level(1).as_xp().unwrap().0, 0);
    }

    #[test]
    fn test_level_99_to_xp() {
        assert_eq!(Level(99).as_xp().unwrap().0, 13_034_431);
    }

    #[test]
    fn test_level_0_to_xp() {
        assert!(Level(0).as_xp().is_none());
    }

    #[test]
    fn test_above_level_126_to_xp() {
        assert!(Level(127).as_xp().is_none());
    }
}
