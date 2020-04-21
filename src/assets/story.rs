use ggez::audio::{Source, SoundSource};
use ggez::Context;
use ggez::graphics::Image;
use crate::constants::*;


const FRAGMENTS_NUMBER: u32 = 42;


pub struct StoryFragments {
	pub fragments: Vec<(Image, Source)>,
}

impl StoryFragments {
	pub fn load(context: &mut Context) -> Result<Self, Box<dyn std::error::Error>> {
		let mut fragments = Vec::new();

		for index in 0..FRAGMENTS_NUMBER {
			let image = Image::new(context, &format!("/story/text/text_{:04}.png", index))?;
			let mut  source = Source::new(context, &format!("/story/audio/audio_{:04}.ogg", index))?;
			source.set_volume(STORY_VOLUME);

			fragments.push((image, source));
		}

		Ok(Self {
			fragments,
		})
	}
}
