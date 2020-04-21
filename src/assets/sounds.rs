use ggez::Context;
use ggez::audio::SoundSource;
use ggez::audio::Source;
use crate::constants::*;


const FIREWOOD_COUNT: usize = 4;
const OWL_COUNT: usize = 4;
const GUITAR_COUNT: usize = 3;
const MAN1_COUNT: usize = 3;
const GIRL1_COUNT: usize = 1;
const GIRL2_COUNT: usize = 1;


pub struct Sounds {
	pub firewood: Vec<Source>,
	pub owl: Vec<Source>,
	pub guitar: Vec<Source>,
	pub man1: Vec<Source>,
	pub girl1: Vec<Source>,
	pub girl2: Vec<Source>,
}

impl Sounds {
	pub fn load(context: &mut Context) -> Result<Self, Box<dyn std::error::Error>> {
		let mut firewood = Vec::new();
		let mut owl = Vec::new();
		let mut guitar = Vec::new();
		let mut man1 = Vec::new();
		let mut girl1 = Vec::new();
		let mut girl2 = Vec::new();

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

		for index in 0..MAN1_COUNT {
			let mut sound = Source::new(context, &format!("/audio/man/{:02}.ogg", index))?;
			sound.set_volume(MAN_VOLUME);
			man1.push(sound);
		}

		for index in 0..GIRL1_COUNT {
			let mut sound = Source::new(context, &format!("/audio/girl1/{:02}.ogg", index))?;
			sound.set_volume(GIRL1_VOLUME);
			girl1.push(sound);
		}

		for index in 0..GIRL2_COUNT {
			let mut sound = Source::new(context, &format!("/audio/girl2/{:02}.ogg", index))?;
			sound.set_volume(GIRL2_VOLUME);
			girl2.push(sound);
		}

		Ok(Self {
			firewood,
			owl,
			guitar,
			man1,
			girl1,
			girl2,
		})
	}
}

pub fn play_random(sounds: &mut Vec<Source>) -> ggez::GameResult {
	use rand::Rng;

	let mut rng = rand::thread_rng();
	let index = rng.gen_range(0, sounds.len());

	sounds.get_mut(index).unwrap().play_detached()
}
