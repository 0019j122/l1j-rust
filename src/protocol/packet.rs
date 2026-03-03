/// Packet builder and reader utilities.
///
/// Mirrors Java's ServerBasePacket (writeC/H/D/S) and
/// ClientBasePacket (readC/H/D/S) binary format.
/// All multi-byte integers are little-endian.
/// Strings are null-terminated (currently UTF-8, Big5 encoding TBD).

// ---------------------------------------------------------------------------
// PacketBuilder - for constructing server → client packets
// ---------------------------------------------------------------------------
use crate::protocol::encoding::encode_big5;
pub struct PacketBuilder {
    buf: Vec<u8>,
}

impl PacketBuilder {
    /// Create a new packet with the given opcode as the first byte.
    pub fn new(opcode: u8) -> Self {
        let mut pb = PacketBuilder {
            buf: Vec::with_capacity(64),
        };
        pb.buf.push(opcode);
        pb
    }

    /// Write a single byte (8-bit).
    pub fn write_c(mut self, value: i32) -> Self {
        self.buf.push((value & 0xFF) as u8);
        self
    }

    /// Write a 16-bit integer (little-endian).
    pub fn write_h(mut self, value: i32) -> Self {
        self.buf.push((value & 0xFF) as u8);
        self.buf.push((value >> 8 & 0xFF) as u8);
        self
    }

    /// Write a 32-bit integer (little-endian).
    pub fn write_d(mut self, value: i32) -> Self {
        self.buf.push((value & 0xFF) as u8);
        self.buf.push((value >> 8 & 0xFF) as u8);
        self.buf.push((value >> 16 & 0xFF) as u8);
        self.buf.push((value >> 24 & 0xFF) as u8);
        self
    }

    /// Write a null-terminated string in Big5 encoding.
    /// If None, writes just the null terminator.
    pub fn write_s(mut self, text: Option<&str>) -> Self {
        if let Some(s) = text {
            let encoded = encode_big5(s);
            self.buf.extend_from_slice(&encoded);
        }
        self.buf.push(0x00);
        self
    }

    /// Consume the builder and return the raw packet bytes.
    pub fn build(self) -> Vec<u8> {
        self.buf
    }
}

// ---------------------------------------------------------------------------
// PacketReader - for parsing client → server packets
// ---------------------------------------------------------------------------

pub struct PacketReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> PacketReader<'a> {
    /// Create a reader over raw packet data.
    pub fn new(data: &'a [u8]) -> Self {
        PacketReader { data, pos: 0 }
    }

    /// Create a reader starting after the opcode byte.
    pub fn after_opcode(data: &'a [u8]) -> Self {
        PacketReader { data, pos: 1 }
    }

    /// Read a single byte (8-bit unsigned).
    pub fn read_c(&mut self) -> u8 {
        if self.pos >= self.data.len() {
            return 0;
        }
        let v = self.data[self.pos];
        self.pos += 1;
        v
    }

    /// Read a 16-bit integer (little-endian, unsigned).
    pub fn read_h(&mut self) -> u16 {
        if self.pos + 1 >= self.data.len() {
            return 0;
        }
        let v = u16::from_le_bytes([self.data[self.pos], self.data[self.pos + 1]]);
        self.pos += 2;
        v
    }

    /// Read a 32-bit integer (little-endian, signed).
    pub fn read_d(&mut self) -> i32 {
        if self.pos + 3 >= self.data.len() {
            return 0;
        }
        let v = i32::from_le_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
        ]);
        self.pos += 4;
        v
    }

    /// Read a null-terminated string, decoding from Big5.
    pub fn read_s(&mut self) -> String {
        let start = self.pos;
        while self.pos < self.data.len() && self.data[self.pos] != 0 {
            self.pos += 1;
        }
        let s = crate::protocol::encoding::decode_big5(&self.data[start..self.pos]);
        if self.pos < self.data.len() {
            self.pos += 1; // skip null terminator
        }
        s
    }

    /// Skip n bytes.
    pub fn skip(&mut self, n: usize) {
        self.pos += n;
    }

    /// Check if there is more data to read.
    pub fn has_remaining(&self) -> bool {
        self.pos < self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_write_types() {
        let pkt = PacketBuilder::new(0x8B)
            .write_c(0x01)
            .write_h(0x1234)
            .write_d(0x12345678)
            .write_s(Some("hi"))
            .build();

        assert_eq!(pkt[0], 0x8B); // opcode
        assert_eq!(pkt[1], 0x01); // writeC
        assert_eq!(pkt[2], 0x34); // writeH low
        assert_eq!(pkt[3], 0x12); // writeH high
        assert_eq!(pkt[4], 0x78); // writeD byte 0
        assert_eq!(pkt[5], 0x56); // writeD byte 1
        assert_eq!(pkt[6], 0x34); // writeD byte 2
        assert_eq!(pkt[7], 0x12); // writeD byte 3
        assert_eq!(pkt[8], b'h');
        assert_eq!(pkt[9], b'i');
        assert_eq!(pkt[10], 0x00); // null terminator
    }

    #[test]
    fn test_reader_read_types() {
        let data = vec![
            0x8B, 0x01, 0x34, 0x12, 0x78, 0x56, 0x34, 0x12, b'h', b'i', 0x00,
        ];
        let mut r = PacketReader::new(&data);
        assert_eq!(r.read_c(), 0x8B);
        assert_eq!(r.read_c(), 0x01);
        assert_eq!(r.read_h(), 0x1234);
        assert_eq!(r.read_d(), 0x12345678);
        assert_eq!(r.read_s(), "hi");
    }
}
