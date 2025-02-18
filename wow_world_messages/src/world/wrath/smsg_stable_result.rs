use std::io::{Read, Write};
use crate::wrath::StableResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_stable_result.wowm:50`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_stable_result.wowm#L50):
/// ```text
/// smsg SMSG_STABLE_RESULT = 0x0273 {
///     StableResult result;
/// }
/// ```
pub struct SMSG_STABLE_RESULT {
    pub result: StableResult,
}

impl crate::Message for SMSG_STABLE_RESULT {
    const OPCODE: u32 = 0x0273;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // result: StableResult
        w.write_all(&u8::from(self.result.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0273, size: body_size as u32 });
        }

        // result: StableResult
        let result: StableResult = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_STABLE_RESULT {}

