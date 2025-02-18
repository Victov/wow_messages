use std::io::{Read, Write};
use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/msg_random_roll_server.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/msg_random_roll_server.wowm#L3):
/// ```text
/// smsg MSG_RANDOM_ROLL_Server = 0x01FB {
///     u32 minimum;
///     u32 maximum;
///     u32 actual_roll;
///     Guid guid;
/// }
/// ```
pub struct MSG_RANDOM_ROLL_Server {
    pub minimum: u32,
    pub maximum: u32,
    pub actual_roll: u32,
    pub guid: Guid,
}

impl crate::Message for MSG_RANDOM_ROLL_Server {
    const OPCODE: u32 = 0x01fb;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // minimum: u32
        w.write_all(&self.minimum.to_le_bytes())?;

        // maximum: u32
        w.write_all(&self.maximum.to_le_bytes())?;

        // actual_roll: u32
        w.write_all(&self.actual_roll.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01FB, size: body_size as u32 });
        }

        // minimum: u32
        let minimum = crate::util::read_u32_le(&mut r)?;

        // maximum: u32
        let maximum = crate::util::read_u32_le(&mut r)?;

        // actual_roll: u32
        let actual_roll = crate::util::read_u32_le(&mut r)?;

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        Ok(Self {
            minimum,
            maximum,
            actual_roll,
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_RANDOM_ROLL_Server {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_RANDOM_ROLL_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_RANDOM_ROLL_Server {}

