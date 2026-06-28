use std::{
    ffi::OsString,
    fs::{self, DirEntry, File},
    io::Write,
    path::{Path, PathBuf},
};

use binrw::{binread, io::SeekFrom, meta::ReadMagic};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum FpkError {
    #[error("Found subdirectory \"{0}\" when creating fpk file")]
    Subdirectory(PathBuf),
    #[error("Found file with too long name \"{0}\", can only have 32 bytes max")]
    FilenameTooLong(String),
}

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
    pub fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.data)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum FileKind {
    Nsbmd,
    Atr,
    Scn,
    Pos,
    Nsbtx,
    Other(String),
}

impl FileKind {
    pub fn from_path(filepath: &Path) -> FileKind {
        let extension = filepath.extension().unwrap().to_str().unwrap().to_string();

        match extension.as_str() {
            "nsbmd" => FileKind::Nsbmd,
            "atr" => FileKind::Atr,
            "scn" => FileKind::Scn,
            "pos" => FileKind::Pos,
            "nsbtx" => FileKind::Nsbtx,
            _ => FileKind::Other(extension),
        }
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
    pub fn from_directory(directory: &Path) -> Result<Fpk, Box<dyn std::error::Error>> {
        let mut children = fs::read_dir(directory)?.collect::<Result<Vec<DirEntry>, _>>()?;
        children.sort_by_key(|child| (FileKind::from_path(&child.path()), child.file_name()));

        // Make sure we can create the fpk
        for child in children.iter() {
            if child.file_type()?.is_dir() {
                return Err(Box::new(FpkError::Subdirectory(child.path())));
            }
        }

        // Read in the files
        let mut files = vec![];
        for child in children.iter() {
            let name_info = encode_name_info(&child.file_name())?;
            let offset = 0; // temporary, since we don't know the size of the header yet
            let data = fs::read(child.path())?;
            let size = data.len().try_into()?;

            files.push(FpkFile {
                name_info,
                offset,
                size,
                data,
            });
        }

        // Set the offsets
        let header_size = 4 + 4 + (0x20 + 4 + 4) * files.len();
        let mut offset = header_size;
        for file in files.iter_mut() {
            file.offset = offset.try_into()?;
            offset += file.data.len();
        }

        Ok(Fpk {
            num_files: files.len().try_into()?,
            files,
        })
    }

    pub fn write_to_directory(&self, directory: &Path) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(directory)?;

        for file in self.files.iter() {
            let filename_bytes: Vec<u8> = file
                .name_info
                .iter()
                .cloned()
                .take_while(|value| *value != 0x00)
                .collect();
            let filename = str::from_utf8(&filename_bytes)?;
            let filepath = directory.join(filename);
            file.write(&mut File::create(filepath)?)?;
        }

        Ok(())
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        // Implemented manually due to file contents being stored in second half, rather than
        // directly with the file definition
        //
        // Assumes that all of the file content offsets are correct
        writer.write_all(&Fpk::MAGIC)?;
        writer.write_all(&self.num_files.to_le_bytes())?;

        for file in self.files.iter() {
            writer.write_all(&file.name_info)?;
            writer.write_all(&file.offset.to_le_bytes())?;
            writer.write_all(&file.size.to_le_bytes())?;
        }

        for file in self.files.iter() {
            writer.write_all(&file.data)?;
        }

        Ok(())
    }
}

fn encode_name_info(file_name: &OsString) -> Result<[u8; 0x20], FpkError> {
    let bytes = file_name.as_encoded_bytes();
    if bytes.len() > 32 {
        return Err(FpkError::FilenameTooLong(
            file_name.to_str().unwrap().to_string(),
        ));
    }

    let mut name_info = [0u8; 32];
    let len = bytes.len();
    name_info[..len].copy_from_slice(&bytes[..len]);

    Ok(name_info)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Cursor};

    use binrw::BinRead;
    use rstest::rstest;

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

    #[test]
    fn test_fpk_from_directory_empty() -> Result<(), Box<dyn std::error::Error>> {
        let dir = Path::new("test/data/fpk_empty");
        fs::create_dir_all(dir)?; // create empty dir, since it can't be tracked by git

        let actual = Fpk::from_directory(dir)?;

        let expected = Fpk {
            num_files: 0,
            files: vec![],
        };

        assert_eq!(actual, expected);

        let mut actual_contents = Cursor::new(vec![]);
        actual.write(&mut actual_contents)?;

        let expected_contents: Vec<u8> = vec![70, 80, 75, 0, 0, 0, 0, 0];

        assert_eq!(expected_contents, actual_contents.into_inner());

        Ok(())
    }

    #[test]
    fn test_fpk_from_directory_single_file() -> Result<(), Box<dyn std::error::Error>> {
        let actual = Fpk::from_directory(Path::new("test/data/fpk_single_file"))?;

        let expected = Fpk {
            num_files: 1,
            files: vec![FpkFile {
                name_info: [
                    102, 105, 108, 101, 49, 46, 116, 120, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                offset: 48,
                size: 3,
                data: vec![97, 98, 99],
            }],
        };

        assert_eq!(actual, expected);

        let mut actual_contents = Cursor::new(vec![]);
        actual.write(&mut actual_contents)?;

        let expected_contents: Vec<u8> = vec![
            70, 80, 75, 0, 1, 0, 0, 0, 102, 105, 108, 101, 49, 46, 116, 120, 116, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 3, 0, 0, 0, 97, 98, 99,
        ];

        assert_eq!(expected_contents, actual_contents.into_inner());

        Ok(())
    }

    #[test]
    fn test_fpk_from_directory_multiple_files() -> Result<(), Box<dyn std::error::Error>> {
        let actual = Fpk::from_directory(Path::new("test/data/fpk_multiple_files"))?;

        let expected = Fpk {
            num_files: 2,
            files: vec![
                FpkFile {
                    name_info: [
                        102, 105, 108, 101, 49, 46, 116, 120, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                    offset: 88,
                    size: 3,
                    data: vec![97, 98, 99],
                },
                FpkFile {
                    name_info: [
                        102, 105, 108, 101, 50, 46, 116, 120, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                    offset: 91,
                    size: 3,
                    data: vec![100, 101, 102],
                },
            ],
        };

        assert_eq!(actual, expected);

        let mut actual_contents = Cursor::new(vec![]);
        actual.write(&mut actual_contents)?;

        let expected_contents: Vec<u8> = vec![
            70, 80, 75, 0, 2, 0, 0, 0, 102, 105, 108, 101, 49, 46, 116, 120, 116, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 88, 0, 0, 0, 3, 0, 0, 0, 102, 105,
            108, 101, 50, 46, 116, 120, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 91, 0, 0, 0, 3, 0, 0, 0, 97, 98, 99, 100, 101, 102,
        ];

        assert_eq!(expected_contents, actual_contents.into_inner());

        Ok(())
    }

    #[rstest]
    #[case("test/data/single_file.fpk")]
    #[case("test/data/two_files.fpk")]
    fn test_read_write(#[case] filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let before = fs::read(filepath)?;
        let fpk = read_fpk_from_file(filepath);

        let mut writer = Cursor::new(vec![]);
        fpk.write(&mut writer)?;

        let after = writer.into_inner();

        assert_eq!(before, after);
        Ok(())
    }
}
