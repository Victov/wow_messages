use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Informs the client that the run speed of a unit has changed.
/// Mangos sends this to third parties that aren't having their speed changed and [`SMSG_FORCE_RUN_SPEED_CHANGE`](crate::world::vanilla::SMSG_FORCE_RUN_SPEED_CHANGE) to the client that has their run speed changed.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_run_speed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_run_speed.wowm#L3):
/// ```text
/// smsg SMSG_SPLINE_SET_RUN_SPEED = 0x02FE {
///     PackedGuid guid;
///     f32 speed;
/// }
/// ```
pub struct SMSG_SPLINE_SET_RUN_SPEED {
    pub guid: Guid,
    pub speed: f32,
}

impl crate::Message for SMSG_SPLINE_SET_RUN_SPEED {
    const OPCODE: u32 = 0x02fe;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        // speed: f32
        w.write_all(&self.speed.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // speed: f32
        let speed = crate::util::read_f32_le(r)?;
        Ok(Self {
            guid,
            speed,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_SPLINE_SET_RUN_SPEED {}

impl SMSG_SPLINE_SET_RUN_SPEED {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 4 // speed: f32
    }
}

#[cfg(test)]
mod test {
    use super::SMSG_SPLINE_SET_RUN_SPEED;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::world::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0xFE, 0x02, 0x01, 0x06, 0x00, 0x00, 0xE0,
         0x40, ];

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_spline_run_speed.wowm` line 11.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_SPLINE_SET_RUN_SPEED0() {
        let expected = SMSG_SPLINE_SET_RUN_SPEED {
            guid: Guid::new(0x6),
            speed: 7_f32,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPLINE_SET_RUN_SPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPLINE_SET_RUN_SPEED, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.speed, expected.speed);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_spline_run_speed.wowm` line 11.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_SPLINE_SET_RUN_SPEED0() {
        let expected = SMSG_SPLINE_SET_RUN_SPEED {
            guid: Guid::new(0x6),
            speed: 7_f32,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPLINE_SET_RUN_SPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPLINE_SET_RUN_SPEED, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.speed, expected.speed);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_spline_run_speed.wowm` line 11.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_SPLINE_SET_RUN_SPEED0() {
        let expected = SMSG_SPLINE_SET_RUN_SPEED {
            guid: Guid::new(0x6),
            speed: 7_f32,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPLINE_SET_RUN_SPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPLINE_SET_RUN_SPEED, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.speed, expected.speed);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

