use ggez::Context;
use ggez::graphics::{Drawable, DrawParam, Image};
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
	BurnMedium,
	BurnHigh,
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
	pub texture: Image,
	pub draw_param: DrawParam,

	pub sprites_grid: Grid,
	pub animation_set: Vec<(AnimationId, AnimationFrames)>,
}

impl Animation {
	pub fn load(context: &mut ggez::Context, dto: AnimationDto) -> Result<Self, Box<dyn std::error::Error>> {
		let texture = ggez::graphics::Image::new(context, &dto.texture)?;

		// TODO: Validate AnimationFrames.

		Ok(Self {
			texture,
			draw_param: DrawParam::default(),

			sprites_grid: dto.sprites_grid,
			animation_set: dto.animation_set,
		})
	}

	pub fn draw(&self, context: &mut Context) -> ggez::GameResult {
		self.texture.draw(context, self.draw_param)?;

		Ok(())
	}

	pub fn animate(&mut self, context: &mut Context) {
		// if not started
		// let start = duration_to_f64(time_since_start(context));

		// TODO: Get time diff and select correct frame in AnimationFrames.
		// TODO: Update self.draw_param.src to change which sprite (from sprite sheet) should be drawn.
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

	pub fn draw(&self, context: &mut Context) {
		// TODO: Iterate all animations and draw() them.
	}

	pub fn animate() {
		// TODO: Same - for all.
	}
}


pub struct Resources {
	pub static_animations: Animations,
	pub menu: Image,
	pub background: Image,
}

impl Resources {
	pub fn load(context: &mut ggez::Context, resource_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
		let static_animations = Animations::load(context, &resource_path.join("static_animations.ron"))?;
		let menu = Image::new(context, "/menu.png")?;
		let background = Image::new(context, "/background.png")?;

		Ok(Self {
			static_animations,
			menu,
			background,
		})
	}
}
