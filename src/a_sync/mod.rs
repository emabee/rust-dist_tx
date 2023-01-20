//! An asynchronous rust language binding for
//! [XA Distributed Transactions](https://pubs.opengroup.org/onlinepubs/009680699/toc.pdf).
//!
//! ## Example
//!
//! Let's assume we have two (or more) different database connections,
//! most likely to different database installations,
//! and we want to use two-phase-commit to ensure that all our changes to these
//! connections are stored consistently (i.e. all or nothing).
//!
//! **Precondition**: both connections (i.e. both drivers) have to be prepared for
//! working with `async_dist_tx`, and both are not using automatic commit.
//!
//! ```rust,ignore
//!     let mut conn_a = ...;
//!     let mut conn_b = ...;
//! ```
//!
//! First instantiate a [`SimpleTransactionManager`](crate::a_sync::tm::SimpleTransactionManager):
//!
//! ```rust
//! use dist_tx::a_sync::tm::SimpleTransactionManager;
//! let mut tm = SimpleTransactionManager::new("XA Demo");
//! ```
//!
//! Then retrieve a [`ResourceManager`](crate::a_sync::rm::ResourceManager)
//! implementation from each connection,
//! and register it at the transaction manager.
//! Every registered resource manager is registered with a distinct ID.
//! In the example below we tell the transaction manager to cleanup eventually
//! existing open transaction left-overs from previous runs for the two ids.
//!
//! This makes already clear that the IDs (of type `u64`) should be chosen in a way
//! that minimizes undesired collision probabilities, and maximizes intended "collisions".
//!
//! ```rust,ignore
//!     tm.register(conn_a.get_resource_manager(), id_1, true).await?;
//!     tm.register(conn_b.get_resource_manager(), id_2, true).await?;
//! ```
//!
//! Now we start a distributed transaction
//!
//! ```rust,ignore
//!     tm.start_transaction().await?;
//! ```
//!
//! and then we're ready
//! to do all required updates via the two database connections:
//!
//! ```rust,ignore
//!     //...
//!     conn_a.dml(&insert_stmt(1, "foo")).await?;
//!     conn_b.dml(&insert_stmt(2, "bar")).await?;
//!     //...
//! ```
//!
//! At this point nothing is committed yet, which we could verify with additional connections
//! to the two databases that are not registered to `tm`.
//!
//! Finally, when all updates were done successfully, we commit the transaction:
//!
//! ```rust,ignore
//!     tm.commit_transaction().await?;
//! ```
//!
//! Now all updates are committed and visible, which we could again verify with
//! additional connections.
//!
//!
//! ## Implementation
//!
//! Database drivers etc, who want to enable their users to take part in distributed
//! transactions that are managed via `dist_tx`, can either implement
//! [`ResourceManager`](crate::a_sync::rm::ResourceManager), which is a more
//! rust-idiomatic interpretation of the XA resource manager,
//! or they implement [`CResourceManager`](crate::a_sync::rm::CResourceManager),
//! which is more C-ish, and wrap it into a [`CRmWrapper`](crate::a_sync::rm::CRmWrapper)
//! (which implements [`ResourceManager`](crate::a_sync::rm::ResourceManager)).
//!
//! This module and its submodules use [`async-trait`](https://crates.io/crates/async-trait)
//! to define and implement traits with asynchronous methods.

pub mod rm;
pub mod tm;
