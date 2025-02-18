use std::io::{Read, Write};
use crate::Guid;
use crate::tbc::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_force_run_back_speed_change_ack.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_force_run_back_speed_change_ack.wowm#L10):
/// ```text
/// cmsg CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK = 0x00E5 {
///     Guid guid;
///     u32 movement_counter;
///     MovementInfo info;
///     f32 new_speed;
/// }
/// ```
pub struct CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK {
    pub guid: Guid,
    pub movement_counter: u32,
    pub info: MovementInfo,
    pub new_speed: f32,
}

impl crate::Message for CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK {
    const OPCODE: u32 = 0x00e5;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        // new_speed: f32
        w.write_all(&self.new_speed.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(45..=98).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00E5, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        // new_speed: f32
        let new_speed = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            guid,
            movement_counter,
            info,
            new_speed,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK {}

impl CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // movement_counter: u32
        + self.info.size() // info: MovementInfo
        + 4 // new_speed: f32
    }
}

