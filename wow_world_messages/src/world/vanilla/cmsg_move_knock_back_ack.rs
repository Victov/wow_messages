use std::io::{Read, Write};
use crate::Guid;
use crate::vanilla::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_knock_back_ack.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_knock_back_ack.wowm#L3):
/// ```text
/// cmsg CMSG_MOVE_KNOCK_BACK_ACK = 0x00F0 {
///     Guid guid;
///     u32 counter;
///     MovementInfo info;
/// }
/// ```
pub struct CMSG_MOVE_KNOCK_BACK_ACK {
    pub guid: Guid,
    pub counter: u32,
    pub info: MovementInfo,
}

impl crate::Message for CMSG_MOVE_KNOCK_BACK_ACK {
    const OPCODE: u32 = 0x00f0;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(40..=93).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00F0, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            guid,
            counter,
            info,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_MOVE_KNOCK_BACK_ACK {}

impl CMSG_MOVE_KNOCK_BACK_ACK {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // counter: u32
        + self.info.size() // info: MovementInfo
    }
}

