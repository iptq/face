use backend_gtk::gtk;
use backend_gtk::gtk::{WidgetExt, GtkWindowExt};
use failure::Error;

use window::WindowExt;

pub struct GtkWindow {
    inner: gtk::Window,
}

impl WindowExt for GtkWindow {
    fn new() -> Result<Self, Error> {
        let inner = gtk::Window::new(gtk::WindowType::Toplevel);
        Ok(GtkWindow { inner })
    }

    fn show(&mut self) {
        self.inner.show_all();
    }

    fn set_title(&mut self, title: &str) {
    	self.inner.set_title(title);
    }
}
