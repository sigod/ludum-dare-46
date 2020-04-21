use ggez::Context;
use ggez::audio::{Source, SoundSource};
use ggez::graphics::{Drawable, DrawParam, Image};
use ggez::timer;
use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

use crate::assets::Sounds;
use crate::assets::StoryFragments;
use crate::constants::*;


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
	pub destination: (f32, f32),
}

pub struct Animation {
	pub texture: Image,
	pub draw_param: DrawParam,

	pub animated: bool,
	pub start_time: f64,

	pub sprites_grid: Grid,
	pub animation_set: Vec<(AnimationId, AnimationFrames)>,
}

impl Animation {
	pub fn load(context: &mut ggez::Context, dto: AnimationDto) -> Result<Self, Box<dyn std::error::Error>> {
		let texture = ggez::graphics::Image::new(context, &dto.texture)?;

		// TODO: Validate AnimationFrames.

		Ok(Self {
			texture,
			draw_param: DrawParam::default()
				.dest(cgmath::Point2::new(dto.destination.0, dto.destination.1)),

			animated: false,
			start_time: 0.0,

			sprites_grid: dto.sprites_grid,
			animation_set: dto.animation_set,
		})
	}

	pub fn draw(&self, context: &mut Context) -> ggez::GameResult {
		if self.animated {
			self.texture.draw(context, self.draw_param)?;
		}

		Ok(())
	}

	pub fn animate(&mut self, context: &mut Context, animation_id: AnimationId) {
		for set in self.animation_set.iter() {
			if set.0 == animation_id {
				let frames: &AnimationFrames = &set.1;

				let current_time = timer::duration_to_f64(timer::time_since_start(context));

				let frame = if !self.animated {
					self.start_time = current_time;
					self.animated = true;

					frames.output.get(0).unwrap()
				}
				else {
					let mut diff = current_time - self.start_time;

					let last = frames.input.last().unwrap().clone() as f64;

					while diff > last {
						diff -= last;
					}

					let mut highest_index = 0;

					for (index, &timing) in frames.input.iter().enumerate() {
						if diff > timing as f64 {
							highest_index = index;
						}
						else {
							break;
						}
					}

					// log::debug!("diff: {}, highest_index: {}", diff, highest_index);

					frames.output.get(highest_index).unwrap()
				};

				let column: u32 = frame % self.sprites_grid.columns;
				let row: u32 = frame / self.sprites_grid.columns;

				let mut source = ggez::graphics::Rect::default();

				let (cell_x, cell_y) = self.sprites_grid.cell_size;

				source.x = (column * cell_x) as f32;
				source.y = (row * cell_y) as f32;
				source.w = ((column + 1) * cell_x) as f32;
				source.h = ((row + 1) * cell_y) as f32;

				// log::debug!("frame {} -> {:?}", frame, source);

				source.x = source.x / self.sprites_grid.texture_width as f32;
				source.y = source.y / self.sprites_grid.texture_height as f32;
				source.w = source.w / self.sprites_grid.texture_width as f32 - source.x;
				source.h = source.h / self.sprites_grid.texture_height as f32 - source.y;

				// log::debug!("frame {} -> {:?}", frame, source);

				self.draw_param.src = source;
			}
		}
	}

	pub fn reset(&mut self) {
		self.animated = false;
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
	pub end_screen_fail: Image,
	pub end_screen_success: Image,
	pub music: Source,
	pub campfire_sound: Source,
	pub sounds: Sounds,
}

impl Resources {
	pub fn load(context: &mut ggez::Context, resource_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
		let static_animations = Animations::load(context, &resource_path.join("static_animations.ron"))?;
		let menu = Image::new(context, "/menu.png")?;
		let background = Image::new(context, "/background.png")?;
		let story = StoryFragments::load(context)?;
		let text_empty = Image::new(context, "/story/text_empty.png")?;
		let mut music = Source::new(context, "/audio/demo_1.2.ogg")?;
		let mut campfire_sound = Source::new(context, "/audio/campfire.ogg")?;
		let sounds = Sounds::load(context)?;
		let end_screen_fail = Image::new(context, "/end_screen_fail.png")?;
		let end_screen_success = Image::new(context, "/end_screen_success.png")?;

		music.set_volume(BACKGROUND_MUSIC_VOLUME);
		campfire_sound.set_volume(FIRE_VOLUME);

		Ok(Self {
			static_animations,
			menu,
			background,
			story,
			text_empty,
			end_screen_fail,
			end_screen_success,
			music,
			campfire_sound,
			sounds,
		})
	}
}
