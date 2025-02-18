use std::io::{Read, Write};
use crate::shared::raid_target_update_vanilla_tbc_wrath::RaidTargetUpdate;
use wow_world_base::shared::raid_target_update_type_vanilla_tbc_wrath::RaidTargetUpdateType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/raid_target.wowm:26`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/raid_target.wowm#L26):
/// ```text
/// smsg MSG_RAID_TARGET_UPDATE_Server = 0x0321 {
///     RaidTargetUpdateType update_type;
///     if (update_type == FULL) {
///         RaidTargetUpdate[8] raid_targets;
///     }
///     else if (update_type == PARTIAL) {
///         RaidTargetUpdate raid_target;
///     }
/// }
/// ```
pub struct MSG_RAID_TARGET_UPDATE_Server {
    pub update_type: MSG_RAID_TARGET_UPDATE_Server_RaidTargetUpdateType,
}

impl crate::Message for MSG_RAID_TARGET_UPDATE_Server {
    const OPCODE: u32 = 0x0321;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // update_type: RaidTargetUpdateType
        w.write_all(&u8::from(self.update_type.as_int()).to_le_bytes())?;

        match &self.update_type {
            MSG_RAID_TARGET_UPDATE_Server_RaidTargetUpdateType::Partial {
                raid_target,
            } => {
                // raid_target: RaidTargetUpdate
                raid_target.write_into_vec(&mut w)?;

            }
            MSG_RAID_TARGET_UPDATE_Server_RaidTargetUpdateType::Full {
                raid_targets,
            } => {
                // raid_targets: RaidTargetUpdate[8]
                for i in raid_targets.iter() {
                    i.write_into_vec(&mut w)?;
                }

            }
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=73).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0321, size: body_size as u32 });
        }

        // update_type: RaidTargetUpdateType
        let update_type: RaidTargetUpdateType = crate::util::read_u8_le(&mut r)?.try_into()?;

        let update_type_if = match update_type {
            RaidTargetUpdateType::Partial => {
                // raid_target: RaidTargetUpdate
                let raid_target = RaidTargetUpdate::read(&mut r)?;

                MSG_RAID_TARGET_UPDATE_Server_RaidTargetUpdateType::Partial {
                    raid_target,
                }
            }
            RaidTargetUpdateType::Full => {
                // raid_targets: RaidTargetUpdate[8]
                let raid_targets = {
                    let mut raid_targets = [RaidTargetUpdate::default(); 8];
                    for i in raid_targets.iter_mut() {
                        *i = RaidTargetUpdate::read(&mut r)?;
                    }
                    raid_targets
                };

                MSG_RAID_TARGET_UPDATE_Server_RaidTargetUpdateType::Full {
                    raid_targets,
                }
            }
        };

        Ok(Self {
            update_type: update_type_if,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_RAID_TARGET_UPDATE_Server {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_RAID_TARGET_UPDATE_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_RAID_TARGET_UPDATE_Server {}

impl MSG_RAID_TARGET_UPDATE_Server {
    pub(crate) fn size(&self) -> usize {
        self.update_type.size() // update_type: MSG_RAID_TARGET_UPDATE_Server_RaidTargetUpdateType
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MSG_RAID_TARGET_UPDATE_Server_RaidTargetUpdateType {
    Partial {
        raid_target: RaidTargetUpdate,
    },
    Full {
        raid_targets: [RaidTargetUpdate; 8],
    },
}

impl Default for MSG_RAID_TARGET_UPDATE_Server_RaidTargetUpdateType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Partial {
            raid_target: Default::default(),
        }
    }
}

impl MSG_RAID_TARGET_UPDATE_Server_RaidTargetUpdateType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Partial { .. } => 0,
            Self::Full { .. } => 1,
        }
    }

}

impl MSG_RAID_TARGET_UPDATE_Server_RaidTargetUpdateType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Partial {
                raid_target,
            } => {
                1
                + 9 // raid_target: RaidTargetUpdate
            }
            Self::Full {
                raid_targets,
            } => {
                1
                + 8 * 9 // raid_targets: RaidTargetUpdate[8]
            }
        }
    }
}

