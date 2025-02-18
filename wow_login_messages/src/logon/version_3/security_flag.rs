/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/common.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/common.wowm#L1):
/// ```text
/// enum SecurityFlag : u8 {
///     NONE = 0x0;
///     PIN = 0x1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum SecurityFlag {
    None,
    Pin,
}

impl SecurityFlag {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::Pin => 0x1,
        }
    }

}

impl Default for SecurityFlag {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for SecurityFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::Pin => f.write_str("Pin"),
        }
    }
}

impl TryFrom<u8> for SecurityFlag {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Pin),
            v => Err(crate::errors::EnumError::new("SecurityFlag", v as u64),)
        }
    }
}

