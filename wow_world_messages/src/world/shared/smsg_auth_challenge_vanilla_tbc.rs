use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Seed used by the client to prove in [`CMSG_AUTH_SESSION`](crate::vanilla::CMSG_AUTH_SESSION) that it has authenticated with the auth server.
///
/// First thing sent when a client connects to the world server.
/// This message is always unencrypted.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm#L2):
/// ```text
/// smsg SMSG_AUTH_CHALLENGE = 0x01EC {
///     u32 server_seed;
/// }
/// ```
pub struct SMSG_AUTH_CHALLENGE {
    pub server_seed: u32,
}

impl crate::Message for SMSG_AUTH_CHALLENGE {
    const OPCODE: u32 = 0x01ec;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // server_seed: u32
        w.write_all(&self.server_seed.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01EC, size: body_size as u32 });
        }

        // server_seed: u32
        let server_seed = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            server_seed,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_AUTH_CHALLENGE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_AUTH_CHALLENGE {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    use super::SMSG_AUTH_CHALLENGE;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 8] = [ 0x00, 0x06, 0xEC, 0x01, 0xEF, 0xBE, 0xAD, 0xDE, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm` line 11.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_AUTH_CHALLENGE0() {
        let expected = SMSG_AUTH_CHALLENGE {
            server_seed: 0xDEADBEEF,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.server_seed, expected.server_seed);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm` line 11.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_AUTH_CHALLENGE0() {
        let expected = SMSG_AUTH_CHALLENGE {
            server_seed: 0xDEADBEEF,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.server_seed, expected.server_seed);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm` line 11.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_AUTH_CHALLENGE0() {
        let expected = SMSG_AUTH_CHALLENGE {
            server_seed: 0xDEADBEEF,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.server_seed, expected.server_seed);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    use super::SMSG_AUTH_CHALLENGE;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ServerOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const RAW0: [u8; 8] = [ 0x00, 0x06, 0xEC, 0x01, 0xEF, 0xBE, 0xAD, 0xDE, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm` line 11.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_AUTH_CHALLENGE0() {
        let expected = SMSG_AUTH_CHALLENGE {
            server_seed: 0xDEADBEEF,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.server_seed, expected.server_seed);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm` line 11.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_AUTH_CHALLENGE0() {
        let expected = SMSG_AUTH_CHALLENGE {
            server_seed: 0xDEADBEEF,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.server_seed, expected.server_seed);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm` line 11.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_AUTH_CHALLENGE0() {
        let expected = SMSG_AUTH_CHALLENGE {
            server_seed: 0xDEADBEEF,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.server_seed, expected.server_seed);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

