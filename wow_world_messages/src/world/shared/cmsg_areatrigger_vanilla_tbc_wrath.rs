use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/cmsg_areatrigger.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/cmsg_areatrigger.wowm#L3):
/// ```text
/// cmsg CMSG_AREATRIGGER = 0x00B4 {
///     u32 trigger_id;
/// }
/// ```
pub struct CMSG_AREATRIGGER {
    pub trigger_id: u32,
}

impl crate::Message for CMSG_AREATRIGGER {
    const OPCODE: u32 = 0x00b4;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // trigger_id: u32
        w.write_all(&self.trigger_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00B4, size: body_size as u32 });
        }

        // trigger_id: u32
        let trigger_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            trigger_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AREATRIGGER {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AREATRIGGER {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AREATRIGGER {}

