use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_area_trigger_message.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_area_trigger_message.wowm#L3):
/// ```text
/// smsg SMSG_AREA_TRIGGER_MESSAGE = 0x02B8 {
///     SizedCString message;
/// }
/// ```
pub struct SMSG_AREA_TRIGGER_MESSAGE {
    pub message: String,
}

impl crate::Message for SMSG_AREA_TRIGGER_MESSAGE {
    const OPCODE: u32 = 0x02b8;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // message: SizedCString
        w.write_all(&((self.message.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        assert_eq!(self.size() as usize, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // message: SizedCString
        let message = crate::util::read_u32_le(r)?;
        let message = crate::util::read_sized_c_string_to_vec(r, message)?;
        let message = String::from_utf8(message)?;;
        Ok(Self {
            message,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_AREA_TRIGGER_MESSAGE {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_AREA_TRIGGER_MESSAGE {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_AREA_TRIGGER_MESSAGE {}

impl SMSG_AREA_TRIGGER_MESSAGE {
    pub(crate) fn size(&self) -> usize {
        self.message.len() + 5 // message: SizedCString
    }
}

