/// XOR-based packet cipher for Lineage 1 (3.80c TW).
///
/// Ported 1:1 from Java `Cipher.java`.
/// Each client session gets its own Cipher instance with independent
/// encrypt (eb) and decrypt (db) key states.

// 修改後
const C1: i32 = 0x9c30d539u32 as i32;
const C2: i32 = 0x930fd7e2u32 as i32;
const C3: i32 = 0x7c72e993;
const C4: i32 = 0x287effc3;

pub struct Cipher {
    read_key: [u8; 4],
    write_key: [u8; 4],
}

impl Cipher {
    pub fn new(seed: i32) -> Self {
        let mut key = [0u8; 4];
        key[0] = (seed & 0xFF) as u8;
        key[1] = ((seed >> 8) & 0xFF) as u8;
        key[2] = ((seed >> 16) & 0xFF) as u8;
        key[3] = ((seed >> 24) & 0xFF) as u8;

        Self {
            read_key: key,
            write_key: key,
        }
    }

    pub fn decrypt(&mut self, data: &mut [u8]) {
        // 3.80c 的解密公式：第一個 byte 單獨 XOR，後續 byte 依賴前一個 byte
        let mut prev = data[0];
        data[0] ^= self.read_key[0];

        for i in 1..data.len() {
            let cur = data[i];
            data[i] ^= self.read_key[i % 4] ^ prev;
            prev = cur;
        }

        // ✨ 關鍵：解密完「立刻」更新 Read Key，否則下一個封包會解不開
        self.update_read_key(data);
    }

    pub fn encrypt(&mut self, data: &mut [u8]) {
        // 3.80c 連續 XOR 加密邏輯
        data[0] ^= self.write_key[0];
        for i in 1..data.len() {
            data[i] ^= data[i - 1] ^ self.write_key[i % 4];
        }

        // ✨ 絕對不能忘記：用『加密後』的密文更新下一次的 Key
        self.update_write_key(data);
    }

    fn update_read_key(&mut self, data: &[u8]) {
        let mut key = i32::from_le_bytes(self.read_key);

        // 使用雙括號確保先轉型再位移
        let mut t: i32 = (data[0] as i32) & 0xff;
        t |= ((data[1] as i32) << 8) & 0xff00;
        t |= ((data[2] as i32) << 16) & 0xff0000;
        t |= ((data[3] as i32) << 24) & 0x7f000000;

        key ^= t;
        key = key.wrapping_mul(C1).wrapping_add(C2);
        key ^= C3;
        key ^= C4;

        self.read_key = key.to_le_bytes();
    }

    // src/network/cipher.rs

    fn update_write_key(&mut self, data: &[u8]) {
        // 1. 將目前的 write_key 轉回 i32
        let mut key = i32::from_le_bytes(self.write_key);

        // 2. 🔹 定義並提取加密後數據的前 4 bytes 作為跳轉種子 (修復 E0425)
        let mut t: i32 = 0;
        t |= (data[0] as i32) & 0xFF;
        t |= ((data[1] as i32) << 8) & 0xFF00;
        t |= ((data[2] as i32) << 16) & 0xFF0000;
        t |= ((data[3] as i32) << 24) & -16777216; // 處理符號位

        // 3. LCG 運算 (使用與 read_key 相同的常數以確保同步)
        key ^= t;
        key = key.wrapping_mul(C1).wrapping_add(C2);
        key ^= C3;
        key ^= C4;

        // 4. 更新回 write_key
        self.write_key = key.to_le_bytes();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cipher_init_deterministic() {
        let c1 = Cipher::new(0x12345678);
        let c2 = Cipher::new(0x12345678);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        // Server encrypts, client decrypts using same key
        let mut server_cipher = Cipher::new(0xDEADBEEFu32 as i32);
        let mut client_cipher = Cipher::new(0xDEADBEEFu32 as i32);

        let original = b"Hello, Lineage!".to_vec();

        // Pad to 4-byte alignment (like Java ServerBasePacket.getBytes())
        let mut data = original.clone();
        while data.len() % 4 != 0 {
            data.push(0);
        }

        let plaintext_copy = data.clone();

        // Server encrypts
        server_cipher.encrypt(&mut data);
        assert_ne!(
            data, plaintext_copy,
            "Encrypted data should differ from plaintext"
        );

        // Client decrypts (using db key, same initial state)
        client_cipher.decrypt(&mut data);
        assert_eq!(data, plaintext_copy, "Decrypted data should match original");
    }

    #[test]
    fn test_multiple_packets_stay_in_sync() {
        let mut server = Cipher::new(0xCAFEBABEu32 as i32);
        let mut client = Cipher::new(0xCAFEBABEu32 as i32);

        for i in 0..10 {
            let mut data = vec![i as u8; 8 + (i * 4)]; // Varying sizes, 4-byte aligned
            let original = data.clone();

            server.encrypt(&mut data);
            client.decrypt(&mut data);

            assert_eq!(data, original, "Packet {} roundtrip failed", i);
        }
    }

    #[test]
    fn test_minimum_4_byte_packet() {
        let mut enc = Cipher::new(0x11111111);
        let mut dec = Cipher::new(0x11111111);

        let mut data = vec![0xAA, 0xBB, 0xCC, 0xDD];
        let original = data.clone();

        enc.encrypt(&mut data);
        dec.decrypt(&mut data);
        assert_eq!(data, original);
    }

    #[test]
    fn test_key_diverges_without_sync() {
        let mut enc = Cipher::new(0x99999999u32 as i32);
        let mut dec = Cipher::new(0x99999999u32 as i32);

        // Encrypt two packets on server side
        let mut pkt1 = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut pkt2 = vec![10, 20, 30, 40];

        enc.encrypt(&mut pkt1);
        enc.encrypt(&mut pkt2);

        // Client only decrypts first packet
        dec.decrypt(&mut pkt1);
        // If we try to decrypt pkt2 now, it should work because keys are in sync
        let pkt2_orig = vec![10, 20, 30, 40];
        dec.decrypt(&mut pkt2);
        assert_eq!(pkt2, pkt2_orig, "Sequential decrypt should stay in sync");
    }
}
