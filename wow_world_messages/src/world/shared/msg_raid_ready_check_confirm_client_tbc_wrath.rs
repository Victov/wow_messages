use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/msg_raid_ready_check_confirm.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/msg_raid_ready_check_confirm.wowm#L1):
/// ```text
/// cmsg MSG_RAID_READY_CHECK_CONFIRM_Client = 0x03AE {
///     optional set {
///         u8 state;
///     }
/// }
/// ```
pub struct MSG_RAID_READY_CHECK_CONFIRM_Client {
    pub set: Option<MSG_RAID_READY_CHECK_CONFIRM_Client_set>,
}

impl crate::Message for MSG_RAID_READY_CHECK_CONFIRM_Client {
    const OPCODE: u32 = 0x03ae;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // optional set
        if let Some(v) = &self.set {
            // state: u8
            w.write_all(&v.state.to_le_bytes())?;

        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size > 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03AE, size: body_size as u32 });
        }

        // optional set
        let current_size = {
            0
        };
        let set = if current_size < body_size as usize {
            // state: u8
            let state = crate::util::read_u8_le(&mut r)?;

            Some(MSG_RAID_READY_CHECK_CONFIRM_Client_set {
                state,
            })
        } else {
            None
        };

        Ok(Self {
            set,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_RAID_READY_CHECK_CONFIRM_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_RAID_READY_CHECK_CONFIRM_Client {}

impl MSG_RAID_READY_CHECK_CONFIRM_Client {
    pub(crate) fn size(&self) -> usize {
        if let Some(set) = &self.set {
            1 // state: u8
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MSG_RAID_READY_CHECK_CONFIRM_Client_set {
    pub state: u8,
}

impl MSG_RAID_READY_CHECK_CONFIRM_Client_set {
    pub(crate) fn size(&self) -> usize {
        1 // state: u8
    }

}

