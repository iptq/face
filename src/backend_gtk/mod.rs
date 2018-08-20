pub extern crate gtk;

mod window;

use failure::{err_msg, Error};

pub fn init() -> Result<(), Error> {
    gtk::init().map_err(|_| err_msg("Failed to initialize GTK."))
}

pub fn main() -> Result<(), Error> {
	gtk::main();
	Ok(())
}

pub mod prelude {
    pub use super::{init, main};
    pub use super::window::GtkWindow as Window;
}
