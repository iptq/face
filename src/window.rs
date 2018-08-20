use failure::Error;

/// An abstract Window. This trait is implemented by backend-specific window implementations.
pub trait WindowExt: Sized {
    /// Creates a new instance of Window.
    fn new() -> Result<Self, Error>;

    /// Shows the window.
    fn show(&mut self);

    /// Change the title of the window.
    fn set_title(&mut self, &str);
}
