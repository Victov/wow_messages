/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L1):
/// ```text
/// enum SheatheType : u8 {
///     NONE = 0;
///     MAIN_HAND = 1;
///     OFF_HAND = 2;
///     LARGE_WEAPON_LEFT = 3;
///     LARGE_WEAPON_RIGHT = 4;
///     HIP_WEAPON_LEFT = 5;
///     HIP_WEAPON_RIGHT = 6;
///     SHIELD = 7;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SheatheType {
    None,
    MainHand,
    OffHand,
    LargeWeaponLeft,
    LargeWeaponRight,
    HipWeaponLeft,
    HipWeaponRight,
    Shield,
}

impl SheatheType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::MainHand => 0x1,
            Self::OffHand => 0x2,
            Self::LargeWeaponLeft => 0x3,
            Self::LargeWeaponRight => 0x4,
            Self::HipWeaponLeft => 0x5,
            Self::HipWeaponRight => 0x6,
            Self::Shield => 0x7,
        }
    }

}

impl Default for SheatheType {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for SheatheType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::MainHand => f.write_str("MainHand"),
            Self::OffHand => f.write_str("OffHand"),
            Self::LargeWeaponLeft => f.write_str("LargeWeaponLeft"),
            Self::LargeWeaponRight => f.write_str("LargeWeaponRight"),
            Self::HipWeaponLeft => f.write_str("HipWeaponLeft"),
            Self::HipWeaponRight => f.write_str("HipWeaponRight"),
            Self::Shield => f.write_str("Shield"),
        }
    }
}

impl TryFrom<u8> for SheatheType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::MainHand),
            2 => Ok(Self::OffHand),
            3 => Ok(Self::LargeWeaponLeft),
            4 => Ok(Self::LargeWeaponRight),
            5 => Ok(Self::HipWeaponLeft),
            6 => Ok(Self::HipWeaponRight),
            7 => Ok(Self::Shield),
            v => Err(crate::errors::EnumError::new("SheatheType", v as u64),)
        }
    }
}

