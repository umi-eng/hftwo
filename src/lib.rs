#![cfg_attr(not(test), no_std)]

pub mod command;

/// Packet view into a byte slice.
pub struct Packet<'a>(&'a [u8]);

impl<'a> Packet<'a> {
    /// Create a new packet from a buffer.
    ///
    /// Panics if `buf` is larger than 64 bytes or less than 1 byte in size.
    pub fn new(buf: &'a [u8]) -> Self {
        assert!(buf.len() > 0);
        assert!(buf.len() <= 64);

        let len = buf[0] as usize & 0b00111111;
        let len = len + 1; // compensate for header

        Self(&buf[0..len])
    }

    /// Returns the payload length of the packet.
    ///
    /// The total packet length is the payload length + 1.
    pub fn len(&self) -> usize {
        self.0[0] as usize & 0b00111111
    }

    /// Returns `true` if the type is an inner command packet.
    pub fn command_inner(&self) -> bool {
        self.0[0] & 0b11000000 == 0x00
    }

    /// Returns `true` if the type is a final command packet.
    pub fn command_final(&self) -> bool {
        self.0[0] & 0b11000000 == 0x40
    }

    /// Returns `true` if the packet contains serial `stdout` data.
    pub fn serial_stdout(&self) -> bool {
        self.0[0] & 0b11000000 == 0x80
    }

    /// Returns `true` if the packet contains serial `stderr` data.
    pub fn serial_stderr(&self) -> bool {
        self.0[0] & 0b11000000 == 0xC0
    }

    /// Access the packet data.
    pub fn data(&self) -> &[u8] {
        &self.0[1..self.len() + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet() {
        let packet = Packet(&[0, 0xFF, 0xFF]);
        assert!(packet.command_inner());
        let packet = Packet(&[0x40, 0xFF, 0xFF]);
        assert!(packet.command_final());
        let packet = Packet(&[0x80, 0xFF, 0xFF]);
        assert!(packet.serial_stdout());
        let packet = Packet(&[0xC0, 0xFF, 0xFF]);
        assert!(packet.serial_stderr());
    }
}
