use std::io::{Read, Write};
use crate::vanilla::WorldResult;

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
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_CHAR_DELETE {}

#[cfg(test)]
mod test {
    use super::SMSG_CHAR_DELETE;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 5] = [ 0x00, 0x03, 0x3C, 0x00, 0x39, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_delete.wowm` line 9.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_CHAR_DELETE0() {
        let expected = SMSG_CHAR_DELETE {
            result: WorldResult::CharDeleteSuccess,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_DELETE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_DELETE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(1 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_delete.wowm` line 9.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_CHAR_DELETE0() {
        let expected = SMSG_CHAR_DELETE {
            result: WorldResult::CharDeleteSuccess,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_DELETE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_DELETE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(1 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_delete.wowm` line 9.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_CHAR_DELETE0() {
        let expected = SMSG_CHAR_DELETE {
            result: WorldResult::CharDeleteSuccess,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_DELETE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_DELETE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(1 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

