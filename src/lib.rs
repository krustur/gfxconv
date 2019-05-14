#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

mod buffer_reader;
mod raw_chunk;
mod raw_chunk_array;
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
            // _ => self == other,
        }
        // self == other
    }
}

// UnknownChunk
pub struct UnknownChunk {
    pub id: String,
}

impl UnknownChunk {
    pub fn new(id: String) -> UnknownChunk {
        UnknownChunk { id: id }
    }
}

// FormIlbmChunk
// #[derive(Debug)]
pub struct FormIlbmChunk {
    pub id: String,
    // pub children: Vec<Box<dyn Chunk>>,
    pub bmhd: Option<BmhdChunk>,
}

impl FormIlbmChunk {
    pub fn new(id: String) -> FormIlbmChunk {
        FormIlbmChunk {
            // children: Vec::new(),
            id: id,
            bmhd: None,
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

// // Chunk trait
// pub trait Chunk {
//     fn get_id(&self) -> &str;
// }

// impl fmt::Debug for Chunk {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.get_id())
//     }
// }

// impl Chunk for FormIlbmChunk {
//     fn get_id(&self) -> &str {
//         &self.id
//     }
// }

// impl Chunk for UnknownChunk {
//     fn get_id(&self) -> &str {
//         &self.id
//     }
// }

// impl Chunk for BmhdChunk {
//     fn get_id(&self) -> &str {
//         &self.id
//     }
// }

// // ParentChunk
// pub trait ParentChunk {
//     fn get_children(&mut self) -> &mut Vec<Box<dyn Chunk>>;
// }

// impl ParentChunk for FormIlbmChunk {
//     fn get_children(&mut self) -> &mut Vec<Box<dyn Chunk>> {
//         &mut self.children
//     }
// }

// // IlbmChild trait
// pub trait IlbmChild {
//     fn attach(self, ilbm_chunk: &mut FormIlbmChunk);
// }

// impl IlbmChild for BmhdChunk {
//     fn attach(self, ilbm_chunk: &mut FormIlbmChunk) {
//         ilbm_chunk.bmhd = Some(self);
//     }
// }

pub fn read_iff_file(file_path: path::PathBuf) -> Result<Box<FormIlbmChunk>, ErrorKind> {
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

    let root_chunk = parse_iff_buffer(&buffer)?;
    // let root_chunks = match root_chunks {
    //     Ok(chunks) => chunks
    // }
    // if root_chunks.len() == 0 {
    //     return Err(ErrorKind::NoChunksFound);
    // }
    // if root_chunks.len() > 1 {
    //     return Err(ErrorKind::MultipleRootChunksFound);
    // }

    // let root_chunk = root_chunks.remove(0);
    // let arne = match root_chunk {
    //     None => return Err(ErrorKind::NoChunksFound),
    //     Some(x) => x,
    // };

    Ok(root_chunk)
}

pub fn parse_iff_buffer(buffer: &Vec<u8>) -> Result<Box<FormIlbmChunk>, ErrorKind> {
    if buffer.len() < 12 {
        return Err(ErrorKind::FileTooShort);
    }

    let iff_chunk = parse_form_ilbm_buffer(buffer)?;

    Ok(iff_chunk)
}

fn parse_form_ilbm_buffer(buffer: &[u8]) -> Result<Box<FormIlbmChunk>, ErrorKind> {
    // println!("parse_chunk_buffer len: {}", buffer.len());

    // let mut iff_chunks: Vec<Box<dyn Chunk>> = Vec::new();

    let mut raw_chunk_array = raw_chunk_array::RawChunkArray::from(buffer);
    let raw_root_chunk = match raw_chunk_array.next() {
        Some(chunk) => chunk,
        None => return Err(ErrorKind::NoChunksFound),
    };

    match raw_chunk_array.next() {
        Some(_) => return Err(ErrorKind::MultipleRootChunksFound),
        None => (),
    };

    let mut iff_form_chunk: FormIlbmChunk;

    // for raw_chunk in raw_chunk_array {
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
                    let mut form_raw_chunk_array =
                        raw_chunk_array::RawChunkArray::from(form_buffer);
                    for form_raw_chunk in form_raw_chunk_array {
                        //             let mut ilbm_children = parse_chunk_buffer(form_buffer)?;
                        //             for child in ilbm_children.iter() {
                        println!("tjoho {:?} {:?}", form_raw_chunk.id, form_raw_chunk.size);
                        //                 //     //     // let cccc = child.as_ref();

                        //                 //     //     // let d = c as IlbmChild;
                        //                 //     //     //     if child is BmhdChunk
                        //             }
                    }
                    //             iff_form_chunk.get_children().append(&mut ilbm_children);

                    //             iff_chunks.push(Box::new(iff_form_chunk));
                }
                "ANIM" => {
                    println!("ANIM");
                    return Err(ErrorKind::UnsupportedFormType);
                }
                _ => return Err(ErrorKind::UnknownFormType),
            };
        }
        _ => return Err(ErrorKind::UnknownChunk(raw_root_chunk.id.to_string())),
    }
    //         "ANNO" => {
    //             let chunk = UnknownChunk::new(chunk_id.to_string());
    //             iff_chunks.push(Box::new(chunk));
    //             // ilbm.Anno = Encoding.UTF8.GetString(chunk.Content, 0, (int)chunk.ContentLength);
    //             //
    //         }

    //         "BMHD" => {
    //             // let chunk = UnknownChunk::new(chunk_id.to_string());
    //             let chunk = get_bmhd_chunk(buffer, pos)?;
    //             println!("Bmhd: {:?}", chunk);
    //             iff_chunks.push(Box::new(chunk));
    //             // ilbm.Bmhd = new Bmhd(chunk);
    //             //
    //         }

    //         "CMAP" => {
    //             let chunk = UnknownChunk::new(chunk_id.to_string());
    //             iff_chunks.push(Box::new(chunk));
    //             // ilbm.Cmap = new Cmap(chunk, ilbm);
    //             //
    //         }
    //         //
    //         "CAMG" => {
    //             let chunk = UnknownChunk::new(chunk_id.to_string());
    //             iff_chunks.push(Box::new(chunk));
    //             // ilbm.Camg = new Camg(chunk);
    //             //
    //         }

    //         "BODY" => {
    //             let chunk = UnknownChunk::new(chunk_id.to_string());
    //             iff_chunks.push(Box::new(chunk));
    //             // ilbm.Body = new Body(chunk, ilbm);
    //             //
    //         }

    //         "ANHD" => {
    //             let chunk = UnknownChunk::new(chunk_id.to_string());
    //             iff_chunks.push(Box::new(chunk));
    //             // ilbm.Anhd = new Anhd(chunk);
    //             //
    //         }

    //         "DLTA" => {
    //             let chunk = UnknownChunk::new(chunk_id.to_string());
    //             iff_chunks.push(Box::new(chunk));
    //             //                 ilbm.Dlta = new Dlta(chunk, ilbm, iffFile);
    //             //
    //         }

    //         "DPPS" => {
    //             let chunk = UnknownChunk::new(chunk_id.to_string());
    //             iff_chunks.push(Box::new(chunk));
    //             // //todo: Handle DPPS
    //             // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
    //             //
    //         }
    //         "DRNG" => {
    //             let chunk = UnknownChunk::new(chunk_id.to_string());
    //             iff_chunks.push(Box::new(chunk));
    //         }
    //         //DPaint IV enhanced color cycle chunk (EA)
    //         // http://wiki.amigaos.net/wiki/ILBM_IFF_Interleaved_Bitmap
    //         // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
    //         "BRNG" => {
    //             let chunk = UnknownChunk::new(chunk_id.to_string());
    //             iff_chunks.push(Box::new(chunk));
    //             //unknown
    //             // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
    //         }

    //         "CRNG" => {
    //             let chunk = UnknownChunk::new(chunk_id.to_string());
    //             iff_chunks.push(Box::new(chunk));
    //             // color register range
    //             // http://wiki.amigaos.net/wiki/ILBM_IFF_Interleaved_Bitmap
    //             // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
    //         }

    //         "DPI " => {
    //             let chunk = UnknownChunk::new(chunk_id.to_string());
    //             iff_chunks.push(Box::new(chunk));
    //             // Dots per inch chunk
    //             // http://wiki.amigaos.net/wiki/ILBM_IFF_Interleaved_Bitmap
    //             // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
    //         }

    //         "GRAB" => {
    //             let chunk = UnknownChunk::new(chunk_id.to_string());
    //             iff_chunks.push(Box::new(chunk));
    //             // locates a “handle” or “hotspot”
    //             // http://wiki.amigaos.net/wiki/ILBM_IFF_Interleaved_Bitmap
    //             // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
    //         }

    //         "DPXT" => {
    //             let chunk = UnknownChunk::new(chunk_id.to_string());
    //             iff_chunks.push(Box::new(chunk));
    //             // unknown
    //             // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
    //         }

    //         "TINY" => {
    //             let chunk = UnknownChunk::new(chunk_id.to_string());
    //             iff_chunks.push(Box::new(chunk));
    //             // Thumbnail
    //             // https://en.m.wikipedia.org/wiki/ILBM
    //             // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
    //         }

    //         _ => return Err(ErrorKind::UnknownChunk(chunk_id.to_string())),
    //     }

    // }

    Ok(Box::new(iff_form_chunk))
}

