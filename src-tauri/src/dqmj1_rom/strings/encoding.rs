use crate::dqmj1_rom::{regions::Region, strings::encoding_na::get_na_character_encoding};

pub struct CharacterEncoding {
    pub byte_to_char_map: Vec<(Vec<u8>, &'static str)>,
}

impl CharacterEncoding {
    pub fn get(region: Region) -> CharacterEncoding {
        match region {
            Region::NorthAmerica => get_na_character_encoding(),
            _ => panic!(),
        }
    }

    pub fn read_string(&self, bytes: &[u8]) -> String {
        let mut buffer: Vec<u8> = Vec::new();
        for (i, byte) in bytes.iter().enumerate() {
            let byte = *byte;

            // The skipping of 0x0A at possible string start is due to an edge case I saw at
            // 0x0207d792
            if (byte == 0x00 || byte == 0x0A) && buffer.is_empty() {
                continue;
            } else if byte == 0xFF || (byte == 0xFE && bytes[i + 1] == 0x00) {
                // Note: The check against 0xFE is due to an edge case at 0x02079c16.
                return self.bytes_to_string(&buffer);
            } else {
                buffer.push(byte);
            }
        }

        panic!()
    }

    fn bytes_to_string(&self, bytes: &[u8]) -> String {
        let mut chars: Vec<&str> = vec![];
        let mut i = 0;
        while i < bytes.len() {
            let byte = bytes[i];
            if byte == 0xFF {
                break;
            }

            let result = self.get_bytes_match(bytes, i);
            let char = result.0;
            i = result.1;

            chars.push(char);
        }

        chars.into_iter().collect()
    }

    fn get_bytes_match(&self, bytes: &[u8], i: usize) -> (&str, usize) {
        let mut matches = self.byte_to_char_map.clone();
        let mut offset: usize = 0;
        while !matches.is_empty() {
            let mut remaining_matches = vec![];
            for (match_bytes, match_char) in matches.iter() {
                if match_bytes[offset] == bytes[i + offset] {
                    if match_bytes.len() == offset + 1 {
                        return (*match_char, i + offset + 1);
                    } else {
                        remaining_matches.push((match_bytes.clone(), *match_char))
                    }
                }
            }

            matches = remaining_matches;
            offset += 1;
        }

        if matches.len() == 1 {
            return (matches[0].1, i + offset);
        }

        panic!("Not implemented yet")
    }
}
