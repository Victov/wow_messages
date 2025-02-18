/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_petition_sign_results.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_petition_sign_results.wowm#L1):
/// ```text
/// enum PetitionResult : u32 {
///     OK = 0;
///     ALREADY_SIGNED = 1;
///     ALREADY_IN_GUILD = 2;
///     CANT_SIGN_OWN = 3;
///     NEED_MORE = 4;
///     NOT_SERVER = 5;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PetitionResult {
    Ok,
    AlreadySigned,
    AlreadyInGuild,
    CantSignOwn,
    NeedMore,
    NotServer,
}

impl PetitionResult {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Ok => 0x0,
            Self::AlreadySigned => 0x1,
            Self::AlreadyInGuild => 0x2,
            Self::CantSignOwn => 0x3,
            Self::NeedMore => 0x4,
            Self::NotServer => 0x5,
        }
    }

}

impl Default for PetitionResult {
    fn default() -> Self {
        Self::Ok
    }
}

impl std::fmt::Display for PetitionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => f.write_str("Ok"),
            Self::AlreadySigned => f.write_str("AlreadySigned"),
            Self::AlreadyInGuild => f.write_str("AlreadyInGuild"),
            Self::CantSignOwn => f.write_str("CantSignOwn"),
            Self::NeedMore => f.write_str("NeedMore"),
            Self::NotServer => f.write_str("NotServer"),
        }
    }
}

impl TryFrom<u32> for PetitionResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Ok),
            1 => Ok(Self::AlreadySigned),
            2 => Ok(Self::AlreadyInGuild),
            3 => Ok(Self::CantSignOwn),
            4 => Ok(Self::NeedMore),
            5 => Ok(Self::NotServer),
            v => Err(crate::errors::EnumError::new("PetitionResult", v as u64),)
        }
    }
}

