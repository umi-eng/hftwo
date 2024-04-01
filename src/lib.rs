#![cfg_attr(not(test), no_std)]

pub mod command;

/// Packet view into a byte slice.
pub struct Packet<'a>(&'a [u8]);

impl<'a> Packet<'a> {
    /// Create a new packet from a buffer.
    ///
    /// Panics if `buf` is larger than 64 bytes or less than 1 byte in size.
    pub fn from_bytes(buf: &'a [u8]) -> Self {
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

    /// Taken from HF2 spec.
    const TEST_PACKET: &[&[u8]] = &[
        &[0x83, 0x01, 0x02, 0x03, 0xAB, 0xFF, 0xFF, 0xFF],
        &[0x85, 0x04, 0x05, 0x06, 0x07, 0x08],
        &[0x80, 0xDE, 0x42, 0x42, 0x42, 0x42, 0xFF, 0xFF],
        &[
            0xD0, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0xFF, 0xFF, 0xFF,
        ],
    ];

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

    #[test]
    fn test_stdout() {
        let packet = Packet(TEST_PACKET[0]);
        assert!(packet.serial_stdout());
        assert_eq!(packet.len(), 3);
        assert_eq!(packet.data(), &[0x01, 0x02, 0x03]);

        let packet = Packet(TEST_PACKET[1]);
        assert!(packet.serial_stdout());
        assert_eq!(packet.len(), 5);
        assert_eq!(packet.data(), &[0x04, 0x05, 0x06, 0x07, 0x08]);

        let packet = Packet(TEST_PACKET[2]);
        assert!(packet.serial_stdout());
        assert_eq!(packet.len(), 0);
        assert_eq!(packet.data(), &[]);

        let packet = Packet(TEST_PACKET[3]);
        assert!(packet.serial_stderr());
        assert_eq!(packet.len(), 16);
        assert_eq!(
            packet.data(),
            &[
                0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16,
                0x17, 0xFF
            ]
        );
    }
}
