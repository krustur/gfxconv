#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

mod buffer_reader;
mod raw_chunk;
mod raw_chunk_array;
use raw_chunk::RawChunk;
use raw_chunk_array::RawChunkArray;
use std::cmp;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;
use std::str;

#[derive(Debug)]
// #[derive(PartialEq)]
pub enum ErrorKind {
    IoError(io::Error),
    FileTooShort,
    UnknownChunk(String),
    ChunkTooShort,
    ZeroSizeChunk,
    UnsupportedFormType,
    UnknownFormType,
    NoChunksFound,
    MultipleRootChunksFound,
    UnknownIlbmChunk(String),
    InvalidChunkSize,
    BmhdNotYetSet,
    IlbmNoOp,
}

impl std::cmp::PartialEq for ErrorKind {
    fn eq(&self, other: &ErrorKind) -> bool {
        match self {
            ErrorKind::IoError(s) => {
                match other {
                    ErrorKind::IoError(o) => false, //TODO: Compare io errors
                    _ => false,
                }
            }
            ErrorKind::UnknownChunk(s) => match other {
                ErrorKind::UnknownChunk(o) => s == o,
                _ => false,
            },
            ErrorKind::FileTooShort => match other {
                ErrorKind::FileTooShort => true,
                _ => false,
            },
            ErrorKind::ChunkTooShort => match other {
                ErrorKind::ChunkTooShort => true,
                _ => false,
            },
            ErrorKind::ZeroSizeChunk => match other {
                ErrorKind::ZeroSizeChunk => true,
                _ => false,
            },
            ErrorKind::UnsupportedFormType => match other {
                ErrorKind::UnsupportedFormType => true,
                _ => false,
            },
            ErrorKind::UnknownFormType => match other {
                ErrorKind::UnknownFormType => true,
                _ => false,
            },
            ErrorKind::NoChunksFound => match other {
                ErrorKind::NoChunksFound => true,
                _ => false,
            },
            ErrorKind::MultipleRootChunksFound => match other {
                ErrorKind::MultipleRootChunksFound => true,
                _ => false,
            },
            ErrorKind::UnknownIlbmChunk(s) => match other {
                ErrorKind::UnknownIlbmChunk(o) => s == o,
                _ => false,
            },
            ErrorKind::InvalidChunkSize => match other {
                ErrorKind::InvalidChunkSize => true,
                _ => false,
            },
            ErrorKind::BmhdNotYetSet => match other {
                ErrorKind::BmhdNotYetSet => true,
                _ => false,
            },
            ErrorKind::IlbmNoOp => match other {
                ErrorKind::IlbmNoOp => true,
                _ => false,
            },
        }
    }
}

#[derive(Debug)]
pub struct IffFile {
    pub ilbm: FormIlbmChunk,
}

// UnknownChunk
pub struct UnknownChunk {
    pub id: String,
}

impl UnknownChunk {
    pub fn new(id: String) -> UnknownChunk {
        UnknownChunk { id }
    }
}

// FormIlbmChunk
// #[derive(Debug)]
pub struct FormIlbmChunk {
    pub id: String,
    // pub children: Vec<Box<dyn Chunk>>,
    pub bmhd: Option<BmhdChunk>,
    pub cmap: Option<CmapChunk>,
    pub body: Option<BodyChunk>,
}

impl FormIlbmChunk {
    pub fn new(id: String) -> FormIlbmChunk {
        FormIlbmChunk {
            // children: Vec::new(),
            id,
            bmhd: None,
            cmap: None,
            body: None,
        }
    }
}

impl fmt::Debug for FormIlbmChunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

// BmhdChunk
#[derive(Debug)]
pub struct BmhdChunk {
    pub id: String,
    pub width: u16,
    pub height: u16,
    pub x: i16,
    pub y: i16,
    pub number_of_planes: u8,
    pub masking: u8,
    pub compression: u8,
    pub transparent_color_number: u16,
    pub x_aspect: u8,
    pub y_aspect: u8,
    pub page_width: i16,
    pub page_height: i16,
}

#[derive(Clone)]
pub struct ColRgbU8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl fmt::Debug for ColRgbU8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02X}-{:02X}-{:02X}", self.r, self.g, self.b)
    }
}

// #[derive(Debug)]
pub struct CmapChunk {
    pub rgb: Vec<ColRgbU8>,
}

impl fmt::Debug for CmapChunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cnt = cmp::min(4, self.rgb.len());

        for i in 0..cnt {
            let wr = write!(f, "{:?} ", self.rgb[i])?;
        }
        Ok(())
    }
}

pub struct BodyChunk {
    pub pixels: Vec<u8>,
    pub interleaved_bitmap_data: Vec<u8>,
    pub raw_buffer: Vec<u8>,
}

pub fn read_iff_file(file_path: path::PathBuf) -> Result<IffFile, ErrorKind> {
    println!("file_path {:?}", file_path);

    let mut f = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => return Err(ErrorKind::IoError(error)),
    };
    let mut buffer = Vec::new();
    match f.read_to_end(&mut buffer) {
        Ok(file) => file,
        Err(error) => return Err(ErrorKind::IoError(error)),
    };

    let iff_file = parse_iff_buffer(&buffer)?;

    Ok(iff_file)
}

