#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ggez::{self, *};
use ggez::audio::SoundSource;
use ggez::graphics::Drawable;
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::input::mouse::MouseButton;

mod assets;
mod resources;
mod constants;

use crate::assets::play_random;
use crate::resources::Resources;
use crate::constants::*;


#[derive(Debug)]
enum GameObject {
	Fire,
	Man1,
	Man2,
	Girl1,
	Girl2,
	Owl1,
	Owl2,
	Owl3,
}

fn get_clicked_object(x: f32, y: f32) -> Option<GameObject> {
	let fire: (f32, f32, f32, f32) = (463.0, 695.0, 796.0, 644.0);
	let fire_2: (f32, f32, f32, f32) =  (559.0, 630.0, 669.0, 519.0);
	let man1: (f32, f32, f32, f32) = (330.0, 603.0, 506.0, 368.0);
	let man2: (f32, f32, f32, f32) = (915.0, 672.0, 990.0, 570.0);
	let man2_2: (f32, f32, f32, f32) = (1001.0, 688.0, 1148.0, 387.0);
	let girl1: (f32, f32, f32, f32) = (72.0, 719.0, 309.0, 437.0);
	let girl2: (f32, f32, f32, f32) = (752.0, 590.0, 885.0, 366.0);
	let owl1: (f32, f32, f32, f32) = (55.0, 292.0, 117.0, 242.0);
	let owl2: (f32, f32, f32, f32) =  (139.0, 65.0, 196.0, 25.0);
	let owl3: (f32, f32, f32, f32) = (1125.0, 161.0, 1182.0, 109.0);

	fn check(x: f32, y: f32, bound: (f32, f32, f32, f32)) -> bool {
		x >= bound.0 && x <= bound.2 && y <= bound.1 && y >= bound.3
	}

	use GameObject::*;

	if check(x, y, fire) { Some(Fire) }
	else if check(x, y, fire_2) { Some(Fire) }
	else if check(x, y, man1) { Some(Man1) }
	else if check(x, y, man2) { Some(Man2) }
	else if check(x, y, man2_2) { Some(Man2) }
	else if check(x, y, girl1) { Some(Girl1) }
	else if check(x, y, girl2) { Some(Girl2) }
	else if check(x, y, owl1) { Some(Owl1) }
	else if check(x, y, owl2) { Some(Owl2) }
	else if check(x, y, owl3) { Some(Owl3) }
	else { None }
}


#[derive(PartialEq)]
pub enum Scene {
	Menu,
	Game,
	EndScreenFail,
	EndScreenSuccess,
}


struct MainState {
	resources: Resources,

	scene: Scene,

	pub fire_intensity: f64,
	pub fire_drop_off: f64,
	pub wood_increase: f64,

	pub story_id: usize,
	pub story_in_progress: bool,

	pub delayed_start_after: f64,
}

impl MainState {
	pub fn new(_context: &mut Context, resources: Resources) -> Self {
		Self {
			resources,

			scene: Scene::Menu,

			fire_intensity: 0.,
			fire_drop_off: 0.,
			wood_increase: 0.,

			story_id: 0,
			story_in_progress: false,

			delayed_start_after: STORY_DELAY,
		}
	}

	pub fn reset_game(&mut self) {
		self.resources.static_animations.animation_id = STARTING_ANIMATION;
		self.resources.static_animations.reset();

		self.fire_intensity = FIRE_STARTING_INTENSITY;
		self.fire_drop_off = -FIRE_DROP_OFF_RATE;
		self.wood_increase = WOOD_INCREASE;

		self.story_id = 0;
		self.story_in_progress = false;
	}

	pub fn update_logic(&mut self, delta: f64) {
		self.fire_intensity += self.fire_drop_off * delta;

		let next_state = if self.fire_intensity < 0.0 {
			None
		}
		else if self.fire_intensity < ANIMATION_LOW_IS_BELOW {
			Some(resources::AnimationId::BurnLow)
		}
		else if self.fire_intensity < ANIMATION_MEDIUM_IS_BELOW {
			Some(resources::AnimationId::BurnMedium)
		}
		else {
			Some(resources::AnimationId::BurnHigh)
		};

		if let Some(state) = next_state {
			if self.resources.static_animations.animation_id != state {
				log::debug!("changing animation state: {:?} -> {:?}", self.resources.static_animations.animation_id, state);
				self.resources.static_animations.animation_id = state;
			}

			if self.delayed_start_after < 0.0 {
				if !self.story_in_progress {
					let (_, source) = self.resources.story.fragments.get_mut(self.story_id).unwrap();
					let _ = source.play();

					self.story_in_progress = true;
					log::debug!("started playing {} story", self.story_id);
				}
				else {
					let (_, source) = self.resources.story.fragments.get(self.story_id).unwrap();

					if !source.playing() {
						self.story_in_progress = false;
						log::debug!("finished playing {} story", self.story_id);

						self.story_id += 1;

						if self.story_id == self.resources.story.fragments.len() {
							log::debug!("finished playing all stories");
							self.scene = Scene::EndScreenSuccess;
						}
					}
				}
			}
			else {
				self.delayed_start_after -= delta;
			}
		}
		else {
			log::debug!("fire's gone");
			self.scene = Scene::EndScreenFail;
		}
	}

	pub fn add_wood(&mut self) {
		self.fire_intensity += self.wood_increase;
	}

