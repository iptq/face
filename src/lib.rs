extern crate failure;

#[cfg(target_os = "linux")]
mod backend_gtk;

#[cfg(feature = "sfc")]
mod sfc;

mod window;

pub mod prelude {
    pub use window::WindowExt;

    #[cfg(target_os = "linux")]
    pub use backend_gtk::prelude::*;

	#[cfg(feature = "sfc")]
	pub use sfc::prelude::*;
}
