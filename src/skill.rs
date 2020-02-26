const MAXIMUM_LEVEL: usize = 127;
const MAXIMUM_XP: usize = 200_000_000;

const XP_TABLE: [u32; MAXIMUM_LEVEL] = [
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

#[derive(Debug, Clone)]
pub struct Skill {
    level: u8,
    xp: u32,
    _private: (),
}

impl Skill {
    pub fn from_level(level: u8) -> Option<Skill> {
        Some(Skill {
            level,
            xp: get_level_to_xp(level)?,
            _private: (),
        })
    }
    pub fn from_xp(xp: u32) -> Option<Skill> {
        Some(Skill {
            level: get_xp_to_level(xp)?,
            xp,
            _private: (),
        })
    }

    pub fn gain_xp(&mut self, xp: u32) {
        let new_xp = self.xp + xp;
        let level = match get_xp_to_level(new_xp) {
            Some(level) => level,
            None => panic!("too much xp is gained! Level doesn't exist for the new xp of {}", new_xp),
        };
        self.xp = new_xp;
        if self.level < level {
            self.level = level;
        }
    }
    pub fn get_current_level(&self) -> u8 { self.level }

    pub fn get_current_xp(&self) -> u32 { self.xp }
}

pub fn get_level_to_xp(level: u8) -> Option<u32> {
    match level {
        level if level == 0 => None,
        level if level >= MAXIMUM_LEVEL as u8 => None,
        level => Some(XP_TABLE[level as usize - 1]),
    }
}

pub fn get_xp_to_level(xp: u32) -> Option<u8> {
    match xp {
        xp if xp > MAXIMUM_XP as u32 => None,
        xp => (1..MAXIMUM_LEVEL as u8)
            .rev()
            .find(|level| get_level_to_xp(*level).unwrap() <= xp),
    }
}

#[cfg(test)]
mod test {
    use super::{get_level_to_xp, get_xp_to_level, Skill};

    #[test]
    fn test_gain_xp_in_skill() {
        let mut level = Skill::from_level(99).unwrap();
        level.gain_xp(1000);
        assert_eq!(level.level, 99);
        assert_eq!(level.xp, 13_035_431);
    }

    #[test]
    fn test_gain_level_in_skill() {
        let mut level = Skill::from_level(1).unwrap();
        level.gain_xp(88);
        assert_eq!(level.level, 2);
        assert_eq!(level.xp, 88);
    }


    #[test]
    #[should_panic]
    fn test_invalid_gain_xp_in_skill() {
        // overflow of xp
        let mut level = Skill::from_level(65).unwrap();
        level.gain_xp(200_000_000);
    }


    #[test]
    fn test_create_skill_valid() {
        let level = Skill::from_level(99).unwrap();
        assert_eq!(level.level, 99);
        assert_eq!(level.xp, 13_034_431)
    }

    #[test]
    fn test_create_skill_from_xp_valid() {
        let level = Skill::from_xp(739_639).unwrap();
        assert_eq!(level.level, 70);
        assert_eq!(level.xp, 739_639)
    }

    #[test]
    fn test_create_skill_invalid() {
        assert!(Skill::from_level(128).is_none());
    }

    #[test]
    fn test_create_skill_from_xp_invalid() {
        assert!(Skill::from_xp(200_000_001).is_none());
    }

    #[test]
    fn test_xp_to_level_70() {
        let xp_level_70 = 739_639;
        assert_eq!(get_xp_to_level(xp_level_70).unwrap(), 70);

        let xp_level_70 = 770_110;
        assert_eq!(get_xp_to_level(xp_level_70).unwrap(), 70);

        let xp_level_70 = get_level_to_xp(70).unwrap();
        assert_eq!(get_xp_to_level(xp_level_70).unwrap(), 70);
    }

    #[test]
    fn test_xp_0_to_level_1() {
        assert_eq!(get_xp_to_level(0).unwrap(), 1);
    }

    #[test]
    fn test_xp_to_level_126() {
        assert_eq!(get_xp_to_level(get_level_to_xp(126).unwrap()).unwrap(), 126);
        assert_eq!(get_xp_to_level(199_884_740).unwrap(), 126);
    }

    #[test]
    fn test_xp_above_level_126() {
        assert!(get_xp_to_level(200_884_740).is_none());
    }

    #[test]
    fn test_level_54_to_xp() {
        assert_eq!(get_level_to_xp(54).unwrap(), 150_872);
    }

    #[test]
    fn test_level_1_to_xp() {
        assert_eq!(get_level_to_xp(1).unwrap(), 0);
    }

    #[test]
    fn test_level_99_to_xp() {
        assert_eq!(get_level_to_xp(99).unwrap(), 13_034_431);
    }

    #[test]
    fn test_level_0_to_xp() {
        assert!(get_level_to_xp(0).is_none());
    }

    #[test]
    fn test_above_level_126_to_xp() {
        assert!(get_level_to_xp(127).is_none());
    }
}
