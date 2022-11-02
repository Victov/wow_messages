use std::convert::{TryFrom, TryInto};
use wow_world_base::vanilla::Vector2d;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gossip/smsg_gossip_poi.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gossip/smsg_gossip_poi.wowm#L3):
/// ```text
/// smsg SMSG_GOSSIP_POI = 0x0224 {
///     u32 flags;
///     Vector2d position;
///     u32 icon;
///     u32 data;
///     CString location_name;
/// }
/// ```
pub struct SMSG_GOSSIP_POI {
    pub flags: u32,
    pub position: Vector2d,
    pub icon: u32,
    pub data: u32,
    pub location_name: String,
}

impl crate::Message for SMSG_GOSSIP_POI {
    const OPCODE: u32 = 0x0224;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        // position: Vector2d
        self.position.write_into_vec(w)?;

        // icon: u32
        w.write_all(&self.icon.to_le_bytes())?;

        // data: u32
        w.write_all(&self.data.to_le_bytes())?;

        // location_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.location_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `location_name` must not be null-terminated.");
        w.write_all(self.location_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // flags: u32
        let flags = crate::util::read_u32_le(r)?;

        // position: Vector2d
        let position = Vector2d::read(r)?;

        // icon: u32
        let icon = crate::util::read_u32_le(r)?;

        // data: u32
        let data = crate::util::read_u32_le(r)?;

        // location_name: CString
        let location_name = crate::util::read_c_string_to_vec(r)?;
        let location_name = String::from_utf8(location_name)?;

        Ok(Self {
            flags,
            position,
            icon,
            data,
            location_name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_GOSSIP_POI {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_GOSSIP_POI {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_GOSSIP_POI {}

impl SMSG_GOSSIP_POI {
    pub(crate) fn size(&self) -> usize {
        4 // flags: u32
        + 8 // position: Vector2d
        + 4 // icon: u32
        + 4 // data: u32
        + self.location_name.len() + 1 // location_name: CString
    }
}

