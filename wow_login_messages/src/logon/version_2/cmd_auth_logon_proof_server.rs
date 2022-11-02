use std::convert::{TryFrom, TryInto};
use crate::logon::version_2::LoginResult;
use crate::ServerMessage;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Reply to [`CMD_AUTH_LOGON_PROOF_Client`](crate::logon::version_2::CMD_AUTH_LOGON_PROOF_Client).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm#L2):
/// ```text
/// slogin CMD_AUTH_LOGON_PROOF_Server = 0x01 {
///     LoginResult result;
///     if (result == SUCCESS) {
///         u8[20] server_proof;
///         u32 hardware_survey_id;
///     }
/// }
/// ```
pub struct CMD_AUTH_LOGON_PROOF_Server {
    pub result: CMD_AUTH_LOGON_PROOF_Server_LoginResult,
}

impl CMD_AUTH_LOGON_PROOF_Server {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        let size_assert_header_size = w.len();
        // result: LoginResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        match &self.result {
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                hardware_survey_id,
                server_proof,
            } => {
                // server_proof: u8[20]
                for i in server_proof.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

                // hardware_survey_id: u32
                w.write_all(&hardware_survey_id.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknown0 => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknown1 => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailBanned => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknownAccount => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailIncorrectPassword => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailAlreadyOnline => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailNoTime => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailDbBusy => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailVersionInvalid => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::LoginDownloadFile => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailInvalidServer => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailSuspended => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailNoAccess => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::SuccessSurvey => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailParentalcontrol => {}
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
}

impl ServerMessage for CMD_AUTH_LOGON_PROOF_Server {
    const OPCODE: u8 = 0x01;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // result: LoginResult
        let result: LoginResult = crate::util::read_u8_le(r)?.try_into()?;

        let result_if = match result {
            LoginResult::Success => {
                // server_proof: u8[20]
                let mut server_proof = [0_u8; 20];
                r.read_exact(&mut server_proof)?;

                // hardware_survey_id: u32
                let hardware_survey_id = crate::util::read_u32_le(r)?;

                CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                    hardware_survey_id,
                    server_proof,
                }
            }
            LoginResult::FailUnknown0 => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknown0,
            LoginResult::FailUnknown1 => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknown1,
            LoginResult::FailBanned => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailBanned,
            LoginResult::FailUnknownAccount => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknownAccount,
            LoginResult::FailIncorrectPassword => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailIncorrectPassword,
            LoginResult::FailAlreadyOnline => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailAlreadyOnline,
            LoginResult::FailNoTime => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailNoTime,
            LoginResult::FailDbBusy => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailDbBusy,
            LoginResult::FailVersionInvalid => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailVersionInvalid,
            LoginResult::LoginDownloadFile => CMD_AUTH_LOGON_PROOF_Server_LoginResult::LoginDownloadFile,
            LoginResult::FailInvalidServer => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailInvalidServer,
            LoginResult::FailSuspended => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailSuspended,
            LoginResult::FailNoAccess => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailNoAccess,
            LoginResult::SuccessSurvey => CMD_AUTH_LOGON_PROOF_Server_LoginResult::SuccessSurvey,
            LoginResult::FailParentalcontrol => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailParentalcontrol,
        };

        Ok(Self {
            result: result_if,
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
            // result: LoginResult
            let result: LoginResult = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            let result_if = match result {
                LoginResult::Success => {
                    // server_proof: u8[20]
                    let mut server_proof = [0_u8; 20];
                    r.read_exact(&mut server_proof).await?;

                    // hardware_survey_id: u32
                    let hardware_survey_id = crate::util::tokio_read_u32_le(r).await?;

                    CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                        hardware_survey_id,
                        server_proof,
                    }
                }
                LoginResult::FailUnknown0 => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknown0,
                LoginResult::FailUnknown1 => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknown1,
                LoginResult::FailBanned => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailBanned,
                LoginResult::FailUnknownAccount => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknownAccount,
                LoginResult::FailIncorrectPassword => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailIncorrectPassword,
                LoginResult::FailAlreadyOnline => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailAlreadyOnline,
                LoginResult::FailNoTime => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailNoTime,
                LoginResult::FailDbBusy => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailDbBusy,
                LoginResult::FailVersionInvalid => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailVersionInvalid,
                LoginResult::LoginDownloadFile => CMD_AUTH_LOGON_PROOF_Server_LoginResult::LoginDownloadFile,
                LoginResult::FailInvalidServer => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailInvalidServer,
                LoginResult::FailSuspended => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailSuspended,
                LoginResult::FailNoAccess => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailNoAccess,
                LoginResult::SuccessSurvey => CMD_AUTH_LOGON_PROOF_Server_LoginResult::SuccessSurvey,
                LoginResult::FailParentalcontrol => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailParentalcontrol,
            };

            Ok(Self {
                result: result_if,
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
            // result: LoginResult
            let result: LoginResult = crate::util::astd_read_u8_le(r).await?.try_into()?;

            let result_if = match result {
                LoginResult::Success => {
                    // server_proof: u8[20]
                    let mut server_proof = [0_u8; 20];
                    r.read_exact(&mut server_proof).await?;

                    // hardware_survey_id: u32
                    let hardware_survey_id = crate::util::astd_read_u32_le(r).await?;

                    CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                        hardware_survey_id,
                        server_proof,
                    }
                }
                LoginResult::FailUnknown0 => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknown0,
                LoginResult::FailUnknown1 => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknown1,
                LoginResult::FailBanned => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailBanned,
                LoginResult::FailUnknownAccount => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknownAccount,
                LoginResult::FailIncorrectPassword => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailIncorrectPassword,
                LoginResult::FailAlreadyOnline => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailAlreadyOnline,
                LoginResult::FailNoTime => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailNoTime,
                LoginResult::FailDbBusy => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailDbBusy,
                LoginResult::FailVersionInvalid => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailVersionInvalid,
                LoginResult::LoginDownloadFile => CMD_AUTH_LOGON_PROOF_Server_LoginResult::LoginDownloadFile,
                LoginResult::FailInvalidServer => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailInvalidServer,
                LoginResult::FailSuspended => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailSuspended,
                LoginResult::FailNoAccess => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailNoAccess,
                LoginResult::SuccessSurvey => CMD_AUTH_LOGON_PROOF_Server_LoginResult::SuccessSurvey,
                LoginResult::FailParentalcontrol => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailParentalcontrol,
            };

            Ok(Self {
                result: result_if,
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

impl CMD_AUTH_LOGON_PROOF_Server {
    pub(crate) fn size(&self) -> usize {
        self.result.size() // result: CMD_AUTH_LOGON_PROOF_Server_LoginResult
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CMD_AUTH_LOGON_PROOF_Server_LoginResult {
    Success {
        hardware_survey_id: u32,
        server_proof: [u8; 20],
    },
    FailUnknown0,
    FailUnknown1,
    FailBanned,
    FailUnknownAccount,
    FailIncorrectPassword,
    FailAlreadyOnline,
    FailNoTime,
    FailDbBusy,
    FailVersionInvalid,
    LoginDownloadFile,
    FailInvalidServer,
    FailSuspended,
    FailNoAccess,
    SuccessSurvey,
    FailParentalcontrol,
}

impl Default for CMD_AUTH_LOGON_PROOF_Server_LoginResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Success {
            hardware_survey_id: Default::default(),
            server_proof: [Default::default(); 20],
        }
    }
}

impl CMD_AUTH_LOGON_PROOF_Server_LoginResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Success { .. } => 0,
            Self::FailUnknown0 => 1,
            Self::FailUnknown1 => 2,
            Self::FailBanned => 3,
            Self::FailUnknownAccount => 4,
            Self::FailIncorrectPassword => 5,
            Self::FailAlreadyOnline => 6,
            Self::FailNoTime => 7,
            Self::FailDbBusy => 8,
            Self::FailVersionInvalid => 9,
            Self::LoginDownloadFile => 10,
            Self::FailInvalidServer => 11,
            Self::FailSuspended => 12,
            Self::FailNoAccess => 13,
            Self::SuccessSurvey => 14,
            Self::FailParentalcontrol => 15,
        }
    }

}

impl CMD_AUTH_LOGON_PROOF_Server_LoginResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Success {
                hardware_survey_id,
                server_proof,
            } => {
                1
                + 4 // hardware_survey_id: u32
                + 20 * core::mem::size_of::<u8>() // server_proof: u8[20]
            }
            Self::FailUnknown0 => {
                1
            }
            Self::FailUnknown1 => {
                1
            }
            Self::FailBanned => {
                1
            }
            Self::FailUnknownAccount => {
                1
            }
            Self::FailIncorrectPassword => {
                1
            }
            Self::FailAlreadyOnline => {
                1
            }
            Self::FailNoTime => {
                1
            }
            Self::FailDbBusy => {
                1
            }
            Self::FailVersionInvalid => {
                1
            }
            Self::LoginDownloadFile => {
                1
            }
            Self::FailInvalidServer => {
                1
            }
            Self::FailSuspended => {
                1
            }
            Self::FailNoAccess => {
                1
            }
            Self::SuccessSurvey => {
                1
            }
            Self::FailParentalcontrol => {
                1
            }
        }
    }
}

