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
pub struct AnimationDto {
	pub texture: String,
	pub sprites_grid: Grid,
	pub animation_set: Vec<(AnimationId, AnimationFrames)>,
}

pub struct Animation {
	pub texture: ggez::graphics::Image,
	pub sprites_grid: Grid,
	pub animation_set: Vec<(AnimationId, AnimationFrames)>,
}

impl Animation {
	pub fn load(context: &mut ggez::Context, dto: AnimationDto) -> Result<Self, Box<dyn std::error::Error>> {
		let texture = ggez::graphics::Image::new(context, &dto.texture)?;

		// TODO: Validate AnimationFrames.

		Ok(Self {
			texture,
			sprites_grid: dto.sprites_grid,
			animation_set: dto.animation_set,
		})
	}
}


#[derive(Debug, Deserialize)]
pub struct AnimationsDto {
	pub animations: Vec<AnimationDto>,
}

pub struct Animations {
	pub animations: Vec<Animation>,
}

impl Animations {
	pub fn load(context: &mut ggez::Context, path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
		log::debug!("Animations::load: path: {:?}", path);

		let f = File::open(&path)?;
		let dto: AnimationsDto = from_reader(f)?;


		let mut animations = Vec::new();
		for dto in dto.animations.into_iter() {
			animations.push(Animation::load(context, dto)?);
		}

		Ok(Self {
			animations,
		})
	}
}


pub struct Resources {
	pub static_animations: Animations,
}

impl Resources {
	pub fn load(context: &mut ggez::Context, resource_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
		let static_animations = Animations::load(context, &resource_path.join("static_animations.ron"))?;

		Ok(Self {
			static_animations,
		})
	}
}
