/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_setqueue.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_setqueue.wowm#L3):
/// ```text
/// enum MeetingStoneStatus : u8 {
///     LEAVE_QUEUE = 0;
///     JOINED_QUEUE = 1;
///     PARTY_MEMBER_LEFT_LFG = 2;
///     PARTY_MEMBER_REMOVED_PARTY_REMOVED = 3;
///     LOOKING_FOR_NEW_PARTY_IN_QUEUE = 4;
///     NONE = 5;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum MeetingStoneStatus {
    LeaveQueue,
    JoinedQueue,
    PartyMemberLeftLfg,
    PartyMemberRemovedPartyRemoved,
    LookingForNewPartyInQueue,
    None,
}

impl MeetingStoneStatus {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::LeaveQueue => 0x0,
            Self::JoinedQueue => 0x1,
            Self::PartyMemberLeftLfg => 0x2,
            Self::PartyMemberRemovedPartyRemoved => 0x3,
            Self::LookingForNewPartyInQueue => 0x4,
            Self::None => 0x5,
        }
    }

}

impl Default for MeetingStoneStatus {
    fn default() -> Self {
        Self::LeaveQueue
    }
}

impl std::fmt::Display for MeetingStoneStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeaveQueue => f.write_str("LeaveQueue"),
            Self::JoinedQueue => f.write_str("JoinedQueue"),
            Self::PartyMemberLeftLfg => f.write_str("PartyMemberLeftLfg"),
            Self::PartyMemberRemovedPartyRemoved => f.write_str("PartyMemberRemovedPartyRemoved"),
            Self::LookingForNewPartyInQueue => f.write_str("LookingForNewPartyInQueue"),
            Self::None => f.write_str("None"),
        }
    }
}

impl TryFrom<u8> for MeetingStoneStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::LeaveQueue),
            1 => Ok(Self::JoinedQueue),
            2 => Ok(Self::PartyMemberLeftLfg),
            3 => Ok(Self::PartyMemberRemovedPartyRemoved),
            4 => Ok(Self::LookingForNewPartyInQueue),
            5 => Ok(Self::None),
            v => Err(crate::errors::EnumError::new("MeetingStoneStatus", v as u64),)
        }
    }
}

