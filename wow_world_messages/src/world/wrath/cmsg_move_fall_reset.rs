use std::io::{Read, Write};
use crate::wrath::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_fall_reset.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_fall_reset.wowm#L13):
/// ```text
/// cmsg CMSG_MOVE_FALL_RESET = 0x02CA {
///     MovementInfo info;
/// }
/// ```
pub struct CMSG_MOVE_FALL_RESET {
    pub info: MovementInfo,
}

impl crate::Message for CMSG_MOVE_FALL_RESET {
    const OPCODE: u32 = 0x02ca;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(30..=84).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02CA, size: body_size as u32 });
        }

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            info,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_MOVE_FALL_RESET {}

impl CMSG_MOVE_FALL_RESET {
    pub(crate) fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

