use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_disabled.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_disabled.wowm#L1):
/// ```text
/// smsg SMSG_LFG_DISABLED = 0x0398 {
/// }
/// ```
pub struct SMSG_LFG_DISABLED {
}

impl crate::Message for SMSG_LFG_DISABLED {
    const OPCODE: u32 = 0x0398;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0398, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LFG_DISABLED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_DISABLED {}

