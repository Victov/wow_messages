use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/cmsg_questgiver_status_multiple_query.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/cmsg_questgiver_status_multiple_query.wowm#L5):
/// ```text
/// cmsg CMSG_QUESTGIVER_STATUS_MULTIPLE_QUERY = 0x0417 {
/// }
/// ```
pub struct CMSG_QUESTGIVER_STATUS_MULTIPLE_QUERY {
}

impl crate::Message for CMSG_QUESTGIVER_STATUS_MULTIPLE_QUERY {
    const OPCODE: u32 = 0x0417;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0417, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_QUESTGIVER_STATUS_MULTIPLE_QUERY {}

