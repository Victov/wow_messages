/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_stable_result.wowm:27`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_stable_result.wowm#L27):
/// ```text
/// enum StableResult : u8 {
///     ERR_MONEY = 0x01;
///     ERR_STABLE = 0x06;
///     SUCCESS_STABLE = 0x08;
///     SUCCESS_UNSTABLE = 0x09;
///     SUCCESS_BUY_SLOT = 0x0A;
///     ERR_EXOTIC = 0x0C;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum StableResult {
    /// you don't have enough money
    ///
    ErrMoney,
    /// currently used in most fail cases
    ///
    ErrStable,
    /// table success
    ///
    SuccessStable,
    /// unstable/swap success
    ///
    SuccessUnstable,
    /// buy slot success
    ///
    SuccessBuySlot,
    /// you are unable to control exotic creatures
    ///
    ErrExotic,
}

impl StableResult {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::ErrMoney => 0x1,
            Self::ErrStable => 0x6,
            Self::SuccessStable => 0x8,
            Self::SuccessUnstable => 0x9,
            Self::SuccessBuySlot => 0xa,
            Self::ErrExotic => 0xc,
        }
    }

}

impl Default for StableResult {
    fn default() -> Self {
        Self::ErrMoney
    }
}

impl std::fmt::Display for StableResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ErrMoney => f.write_str("ErrMoney"),
            Self::ErrStable => f.write_str("ErrStable"),
            Self::SuccessStable => f.write_str("SuccessStable"),
            Self::SuccessUnstable => f.write_str("SuccessUnstable"),
            Self::SuccessBuySlot => f.write_str("SuccessBuySlot"),
            Self::ErrExotic => f.write_str("ErrExotic"),
        }
    }
}

impl TryFrom<u8> for StableResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::ErrMoney),
            6 => Ok(Self::ErrStable),
            8 => Ok(Self::SuccessStable),
            9 => Ok(Self::SuccessUnstable),
            10 => Ok(Self::SuccessBuySlot),
            12 => Ok(Self::ErrExotic),
            v => Err(crate::errors::EnumError::new("StableResult", v as u64),)
        }
    }
}

