use std::io::{Read, Write};
use crate::wrath::WorldResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Response to [`CMSG_CHAR_DELETE`](crate::vanilla::CMSG_CHAR_DELETE).
///
/// The result of this message will update the client character screen without them sending another [`CMSG_CHAR_ENUM`](crate::vanilla::CMSG_CHAR_ENUM).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_char_delete.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_char_delete.wowm#L1):
/// ```text
/// smsg SMSG_CHAR_DELETE = 0x003C {
///     WorldResult result;
/// }
/// ```
pub struct SMSG_CHAR_DELETE {
    pub result: WorldResult,
}

impl crate::Message for SMSG_CHAR_DELETE {
    const OPCODE: u32 = 0x003c;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // result: WorldResult
        w.write_all(&u8::from(self.result.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x003C, size: body_size as u32 });
        }

        // result: WorldResult
        let result: WorldResult = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CHAR_DELETE {}

