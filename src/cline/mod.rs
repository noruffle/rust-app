mod command;
mod fn_comm;
mod impl_comm;
mod struct_comm;

pub use struct_comm::Config;
pub use impl_comm::{run, search};
pub use fn_comm::smth;