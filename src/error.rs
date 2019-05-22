//use std::io;
use std::str::Utf8Error;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorKind {
    //    IoError(Error),
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
    UnparseableString(Utf8Error),
    ChunkLengthMismatch,
    CouldNotDetermineOutputPath,
    CouldNotDetermineFileStem,

}

//impl std::cmp::PartialEq for ErrorKind {
//    fn eq(&self, other: &ErrorKind) -> bool {
//        match self {
////            ErrorKind::IoError(_s) => {
////                match other {
////                    ErrorKind::IoError(_o) => false, //TODO: Compare io errors
////                    _ => false,
////                }
////            }
//            ErrorKind::UnknownChunk(s) => match other {
//                ErrorKind::UnknownChunk(o) => s == o,
//                _ => false,
//            },
//            ErrorKind::FileTooShort => match other {
//                ErrorKind::FileTooShort => true,
//                _ => false,
//            },
//            ErrorKind::ChunkTooShort => match other {
//                ErrorKind::ChunkTooShort => true,
//                _ => false,
//            },
//            ErrorKind::ZeroSizeChunk => match other {
//                ErrorKind::ZeroSizeChunk => true,
//                _ => false,
//            },
//            ErrorKind::UnsupportedFormType => match other {
//                ErrorKind::UnsupportedFormType => true,
//                _ => false,
//            },
//            ErrorKind::UnknownFormType => match other {
//                ErrorKind::UnknownFormType => true,
//                _ => false,
//            },
//            ErrorKind::NoChunksFound => match other {
//                ErrorKind::NoChunksFound => true,
//                _ => false,
//            },
//            ErrorKind::MultipleRootChunksFound => match other {
//                ErrorKind::MultipleRootChunksFound => true,
//                _ => false,
//            },
//            ErrorKind::UnknownIlbmChunk(s) => match other {
//                ErrorKind::UnknownIlbmChunk(o) => s == o,
//                _ => false,
//            },
//            ErrorKind::InvalidChunkSize => match other {
//                ErrorKind::InvalidChunkSize => true,
//                _ => false,
//            },
//            ErrorKind::BmhdNotYetSet => match other {
//                ErrorKind::BmhdNotYetSet => true,
//                _ => false,
//            },
//            ErrorKind::IlbmNoOp => match other {
//                ErrorKind::IlbmNoOp => true,
//                _ => false,
//            },
//            ErrorKind::UnparseableString(_s) => {
//                match other {
//                    ErrorKind::UnparseableString(_o) => false, //TODO: Compare utf errors
//                    _ => false,
//                }
//            }
//            ErrorKind::ChunkLengthMismatch => match other {
//                ErrorKind::ChunkLengthMismatch => true,
//                _ => false,
//            },
//        }
//    }
//}
