use crate::tm::XaError;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::convert::TryInto;
use std::io::{Read, Write};
use std::iter::repeat;

/// The ID of a distributed transaction, in analogy to the
/// [X/Open XA standard](http://pubs.opengroup.org/onlinepubs/009680699/toc.pdf).
///
#[derive(Clone)]
pub struct XaTransactionId {
    format_id: i32,
    global_tid: Vec<u8>,       // do it with u64
    branch_qualifier: Vec<u8>, // do it with u64
}

// Maximum size in bytes of `XaTransactionId::global_tid`
const MAX_BYTES_GLOBAL_TRANSACTION_ID: usize = 64;

// Maximum size in bytes of `XaTransactionId::branch_qualifier`
const MAX_BYTES_BRANCH_QUALIFIER: usize = 64;

impl XaTransactionId {
    /// Creates an instance of `XaTransactionId` from the three components
    /// `format_id`, `global_tid`, and `branch_qualifier`.
    ///
    /// Note that the lengths of the binary parameters must not exceed `64`.
    ///
    /// XA uses a signed int for the `format_id`, but recommends using only -1,
    /// 0, and positive values, where -1 is used to represent the NULL
    /// value.
    ///
    /// # Errors
    ///
    /// `XaError::Usage` if one of the parameters is ill-formed.
    pub fn try_new(
        format_id: i32,
        global_tid: Vec<u8>,
        branch_qualifier: Vec<u8>,
    ) -> Result<XaTransactionId, XaError> {
        if format_id < -1 {
            Err(XaError::Usage("Bad XA transaction id: invalid format-id"))
        } else if global_tid.len() > MAX_BYTES_GLOBAL_TRANSACTION_ID {
            Err(XaError::Usage("Invalid global ta id (too long)"))
        } else if branch_qualifier.len() > MAX_BYTES_BRANCH_QUALIFIER {
            Err(XaError::Usage("Invalid branch_qualifier (too long)"))
        } else {
            Ok(XaTransactionId {
                format_id,
                global_tid,
                branch_qualifier,
            })
        }
    }

    /// Creates an instance of `XaTransactionId` that represents NULL.
    #[must_use]
    pub fn null_ta() -> XaTransactionId {
        XaTransactionId {
            format_id: -1,
            global_tid: vec![],
            branch_qualifier: vec![],
        }
    }

    /// Returns the `format_id`.
    #[must_use]
    pub fn get_format_id(&self) -> i32 {
        self.format_id
    }

    /// Returns a reference to the global transaction id.
    #[must_use]
    pub fn get_global_tid(&self) -> &Vec<u8> {
        &self.global_tid
    }

    /// Returns a reference to the branch qualifier.
    #[must_use]
    pub fn get_branch_qualifier(&self) -> &Vec<u8> {
        &self.branch_qualifier
    }

    /// Provides a binary representation.
    /// If padding is true, and the combined length of the binary fields is
    /// below 128 bytes, the missing number of zero bytes are appended to
    /// make the byte pattern compatible with the XA structure in C.
    ///
    /// # Errors
    ///
    /// No errors should be possible. The message would panic if allocation fails.
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn as_bytes(&self, padding: bool) -> std::io::Result<Vec<u8>> {
        let mut result = Vec::<u8>::with_capacity(128);
        result.write_i32::<LittleEndian>(self.format_id)?;
        result.write_i32::<LittleEndian>(self.global_tid.len() as i32)?;
        result.write_i32::<LittleEndian>(self.branch_qualifier.len() as i32)?;
        result.write_all(&self.global_tid)?;
        result.write_all(&self.branch_qualifier)?;
        if padding {
            let missing = 128 - self.branch_qualifier.len() - self.global_tid.len();
            for _ in 0..missing {
                result.write_u8(0)?;
            }
        }
        Ok(result)
    }

    /// Reads a Vec of instances from a binary representation.
    /// If padding is true, and the combined length of the binary fields is
    /// below 128 bytes, the missing number of bytes are skipped to make
    /// the byte pattern compatible with the XA structure in C.
    ///
    /// # Errors
    ///
    /// `XaError::ReadXid`
    pub fn parse(bytes: &[u8], count: u64, padding: bool) -> Result<Vec<XaTransactionId>, XaError> {
        let mut rdr = std::io::Cursor::new(bytes);
        let mut result = Vec::<XaTransactionId>::new();

        for _ in 0..count {
            let format_id: i32 = rdr.read_i32::<LittleEndian>()?;
            let global_tid_len: usize = rdr
                .read_i32::<LittleEndian>()?
                .try_into()
                .map_err(|_| XaError::ReadXid("Negative global tid length".to_owned()))?;
            let branch_qualifier_len: usize = rdr
                .read_i32::<LittleEndian>()?
                .try_into()
                .map_err(|_| XaError::ReadXid("Negative branch qualifier length".to_owned()))?;

            let mut global_tid: Vec<u8> = repeat(0_u8).take(global_tid_len).collect();
            rdr.read_exact(&mut global_tid)?;

            let mut branch_qualifier: Vec<u8> = repeat(0_u8).take(branch_qualifier_len).collect();
            rdr.read_exact(&mut branch_qualifier)?;

            if padding {
                let missing = 128 - branch_qualifier_len - global_tid_len;
                for _ in 0..missing {
                    rdr.read_u8()?;
                }
            }

            result.push(XaTransactionId::try_new(
                format_id,
                global_tid,
                branch_qualifier,
            )?);
        }
        Ok(result)
    }
}

impl std::fmt::Debug for XaTransactionId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.format_id == -1 {
            write!(f, "XaTransactionId {{NULL}}")
        } else {
            write!(
                f,
                "XaTransactionId {{format_id: {}, global_tid: {:?}, branch_qualifier: {:?} }}",
                self.format_id, self.global_tid, self.branch_qualifier
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::XaTransactionId;
    use byteorder::{LittleEndian, WriteBytesExt};

    #[test]
    fn test_xa_transaction_id() {
        let xa_tid = new_xatid(255_u64, 255_u64, 255_u64);
        println!("xa:tid: {:?}", xa_tid);
    }

    fn new_xatid(global_tid: u64, transman_id: u64, resman_id: u64) -> XaTransactionId {
        let mut v_gt = Vec::<u8>::with_capacity(64);
        v_gt.write_u64::<LittleEndian>(global_tid).unwrap();
        let mut v_bq = Vec::<u8>::with_capacity(128);
        v_bq.write_u64::<LittleEndian>(transman_id).unwrap();
        v_bq.write_u64::<LittleEndian>(resman_id).unwrap();
        XaTransactionId::try_new(99, v_gt, v_bq).unwrap()
    }
}
