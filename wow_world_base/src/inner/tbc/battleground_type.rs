/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_list.wowm:34`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_list.wowm#L34):
/// ```text
/// enum BattlegroundType : u32 {
///     NONE = 0;
///     ALTERAC_VALLEY = 1;
///     WARSONG_GULCH = 2;
///     ARATHI_BASIN = 3;
///     NETHERSTORM = 4;
///     BLADES_EDGE_ARENA = 5;
///     ARENA = 6;
///     EYE_OF_THE_STORM = 7;
///     RUINS_OF_LORDAERON = 8;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum BattlegroundType {
    None,
    AlteracValley,
    WarsongGulch,
    ArathiBasin,
    Netherstorm,
    BladesEdgeArena,
    Arena,
    EyeOfTheStorm,
    RuinsOfLordaeron,
}

impl BattlegroundType {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::None => 0x0,
            Self::AlteracValley => 0x1,
            Self::WarsongGulch => 0x2,
            Self::ArathiBasin => 0x3,
            Self::Netherstorm => 0x4,
            Self::BladesEdgeArena => 0x5,
            Self::Arena => 0x6,
            Self::EyeOfTheStorm => 0x7,
            Self::RuinsOfLordaeron => 0x8,
        }
    }

}

impl Default for BattlegroundType {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for BattlegroundType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::AlteracValley => f.write_str("AlteracValley"),
            Self::WarsongGulch => f.write_str("WarsongGulch"),
            Self::ArathiBasin => f.write_str("ArathiBasin"),
            Self::Netherstorm => f.write_str("Netherstorm"),
            Self::BladesEdgeArena => f.write_str("BladesEdgeArena"),
            Self::Arena => f.write_str("Arena"),
            Self::EyeOfTheStorm => f.write_str("EyeOfTheStorm"),
            Self::RuinsOfLordaeron => f.write_str("RuinsOfLordaeron"),
        }
    }
}

impl TryFrom<u32> for BattlegroundType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::AlteracValley),
            2 => Ok(Self::WarsongGulch),
            3 => Ok(Self::ArathiBasin),
            4 => Ok(Self::Netherstorm),
            5 => Ok(Self::BladesEdgeArena),
            6 => Ok(Self::Arena),
            7 => Ok(Self::EyeOfTheStorm),
            8 => Ok(Self::RuinsOfLordaeron),
            v => Err(crate::errors::EnumError::new("BattlegroundType", v as u64),)
        }
    }
}

