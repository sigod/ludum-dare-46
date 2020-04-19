use amethyst::audio::{output::Output, AudioSink, Source};
use amethyst::core::Named;
use amethyst::core::transform::Transform;
use amethyst::ecs::{Join, ReadStorage, WriteExpect, WriteStorage};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::Camera;
use amethyst::winit;

use amethyst::{
	animation::{
		get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet,
		EndControl,
	},
	assets::AssetStorage,
	ecs::{Entities, Read, ReadExpect, System, Write},
	input::{InputHandler, StringBindings},
	renderer::{
		sprite::{SpriteRender},
	},
	winit::MouseButton,
};
use std::ops::Deref;


use crate::utils::{screen_dimensions};
use crate::audio::{play_background_sound, Sounds};
use super::{GameState, loading::GameEntities};


pub struct MenuState {
	pub entities: GameEntities,
}

impl SimpleState for MenuState {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		log::info!("MenuState::on_start");

		let world = data.world;

		initialize_camera(world);
		self.show_background(world);
		self.play_audio(world);
	}

	fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		self.hide_background(data.world);
	}

	fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		self.show_background(data.world);
	}

	fn handle_event(&mut self,
		_data: StateData<'_, GameData<'_, '_>>,
		event: StateEvent,
	) -> SimpleTrans {
		if let StateEvent::Window(event) = &event {
			if is_close_requested(&event) || is_key_down(&event, winit::VirtualKeyCode::Escape) {
				Trans::Quit
			}
			else if is_key_down(&event, winit::VirtualKeyCode::Space) {
				Trans::Push(Box::new(
					GameState::new(self.entities.clone()),
				))
			}
			else {
				Trans::None
			}
		}
		else {
			Trans::None
		}
	}
}

impl MenuState {
	fn show_background(&mut self, world: &mut World) {
		world.exec(
			|(nameds, mut transforms): (
				ReadStorage<Named>,
				WriteStorage<Transform>,
			)| {
				for (named, transform) in (&nameds, &mut transforms).join() {
					if named.name == "menu" {
						transform.set_translation_z(0.0);
					}
				}
			},
		);
	}

	fn hide_background(&mut self, world: &mut World) {
		world.exec(
			|(nameds, mut transforms): (
				ReadStorage<Named>,
				WriteStorage<Transform>,
			)| {
				for (named, transform) in (&nameds, &mut transforms).join() {
					if named.name == "menu" {
						transform.set_translation_z(100.0);
					}
				}
			},
		);
	}

	fn play_audio(&mut self, world: &mut World) {
		world.exec(
			|(storage, sounds, audio_sink): (
				Read<AssetStorage<Source>>,
				ReadExpect<Sounds>,
				// Option<Read<Output>>,
				WriteExpect<AudioSink>,
			)| {
				// TODO: thread 'rodio audio processing' panicked at 'assertion failed: self.next_frame.is_empty()', <::std::macros::panic macros>:2:4

				// play_background_sound(&*sounds, &storage, audio_output.as_deref());

				if let Some(sound) = storage.get(&sounds.background_sfx) {
					audio_sink.append(sound).unwrap();
				}
			},
		);
	}
}


fn initialize_camera(world: &mut World) {
	let (width, height) = screen_dimensions(world);

	// Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
	let mut transform = Transform::default();
	transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

	world
		.create_entity()
		.with(Camera::standard_2d(width, height))
		.with(transform)
		.named("camera")
		.build();
}
