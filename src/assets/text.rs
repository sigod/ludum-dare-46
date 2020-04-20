use ggez::graphics::Image;
use ggez::Context;


const FRAGMENTS_NUMBER: u32 = 42;


pub struct TextFragments {
	fragments: Vec<Image>,
}

impl TextFragments {
	pub fn load(context: &mut Context) -> Result<Self, Box<dyn std::error::Error>> {
		let mut fragments = Vec::new();

		for index in 0..FRAGMENTS_NUMBER {
			let image = Image::new(context, &format!("/text/text_{:04}.png", index))?;
			fragments.push(image);
		}

		Ok(Self {
			fragments,
		})
	}
}