	pub fn handle_game_click(&mut self, x: f32, y: f32) {
		let object = get_clicked_object(x, y);

		if let Some(object) = object {
			match object {
				GameObject::Fire => {
					log::debug!("clicked: fire");
					self.add_wood();
					let _ = play_random(&mut self.resources.sounds.firewood);
				},
				GameObject::Man1 => {
					log::debug!("clicked: man1");
					let _ = play_random(&mut self.resources.sounds.man1);
				},
				GameObject::Man2 => {
					log::debug!("clicked: man2");
					let _ = play_random(&mut self.resources.sounds.guitar);
				},
				GameObject::Girl1 => {
					log::debug!("clicked: girl1");
					let _ = play_random(&mut self.resources.sounds.girl1);
				},

				GameObject::Girl2 => {
					log::debug!("clicked: girl2");
					let _ = play_random(&mut self.resources.sounds.girl2);
				},

				GameObject::Owl1
				| GameObject::Owl2
				| GameObject::Owl3 => {
					log::debug!("clicked: an own");
					let _ = play_random(&mut self.resources.sounds.owl);
				},
			};
		}
	}
}

impl event::EventHandler for MainState {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
		let mut has_updated = false;

		while timer::check_update_time(context, DESIRED_FPS) {
			if has_updated {
				continue;
			}

			let delta = timer::duration_to_f64(timer::delta(context));

			if self.scene == Scene::Game {
				self.update_logic(delta);
				self.resources.static_animations.animate(context);
			}

			let _ = self.resources.music.play_later();

			if self.scene != Scene::EndScreenFail {
				let _ = self.resources.campfire_sound.play_later();
			}

			has_updated = true;
		}

		Ok(())
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		let pink = (1.0, 0.078, 0.576, 0.0);
		graphics::clear(context, graphics::Color::from(pink));

		match self.scene {
			Scene::Menu => {
				self.resources.menu.draw(context, graphics::DrawParam::default())?;
			},
			Scene::Game => {
				self.resources.background.draw(context, graphics::DrawParam::default())?;
				self.resources.static_animations.draw(context)?;

				if self.delayed_start_after < 0.0 {
					let text_param = graphics::DrawParam::default().dest(cgmath::Point2::new(100.0, 165.0));
					if self.story_in_progress {
						let (image, _) = self.resources.story.fragments.get(self.story_id).unwrap();

						image.draw(context, text_param)?;
					}
					else {
						self.resources.text_empty.draw(context, text_param)?;
					}
				}
			},
			Scene::EndScreenFail => {
				self.resources.end_screen_fail.draw(context, graphics::DrawParam::default())?;
			},
			Scene::EndScreenSuccess => {
				self.resources.end_screen_success.draw(context, graphics::DrawParam::default())?;
			},
		}

		graphics::present(context)
	}

	fn resize_event(&mut self, context: &mut Context, width: f32, height: f32) {
		log::info!("received resize event: {}x{}", width, height);
		log::info!("screen_coordinates: {:?}", graphics::screen_coordinates(context));
	}

	fn mouse_button_down_event(&mut self, _context: &mut Context, button: MouseButton, x: f32, y: f32) {
		log::debug!("mouse down: {:?} - {}x{}", button, x, y);
	}

	fn mouse_button_up_event(&mut self, _context: &mut Context, button: MouseButton, x: f32, y: f32) {
		log::debug!("mouse up: {:?} - {}x{}", button, x, y);

		match self.scene {
			Scene::Menu => {
				self.delayed_start_after = STORY_DELAY;
				self.scene = Scene::Game;
				self.reset_game();
			},
			Scene::EndScreenFail
			| Scene::EndScreenSuccess => {
				self.delayed_start_after = STORY_DELAY;
				self.scene = Scene::Menu;
			}
			Scene::Game => {
				self.handle_game_click(x, y);
			},
		}
	}

	fn key_down_event(&mut self, context: &mut Context, keycode: KeyCode, _keymods: KeyMods, repeat: bool) {
		if !repeat {
			if keycode == KeyCode::Escape {
				if self.scene == Scene::Menu {
					ggez::event::quit(context);
				}
				else {
					self.scene = Scene::Menu;
				}
			}
		}
	}

	fn key_up_event(&mut self, _context: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {
	}
}


fn main() {
	flexi_logger::Logger::with_env_or_str("error, ludum_dare_46=debug")
		.format(flexi_logger::detailed_format)
		// TODO: Log into stderr too.
		.log_to_file()
		.start()
		.unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

	std::panic::set_hook(Box::new(|panic_info| {
		log::error!("{}", panic_info);
		eprintln!("{}", panic_info);
	}));


	let resource_path = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
		let mut path = std::path::PathBuf::from(manifest_dir);
		path.push("resources");

		path
	}
	else {
		std::path::PathBuf::from("./resources")
	};
	log::info!("resource path: {:?}", resource_path);


	let cb = ContextBuilder::new(GAME_ID, AUTHOR)
		.window_setup(conf::WindowSetup::default().title(GAME_TITLE))
		.window_mode(conf::WindowMode::default()
			.dimensions(DIMENSIONS.0, DIMENSIONS.1)
		)
		.add_resource_path(&resource_path);
	let (context, ev) = &mut cb.build().unwrap();

	log::info!("main: screen_coordinates: {:?}", graphics::screen_coordinates(context));


	let resources = Resources::load(context, &resource_path).expect("Failed to load resources!");
	let state = &mut MainState::new(context, resources);

	if let Err(e) = event::run(context, ev, state) {
		log::error!("Error encountered: {}", e);
	}
	else {
		log::info!("Game exited cleanly.");
	}
}
