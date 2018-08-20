extern crate quick_xml;

use failure::Error;

pub mod prelude {
	pub use super::Component;
}

pub struct Component {
}

impl Component {
	pub fn from_str(contents: &str) -> Result<Self, Error> {
		use self::quick_xml::{Reader, events::Event};

		let mut reader = Reader::from_str(contents);
		reader.trim_text(true);

		let mut buf = Vec::new();
		loop {
			match reader.read_event(&mut buf) {
				Ok(Event::Start(ref e)) => {
					match e.name() {
						b"template" => {

						}
						_ => (),
					}
				}
				Ok(Event::Eof) => break,
				_ => (),
			}
		}
		buf.clear();
		Ok(Component { })
	}
}
