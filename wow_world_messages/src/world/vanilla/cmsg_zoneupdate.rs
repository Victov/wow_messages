use std::io::{Read, Write};
use crate::vanilla::Area;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Sent by the client whenever it reaches a new area.
///
/// The client does not send an accurate area. For example when going to Sen'jin Village, the client will send `DUROTAR` (0x0E) and not `SENJIN_VILLAGE` (0x16F).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/client_set/cmsg_zoneupdate.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/client_set/cmsg_zoneupdate.wowm#L1):
/// ```text
/// cmsg CMSG_ZONEUPDATE = 0x01F4 {
///     Area area;
/// }
/// ```
pub struct CMSG_ZONEUPDATE {
    pub area: Area,
}

impl crate::Message for CMSG_ZONEUPDATE {
    const OPCODE: u32 = 0x01f4;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // area: Area
        w.write_all(&u32::from(self.area.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01F4, size: body_size as u32 });
        }

        // area: Area
        let area: Area = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            area,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_ZONEUPDATE {}

#[cfg(test)]
mod test {
    use super::CMSG_ZONEUPDATE;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0xF4, 0x01, 0x00, 0x00, 0x65, 0x06, 0x00,
         0x00, ];

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_zoneupdate.wowm` line 9.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_ZONEUPDATE0() {
        let expected = CMSG_ZONEUPDATE {
            area: Area::Orgrimmar,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ZONEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ZONEUPDATE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.area, expected.area);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_zoneupdate.wowm` line 9.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_ZONEUPDATE0() {
        let expected = CMSG_ZONEUPDATE {
            area: Area::Orgrimmar,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ZONEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ZONEUPDATE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.area, expected.area);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_zoneupdate.wowm` line 9.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_ZONEUPDATE0() {
        let expected = CMSG_ZONEUPDATE {
            area: Area::Orgrimmar,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ZONEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ZONEUPDATE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.area, expected.area);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

