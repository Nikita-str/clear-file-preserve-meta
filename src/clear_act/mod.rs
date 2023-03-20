
type ResultIO = std::io::Result<()>;

mod traits;
pub use traits::{ClearFile, ClearDir};

// [+] impls
mod const_change_cont;
pub use const_change_cont::{ConstChangeContF, ConstChangeContD};
// [-] impls

