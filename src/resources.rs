use ggez::Context;
use ggez::graphics::{Drawable, DrawParam, Image};
use ggez::timer;
use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

use crate::assets::StoryFragments;


#[derive(Debug, Deserialize)]
pub struct Grid {
	pub texture_width: u32,
	pub texture_height: u32,
	pub columns: u32,
	pub rows: u32,
	pub cell_size: (u32, u32),
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
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

	// TODO: Store the time we've started animation?

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

	pub fn animate(&mut self, context: &mut Context, animation_id: AnimationId) {
		for set in self.animation_set.iter() {
			if set.0 == animation_id {
				let frames: &AnimationFrames = &set.1;

				let current_time = timer::duration_to_f64(timer::time_since_start(context));

				// TODO: Use `frames` to figure out which frame we should be showing.
				// TODO: Update self.draw_param accordingly.
				// See https://docs.rs/ggez/0.5.1/ggez/graphics/struct.DrawParam.html
			}
		}
	}

	pub fn reset(&mut self) {
		// TODO: Reset animation counters or whatever.
	}
}


#[derive(Debug, Deserialize)]
pub struct AnimationsDto {
	pub animations: Vec<AnimationDto>,
}

pub struct Animations {
	pub animations: Vec<Animation>,

	pub animation_id: AnimationId,
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
			animation_id: AnimationId::BurnLow,
		})
	}

	pub fn draw(&self, context: &mut Context) -> ggez::GameResult {
		for animation in self.animations.iter() {
			animation.draw(context)?;
		}

		Ok(())
	}

	pub fn animate(&mut self, context: &mut Context) {
		for animation in self.animations.iter_mut() {
			animation.animate(context, self.animation_id);
		}
	}

	pub fn reset(&mut self) {
		for animation in self.animations.iter_mut() {
			animation.reset();
		}
	}
}


pub struct Resources {
	pub static_animations: Animations,
	pub menu: Image,
	pub background: Image,
	pub story: StoryFragments,
	pub text_empty: Image,
}

impl Resources {
	pub fn load(context: &mut ggez::Context, resource_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
		let static_animations = Animations::load(context, &resource_path.join("static_animations.ron"))?;
		let menu = Image::new(context, "/menu.png")?;
		let background = Image::new(context, "/background.png")?;
		let story = StoryFragments::load(context)?;
		let text_empty = Image::new(context, "/story/text_empty.png")?;

		Ok(Self {
			static_animations,
			menu,
			background,
			story,
			text_empty,
		})
	}
}
