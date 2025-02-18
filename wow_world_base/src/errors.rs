use std::fmt::{Display, Formatter, Write};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct EnumError {
    pub name: &'static str,
    pub value: u64,
}

impl EnumError {
    pub fn new(name: &'static str, value: u64) -> Self {
        Self { name, value }
    }
}

impl Display for EnumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Enum '{}' can not have value: '{}'",
            self.name, self.value
        ))
    }
}

impl std::error::Error for EnumError {}

#[derive(Debug)]
pub enum ParseError {
    Io(std::io::Error),
    Enum(EnumError),
    String(std::string::FromUtf8Error),
    InvalidSize {
        opcode: u32,
        size: u32,
    },
    BufferSizeTooSmall {
        opcode: u32,
        size: u32,
        io: std::io::Error,
    },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Io(i) => i.fmt(f),
            ParseError::Enum(i) => i.fmt(f),
            ParseError::String(i) => i.fmt(f),
            ParseError::InvalidSize { opcode, size } => f.write_fmt(format_args!(
                "message '{opcode:#06X}' has invalid size: '{size}'"
            )),
            ParseError::BufferSizeTooSmall { opcode, size, io } => f.write_fmt(format_args!(
                "opcode '{opcode}' has received too small size '{size}' with io error: '{io}'"
            )),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<EnumError> for ParseError {
    fn from(e: EnumError) -> Self {
        Self::Enum(e)
    }
}

impl From<std::string::FromUtf8Error> for ParseError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<std::io::Error> for ParseError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

#[derive(Debug)]
pub enum ExpectedOpcodeError {
    Opcode {
        opcode: u32,
        name: Option<&'static str>,
        size: u32,
    },
    Parse(ParseError),
}

impl Display for ExpectedOpcodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opcode { opcode, name, size } => {
                if let Some(name) = name {
                    write!(
                        f,
                        "unexpected message '{name}' found: opcode '{opcode:#06X}' and size '{size}'"
                    )
                } else {
                    write!(
                        f,
                        "unexpected opcode without name found: '{opcode:#06X}' and size '{size}'"
                    )
                }
            }
            Self::Parse(i) => i.fmt(f),
        }
    }
}

impl std::error::Error for ExpectedOpcodeError {}

impl From<ParseError> for ExpectedOpcodeError {
    fn from(e: ParseError) -> Self {
        Self::Parse(e)
    }
}

impl From<std::io::Error> for ExpectedOpcodeError {
    fn from(e: std::io::Error) -> Self {
        Self::Parse(e.into())
    }
}
