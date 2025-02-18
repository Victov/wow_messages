use std::io::{Read, Write};
use crate::tbc::Map;
use crate::tbc::Vector3d;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Sent when using the `worldport` console command.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_world_teleport.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_world_teleport.wowm#L1):
/// ```text
/// cmsg CMSG_WORLD_TELEPORT = 0x0008 {
///     u32 time_in_msec;
///     Map map;
///     Vector3d position;
///     f32 orientation;
/// }
/// ```
pub struct CMSG_WORLD_TELEPORT {
    pub time_in_msec: u32,
    pub map: Map,
    pub position: Vector3d,
    pub orientation: f32,
}

impl crate::Message for CMSG_WORLD_TELEPORT {
    const OPCODE: u32 = 0x0008;

    fn size_without_header(&self) -> u32 {
        24
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // time_in_msec: u32
        w.write_all(&self.time_in_msec.to_le_bytes())?;

        // map: Map
        w.write_all(&u32::from(self.map.as_int()).to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(&mut w)?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 24 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0008, size: body_size as u32 });
        }

        // time_in_msec: u32
        let time_in_msec = crate::util::read_u32_le(&mut r)?;

        // map: Map
        let map: Map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // position: Vector3d
        let position = Vector3d::read(&mut r)?;

        // orientation: f32
        let orientation = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            time_in_msec,
            map,
            position,
            orientation,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_WORLD_TELEPORT {}

