/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_complain.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_complain.wowm#L1):
/// ```text
/// enum SpamType : u8 {
///     MAIL = 0;
///     CHAT = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SpamType {
    Mail,
    Chat,
}

impl SpamType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Mail => 0x0,
            Self::Chat => 0x1,
        }
    }

}

impl Default for SpamType {
    fn default() -> Self {
        Self::Mail
    }
}

impl std::fmt::Display for SpamType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mail => f.write_str("Mail"),
            Self::Chat => f.write_str("Chat"),
        }
    }
}

impl TryFrom<u8> for SpamType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Mail),
            1 => Ok(Self::Chat),
            v => Err(crate::errors::EnumError::new("SpamType", v as u64),)
        }
    }
}

