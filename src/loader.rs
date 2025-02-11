use crate::filesystem::RDAFileSystem;
use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt};
use std::borrow::ToOwned;
use std::io::{BufReader, Read, Seek};
use std::path::Path;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct BlockFlags: u32 {
        const IS_COMPRESSED = 1;
        const IS_ENCRYPTED = 2;
        const HAS_CONTIGUOUSDATASECTION = 4;
        const IS_DELETED = 8;
    }
}

pub struct BlockHeader {
    flags: BlockFlags,
    offset: i64,
    num_files: u32,
    compressed_file_headers_size: i64,
    uncompressed_file_headers_size: i64,
    next_block_offset: i64,
}

#[derive(Debug)]
pub enum LoaderError {
    IoError(std::io::Error),
    FileFormatError(usize),
}

impl From<std::io::Error> for LoaderError {
    fn from(e: std::io::Error) -> Self {
        LoaderError::IoError(e)
    }
}

impl std::fmt::Display for LoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for LoaderError {}

type Result<T> = std::result::Result<T, LoaderError>;

pub fn load_rda<R: Read + Seek>(mut reader: BufReader<R>) -> Result<RDAFileSystem> {
    check_header(&mut reader)?;

    let mut next_block_offset = read_first_block_offset(&mut reader)?;

    let mut filesystem = RDAFileSystem::default();

    while let Ok(offset) = reader.seek(std::io::SeekFrom::Start(next_block_offset as u64)) {
        let block_header = BlockHeader {
            offset: next_block_offset,
            flags: BlockFlags::from_bits_truncate(reader.read_u32::<LittleEndian>()?),
            num_files: reader.read_u32::<LittleEndian>()?,
            compressed_file_headers_size: reader.read_i64::<LittleEndian>()?,
            uncompressed_file_headers_size: reader.read_i64::<LittleEndian>()?,
            next_block_offset: reader.read_i64::<LittleEndian>()?,
        };

        load_files(&mut reader, &block_header)?;

        todo!("write accessor / push functions  for filesystem (recursively for paths)");

        next_block_offset = block_header.next_block_offset;
    }

    Ok(filesystem)
}

const FILEHEADER: [u8; 18] = *b"Resource File V2.2";

// private impls
fn check_header<R: Read>(reader: &mut BufReader<R>) -> Result<()> {
    let mut header = [0u8; 18];
    reader.read_exact(&mut header);

    if header != FILEHEADER {
        Err(LoaderError::FileFormatError(0))
    } else {
        Ok(())
    }
}

fn read_first_block_offset<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<i64> {
    reader.seek(std::io::SeekFrom::Start(784))?;
    Ok(reader.read_i64::<LittleEndian>()?)
}

fn load_files<R: Read>(reader: &mut BufReader<R>, block_header: &BlockHeader) -> Result<()> {
    for file_idx in 0..block_header.num_files {
        let header = read_file(reader);
    }
    Ok(())
}

struct FileHeader{}

fn read_file<R: Read>(reader: &mut BufReader<R>) -> Result<FileHeader> { Ok(FileHeader{})}
pub fn read_rda<P: AsRef<Path>>(path: P) -> std::io::Result<RDAFileSystem> {
    Ok(load_rda(BufReader::new(std::fs::File::open(path)?)).unwrap())
}
