/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_text_query_response.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_text_query_response.wowm#L10):
/// ```text
/// enum ItemTextQuery : u8 {
///     HAS_TEXT = 0;
///     NO_TEXT = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ItemTextQuery {
    HasText,
    NoText,
}

impl ItemTextQuery {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::HasText => 0x0,
            Self::NoText => 0x1,
        }
    }

}

impl Default for ItemTextQuery {
    fn default() -> Self {
        Self::HasText
    }
}

impl std::fmt::Display for ItemTextQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HasText => f.write_str("HasText"),
            Self::NoText => f.write_str("NoText"),
        }
    }
}

impl TryFrom<u8> for ItemTextQuery {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::HasText),
            1 => Ok(Self::NoText),
            v => Err(crate::errors::EnumError::new("ItemTextQuery", v as u64),)
        }
    }
}

