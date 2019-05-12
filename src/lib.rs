// mod model;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::mem;
use std::path;
use std::str;

#[derive(Debug)]
pub enum ErrorKind {
    IoError(io::Error),
    FileTooShort,
    UnknownChunk(String),
    ZeroSizeChunk,
    UnsupportedFormType,
    UnknownFormType,
    NoChunksFound,
    MultipleRootChunksFound,
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
    pub children: Vec<Box<dyn Chunk>>,
    pub bmhd: Option<BmhdChunk>,
}

impl FormIlbmChunk {
    pub fn new(id: String) -> FormIlbmChunk {
        FormIlbmChunk {
            children: Vec::new(),
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

// Chunk trait
pub trait Chunk {
    fn get_id(&self) -> &str;
}

impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_id())
    }
}

impl Chunk for FormIlbmChunk {
    fn get_id(&self) -> &str {
        &self.id
    }
}

impl Chunk for UnknownChunk {
    fn get_id(&self) -> &str {
        &self.id
    }
}

impl Chunk for BmhdChunk {
    fn get_id(&self) -> &str {
        &self.id
    }
}

// ParentChunk
pub trait ParentChunk {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Chunk>>;
}

impl ParentChunk for FormIlbmChunk {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Chunk>> {
        &mut self.children
    }
}

// IlbmChild trait
pub trait IlbmChild {
    fn attach(self, ilbm_chunk: &mut FormIlbmChunk);
}

impl IlbmChild for BmhdChunk {
    fn attach(self, ilbm_chunk: &mut FormIlbmChunk) {
        ilbm_chunk.bmhd = Some(self);
    }
}

pub fn read_iff_file(file_path: path::PathBuf) -> Result<Box<dyn Chunk>, ErrorKind> {
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

    let mut root_chunks = parse_iff_buffer(&buffer)?;
    // let root_chunks = match root_chunks {
    //     Ok(chunks) => chunks
    // }
    if root_chunks.len() == 0 {
        return Err(ErrorKind::NoChunksFound);
    }
    if root_chunks.len() > 1 {
        return Err(ErrorKind::MultipleRootChunksFound);
    }

    let root_chunk = root_chunks.remove(0);
    // let arne = match root_chunk {
    //     None => return Err(ErrorKind::NoChunksFound),
    //     Some(x) => x,
    // };

    Ok(root_chunk)
}

pub fn parse_iff_buffer(buffer: &Vec<u8>) -> Result<Vec<Box<dyn Chunk>>, ErrorKind> {
    if buffer.len() < 12 {
        return Err(ErrorKind::FileTooShort);
    }

    let iff_chunks = parse_chunk_buffer(buffer)?;

    Ok(iff_chunks)
}