// fn get_bmhd_chunk(buffer: &[u8], pos: usize) -> Result<BmhdChunk, ErrorKind> {
//     let chunk = BmhdChunk {
//         id: String::from("BMHD"),
//         width: get_u16(buffer, pos + 8 + 0)?,
//         height: get_u16(buffer, pos + 8 + 2)?,
//         x: get_i16(buffer, pos + 8 + 4)?,
//         y: get_i16(buffer, pos + 8 + 6)?,
//         number_of_planes: get_u8(buffer, pos + 8 + 8)?,
//         masking: get_u8(buffer, pos + 8 + 9)?,
//         compression: 0,
//         transparent_color_number: 0,
//         x_aspect: 0,
//         y_aspect: 0,
//         page_width: 0,
//         page_height: 0,
//     };

//     // let chunk_size = get_u32(buffer, pos + 4)? as usize;
//     Ok(chunk)
//     // public Bmhd(IffChunk innerIlbmChunk)
//     // {
//     //     Width = ContentReader.ReadUShort(innerIlbmChunk.Content, 0);
//     //     Height = ContentReader.ReadUShort(innerIlbmChunk.Content, 2);
//     //     X = ContentReader.ReadShort(innerIlbmChunk.Content, 4);
//     //     Y = ContentReader.ReadShort(innerIlbmChunk.Content, 6);
//     //     NumberOfPlanes = ContentReader.ReadUByte(innerIlbmChunk.Content, 8);
//     //     Masking = ContentReader.ReadUByte(innerIlbmChunk.Content, 9);
//     //     Compression = ContentReader.ReadUByte(innerIlbmChunk.Content, 10);
//     //     // UBYTE pad1
//     //     TransparentColorNumber = ContentReader.ReadUShort(innerIlbmChunk.Content, 12);
//     //     XAspect = ContentReader.ReadUByte(innerIlbmChunk.Content, 14);
//     //     YAspect = ContentReader.ReadUByte(innerIlbmChunk.Content, 15);
//     //     PageWidth = ContentReader.ReadShort(innerIlbmChunk.Content, 16);
//     //     PageHeight = ContentReader.ReadShort(innerIlbmChunk.Content, 18);
//     // }
// }
