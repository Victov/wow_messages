use std::io::{Read, Write};
use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_tabardvendor_activate.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_tabardvendor_activate.wowm#L3):
/// ```text
/// msg MSG_TABARDVENDOR_ACTIVATE = 0x01F2 {
///     Guid guid;
/// }
/// ```
pub struct MSG_TABARDVENDOR_ACTIVATE {
    pub guid: Guid,
}

impl crate::Message for MSG_TABARDVENDOR_ACTIVATE {
    const OPCODE: u32 = 0x01f2;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01F2, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_TABARDVENDOR_ACTIVATE {}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_TABARDVENDOR_ACTIVATE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_TABARDVENDOR_ACTIVATE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_TABARDVENDOR_ACTIVATE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_TABARDVENDOR_ACTIVATE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_TABARDVENDOR_ACTIVATE {}

