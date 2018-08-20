extern crate failure;
extern crate ui;

use failure::Error;
use ui::prelude::*;

fn main() -> Result<(), Error> {
    init()?;
    let mut window = Window::new()?;
    window.set_title("Hello");
    window.show();
    ui::prelude::main()
}
