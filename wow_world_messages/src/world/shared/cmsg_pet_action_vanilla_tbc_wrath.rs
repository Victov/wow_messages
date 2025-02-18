use std::io::{Read, Write};
use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_pet_action.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_pet_action.wowm#L3):
/// ```text
/// cmsg CMSG_PET_ACTION = 0x0175 {
///     Guid pet;
///     u32 data;
///     Guid target;
/// }
/// ```
pub struct CMSG_PET_ACTION {
    pub pet: Guid,
    pub data: u32,
    pub target: Guid,
}

impl crate::Message for CMSG_PET_ACTION {
    const OPCODE: u32 = 0x0175;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // pet: Guid
        w.write_all(&self.pet.guid().to_le_bytes())?;

        // data: u32
        w.write_all(&self.data.to_le_bytes())?;

        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0175, size: body_size as u32 });
        }

        // pet: Guid
        let pet = Guid::read(&mut r)?;

        // data: u32
        let data = crate::util::read_u32_le(&mut r)?;

        // target: Guid
        let target = Guid::read(&mut r)?;

        Ok(Self {
            pet,
            data,
            target,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PET_ACTION {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_PET_ACTION {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PET_ACTION {}

