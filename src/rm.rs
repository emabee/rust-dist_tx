//! Interface of a resource manager, as required by a transaction manager.
//!
//! Implementors can choose between implementing `ResourceManager` directly, or implementing
//! `CResourceManager` (which is closer to the XA C API) and wrap it into `CRmWrapper` to get
//! an implementation of the more idiomatic `ResourceManager` trait.
mod c_resource_manager;
mod c_rm_wrapper;
mod flags;
mod resource_manager;
mod rm_error;

pub use self::c_resource_manager::CResourceManager;
pub use self::c_rm_wrapper::CRmWrapper;
pub use self::flags::Flags;
pub use self::resource_manager::ResourceManager;
pub use self::rm_error::{ErrorCode, RmError, RmRc, RmResult};
