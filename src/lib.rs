// mod model;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub enum ErrorKind {
    IoError(io::Error),
    FileTooShort,
    UnknownChunk(String),
    ZeroSizeChunk,
    FoundMultipleChunksInBuffer,
    UnsupportedFormType,
    UnknownFormType,
}

pub trait Chunk {
    fn get_id(&self) -> &str;
}

pub trait ParentChunk {
    fn get_children(&mut self) -> &mut Vec<Box<Chunk>>;
}

// #[derive(Debug)]
pub struct FormIlbmChunk {
    pub id: String,
    children: Vec<Box<Chunk>>,
}

pub struct UnknownChunk {
    pub id: String,
}

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

impl FormIlbmChunk {
    pub fn new(id: String) -> FormIlbmChunk {
        FormIlbmChunk {
            children: Vec::new(),
            id: id,
        }
    }
}

impl UnknownChunk {
    pub fn new(id: String) -> UnknownChunk {
        UnknownChunk { id: id }
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

impl ParentChunk for FormIlbmChunk {
    fn get_children(&mut self) -> &mut Vec<Box<Chunk>> {
        &mut self.children
    }
}

impl std::fmt::Debug for FormIlbmChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub fn read_iff_file(file_path: std::path::PathBuf) -> Result<Vec<Box<Chunk>>, ErrorKind> {
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

    let ending = parse_iff_buffer(&buffer);
    // println!("ending: {:?}", ending);
    ending
}

pub fn parse_iff_buffer(buffer: &Vec<u8>) -> Result<Vec<Box<Chunk>>, ErrorKind> {
    if buffer.len() < 12 {
        return Err(ErrorKind::FileTooShort);
    }

    let iff_chunks = parse_chunk_buffer(buffer)?;

    Ok(iff_chunks)
}

fn parse_chunk_buffer(buffer: &[u8]) -> Result<Vec<Box<Chunk>>, ErrorKind> {
    println!("parse_chunk_buffer len: {}", buffer.len());

    let mut pos = 0usize;
    let mut iff_chunks: Vec<Box<Chunk>>;

    iff_chunks = Vec::new();

    while pos < buffer.len() {
        let chunk_id = get_chunk_id(buffer, pos + 0)?;
        let chunk_size = get_chunk_size(buffer, pos + 4)?;

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

                // iff_form_chunk.id = String::from("knut");
                // iff_file.width = 320;

                let form_buffer = &buffer[pos + 12..pos + 12 + chunk_size - 4];
                println!("form_buffer len: {}", form_buffer.len());
                // let form_pos = 0usize;
                // // while form_pos < buffer.len() {
                // let form_chunk_id = get_chunk_id(form_buffer, form_pos + 0)?;
                // let form_chunk_size = get_chunk_size(form_buffer, form_pos + 4)?;
                // println!("form_chunk_id: {}", form_chunk_id);
                // println!("form_chunk_size: {}", form_chunk_size);

                let mut ilbm_children = parse_chunk_buffer(form_buffer)?;
                iff_form_chunk.get_children().append(&mut ilbm_children);
                // iff_chunks.
                // form_pos += 8;
                // form_pos += chunk_size;
                // }
                // parse_iff_chunk()
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
                let chunk = get_bmhd_chunk()?;
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

fn get_bmhd_chunk() -> Result<BmhdChunk, ErrorKind> {
    let chunk = BmhdChunk {
        width: 0,
        height: 0,
        x: 0,
        y: 0,
        number_of_planes: 0,
        masking: 0,
        compression: 0,
        transparent_color_number: 0,
        x_aspect: 0,
        y_aspect: 0,
        page_width: 0,
        page_height: 0,
    };

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
    let chunk_id = std::str::from_utf8(&buffer[pos..pos + 4]);
    let chunk_id2 = match chunk_id {
        Ok(x) => x,
        Err(err) => {
            let err_msg = std::fmt::format(format_args!(
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
    println!("group_id {:?}", chunk_id);

    Ok(chunk_id2)
}

fn get_chunk_size(buffer: &[u8], pos: usize) -> Result<usize, ErrorKind> {
    let chunk_size_slize = &buffer[pos..pos + 4];
    let mut chunk_size_array: [u8; 4] = [0; 4];
    chunk_size_array.copy_from_slice(chunk_size_slize);
    let chunk_size = unsafe { std::mem::transmute::<[u8; 4], u32>(chunk_size_array).to_be() };
    let chunk_size = chunk_size as usize;
    println!("chunk_size {:?}", chunk_size);

    Ok(chunk_size)
}
