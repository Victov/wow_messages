use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/survey_result.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/survey_result.wowm#L3):
/// ```text
/// clogin CMD_SURVEY_RESULT = 0x04 {
///     u32 survey_id;
///     u8 error;
///     u16 compressed_data_length;
///     u8[compressed_data_length] data;
/// }
/// ```
pub struct CMD_SURVEY_RESULT {
    pub survey_id: u32,
    pub error: u8,
    pub data: Vec<u8>,
}

impl CMD_SURVEY_RESULT {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // survey_id: u32
        w.write_all(&self.survey_id.to_le_bytes())?;

        // error: u8
        w.write_all(&self.error.to_le_bytes())?;

        // compressed_data_length: u16
        w.write_all(&(self.data.len() as u16).to_le_bytes())?;

        // data: u8[compressed_data_length]
        for i in self.data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        assert_eq!(self.size() as usize, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
}

impl ClientMessage for CMD_SURVEY_RESULT {
    const OPCODE: u8 = 0x04;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // survey_id: u32
        let survey_id = crate::util::read_u32_le(r)?;

        // error: u8
        let error = crate::util::read_u8_le(r)?;

        // compressed_data_length: u16
        let compressed_data_length = crate::util::read_u16_le(r)?;

        // data: u8[compressed_data_length]
        let mut data = Vec::with_capacity(compressed_data_length as usize);
        for i in 0..compressed_data_length {
            data.push(crate::util::read_u8_le(r)?);
        }

        Ok(Self {
            survey_id,
            error,
            data,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(self.size() + 1);
        self.write_into_vec(&mut v)?;
        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, crate::errors::ParseError>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + tokio::io::AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // survey_id: u32
            let survey_id = crate::util::tokio_read_u32_le(r).await?;

            // error: u8
            let error = crate::util::tokio_read_u8_le(r).await?;

            // compressed_data_length: u16
            let compressed_data_length = crate::util::tokio_read_u16_le(r).await?;

            // data: u8[compressed_data_length]
            let mut data = Vec::with_capacity(compressed_data_length as usize);
            for i in 0..compressed_data_length {
                data.push(crate::util::tokio_read_u8_le(r).await?);
            }

            Ok(Self {
                survey_id,
                error,
                data,
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(self.size() + 1);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, crate::errors::ParseError>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + async_std::io::ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // survey_id: u32
            let survey_id = crate::util::astd_read_u32_le(r).await?;

            // error: u8
            let error = crate::util::astd_read_u8_le(r).await?;

            // compressed_data_length: u16
            let compressed_data_length = crate::util::astd_read_u16_le(r).await?;

            // data: u8[compressed_data_length]
            let mut data = Vec::with_capacity(compressed_data_length as usize);
            for i in 0..compressed_data_length {
                data.push(crate::util::astd_read_u8_le(r).await?);
            }

            Ok(Self {
                survey_id,
                error,
                data,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(self.size() + 1);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

}

impl CMD_SURVEY_RESULT {
    pub(crate) fn size(&self) -> usize {
        4 // survey_id: u32
        + 1 // error: u8
        + 2 // compressed_data_length: u16
        + self.data.len() * core::mem::size_of::<u8>() // data: u8[compressed_data_length]
    }
}

#[cfg(test)]
mod test {
    use super::CMD_SURVEY_RESULT;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ClientOpcodeMessage;

    const RAW0: [u8; 9] = [ 0x04, 0xDE, 0xFA, 0x00, 0x00, 0x00, 0x01, 0x00, 0xFF, ];

    // Generated from `wow_message_parser/wowm/login/survey_result.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_SURVEY_RESULT0() {
        let expected = CMD_SURVEY_RESULT {
            survey_id: 0xFADE,
            error: 0x0,
            data: vec![ 0xFF, ],
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_SURVEY_RESULT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_SURVEY_RESULT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.survey_id, expected.survey_id);
        assert_eq!(t.error, expected.error);
        assert_eq!(t.data, expected.data);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/survey_result.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_SURVEY_RESULT0() {
        let expected = CMD_SURVEY_RESULT {
            survey_id: 0xFADE,
            error: 0x0,
            data: vec![ 0xFF, ],
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_SURVEY_RESULT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_SURVEY_RESULT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.survey_id, expected.survey_id);
        assert_eq!(t.error, expected.error);
        assert_eq!(t.data, expected.data);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/survey_result.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_SURVEY_RESULT0() {
        let expected = CMD_SURVEY_RESULT {
            survey_id: 0xFADE,
            error: 0x0,
            data: vec![ 0xFF, ],
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_SURVEY_RESULT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_SURVEY_RESULT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.survey_id, expected.survey_id);
        assert_eq!(t.error, expected.error);
        assert_eq!(t.data, expected.data);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

