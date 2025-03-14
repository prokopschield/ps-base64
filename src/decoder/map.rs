#[must_use]
pub const fn decode_base64_char(char: u8) -> u8 {
    if char >= b'0' && char <= b'9' {
        char - b'0' + 52
    } else if char >= b'A' && char <= b'Z' {
        char - b'A'
    } else if char >= b'a' && char <= b'z' {
        char - b'a' + 26
    } else if matches!(char, b'+' | b'-' | b'~') {
        62
    } else if matches!(char, b',' | b'/' | b'_') {
        63
    } else if matches!(char, b'=') {
        0
    } else {
        char % 64
    }
}

#[allow(clippy::too_many_lines)]
#[must_use]
pub const fn create_decode_map() -> [u8; 256] {
    let mut map = [0u8; 256];

    {
        map[0x00] = decode_base64_char(0x00);
        map[0x01] = decode_base64_char(0x01);
        map[0x02] = decode_base64_char(0x02);
        map[0x03] = decode_base64_char(0x03);
        map[0x04] = decode_base64_char(0x04);
        map[0x05] = decode_base64_char(0x05);
        map[0x06] = decode_base64_char(0x06);
        map[0x07] = decode_base64_char(0x07);
        map[0x08] = decode_base64_char(0x08);
        map[0x09] = decode_base64_char(0x09);
        map[0x0A] = decode_base64_char(0x0A);
        map[0x0B] = decode_base64_char(0x0B);
        map[0x0C] = decode_base64_char(0x0C);
        map[0x0D] = decode_base64_char(0x0D);
        map[0x0E] = decode_base64_char(0x0E);
        map[0x0F] = decode_base64_char(0x0F);
        map[0x10] = decode_base64_char(0x10);
        map[0x11] = decode_base64_char(0x11);
        map[0x12] = decode_base64_char(0x12);
        map[0x13] = decode_base64_char(0x13);
        map[0x14] = decode_base64_char(0x14);
        map[0x15] = decode_base64_char(0x15);
        map[0x16] = decode_base64_char(0x16);
        map[0x17] = decode_base64_char(0x17);
        map[0x18] = decode_base64_char(0x18);
        map[0x19] = decode_base64_char(0x19);
        map[0x1A] = decode_base64_char(0x1A);
        map[0x1B] = decode_base64_char(0x1B);
        map[0x1C] = decode_base64_char(0x1C);
        map[0x1D] = decode_base64_char(0x1D);
        map[0x1E] = decode_base64_char(0x1E);
        map[0x1F] = decode_base64_char(0x1F);
        map[0x20] = decode_base64_char(0x20);
        map[0x21] = decode_base64_char(0x21);
        map[0x22] = decode_base64_char(0x22);
        map[0x23] = decode_base64_char(0x23);
        map[0x24] = decode_base64_char(0x24);
        map[0x25] = decode_base64_char(0x25);
        map[0x26] = decode_base64_char(0x26);
        map[0x27] = decode_base64_char(0x27);
        map[0x28] = decode_base64_char(0x28);
        map[0x29] = decode_base64_char(0x29);
        map[0x2A] = decode_base64_char(0x2A);
        map[0x2B] = decode_base64_char(0x2B);
        map[0x2C] = decode_base64_char(0x2C);
        map[0x2D] = decode_base64_char(0x2D);
        map[0x2E] = decode_base64_char(0x2E);
        map[0x2F] = decode_base64_char(0x2F);
        map[0x30] = decode_base64_char(0x30);
        map[0x31] = decode_base64_char(0x31);
        map[0x32] = decode_base64_char(0x32);
        map[0x33] = decode_base64_char(0x33);
        map[0x34] = decode_base64_char(0x34);
        map[0x35] = decode_base64_char(0x35);
        map[0x36] = decode_base64_char(0x36);
        map[0x37] = decode_base64_char(0x37);
        map[0x38] = decode_base64_char(0x38);
        map[0x39] = decode_base64_char(0x39);
        map[0x3A] = decode_base64_char(0x3A);
        map[0x3B] = decode_base64_char(0x3B);
        map[0x3C] = decode_base64_char(0x3C);
        map[0x3D] = decode_base64_char(0x3D);
        map[0x3E] = decode_base64_char(0x3E);
        map[0x3F] = decode_base64_char(0x3F);
        map[0x40] = decode_base64_char(0x40);
        map[0x41] = decode_base64_char(0x41);
        map[0x42] = decode_base64_char(0x42);
        map[0x43] = decode_base64_char(0x43);
        map[0x44] = decode_base64_char(0x44);
        map[0x45] = decode_base64_char(0x45);
        map[0x46] = decode_base64_char(0x46);
        map[0x47] = decode_base64_char(0x47);
        map[0x48] = decode_base64_char(0x48);
        map[0x49] = decode_base64_char(0x49);
        map[0x4A] = decode_base64_char(0x4A);
        map[0x4B] = decode_base64_char(0x4B);
        map[0x4C] = decode_base64_char(0x4C);
        map[0x4D] = decode_base64_char(0x4D);
        map[0x4E] = decode_base64_char(0x4E);
        map[0x4F] = decode_base64_char(0x4F);
        map[0x50] = decode_base64_char(0x50);
        map[0x51] = decode_base64_char(0x51);
        map[0x52] = decode_base64_char(0x52);
        map[0x53] = decode_base64_char(0x53);
        map[0x54] = decode_base64_char(0x54);
        map[0x55] = decode_base64_char(0x55);
        map[0x56] = decode_base64_char(0x56);
        map[0x57] = decode_base64_char(0x57);
        map[0x58] = decode_base64_char(0x58);
        map[0x59] = decode_base64_char(0x59);
        map[0x5A] = decode_base64_char(0x5A);
        map[0x5B] = decode_base64_char(0x5B);
        map[0x5C] = decode_base64_char(0x5C);
        map[0x5D] = decode_base64_char(0x5D);
        map[0x5E] = decode_base64_char(0x5E);
        map[0x5F] = decode_base64_char(0x5F);
        map[0x60] = decode_base64_char(0x60);
        map[0x61] = decode_base64_char(0x61);
        map[0x62] = decode_base64_char(0x62);
        map[0x63] = decode_base64_char(0x63);
        map[0x64] = decode_base64_char(0x64);
        map[0x65] = decode_base64_char(0x65);
        map[0x66] = decode_base64_char(0x66);
        map[0x67] = decode_base64_char(0x67);
        map[0x68] = decode_base64_char(0x68);
        map[0x69] = decode_base64_char(0x69);
        map[0x6A] = decode_base64_char(0x6A);
        map[0x6B] = decode_base64_char(0x6B);
        map[0x6C] = decode_base64_char(0x6C);
        map[0x6D] = decode_base64_char(0x6D);
        map[0x6E] = decode_base64_char(0x6E);
        map[0x6F] = decode_base64_char(0x6F);
        map[0x70] = decode_base64_char(0x70);
        map[0x71] = decode_base64_char(0x71);
        map[0x72] = decode_base64_char(0x72);
        map[0x73] = decode_base64_char(0x73);
        map[0x74] = decode_base64_char(0x74);
        map[0x75] = decode_base64_char(0x75);
        map[0x76] = decode_base64_char(0x76);
        map[0x77] = decode_base64_char(0x77);
        map[0x78] = decode_base64_char(0x78);
        map[0x79] = decode_base64_char(0x79);
        map[0x7A] = decode_base64_char(0x7A);
        map[0x7B] = decode_base64_char(0x7B);
        map[0x7C] = decode_base64_char(0x7C);
        map[0x7D] = decode_base64_char(0x7D);
        map[0x7E] = decode_base64_char(0x7E);
        map[0x7F] = decode_base64_char(0x7F);
        map[0x80] = decode_base64_char(0x80);
        map[0x81] = decode_base64_char(0x81);
        map[0x82] = decode_base64_char(0x82);
        map[0x83] = decode_base64_char(0x83);
        map[0x84] = decode_base64_char(0x84);
        map[0x85] = decode_base64_char(0x85);
        map[0x86] = decode_base64_char(0x86);
        map[0x87] = decode_base64_char(0x87);
        map[0x88] = decode_base64_char(0x88);
        map[0x89] = decode_base64_char(0x89);
        map[0x8A] = decode_base64_char(0x8A);
        map[0x8B] = decode_base64_char(0x8B);
        map[0x8C] = decode_base64_char(0x8C);
        map[0x8D] = decode_base64_char(0x8D);
        map[0x8E] = decode_base64_char(0x8E);
        map[0x8F] = decode_base64_char(0x8F);
        map[0x90] = decode_base64_char(0x90);
        map[0x91] = decode_base64_char(0x91);
        map[0x92] = decode_base64_char(0x92);
        map[0x93] = decode_base64_char(0x93);
        map[0x94] = decode_base64_char(0x94);
        map[0x95] = decode_base64_char(0x95);
        map[0x96] = decode_base64_char(0x96);
        map[0x97] = decode_base64_char(0x97);
        map[0x98] = decode_base64_char(0x98);
        map[0x99] = decode_base64_char(0x99);
        map[0x9A] = decode_base64_char(0x9A);
        map[0x9B] = decode_base64_char(0x9B);
        map[0x9C] = decode_base64_char(0x9C);
        map[0x9D] = decode_base64_char(0x9D);
        map[0x9E] = decode_base64_char(0x9E);
        map[0x9F] = decode_base64_char(0x9F);
        map[0xA0] = decode_base64_char(0xA0);
        map[0xA1] = decode_base64_char(0xA1);
        map[0xA2] = decode_base64_char(0xA2);
        map[0xA3] = decode_base64_char(0xA3);
        map[0xA4] = decode_base64_char(0xA4);
        map[0xA5] = decode_base64_char(0xA5);
        map[0xA6] = decode_base64_char(0xA6);
        map[0xA7] = decode_base64_char(0xA7);
        map[0xA8] = decode_base64_char(0xA8);
        map[0xA9] = decode_base64_char(0xA9);
        map[0xAA] = decode_base64_char(0xAA);
        map[0xAB] = decode_base64_char(0xAB);
        map[0xAC] = decode_base64_char(0xAC);
        map[0xAD] = decode_base64_char(0xAD);
        map[0xAE] = decode_base64_char(0xAE);
        map[0xAF] = decode_base64_char(0xAF);
        map[0xB0] = decode_base64_char(0xB0);
        map[0xB1] = decode_base64_char(0xB1);
        map[0xB2] = decode_base64_char(0xB2);
        map[0xB3] = decode_base64_char(0xB3);
        map[0xB4] = decode_base64_char(0xB4);
        map[0xB5] = decode_base64_char(0xB5);
        map[0xB6] = decode_base64_char(0xB6);
        map[0xB7] = decode_base64_char(0xB7);
        map[0xB8] = decode_base64_char(0xB8);
        map[0xB9] = decode_base64_char(0xB9);
        map[0xBA] = decode_base64_char(0xBA);
        map[0xBB] = decode_base64_char(0xBB);
        map[0xBC] = decode_base64_char(0xBC);
        map[0xBD] = decode_base64_char(0xBD);
        map[0xBE] = decode_base64_char(0xBE);
        map[0xBF] = decode_base64_char(0xBF);
        map[0xC0] = decode_base64_char(0xC0);
        map[0xC1] = decode_base64_char(0xC1);
        map[0xC2] = decode_base64_char(0xC2);
        map[0xC3] = decode_base64_char(0xC3);
        map[0xC4] = decode_base64_char(0xC4);
        map[0xC5] = decode_base64_char(0xC5);
        map[0xC6] = decode_base64_char(0xC6);
        map[0xC7] = decode_base64_char(0xC7);
        map[0xC8] = decode_base64_char(0xC8);
        map[0xC9] = decode_base64_char(0xC9);
        map[0xCA] = decode_base64_char(0xCA);
        map[0xCB] = decode_base64_char(0xCB);
        map[0xCC] = decode_base64_char(0xCC);
        map[0xCD] = decode_base64_char(0xCD);
        map[0xCE] = decode_base64_char(0xCE);
        map[0xCF] = decode_base64_char(0xCF);
        map[0xD0] = decode_base64_char(0xD0);
        map[0xD1] = decode_base64_char(0xD1);
        map[0xD2] = decode_base64_char(0xD2);
        map[0xD3] = decode_base64_char(0xD3);
        map[0xD4] = decode_base64_char(0xD4);
        map[0xD5] = decode_base64_char(0xD5);
        map[0xD6] = decode_base64_char(0xD6);
        map[0xD7] = decode_base64_char(0xD7);
        map[0xD8] = decode_base64_char(0xD8);
        map[0xD9] = decode_base64_char(0xD9);
        map[0xDA] = decode_base64_char(0xDA);
        map[0xDB] = decode_base64_char(0xDB);
        map[0xDC] = decode_base64_char(0xDC);
        map[0xDD] = decode_base64_char(0xDD);
        map[0xDE] = decode_base64_char(0xDE);
        map[0xDF] = decode_base64_char(0xDF);
        map[0xE0] = decode_base64_char(0xE0);
        map[0xE1] = decode_base64_char(0xE1);
        map[0xE2] = decode_base64_char(0xE2);
        map[0xE3] = decode_base64_char(0xE3);
        map[0xE4] = decode_base64_char(0xE4);
        map[0xE5] = decode_base64_char(0xE5);
        map[0xE6] = decode_base64_char(0xE6);
        map[0xE7] = decode_base64_char(0xE7);
        map[0xE8] = decode_base64_char(0xE8);
        map[0xE9] = decode_base64_char(0xE9);
        map[0xEA] = decode_base64_char(0xEA);
        map[0xEB] = decode_base64_char(0xEB);
        map[0xEC] = decode_base64_char(0xEC);
        map[0xED] = decode_base64_char(0xED);
        map[0xEE] = decode_base64_char(0xEE);
        map[0xEF] = decode_base64_char(0xEF);
        map[0xF0] = decode_base64_char(0xF0);
        map[0xF1] = decode_base64_char(0xF1);
        map[0xF2] = decode_base64_char(0xF2);
        map[0xF3] = decode_base64_char(0xF3);
        map[0xF4] = decode_base64_char(0xF4);
        map[0xF5] = decode_base64_char(0xF5);
        map[0xF6] = decode_base64_char(0xF6);
        map[0xF7] = decode_base64_char(0xF7);
        map[0xF8] = decode_base64_char(0xF8);
        map[0xF9] = decode_base64_char(0xF9);
        map[0xFA] = decode_base64_char(0xFA);
        map[0xFB] = decode_base64_char(0xFB);
        map[0xFC] = decode_base64_char(0xFC);
        map[0xFD] = decode_base64_char(0xFD);
        map[0xFE] = decode_base64_char(0xFE);
        map[0xFF] = decode_base64_char(0xFF);
    }

    map
}

pub const DECODE_MAP: [u8; 256] = create_decode_map();