pub fn parse_iff_buffer(buffer: &Vec<u8>) -> Result<IffFile, ErrorKind> {
    if buffer.len() < 12 {
        return Err(ErrorKind::FileTooShort);
    }

    let iff_file = IffFile {
        ilbm: parse_form_ilbm_buffer(buffer)?,
    };

    Ok(iff_file)
}

fn parse_form_ilbm_buffer(buffer: &[u8]) -> Result<FormIlbmChunk, ErrorKind> {
    let mut raw_chunk_array = RawChunkArray::from(buffer);
    let raw_root_chunk = match raw_chunk_array.get_first()? {
        Some(chunk) => chunk,
        None => return Err(ErrorKind::NoChunksFound),
    };

    match raw_chunk_array.get_next()? {
        Some(_) => return Err(ErrorKind::MultipleRootChunksFound),
        None => (),
    };

    let mut iff_form_chunk: FormIlbmChunk;

    println!("raw_root_chunk: {:?}", raw_root_chunk);

    match raw_root_chunk.id {
        "FORM" => {
            let form_type = raw_root_chunk.get_str(8)?;
            println!("FORM type {}", form_type);
            let root_chunk = match form_type {
                "ILBM" => {
                    println!("ILBM");
                    iff_form_chunk = FormIlbmChunk::new(raw_root_chunk.id.to_string());
                    let form_buffer = raw_root_chunk.get_slice_to_end(12);

                    let mut form_raw_chunk_array = RawChunkArray::from(form_buffer);

                    parse_ilbm_buffer(&mut form_raw_chunk_array, &mut iff_form_chunk)?;
                }
                // TODO: Move to separate method, decide for anim or ilbm earlier
                "ANIM" => {
                    println!("ANIM");
                    return Err(ErrorKind::UnsupportedFormType);
                }
                _ => return Err(ErrorKind::UnknownFormType),
            };
        }
        _ => return Err(ErrorKind::UnknownChunk(raw_root_chunk.id.to_string())),
    }

    Ok(iff_form_chunk)
}

fn parse_ilbm_buffer(
    raw_chunk_array: &mut RawChunkArray,
    ilbm: &mut FormIlbmChunk,
) -> Result<(), ErrorKind> {
    let mut raw_chunk = raw_chunk_array.get_first()?;
    while let Some(chunk) = raw_chunk {
        println!("tjoho {:?} {:?}", chunk.id, chunk.size);

        match chunk.id {
            "ANNO" => {
                let chunk = UnknownChunk::new(chunk.id.to_string());
                // iff_chunks.push(Box::new(chunk));
                // ilbm.Anno = Encoding.UTF8.GetString(chunk.Content, 0, (int)chunk.ContentLength);
                //
            }

            "BMHD" => {
                let chunk = get_bmhd_chunk(&chunk)?;
                println!("Bmhd: {:?}", chunk);
                ilbm.bmhd = Some(chunk);
            }

            "CMAP" => {
                let chunk = get_cmap_chunk(&chunk)?;
                println!("Cmap: {:?}", chunk);
                ilbm.cmap = Some(chunk);
                //
            }
            //
            "CAMG" => {
                let chunk = UnknownChunk::new(chunk.id.to_string());
                // iff_chunks.push(Box::new(chunk));
                // ilbm.Camg = new Camg(chunk);
                //
            }

            "BODY" => {
                // let chunk = UnknownChunk::new(chunk.id.to_string());
                let chunk = get_body_chunk(&chunk, &ilbm.bmhd)?;
                // println!("Cmap: {:?}", chunk);
                ilbm.body = Some(chunk);
                //
            }

            "ANHD" => {
                let chunk = UnknownChunk::new(chunk.id.to_string());
                // iff_chunks.push(Box::new(chunk));
                // ilbm.Anhd = new Anhd(chunk);
                //
            }

            "DLTA" => {
                let chunk = UnknownChunk::new(chunk.id.to_string());
                // iff_chunks.push(Box::new(chunk));
                //                 ilbm.Dlta = new Dlta(chunk, ilbm, iffFile);
                //
            }

            "DPPS" => {
                let chunk = UnknownChunk::new(chunk.id.to_string());
                // iff_chunks.push(Box::new(chunk));
                // //todo: Handle DPPS
                // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
                //
            }
            "DRNG" => {
                let chunk = UnknownChunk::new(chunk.id.to_string());
                // iff_chunks.push(Box::new(chunk));
            }
            //DPaint IV enhanced color cycle chunk (EA)
            // http://wiki.amigaos.net/wiki/ILBM_IFF_Interleaved_Bitmap
            // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
            "BRNG" => {
                let chunk = UnknownChunk::new(chunk.id.to_string());
                // iff_chunks.push(Box::new(chunk));
                //unknown
                // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
            }

            "CRNG" => {
                let chunk = UnknownChunk::new(chunk.id.to_string());
                // iff_chunks.push(Box::new(chunk));
                // color register range
                // http://wiki.amigaos.net/wiki/ILBM_IFF_Interleaved_Bitmap
                // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
            }

            "DPI " => {
                let chunk = UnknownChunk::new(chunk.id.to_string());
                // iff_chunks.push(Box::new(chunk));
                // Dots per inch chunk
                // http://wiki.amigaos.net/wiki/ILBM_IFF_Interleaved_Bitmap
                // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
            }

            "GRAB" => {
                let chunk = UnknownChunk::new(chunk.id.to_string());
                // iff_chunks.push(Box::new(chunk));
                // locates a “handle” or “hotspot”
                // http://wiki.amigaos.net/wiki/ILBM_IFF_Interleaved_Bitmap
                // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
            }

            "DPXT" => {
                let chunk = UnknownChunk::new(chunk.id.to_string());
                // iff_chunks.push(Box::new(chunk));
                // unknown
                // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
            }

            "TINY" => {
                let chunk = UnknownChunk::new(chunk.id.to_string());
                // iff_chunks.push(Box::new(chunk));
                // Thumbnail
                // https://en.m.wikipedia.org/wiki/ILBM
                // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
            }

            _ => return Err(ErrorKind::UnknownIlbmChunk(chunk.id.to_string())),
        }

        raw_chunk = raw_chunk_array.get_next()?;
    }

    Ok(())
}

