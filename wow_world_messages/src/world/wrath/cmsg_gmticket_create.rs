use std::convert::{TryFrom, TryInto};
use crate::world::wrath::Vector3d;
use crate::world::wrath::Map;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_create.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_create.wowm#L21):
/// ```text
/// cmsg CMSG_GMTICKET_CREATE = 0x0205 {
///     Map map;
///     Vector3d position;
///     CString message;
///     Bool needs_response;
///     Bool needs_more_help;
///     u32 num_of_times;
///     u32[num_of_times] times;
///     u32 decompressed_size;
///     u8[-] compressed_data;
/// }
/// ```
pub struct CMSG_GMTICKET_CREATE {
    pub map: Map,
    pub position: Vector3d,
    pub message: String,
    pub needs_response: bool,
    pub needs_more_help: bool,
    pub times: Vec<u32>,
    pub decompressed_size: u32,
    pub compressed_data: Vec<u8>,
}

impl crate::Message for CMSG_GMTICKET_CREATE {
    const OPCODE: u32 = 0x0205;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(w)?;

        // message: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.message.as_bytes().iter().rev().next(), Some(&0_u8), "String `message` must not be null-terminated.");
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // needs_response: Bool
        w.write_all(u8::from(self.needs_response).to_le_bytes().as_slice())?;

        // needs_more_help: Bool
        w.write_all(u8::from(self.needs_more_help).to_le_bytes().as_slice())?;

        // num_of_times: u32
        w.write_all(&(self.times.len() as u32).to_le_bytes())?;

        // times: u32[num_of_times]
        for i in self.times.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // decompressed_size: u32
        w.write_all(&self.decompressed_size.to_le_bytes())?;

        // compressed_data: u8[-]
        let mut encoder = flate2::write::ZlibEncoder::new(w, flate2::Compression::default());
        for i in self.compressed_data.iter() {
            encoder.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // position: Vector3d
        let position = Vector3d::read(r)?;

        // message: CString
        let message = crate::util::read_c_string_to_vec(r)?;
        let message = String::from_utf8(message)?;

        // needs_response: Bool
        let needs_response = crate::util::read_u8_le(r)? != 0;
        // needs_more_help: Bool
        let needs_more_help = crate::util::read_u8_le(r)? != 0;
        // num_of_times: u32
        let num_of_times = crate::util::read_u32_le(r)?;

        // times: u32[num_of_times]
        let mut times = Vec::with_capacity(num_of_times as usize);
        for i in 0..num_of_times {
            times.push(crate::util::read_u32_le(r)?);
        }

        // decompressed_size: u32
        let decompressed_size = crate::util::read_u32_le(r)?;

        // compressed_data: u8[-]
        let mut decoder = &mut flate2::read::ZlibDecoder::new(r);

        let mut current_size = {
            4 // map: Map
            + 12 // position: Vector3d
            + message.len() + 1 // message: CString
            + 1 // needs_response: Bool
            + 1 // needs_more_help: Bool
            + 4 // num_of_times: u32
            + times.len() * core::mem::size_of::<u32>() // times: u32[num_of_times]
            + 4 // decompressed_size: u32
        };
        let mut compressed_data = Vec::with_capacity(body_size as usize - current_size);
        while decoder.total_out() < (decompressed_size as u64) {
            compressed_data.push(crate::util::read_u8_le(decoder)?);
            current_size += 1;
        }

        Ok(Self {
            map,
            position,
            message,
            needs_response,
            needs_more_help,
            times,
            decompressed_size,
            compressed_data,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_GMTICKET_CREATE {}

impl CMSG_GMTICKET_CREATE {
    pub(crate) fn size(&self) -> usize {
        4 // map: Map
        + 12 // position: Vector3d
        + self.message.len() + 1 // message: CString
        + 1 // needs_response: Bool
        + 1 // needs_more_help: Bool
        + 4 // num_of_times: u32
        + self.times.len() * core::mem::size_of::<u32>() // times: u32[num_of_times]
        + 4 // decompressed_size: u32
        + crate::util::zlib_compressed_size(&self.compressed_data) // compressed_data: u8[-]
    }
}

