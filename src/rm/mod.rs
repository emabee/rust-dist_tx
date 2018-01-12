//! Interface of a resource manager, as required by a transaction manager.
mod flags;
mod resource_manager;
mod rm_error;

pub use self::resource_manager::ResourceManager;
pub use self::rm_error::{Kind, RmError, RmResult};
