use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::tbc::Emote;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_text_emote.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_text_emote.wowm#L1):
/// ```text
/// smsg SMSG_TEXT_EMOTE = 0x0105 {
///     Guid guid;
///     u32 text_emote;
///     Emote emote;
///     SizedCString name;
/// }
/// ```
pub struct SMSG_TEXT_EMOTE {
    pub guid: Guid,
    pub text_emote: u32,
    pub emote: Emote,
    pub name: String,
}

impl crate::Message for SMSG_TEXT_EMOTE {
    const OPCODE: u32 = 0x0105;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // text_emote: u32
        w.write_all(&self.text_emote.to_le_bytes())?;

        // emote: Emote
        w.write_all(&(self.emote.as_int() as u32).to_le_bytes())?;

        // name: SizedCString
        w.write_all(&((self.name.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // text_emote: u32
        let text_emote = crate::util::read_u32_le(r)?;

        // emote: Emote
        let emote: Emote = crate::util::read_u32_le(r)?.try_into()?;

        // name: SizedCString
        let name = crate::util::read_u32_le(r)?;
        let name = crate::util::read_sized_c_string_to_vec(r, name)?;
        let name = String::from_utf8(name)?;;
        Ok(Self {
            guid,
            text_emote,
            emote,
            name,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_TEXT_EMOTE {}

impl SMSG_TEXT_EMOTE {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // text_emote: u32
        + 4 // emote: Emote
        + self.name.len() + 5 // name: SizedCString
    }
}

