use std::io::{Read, Write};
use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/msg_list_stabled_pets_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/msg_list_stabled_pets_client.wowm#L3):
/// ```text
/// cmsg MSG_LIST_STABLED_PETS_Client = 0x026F {
///     Guid npc;
/// }
/// ```
pub struct MSG_LIST_STABLED_PETS_Client {
    pub npc: Guid,
}

impl crate::Message for MSG_LIST_STABLED_PETS_Client {
    const OPCODE: u32 = 0x026f;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x026F, size: body_size as u32 });
        }

        // npc: Guid
        let npc = Guid::read(&mut r)?;

        Ok(Self {
            npc,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_LIST_STABLED_PETS_Client {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_LIST_STABLED_PETS_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_LIST_STABLED_PETS_Client {}

