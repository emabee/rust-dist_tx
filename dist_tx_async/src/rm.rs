//! Interface of a resource manager, as required by a transaction manager.
//!
//! Implementors can choose between implementing `ResourceManager` directly, or implementing
//! `CResourceManager` (which is closer to the XA C API) and wrap it into `CRmWrapper` to get
//! an implementation of the more idiomatic `ResourceManager` trait.
mod c_resource_manager;
mod c_rm_wrapper;
mod resource_manager;

pub use self::{
    c_resource_manager::CResourceManager, c_rm_wrapper::CRmWrapper,
    resource_manager::ResourceManager,
};
