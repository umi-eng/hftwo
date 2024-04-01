/// Commands.
///
/// Specifies the commands in the spec as well as `Other` for user-defined
/// commands.
#[derive(Debug, Clone, Copy)]
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

/// Command request.
#[derive(Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(C)]
pub struct Request<'a> {
    command_id: Command,
    tag: u16,
    _reserved_0: u8,
    _reserved_1: u8,
    data: &'a [u8],
}

/// Response status.
#[derive(Debug, Clone, Copy)]
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

/// Command response.
#[derive(Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(C)]
pub struct Response<'a> {
    tag: u16,
    status: u8,
    status_info: ResponseStatus,
    data: &'a [u8],
}
