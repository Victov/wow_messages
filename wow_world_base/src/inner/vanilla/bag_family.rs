/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L15):
/// ```text
/// enum BagFamily : u8 {
///     NONE = 0;
///     ARROWS = 1;
///     BULLETS = 2;
///     SOUL_SHARDS = 3;
///     UNKNOWN4 = 4;
///     UNKNOWN5 = 5;
///     HERBS = 6;
///     ENCHANTING_SUPPLIES = 7;
///     ENGINEERING_SUPPLIES = 8;
///     KEYS = 9;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum BagFamily {
    None,
    Arrows,
    Bullets,
    SoulShards,
    Unknown4,
    Unknown5,
    Herbs,
    EnchantingSupplies,
    EngineeringSupplies,
    Keys,
}

impl BagFamily {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::Arrows => 0x1,
            Self::Bullets => 0x2,
            Self::SoulShards => 0x3,
            Self::Unknown4 => 0x4,
            Self::Unknown5 => 0x5,
            Self::Herbs => 0x6,
            Self::EnchantingSupplies => 0x7,
            Self::EngineeringSupplies => 0x8,
            Self::Keys => 0x9,
        }
    }

}

impl Default for BagFamily {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for BagFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::Arrows => f.write_str("Arrows"),
            Self::Bullets => f.write_str("Bullets"),
            Self::SoulShards => f.write_str("SoulShards"),
            Self::Unknown4 => f.write_str("Unknown4"),
            Self::Unknown5 => f.write_str("Unknown5"),
            Self::Herbs => f.write_str("Herbs"),
            Self::EnchantingSupplies => f.write_str("EnchantingSupplies"),
            Self::EngineeringSupplies => f.write_str("EngineeringSupplies"),
            Self::Keys => f.write_str("Keys"),
        }
    }
}

impl TryFrom<u8> for BagFamily {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Arrows),
            2 => Ok(Self::Bullets),
            3 => Ok(Self::SoulShards),
            4 => Ok(Self::Unknown4),
            5 => Ok(Self::Unknown5),
            6 => Ok(Self::Herbs),
            7 => Ok(Self::EnchantingSupplies),
            8 => Ok(Self::EngineeringSupplies),
            9 => Ok(Self::Keys),
            v => Err(crate::errors::EnumError::new("BagFamily", v as u64),)
        }
    }
}

