#![cfg_attr(not(test), no_std)]

pub mod command;

/// Packet kind.
///
/// Stored in the top two bits of the first byte of the packet.
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum PacketKind {
    CommandInner = 0x00,
    CommandFinal = 0x40,
    StdOut = 0x80,
    StdErr = 0xC0,
}

impl From<u8> for PacketKind {
    fn from(value: u8) -> Self {
        match value & 0b11000000 {
            0x00 => Self::CommandInner,
            0x40 => Self::CommandFinal,
            0x80 => Self::StdOut,
            0xC0 => Self::StdErr,
            // since we're masking the top two bits, there are only 4 possible values.
            _ => unreachable!(),
        }
    }
}

impl From<&Packet<'_>> for PacketKind {
    fn from(value: &Packet) -> Self {
        Self::from(value.0[0])
    }
}

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

    pub fn kind(&self) -> PacketKind {
        PacketKind::from(self)
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
        assert!(packet.kind() == PacketKind::CommandInner);
        let packet = Packet(&[0x40, 0xFF, 0xFF]);
        assert!(packet.kind() == PacketKind::CommandFinal);
        let packet = Packet(&[0x80, 0xFF, 0xFF]);
        assert!(packet.kind() == PacketKind::StdOut);
        let packet = Packet(&[0xC0, 0xFF, 0xFF]);
        assert!(packet.kind() == PacketKind::StdErr);
    }

    #[test]
    fn test_stdout() {
        let packet = Packet(TEST_PACKET[0]);
        assert!(packet.kind() == PacketKind::StdOut);
        assert_eq!(packet.len(), 3);
        assert_eq!(packet.data(), &[0x01, 0x02, 0x03]);

        let packet = Packet(TEST_PACKET[1]);
        assert!(packet.kind() == PacketKind::StdOut);
        assert_eq!(packet.len(), 5);
        assert_eq!(packet.data(), &[0x04, 0x05, 0x06, 0x07, 0x08]);

        let packet = Packet(TEST_PACKET[2]);
        assert!(packet.kind() == PacketKind::StdOut);
        assert_eq!(packet.len(), 0);
        assert_eq!(packet.data(), &[]);

        let packet = Packet(TEST_PACKET[3]);
        assert!(packet.kind() == PacketKind::StdErr);
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