fn get_bmhd_chunk(raw_chunk: &RawChunk) -> Result<BmhdChunk, ErrorKind> {
    let chunk = BmhdChunk {
        id: String::from("BMHD"),
        width: raw_chunk.get_u16(8 + 0)?,
        height: raw_chunk.get_u16(8 + 2)?,
        x: raw_chunk.get_i16(8 + 4)?,
        y: raw_chunk.get_i16(8 + 6)?,
        number_of_planes: raw_chunk.get_u8(8 + 8)?,
        masking: raw_chunk.get_u8(8 + 9)?,
        compression: raw_chunk.get_u8(8 + 10)?,
        // UBYTE pad1
        transparent_color_number: raw_chunk.get_u16(8 + 12)?,
        x_aspect: raw_chunk.get_u8(8 + 14)?,
        y_aspect: raw_chunk.get_u8(8 + 15)?,
        page_width: raw_chunk.get_i16(8 + 16)?,
        page_height: raw_chunk.get_i16(8 + 18)?,
    };

    Ok(chunk)
}

fn get_cmap_chunk(raw_chunk: &RawChunk) -> Result<CmapChunk, ErrorKind> {
    let no_colors = (raw_chunk.size) / 3;
    let mut chunk = CmapChunk {
        rgb: vec![ColRgbU8 { r: 0, g: 0, b: 0 }; no_colors],
    };

    for i in 0..no_colors {
        chunk.rgb[i].r = raw_chunk.get_u8(8 + i * 3 + 0)?;
        chunk.rgb[i].g = raw_chunk.get_u8(8 + i * 3 + 1)?;
        chunk.rgb[i].b = raw_chunk.get_u8(8 + i * 3 + 2)?;
    }

    Ok(chunk)
}

fn get_body_chunk(raw_chunk: &RawChunk, bmhd: &Option<BmhdChunk>) -> Result<BodyChunk, ErrorKind> {
    let bmhd = match bmhd {
        None => return Err(ErrorKind::BmhdNotYetSet),
        Some(b) => b,
    };

    let no_pixels = bmhd.width as usize * bmhd.height as usize;

    let mut actual_number_of_planes = bmhd.number_of_planes as usize;
    if bmhd.masking == 1 {
        actual_number_of_planes += 1;
    }
    let bytes_per_row_per_plane = ((bmhd.width as usize + 15) & 0xfffffff0) / 8;
    let bytes_per_row_all_planes = bytes_per_row_per_plane * actual_number_of_planes;
    let interleaved_size = bytes_per_row_all_planes * bmhd.height as usize;

    let mut chunk = BodyChunk {
        raw_buffer: vec![0; raw_chunk.size],
        interleaved_bitmap_data: vec![0; interleaved_size],
        pixels: vec![0; no_pixels],
    };

    &chunk
        .raw_buffer
        .clone_from_slice(raw_chunk.get_slice(8..raw_chunk.size + 8));

    let mut pos   : usize = 0;
    let mut target_pos  : usize = 0;
    let mut written_bytes : usize = 0;
    while pos < raw_chunk.size {
        let n = raw_chunk.get_i8(pos)?;
        pos += 1;
        if n == -128
        {
            return Err(ErrorKind::IlbmNoOp);
        }
        else if n < 0{
            let new_n = -n;
            for i in 0..new_n + 1 {
                chunk.interleaved_bitmap_data[target_pos] = raw_chunk.get_u8(pos)?;
                target_pos += 1;
            }
            written_bytes += new_n as usize + 1;
            pos += 1;
        }
        else{
            for i in 0..n+1 {
                chunk.interleaved_bitmap_data[target_pos] = raw_chunk.get_u8(pos)?;
                target_pos += 1;
                pos += 1;
            }
            written_bytes += n as usize + 1;
        }
    }

    Ok(chunk)
}
