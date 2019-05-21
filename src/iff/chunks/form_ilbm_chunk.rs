use std::fmt;

use crate::common::buffer_reader;
use crate::error::ErrorKind;
use crate::iff::chunks::bmhd_chunk::BmhdChunk;
use crate::iff::chunks::body_chunk::BodyChunk;
use crate::iff::chunks::cmap_chunk::CmapChunk;
use crate::iff::chunks::unknown_chunk::UnknownChunk;
use crate::iff::raw::raw_chunk_array::RawChunkArray;

// FormIlbmChunk
// #[derive(Debug)]
pub struct FormIlbmChunk {
    pub id: String,
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

    pub fn parse_form_ilbm_buffer(buffer: &[u8]) -> Result<FormIlbmChunk, ErrorKind> {
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
                let _root_chunk = match form_type {
                    "ILBM" => {
                        println!("ILBM");
                        iff_form_chunk = FormIlbmChunk::new(raw_root_chunk.id.to_string());
                        let form_buffer = raw_root_chunk.get_slice_to_end(12);

                        let mut form_raw_chunk_array = RawChunkArray::from(form_buffer);

                        FormIlbmChunk::parse_ilbm_buffer(&mut form_raw_chunk_array, &mut iff_form_chunk)?;
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
            match chunk.id {
                "ANNO" => {
//                let _chunk = UnknownChunk::new(chunk.id.to_string());
//                let an = "jek";
                    let bytes = chunk.get_slice(8..8 + chunk.size);
                    let anno = buffer_reader::get_string(bytes)?;

                    println!("Anno: {}", anno);
                    // iff_chunks.push(Box::new(chunk));
                    // ilbm.Anno = Encoding.UTF8.GetString(chunk.Content, 0, (int)chunk.ContentLength);
                    //
                }

                "BMHD" => {
                    let chunk = BmhdChunk::get_bmhd_chunk(&chunk)?;
                    println!("Bmhd: {:?}", chunk);
                    ilbm.bmhd = Some(chunk);
                }

                "CMAP" => {
                    let chunk = CmapChunk::get_cmap_chunk(&chunk)?;
                    println!("Cmap: {:?}", chunk);
                    ilbm.cmap = Some(chunk);
                    //
                }
                //
                "CAMG" => {
                    let _chunk = UnknownChunk::new(chunk.id.to_string());
                    // iff_chunks.push(Box::new(chunk));
                    // ilbm.Camg = new Camg(chunk);
                    //
                }

                "BODY" => {
                    // let _chunk = UnknownChunk::new(chunk.id.to_string());
                    let chunk = BodyChunk::get_body_chunk(&chunk, &ilbm.bmhd)?;
                    // println!("Cmap: {:?}", chunk);
                    ilbm.body = Some(chunk);
                    //
                }

                "ANHD" => {
                    let _chunk = UnknownChunk::new(chunk.id.to_string());
                    // iff_chunks.push(Box::new(chunk));
                    // ilbm.Anhd = new Anhd(chunk);
                    //
                }

                "DLTA" => {
                    let _chunk = UnknownChunk::new(chunk.id.to_string());
                    // iff_chunks.push(Box::new(chunk));
                    //                 ilbm.Dlta = new Dlta(chunk, ilbm, iffFile);
                    //
                }

                "DPPS" => {
                    let _chunk = UnknownChunk::new(chunk.id.to_string());
                    // iff_chunks.push(Box::new(chunk));
                    // //todo: Handle DPPS
                    // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
                    //
                }
                "DRNG" => {
                    let _chunk = UnknownChunk::new(chunk.id.to_string());
                    // iff_chunks.push(Box::new(chunk));
                }
                //DPaint IV enhanced color cycle chunk (EA)
                // http://wiki.amigaos.net/wiki/ILBM_IFF_Interleaved_Bitmap
                // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
                "BRNG" => {
                    let _chunk = UnknownChunk::new(chunk.id.to_string());
                    // iff_chunks.push(Box::new(chunk));
                    //unknown
                    // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
                }

                "CRNG" => {
                    let _chunk = UnknownChunk::new(chunk.id.to_string());
                    // iff_chunks.push(Box::new(chunk));
                    // color register range
                    // http://wiki.amigaos.net/wiki/ILBM_IFF_Interleaved_Bitmap
                    // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
                }

                "DPI " => {
                    let _chunk = UnknownChunk::new(chunk.id.to_string());
                    // iff_chunks.push(Box::new(chunk));
                    // Dots per inch chunk
                    // http://wiki.amigaos.net/wiki/ILBM_IFF_Interleaved_Bitmap
                    // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
                }

                "GRAB" => {
                    let _chunk = UnknownChunk::new(chunk.id.to_string());
                    // iff_chunks.push(Box::new(chunk));
                    // locates a “handle” or “hotspot”
                    // http://wiki.amigaos.net/wiki/ILBM_IFF_Interleaved_Bitmap
                    // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
                }

                "DPXT" => {
                    let _chunk = UnknownChunk::new(chunk.id.to_string());
                    // iff_chunks.push(Box::new(chunk));
                    // unknown
                    // _logger.Information($"Unsupported ILBM inner chunk [{chunk.TypeId}]");
                }

                "TINY" => {
                    let _chunk = UnknownChunk::new(chunk.id.to_string());
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
}

impl fmt::Debug for FormIlbmChunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
