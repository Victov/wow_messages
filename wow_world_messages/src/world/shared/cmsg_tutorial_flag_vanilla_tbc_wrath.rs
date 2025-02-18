use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_tutorial_flag.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_tutorial_flag.wowm#L3):
/// ```text
/// cmsg CMSG_TUTORIAL_FLAG = 0x00FE {
///     u32 tutorial_flag;
/// }
/// ```
pub struct CMSG_TUTORIAL_FLAG {
    /// arcemu indexes into the tutorials by dividing by 32 and modulo 32.
    ///
    pub tutorial_flag: u32,
}

impl crate::Message for CMSG_TUTORIAL_FLAG {
    const OPCODE: u32 = 0x00fe;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // tutorial_flag: u32
        w.write_all(&self.tutorial_flag.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00FE, size: body_size as u32 });
        }

        // tutorial_flag: u32
        let tutorial_flag = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            tutorial_flag,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_TUTORIAL_FLAG {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_TUTORIAL_FLAG {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_TUTORIAL_FLAG {}

