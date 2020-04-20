use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Grid {
	pub texture_width: u32,
	pub texture_height: u32,
	pub columns: u32,
	pub rows: u32,
	pub cell_size: (u32, u32),
}

#[derive(Debug, Deserialize)]
pub enum AnimationId {
	BurnLow,
}

#[derive(Debug, Deserialize)]
pub struct AnimationFrames {
	pub input: Vec<f32>,
	pub output: Vec<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Animation {
	pub texture: String,
	pub sprites_grid: Grid,
	pub animation_set: Vec<(AnimationId, AnimationFrames)>,
}

#[derive(Debug, Deserialize)]
pub struct Animations {
	pub animations: Vec<Animation>,
}

impl Animations {
	pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
		log::debug!("Animations::load: path: {:?}", path);

		let f = File::open(&path)?;
		let ret: Self = from_reader(f)?;

		// TODO: Validate AnimationFrames.

		Ok(ret)
	}
}


pub struct Resources {
	pub static_animations: Animations,
}

impl Resources {
	pub fn load(resource_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
		let static_animations = Animations::load(&resource_path.join("static_animations.ron"))?;

		Ok(Self {
			static_animations,
		})
	}
}