fn parse_chunk_buffer(buffer: &[u8]) -> Result<Vec<Box<dyn Chunk>>, ErrorKind> {
    println!("parse_chunk_buffer len: {}", buffer.len());

    let mut pos = 0usize;
    let mut iff_chunks: Vec<Box<dyn Chunk>>;

    iff_chunks = Vec::new();

    while pos < buffer.len() {
        let chunk_id = get_chunk_id(buffer, pos + 0)?;
        let chunk_size = get_u32(buffer, pos + 4)? as usize;

        if chunk_size == 0 {
            return Err(ErrorKind::ZeroSizeChunk);
        }

        match chunk_id {
            "FORM" => {
                let form_type = get_chunk_id(buffer, pos + 8)?;
                println!("FORM type {}", form_type);
                match form_type {
                    "ILBM" => println!("ILBM"),
                    "ANIM" => {
                        println!("ANIM");
                        return Err(ErrorKind::UnsupportedFormType);
                    }
                    _ => return Err(ErrorKind::UnknownFormType),
                }

                let mut iff_form_chunk = FormIlbmChunk::new(chunk_id.to_string());
                let form_buffer = &buffer[pos + 12..pos + 12 + chunk_size - 4];

                let mut ilbm_children = parse_chunk_buffer(form_buffer)?;
                for child in ilbm_children.iter() {
                    println!("tjoho {:?}", child);
                    //     //     // let cccc = child.as_ref();

                    //     //     // let d = c as IlbmChild;
                    //     //     //     if child is BmhdChunk
                }
                iff_form_chunk.get_children().append(&mut ilbm_children);

                iff_chunks.push(Box::new(iff_form_chunk));
            }

            "ANNO" => {
                let chunk = UnknownChunk::new(chunk_id.to_string());
                iff_chunks.push(Box::new(chunk));
                // ilbm.Anno = Encoding.UTF8.GetString(chunk.Content, 0, (int)chunk.ContentLength);
                //
            }

            "BMHD" => {
                // let chunk = UnknownChunk::new(chunk_id.to_string());
                let chunk = get_bmhd_chunk(buffer, pos)?;
                println!("Bmhd: {:?}", chunk);
                iff_chunks.push(Box::new(chunk));
                // ilbm.Bmhd = new Bmhd(chunk);
                //
            }

            "CMAP" => {
                let chunk = UnknownChunk::new(chunk_id.to_string());
                iff_chunks.push(Box::new(chunk));
                // ilbm.Cmap = new Cmap(chunk, ilbm);
                //
            }
            //
            "CAMG" => {
                let chunk = UnknownChunk::new(chunk_id.to_string());
                iff_chunks.push(Box::new(chunk));
                // ilbm.Camg = new Camg(chunk);
                //
            }

            "BODY" => {
                let chunk = UnknownChunk::new(chunk_id.to_string());
                iff_chunks.push(Box::new(chunk));
                // ilbm.Body = new Body(chunk, ilbm);
                //
            }

            "ANHD" => {
                let chunk = UnknownChunk::new(chunk_id.to_string());
                iff_chunks.push(Box::new(chunk));
                // ilbm.Anhd = new Anhd(chunk);
                //
            }

            "DLTA" => {
                let chunk = UnknownChunk::new(chunk_id.to_string());
                iff_chunks.push(Box::new(chunk));
                //                 ilbm.Dlta = new Dlta(chunk, ilbm, iffFile);
                //
            }

            "DPPS" => {
                let chunk = UnknownChunk::new(chunk_id.to_string());
                iff_chunks.push(Box::new(chunk));
                // //todo: Handle DPPS
                // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
                //
            }
            "DRNG" => {
                let chunk = UnknownChunk::new(chunk_id.to_string());
                iff_chunks.push(Box::new(chunk));
            }
            //DPaint IV enhanced color cycle chunk (EA)
            // http://wiki.amigaos.net/wiki/ILBM_IFF_Interleaved_Bitmap
            // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
            "BRNG" => {
                let chunk = UnknownChunk::new(chunk_id.to_string());
                iff_chunks.push(Box::new(chunk));
                //unknown
                // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
            }

            "CRNG" => {
                let chunk = UnknownChunk::new(chunk_id.to_string());
                iff_chunks.push(Box::new(chunk));
                // color register range
                // http://wiki.amigaos.net/wiki/ILBM_IFF_Interleaved_Bitmap
                // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
            }

            "DPI " => {
                let chunk = UnknownChunk::new(chunk_id.to_string());
                iff_chunks.push(Box::new(chunk));
                // Dots per inch chunk
                // http://wiki.amigaos.net/wiki/ILBM_IFF_Interleaved_Bitmap
                // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
            }

            "GRAB" => {
                let chunk = UnknownChunk::new(chunk_id.to_string());
                iff_chunks.push(Box::new(chunk));
                // locates a “handle” or “hotspot”
                // http://wiki.amigaos.net/wiki/ILBM_IFF_Interleaved_Bitmap
                // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
            }

            "DPXT" => {
                let chunk = UnknownChunk::new(chunk_id.to_string());
                iff_chunks.push(Box::new(chunk));
                // unknown
                // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
            }

            "TINY" => {
                let chunk = UnknownChunk::new(chunk_id.to_string());
                iff_chunks.push(Box::new(chunk));
                // Thumbnail
                // https://en.m.wikipedia.org/wiki/ILBM
                // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
            }

            _ => return Err(ErrorKind::UnknownChunk(chunk_id.to_string())),
        }

        pos += 8;
        pos += (chunk_size + 1) & 0xfffffffffffffffe;

        // if pos < buffer.len() {
        //     return Err(ErrorKind::FoundMultipleChunksInBuffer);
    }

    Ok(iff_chunks)
}

