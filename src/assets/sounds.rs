use ggez::Context;
use ggez::audio::SoundSource;
use ggez::audio::Source;
use crate::constants::{FIREWOOD_THROW_VOLUME, OWL_VOLUME, GUITAR_VOLUME};


const FIREWOOD_COUNT: usize = 4;
const OWL_COUNT: usize = 4;
const GUITAR_COUNT: usize = 3;


pub struct Sounds {
	pub firewood: Vec<Source>,
	pub owl: Vec<Source>,
	pub guitar: Vec<Source>,
}

impl Sounds {
	pub fn load(context: &mut ggez::Context) -> Result<Self, Box<dyn std::error::Error>> {
		let mut firewood = Vec::new();
		let mut owl = Vec::new();
		let mut guitar = Vec::new();

		for index in 0..FIREWOOD_COUNT {
			let mut sound = Source::new(context, &format!("/audio/firewood/{:02}.ogg", index))?;
			sound.set_volume(FIREWOOD_THROW_VOLUME);
			firewood.push(sound);
		}

		for index in 0..OWL_COUNT {
			let mut sound = Source::new(context, &format!("/audio/owl/{:02}.ogg", index))?;
			sound.set_volume(OWL_VOLUME);
			owl.push(sound);
		}

		for index in 0..GUITAR_COUNT {
			let mut sound = Source::new(context, &format!("/audio/guitar/{:02}.ogg", index))?;
			sound.set_volume(GUITAR_VOLUME);
			guitar.push(sound);
		}

		Ok(Self {
			firewood,
			owl,
			guitar,
		})
	}
}

pub fn play_random(sounds: &mut Vec<Source>) -> ggez::GameResult {
	use rand::Rng;

	let mut rng = rand::thread_rng();
	let index = rng.gen_range(0, sounds.len());

	sounds.get_mut(index).unwrap().play_detached()
}
