use std::io::{Read, Write};
use crate::Guid;
use crate::vanilla::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_start_backward.wowm:46`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_start_backward.wowm#L46):
/// ```text
/// smsg MSG_MOVE_START_BACKWARD_Server = 0x00B6 {
///     PackedGuid guid;
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_START_BACKWARD_Server {
    pub guid: Guid,
    pub info: MovementInfo,
}

impl crate::Message for MSG_MOVE_START_BACKWARD_Server {
    const OPCODE: u32 = 0x00b6;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(&mut w)?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(30..=90).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00B6, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            guid,
            info,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_MOVE_START_BACKWARD_Server {}

impl MSG_MOVE_START_BACKWARD_Server {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
        + self.info.size() // info: MovementInfo
    }
}

#[cfg(test)]
mod test {
    use super::MSG_MOVE_START_BACKWARD_Server;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 34] = [ 0x00, 0x20, 0xB6, 0x00, 0x01, 0x05, 0x02, 0x00, 0x00,
         0x00, 0x10, 0x57, 0x5B, 0x02, 0x75, 0xA5, 0x0B, 0xC6, 0x6F, 0xF4, 0xF4,
         0xC2, 0xBD, 0x0D, 0xA5, 0x42, 0x6B, 0x6C, 0x92, 0x40, 0x00, 0x00, 0x00,
         0x00, ];

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_start_backward.wowm` line 60.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn MSG_MOVE_START_BACKWARD_Server0() {
        let expected = MSG_MOVE_START_BACKWARD_Server {
            guid: Guid::new(0x5),
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_BACKWARD()
                    ,
                timestamp: 0x25B5710,
                position: Vector3d {
                    x: -8937.364_f32,
                    y: -122.47741_f32,
                    z: 82.52683_f32,
                },
                orientation: 4.5757346_f32,
                fall_time: 0_f32,
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_START_BACKWARD(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_START_BACKWARD, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.info, expected.info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_start_backward.wowm` line 60.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_MSG_MOVE_START_BACKWARD_Server0() {
        let expected = MSG_MOVE_START_BACKWARD_Server {
            guid: Guid::new(0x5),
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_BACKWARD()
                    ,
                timestamp: 0x25B5710,
                position: Vector3d {
                    x: -8937.364_f32,
                    y: -122.47741_f32,
                    z: 82.52683_f32,
                },
                orientation: 4.5757346_f32,
                fall_time: 0_f32,
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_START_BACKWARD(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_START_BACKWARD, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.info, expected.info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_start_backward.wowm` line 60.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_MSG_MOVE_START_BACKWARD_Server0() {
        let expected = MSG_MOVE_START_BACKWARD_Server {
            guid: Guid::new(0x5),
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_BACKWARD()
                    ,
                timestamp: 0x25B5710,
                position: Vector3d {
                    x: -8937.364_f32,
                    y: -122.47741_f32,
                    z: 82.52683_f32,
                },
                orientation: 4.5757346_f32,
                fall_time: 0_f32,
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_START_BACKWARD(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_START_BACKWARD, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.info, expected.info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