#[cfg(test)]
mod test_version_2 {
    use super::CMD_AUTH_LOGON_PROOF_Server;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_2::opcodes::ServerOpcodeMessage;

    const RAW0: [u8; 26] = [ 0x01, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
         0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12,
         0x13, 0xEF, 0xBE, 0xAD, 0xDE, ];

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 13.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_PROOF_Server0() {
        let expected = CMD_AUTH_LOGON_PROOF_Server {
            result: CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                hardware_survey_id: 0xDEADBEEF,
                server_proof: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                     0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                     0x12, 0x13, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 13.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_PROOF_Server0() {
        let expected = CMD_AUTH_LOGON_PROOF_Server {
            result: CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                hardware_survey_id: 0xDEADBEEF,
                server_proof: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                     0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                     0x12, 0x13, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 13.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_PROOF_Server0() {
        let expected = CMD_AUTH_LOGON_PROOF_Server {
            result: CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                hardware_survey_id: 0xDEADBEEF,
                server_proof: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                     0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                     0x12, 0x13, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(test)]
mod test_version_3 {
    use super::CMD_AUTH_LOGON_PROOF_Server;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ServerOpcodeMessage;

    const RAW0: [u8; 26] = [ 0x01, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
         0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12,
         0x13, 0xEF, 0xBE, 0xAD, 0xDE, ];

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 13.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_PROOF_Server0() {
        let expected = CMD_AUTH_LOGON_PROOF_Server {
            result: CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                hardware_survey_id: 0xDEADBEEF,
                server_proof: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                     0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                     0x12, 0x13, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 13.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_PROOF_Server0() {
        let expected = CMD_AUTH_LOGON_PROOF_Server {
            result: CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                hardware_survey_id: 0xDEADBEEF,
                server_proof: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                     0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                     0x12, 0x13, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 13.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_PROOF_Server0() {
        let expected = CMD_AUTH_LOGON_PROOF_Server {
            result: CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                hardware_survey_id: 0xDEADBEEF,
                server_proof: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                     0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                     0x12, 0x13, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

