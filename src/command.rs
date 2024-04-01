/// Commands.
///
/// Specifies the commands in the spec as well as `Other` for user-defined
/// commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(u32)]
pub enum Command {
    BinInfo = 0x0001,
    Info = 0x0002,
    ResetIntoApp = 0x0003,
    ResetIntoBootloader = 0x0004,
    StartFlash = 0x0005,
    WriteFlashPage = 0x0006,
    ChecksumPages = 0x0007,
    ReadWords = 0x0008,
    WriteWords = 0x0009,
    Dmesg = 0x0010,
    Other(u32),
}

impl From<u32> for Command {
    fn from(value: u32) -> Self {
        match value {
            0x0001 => Self::BinInfo,
            0x0002 => Self::Info,
            0x0003 => Self::ResetIntoApp,
            0x0004 => Self::ResetIntoBootloader,
            0x0005 => Self::StartFlash,
            0x0006 => Self::WriteFlashPage,
            0x0007 => Self::ReadWords,
            0x0008 => Self::WriteWords,
            0x0010 => Self::Dmesg,
            _ => Self::Other(value),
        }
    }
}

impl Into<u32> for Command {
    fn into(self) -> u32 {
        match self {
            Self::BinInfo => 0x0001,
            Self::Info => 0x0002,
            Self::ResetIntoApp => 0x0003,
            Self::ResetIntoBootloader => 0x0004,
            Self::StartFlash => 0x0005,
            Self::WriteFlashPage => 0x0006,
            Self::ChecksumPages => 0x0007,
            Self::ReadWords => 0x0008,
            Self::WriteWords => 0x0009,
            Self::Dmesg => 0x0010,
            Self::Other(value) => value,
        }
    }
}

/// Command request.
#[derive(Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(C)]
pub struct Request<'a>(&'a [u8]);

impl<'a> Request<'a> {
    pub const HEADER_LEN: usize = 8;

    /// Creates a new [`Request`].
    ///
    /// `buf` must be 8 bytes larger than `data` to fit the header.
    pub fn new(buf: &'a mut [u8], command: Command, tag: u16, data: &[u8]) -> Self {
        // ensure header and data will fit in buffer
        assert!(buf.len() == (data.len() + Self::HEADER_LEN));

        // write command id
        let cmd: u32 = command.into();
        buf[0..4].copy_from_slice(&cmd.to_le_bytes());

        // write tag
        buf[4..6].copy_from_slice(&tag.to_le_bytes());

        // write data
        buf[8..].copy_from_slice(data);

        Self(buf)
    }

    /// Creates a new [`Request`] from a byte array.
    pub fn from_bytes(buf: &'a [u8]) -> Self {
        assert!(buf.len() >= Self::HEADER_LEN);
        Self(buf)
    }

    /// Data length.
    pub fn len(&self) -> usize {
        self.0.len() - Self::HEADER_LEN
    }

    /// Get command.
    pub fn command(&self) -> Command {
        let bytes = &self.0[0..4];
        Command::from(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    /// Get tag.
    pub fn tag(&self) -> u16 {
        let bytes = &self.0[4..6];
        u16::from_le_bytes([bytes[0], bytes[1]])
    }

    /// Command data.
    pub fn data(&self) -> &[u8] {
        &self.0[8..]
    }
}

/// Response status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(u32)]
pub enum Status {
    /// The command was processed successfully.
    Sucess = 0x00,
    /// Command ID was not known to the device.
    Unknown = 0x01,
    /// An error occurred during execution of the command.
    Error = 0x02,
    /// Any other status response.
    Other(u8),
}

impl From<u8> for Status {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Sucess,
            0x01 => Self::Unknown,
            0x02 => Self::Error,
            _ => Self::Other(value),
        }
    }
}

impl Into<u8> for Status {
    fn into(self) -> u8 {
        match self {
            Self::Sucess => 0x00,
            Self::Unknown => 0x01,
            Self::Error => 0x02,
            Self::Other(value) => value,
        }
    }
}

/// Command response.
#[derive(Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(C)]
pub struct Response<'a>(&'a [u8]);

impl<'a> Response<'a> {
    pub const HEADER_LEN: usize = 4;

    /// Creates a new [`Response`].
    ///
    /// `buf` must be 8 bytes larger than `data` to fit the header.
    pub fn new(buf: &'a mut [u8], tag: u16, status: Status, status_info: u8, data: &[u8]) -> Self {
        // ensure header and data will fit in buffer
        assert!(buf.len() == data.len() + Self::HEADER_LEN);

        buf[0..2].copy_from_slice(&tag.to_le_bytes());
        buf[2] = status.into();
        buf[3] = status_info;
        buf[Self::HEADER_LEN..].copy_from_slice(data);

        Self(buf)
    }

    /// Creates a new [`Response`] from a byte array.
    pub fn from_bytes(buf: &'a [u8]) -> Self {
        assert!(buf.len() >= Self::HEADER_LEN);
        Self(buf)
    }

    /// Returns the tag.
    pub fn tag(&self) -> u16 {
        let bytes = &self.0[0..2];
        u16::from_le_bytes([bytes[0], bytes[1]])
    }

    /// Returns the status.
    pub fn status(&self) -> Status {
        Status::from(self.0[2])
    }

    /// Returns the status info byte.
    pub fn status_info(&self) -> u8 {
        self.0[3]
    }

    /// Returns a slice containing the data.
    pub fn data(&self) -> &[u8] {
        &self.0[Self::HEADER_LEN..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command() {
        let command = Command::from(u32::MAX);
        assert_eq!(command, Command::Other(u32::MAX));

        let value = 123456;
        let input = Command::from(value);
        let output: u32 = input.into();
        assert_eq!(value, output);
    }
}
