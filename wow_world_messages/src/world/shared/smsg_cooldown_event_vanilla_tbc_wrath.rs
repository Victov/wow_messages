use std::io::{Read, Write};
use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_cooldown_event.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_cooldown_event.wowm#L3):
/// ```text
/// smsg SMSG_COOLDOWN_EVENT = 0x0135 {
///     u32 id;
///     Guid guid;
/// }
/// ```
pub struct SMSG_COOLDOWN_EVENT {
    pub id: u32,
    pub guid: Guid,
}

impl crate::Message for SMSG_COOLDOWN_EVENT {
    const OPCODE: u32 = 0x0135;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0135, size: body_size as u32 });
        }

        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        Ok(Self {
            id,
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_COOLDOWN_EVENT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_COOLDOWN_EVENT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_COOLDOWN_EVENT {}

