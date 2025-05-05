pub mod disk_plugin;
pub mod saveload;

pub use disk_plugin::*;
pub use saveload::*;

/// Disk Space Minimizer. Returns [None] if there is nothing to save.
pub trait Slim where Self: Sized {
    fn slim(&self) -> Option<Self>;
}