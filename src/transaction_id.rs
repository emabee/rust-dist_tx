use tm::{TmError, TmResult};
use byteorder::{LittleEndian, WriteBytesExt};
use std::fmt;
use std::io::{self, Write};
use std::i32;

/// The ID of a distributed transaction, analogous to the
/// [X/Open XA standard](http://pubs.opengroup.org/onlinepubs/009680699/toc.pdf).
#[derive(Clone)]
pub struct TransactionId(OTid);

#[derive(Clone, Debug)]
enum OTid {
    None,
    Some(Tid),
}
#[derive(Clone, Debug)]
struct Tid {
    format_id: u32,
    global_ta: Vec<u8>,
    branch_qualifier: Vec<u8>,
}

/// Maximum allowed value for `format_id`.
pub const MAX_FORMAT_ID: u32 = i32::MAX as u32;

/// maximum size in bytes of `TransactionId::global_ta`
pub const MAX_GTRID_SIZE: usize = 64;

/// maximum size in bytes of `TransactionId::branch_qualifier`
pub const MAX_BQUAL_SIZE: usize = 64;

impl TransactionId {
    /// Creates an instance of `TransactionId` from the three components format_id, global_ta, and
    /// branch_qualifier.
    ///
    /// Note that the lengths of the binary parameters must not exceed 64.
    pub fn new(
        format_id: u32,
        global_ta: Vec<u8>,
        branch_qualifier: Vec<u8>,
    ) -> TmResult<TransactionId> {
        if format_id > MAX_FORMAT_ID {
            Err(TmError::Usage("Bad XA transaction id: invalid format-id"))
        } else if global_ta.len() > MAX_GTRID_SIZE {
            Err(TmError::Usage(
                "Bad XA transaction id: invalid global ta id (too long)",
            ))
        } else if branch_qualifier.len() > MAX_BQUAL_SIZE {
            Err(TmError::Usage(
                "Bad XA transaction id: invalid branch_qualifier (too long)",
            ))
        } else {
            Ok(TransactionId(OTid::Some(Tid {
                format_id: format_id as u32,
                global_ta: global_ta,
                branch_qualifier: branch_qualifier,
            })))
        }
    }

    /// Creates an instance of `TransactionId` that represents NULL.
    pub fn null() -> TransactionId {
        TransactionId(OTid::None)
    }


    /// Returns the format-id.
    pub fn get_format_id(&self) -> i32 {
        match self.0 {
            OTid::None => -1,
            OTid::Some(ref tid) => tid.format_id as i32,
        }
    }

    /// Returns a clone of the global transaction id, or None (for a NULL-instance).
    pub fn get_global_ta(&self) -> Option<Vec<u8>> {
        match self.0 {
            OTid::None => None,
            OTid::Some(ref tid) => Some(tid.global_ta.clone()),
        }
    }

    /// Returns a clone of the branch qualifier, or None (for a NULL-instance).
    pub fn get_branch_qualifier(&self) -> Option<Vec<u8>> {
        match self.0 {
            OTid::None => None,
            OTid::Some(ref tid) => Some(tid.branch_qualifier.clone()),
        }
    }

    /// Provides a binary representation.
    /// If padding is true, and the combined length of the binary fields is below 128 bytes,
    /// the missing number of zero bytes are appended to make the byte pattern compatible with
    /// the XA structure in C.
    pub fn as_bytes(&self, padding: bool) -> io::Result<Vec<u8>> {
        let mut result = Vec::<u8>::new();
        match self.0 {
            OTid::None => {
                result.write_i32::<LittleEndian>(-1)?;
                result.write_i32::<LittleEndian>(0)?;
                result.write_i32::<LittleEndian>(0)?;
                if padding {
                    for _ in 0..128 {
                        result.write_u8(0)?;
                    }
                }
            }
            OTid::Some(ref tid) => {
                result.write_i32::<LittleEndian>(tid.format_id as i32)?;
                result.write_i32::<LittleEndian>(tid.global_ta.len() as i32)?;
                result.write_i32::<LittleEndian>(tid.branch_qualifier.len() as i32)?;
                result.write_all(&tid.global_ta)?;
                result.write_all(&tid.branch_qualifier)?;
                if padding {
                    let missing = 128 - tid.branch_qualifier.len() - tid.global_ta.len();
                    for _ in 0..missing {
                        result.write_u8(0)?;
                    }
                }
            }
        }
        Ok(result)
    }
}

impl fmt::Debug for TransactionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            OTid::None => write!(f, "TransactionId {{NULL}}"),
            OTid::Some(ref tid) => write!(
                f,
                "TransactionId {{format_id: {}, global_ta: {:?}, branch_qualifier: {:?} }}",
                tid.format_id,
                tid.global_ta,
                tid.branch_qualifier
            ),
        }
    }
}
