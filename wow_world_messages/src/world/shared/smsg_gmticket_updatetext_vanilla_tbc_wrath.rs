use std::io::{Read, Write};
use wow_world_base::shared::gm_ticket_response_vanilla_tbc_wrath::GmTicketResponse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/smsg_gmticket_updatetext.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/smsg_gmticket_updatetext.wowm#L3):
/// ```text
/// smsg SMSG_GMTICKET_UPDATETEXT = 0x0208 {
///     GmTicketResponse response;
/// }
/// ```
pub struct SMSG_GMTICKET_UPDATETEXT {
    pub response: GmTicketResponse,
}

impl crate::Message for SMSG_GMTICKET_UPDATETEXT {
    const OPCODE: u32 = 0x0208;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // response: GmTicketResponse
        w.write_all(&u32::from(self.response.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0208, size: body_size as u32 });
        }

        // response: GmTicketResponse
        let response: GmTicketResponse = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            response,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_GMTICKET_UPDATETEXT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_GMTICKET_UPDATETEXT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GMTICKET_UPDATETEXT {}

