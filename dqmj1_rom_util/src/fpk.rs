use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use binrw::{binread, io::SeekFrom};

#[binread]
#[brw(little)]
#[derive(Debug, PartialEq)]
pub struct FpkFile {
    pub name_info: [u8; 0x20],
    pub offset: u32,
    pub size: u32,

    #[br(
        seek_before = SeekFrom::Start(offset as u64),
        count = size,
        restore_position
    )]
    pub data: Vec<u8>,
}

impl FpkFile {
    pub fn write(&self, filepath: &Path) -> std::io::Result<()> {
        let mut file = File::create(filepath)?;
        file.write_all(&self.data)?;

        Ok(())
    }
}

#[binread]
#[brw(little)]
#[derive(Debug, PartialEq)]
#[br(magic = b"\x46\x50\x4B\x00")] // FPK
pub struct Fpk {
    pub num_files: u32,

    #[br(count = num_files)]
    pub files: Vec<FpkFile>,
}

impl Fpk {
    pub fn write_to_directory(&self, directory: &Path) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir(directory)?;

        for file in self.files.iter() {
            let filename_bytes: Vec<u8> = file
                .name_info
                .iter()
                .cloned()
                .take_while(|value| *value != 0x00)
                .collect();
            let filename = str::from_utf8(&filename_bytes)?;
            let filepath = directory.join(filename);
            file.write(&filepath)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use binrw::BinRead;

    use super::*;

    fn read_fpk_from_file(filepath: &str) -> Fpk {
        let mut reader = File::open(filepath).unwrap();
        Fpk::read(&mut reader).unwrap()
    }

    #[test]
    fn test_read_single_file_fpk() {
        let actual = read_fpk_from_file("test/data/single_file.fpk");

        let expected = Fpk {
            num_files: 1,
            files: vec![FpkFile {
                name_info: [
                    102, 105, 108, 101, 46, 116, 120, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                offset: 48,
                size: 16,
                data: vec![
                    104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 0, 0, 0, 0, 0,
                ],
            }],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_read_two_file_fpk() {
        let actual = read_fpk_from_file("test/data/two_files.fpk");

        let expected = Fpk {
            num_files: 2,
            files: vec![
                FpkFile {
                    name_info: [
                        102, 105, 108, 101, 46, 116, 120, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                    offset: 88,
                    size: 16,
                    data: vec![
                        104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 0, 0, 0, 0, 0,
                    ],
                },
                FpkFile {
                    name_info: [
                        102, 105, 108, 101, 95, 50, 46, 116, 120, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                    offset: 104,
                    size: 28,
                    data: vec![
                        103, 111, 111, 100, 98, 121, 101, 32, 119, 111, 114, 108, 100, 44, 32, 102,
                        111, 114, 32, 116, 111, 100, 97, 121, 0, 0, 0, 0,
                    ],
                },
            ],
        };

        assert_eq!(actual, expected);
    }
}
