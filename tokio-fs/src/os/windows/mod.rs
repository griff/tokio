//! Windows-specific extensions for the primitives in the `tokio_fs` module.

mod symlink_dir;

pub use symlink_dir::{symlink_dir, SymlinkDirFuture};
