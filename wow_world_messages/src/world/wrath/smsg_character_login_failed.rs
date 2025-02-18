use std::io::{Read, Write};
use crate::wrath::WorldResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Response if [`CMSG_PLAYER_LOGIN`](crate::vanilla::CMSG_PLAYER_LOGIN) fails. If successful it should instead be [`SMSG_LOGIN_VERIFY_WORLD`](crate::wrath::SMSG_LOGIN_VERIFY_WORLD).
///
/// Client seems to always send a [`CMSG_CANCEL_TRADE`](crate::vanilla::CMSG_CANCEL_TRADE) after receiving this message, for unknown reasons.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_character_login_failed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_character_login_failed.wowm#L1):
/// ```text
/// smsg SMSG_CHARACTER_LOGIN_FAILED = 0x0041 {
///     WorldResult result;
/// }
/// ```
pub struct SMSG_CHARACTER_LOGIN_FAILED {
    pub result: WorldResult,
}

impl crate::Message for SMSG_CHARACTER_LOGIN_FAILED {
    const OPCODE: u32 = 0x0041;

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
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0041, size: body_size as u32 });
        }

        // result: WorldResult
        let result: WorldResult = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CHARACTER_LOGIN_FAILED {}

