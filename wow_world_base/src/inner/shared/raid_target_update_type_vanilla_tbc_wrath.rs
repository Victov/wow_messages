/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/raid_target.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/raid_target.wowm#L3):
/// ```text
/// enum RaidTargetUpdateType : u8 {
///     PARTIAL = 0;
///     FULL = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum RaidTargetUpdateType {
    Partial,
    Full,
}

impl RaidTargetUpdateType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Partial => 0x0,
            Self::Full => 0x1,
        }
    }

}

impl Default for RaidTargetUpdateType {
    fn default() -> Self {
        Self::Partial
    }
}

impl std::fmt::Display for RaidTargetUpdateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Partial => f.write_str("Partial"),
            Self::Full => f.write_str("Full"),
        }
    }
}

impl TryFrom<u8> for RaidTargetUpdateType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Partial),
            1 => Ok(Self::Full),
            v => Err(crate::errors::EnumError::new("RaidTargetUpdateType", v as u64),)
        }
    }
}

