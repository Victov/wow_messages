/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_friend_status.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_friend_status.wowm#L1):
/// ```text
/// enum FriendResult : u8 {
///     DB_ERROR = 0x00;
///     LIST_FULL = 0x01;
///     ONLINE = 0x02;
///     OFFLINE = 0x03;
///     NOT_FOUND = 0x04;
///     REMOVED = 0x05;
///     ADDED_ONLINE = 0x06;
///     ADDED_OFFLINE = 0x07;
///     ALREADY = 0x08;
///     SELF = 0x09;
///     ENEMY = 0x0A;
///     IGNORE_FULL = 0x0B;
///     IGNORE_SELF = 0x0C;
///     IGNORE_NOT_FOUND = 0x0D;
///     IGNORE_ALREADY = 0x0E;
///     IGNORE_ADDED = 0x0F;
///     IGNORE_REMOVED = 0x10;
///     IGNORE_AMBIGUOUS = 0x11;
///     MUTE_FULL = 0x12;
///     MUTE_SELF = 0x13;
///     MUTE_NOT_FOUND = 0x14;
///     MUTE_ALREADY = 0x15;
///     MUTE_ADDED = 0x16;
///     MUTE_REMOVED = 0x17;
///     MUTE_AMBIGUOUS = 0x18;
///     UNKNOWN19 = 0x19;
///     UNKNOWN20 = 0x1A;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FriendResult {
    DbError,
    ListFull,
    Online,
    Offline,
    NotFound,
    Removed,
    AddedOnline,
    AddedOffline,
    Already,
    SelfX,
    Enemy,
    IgnoreFull,
    IgnoreSelf,
    IgnoreNotFound,
    IgnoreAlready,
    IgnoreAdded,
    IgnoreRemoved,
    IgnoreAmbiguous,
    MuteFull,
    MuteSelf,
    MuteNotFound,
    MuteAlready,
    MuteAdded,
    MuteRemoved,
    MuteAmbiguous,
    Unknown19,
    Unknown20,
}

impl FriendResult {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::DbError => 0x0,
            Self::ListFull => 0x1,
            Self::Online => 0x2,
            Self::Offline => 0x3,
            Self::NotFound => 0x4,
            Self::Removed => 0x5,
            Self::AddedOnline => 0x6,
            Self::AddedOffline => 0x7,
            Self::Already => 0x8,
            Self::SelfX => 0x9,
            Self::Enemy => 0xa,
            Self::IgnoreFull => 0xb,
            Self::IgnoreSelf => 0xc,
            Self::IgnoreNotFound => 0xd,
            Self::IgnoreAlready => 0xe,
            Self::IgnoreAdded => 0xf,
            Self::IgnoreRemoved => 0x10,
            Self::IgnoreAmbiguous => 0x11,
            Self::MuteFull => 0x12,
            Self::MuteSelf => 0x13,
            Self::MuteNotFound => 0x14,
            Self::MuteAlready => 0x15,
            Self::MuteAdded => 0x16,
            Self::MuteRemoved => 0x17,
            Self::MuteAmbiguous => 0x18,
            Self::Unknown19 => 0x19,
            Self::Unknown20 => 0x1a,
        }
    }

}

impl Default for FriendResult {
    fn default() -> Self {
        Self::DbError
    }
}

impl std::fmt::Display for FriendResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DbError => f.write_str("DbError"),
            Self::ListFull => f.write_str("ListFull"),
            Self::Online => f.write_str("Online"),
            Self::Offline => f.write_str("Offline"),
            Self::NotFound => f.write_str("NotFound"),
            Self::Removed => f.write_str("Removed"),
            Self::AddedOnline => f.write_str("AddedOnline"),
            Self::AddedOffline => f.write_str("AddedOffline"),
            Self::Already => f.write_str("Already"),
            Self::SelfX => f.write_str("SelfX"),
            Self::Enemy => f.write_str("Enemy"),
            Self::IgnoreFull => f.write_str("IgnoreFull"),
            Self::IgnoreSelf => f.write_str("IgnoreSelf"),
            Self::IgnoreNotFound => f.write_str("IgnoreNotFound"),
            Self::IgnoreAlready => f.write_str("IgnoreAlready"),
            Self::IgnoreAdded => f.write_str("IgnoreAdded"),
            Self::IgnoreRemoved => f.write_str("IgnoreRemoved"),
            Self::IgnoreAmbiguous => f.write_str("IgnoreAmbiguous"),
            Self::MuteFull => f.write_str("MuteFull"),
            Self::MuteSelf => f.write_str("MuteSelf"),
            Self::MuteNotFound => f.write_str("MuteNotFound"),
            Self::MuteAlready => f.write_str("MuteAlready"),
            Self::MuteAdded => f.write_str("MuteAdded"),
            Self::MuteRemoved => f.write_str("MuteRemoved"),
            Self::MuteAmbiguous => f.write_str("MuteAmbiguous"),
            Self::Unknown19 => f.write_str("Unknown19"),
            Self::Unknown20 => f.write_str("Unknown20"),
        }
    }
}

impl TryFrom<u8> for FriendResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DbError),
            1 => Ok(Self::ListFull),
            2 => Ok(Self::Online),
            3 => Ok(Self::Offline),
            4 => Ok(Self::NotFound),
            5 => Ok(Self::Removed),
            6 => Ok(Self::AddedOnline),
            7 => Ok(Self::AddedOffline),
            8 => Ok(Self::Already),
            9 => Ok(Self::SelfX),
            10 => Ok(Self::Enemy),
            11 => Ok(Self::IgnoreFull),
            12 => Ok(Self::IgnoreSelf),
            13 => Ok(Self::IgnoreNotFound),
            14 => Ok(Self::IgnoreAlready),
            15 => Ok(Self::IgnoreAdded),
            16 => Ok(Self::IgnoreRemoved),
            17 => Ok(Self::IgnoreAmbiguous),
            18 => Ok(Self::MuteFull),
            19 => Ok(Self::MuteSelf),
            20 => Ok(Self::MuteNotFound),
            21 => Ok(Self::MuteAlready),
            22 => Ok(Self::MuteAdded),
            23 => Ok(Self::MuteRemoved),
            24 => Ok(Self::MuteAmbiguous),
            25 => Ok(Self::Unknown19),
            26 => Ok(Self::Unknown20),
            v => Err(crate::errors::EnumError::new("FriendResult", v as u64),)
        }
    }
}

