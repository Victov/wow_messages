use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_set_channel_watch.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_set_channel_watch.wowm#L1):
/// ```text
/// cmsg CMSG_SET_CHANNEL_WATCH = 0x03EE {
///     CString channel;
/// }
/// ```
pub struct CMSG_SET_CHANNEL_WATCH {
    pub channel: String,
}

impl crate::Message for CMSG_SET_CHANNEL_WATCH {
    const OPCODE: u32 = 0x03ee;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // channel: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.channel.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel` must not be null-terminated.");
        w.write_all(self.channel.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03EE, size: body_size as u32 });
        }

        // channel: CString
        let channel = {
            let channel = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(channel)?
        };

        Ok(Self {
            channel,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_CHANNEL_WATCH {}

impl CMSG_SET_CHANNEL_WATCH {
    pub(crate) fn size(&self) -> usize {
        self.channel.len() + 1 // channel: CString
    }
}

