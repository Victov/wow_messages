use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// All that exists of this is an implementation in cmangos-tbc.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_kick_reason.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_kick_reason.wowm#L9):
/// ```text
/// smsg SMSG_KICK_REASON = 0x03C5 {
///     u8 reason;
///     CString text;
/// }
/// ```
pub struct SMSG_KICK_REASON {
    pub reason: u8,
    pub text: String,
}

impl crate::Message for SMSG_KICK_REASON {
    const OPCODE: u32 = 0x03c5;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // reason: u8
        w.write_all(&self.reason.to_le_bytes())?;

        // text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.text.as_bytes().iter().rev().next(), Some(&0_u8), "String `text` must not be null-terminated.");
        w.write_all(self.text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=257).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03C5, size: body_size as u32 });
        }

        // reason: u8
        let reason = crate::util::read_u8_le(&mut r)?;

        // text: CString
        let text = {
            let text = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(text)?
        };

        Ok(Self {
            reason,
            text,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_KICK_REASON {}

impl SMSG_KICK_REASON {
    pub(crate) fn size(&self) -> usize {
        1 // reason: u8
        + self.text.len() + 1 // text: CString
    }
}

