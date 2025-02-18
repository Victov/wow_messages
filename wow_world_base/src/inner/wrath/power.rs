/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/power.wowm:26`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/power.wowm#L26):
/// ```text
/// enum Power : u8 {
///     MANA = 0;
///     RAGE = 1;
///     FOCUS = 2;
///     ENERGY = 3;
///     HAPPINESS = 4;
///     RUNE = 5;
///     RUNIC_POWER = 6;
///     HEALTH = 0xFE;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Power {
    Mana,
    Rage,
    Focus,
    Energy,
    Happiness,
    Rune,
    RunicPower,
    Health,
}

impl Power {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Mana => 0x0,
            Self::Rage => 0x1,
            Self::Focus => 0x2,
            Self::Energy => 0x3,
            Self::Happiness => 0x4,
            Self::Rune => 0x5,
            Self::RunicPower => 0x6,
            Self::Health => 0xfe,
        }
    }

}

impl Default for Power {
    fn default() -> Self {
        Self::Mana
    }
}

impl std::fmt::Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mana => f.write_str("Mana"),
            Self::Rage => f.write_str("Rage"),
            Self::Focus => f.write_str("Focus"),
            Self::Energy => f.write_str("Energy"),
            Self::Happiness => f.write_str("Happiness"),
            Self::Rune => f.write_str("Rune"),
            Self::RunicPower => f.write_str("RunicPower"),
            Self::Health => f.write_str("Health"),
        }
    }
}

impl TryFrom<u8> for Power {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Mana),
            1 => Ok(Self::Rage),
            2 => Ok(Self::Focus),
            3 => Ok(Self::Energy),
            4 => Ok(Self::Happiness),
            5 => Ok(Self::Rune),
            6 => Ok(Self::RunicPower),
            254 => Ok(Self::Health),
            v => Err(crate::errors::EnumError::new("Power", v as u64),)
        }
    }
}