fn get_bmhd_chunk(buffer: &[u8], pos: usize) -> Result<BmhdChunk, ErrorKind> {
    let chunk = BmhdChunk {
        id: String::from("BMHD"),
        width: get_u16(buffer, pos + 8 + 0)?,
        height: get_u16(buffer, pos + 8 + 2)?,
        x: get_i16(buffer, pos + 8 + 4)?,
        y: get_i16(buffer, pos + 8 + 6)?,
        number_of_planes: get_u8(buffer, pos + 8 + 8)?,
        masking: get_u8(buffer, pos + 8 + 9)?,
        compression: 0,
        transparent_color_number: 0,
        x_aspect: 0,
        y_aspect: 0,
        page_width: 0,
        page_height: 0,
    };

    // let chunk_size = get_u32(buffer, pos + 4)? as usize;
    Ok(chunk)
    // public Bmhd(IffChunk innerIlbmChunk)
    // {
    //     Width = ContentReader.ReadUShort(innerIlbmChunk.Content, 0);
    //     Height = ContentReader.ReadUShort(innerIlbmChunk.Content, 2);
    //     X = ContentReader.ReadShort(innerIlbmChunk.Content, 4);
    //     Y = ContentReader.ReadShort(innerIlbmChunk.Content, 6);
    //     NumberOfPlanes = ContentReader.ReadUByte(innerIlbmChunk.Content, 8);
    //     Masking = ContentReader.ReadUByte(innerIlbmChunk.Content, 9);
    //     Compression = ContentReader.ReadUByte(innerIlbmChunk.Content, 10);
    //     // UBYTE pad1
    //     TransparentColorNumber = ContentReader.ReadUShort(innerIlbmChunk.Content, 12);
    //     XAspect = ContentReader.ReadUByte(innerIlbmChunk.Content, 14);
    //     YAspect = ContentReader.ReadUByte(innerIlbmChunk.Content, 15);
    //     PageWidth = ContentReader.ReadShort(innerIlbmChunk.Content, 16);
    //     PageHeight = ContentReader.ReadShort(innerIlbmChunk.Content, 18);
    // }
}

fn get_chunk_id(buffer: &[u8], pos: usize) -> Result<&str, ErrorKind> {
    let chunk_id = str::from_utf8(&buffer[pos..pos + 4]);
    let chunk_id2 = match chunk_id {
        Ok(x) => x,
        Err(err) => {
            let err_msg = fmt::format(format_args!(
                "{}: [{:X}] [{:X}] [{:X}] [{:X}]",
                err,
                buffer[pos + 0],
                buffer[pos + 1],
                buffer[pos + 2],
                buffer[pos + 3],
            ));
            return Err(ErrorKind::UnknownChunk(err_msg));
        }
    };
    // println!("group_id {:?}", chunk_id);

    Ok(chunk_id2)
}

fn get_u32(buffer: &[u8], pos: usize) -> Result<u32, ErrorKind> {
    let slize = &buffer[pos..pos + 4];
    let mut array: [u8; 4] = [0; 4];
    array.copy_from_slice(slize);
    let value = unsafe { mem::transmute::<[u8; 4], u32>(array).to_be() };
    let value = value as u32;

    Ok(value)
}

fn get_u16(buffer: &[u8], pos: usize) -> Result<u16, ErrorKind> {
    let slize = &buffer[pos..pos + 2];
    let mut array: [u8; 2] = [0; 2];
    array.copy_from_slice(slize);
    let value = unsafe { mem::transmute::<[u8; 2], u16>(array).to_be() };
    let value = value as u16;

    Ok(value)
}

fn get_i16(buffer: &[u8], pos: usize) -> Result<i16, ErrorKind> {
    let slize = &buffer[pos..pos + 2];
    let mut array: [u8; 2] = [0; 2];
    array.copy_from_slice(slize);
    let value = unsafe { mem::transmute::<[u8; 2], i16>(array).to_be() };
    let value = value as i16;

    Ok(value)
}

fn get_u8(buffer: &[u8], pos: usize) -> Result<u8, ErrorKind> {
    // let slize = &buffer[pos..pos + 2];
    // let mut array: [u8; 2] = [0; 2];
    // array.copy_from_slice(slize);
    // let value = unsafe { mem::transmute::<[u8; 2], get_u8>(array).to_be() };
    // let value = value as get_u8;

    let value = buffer[pos];
    Ok(value)
}
