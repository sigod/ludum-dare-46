use amethyst::{
	animation::{
		get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet,
		EndControl,
	},
	core::{
		transform::Transform,
		Named
	},
	ecs::{Entities, Join, Read, ReadStorage, WriteStorage},
	input::{is_key_down},
	prelude::*,
	renderer::{
		sprite::{ SpriteRender},
	},
	GameData, SimpleState, SimpleTrans, StateData, Trans, winit,
};

use crate::audio::initialise_audio;
use crate::audio::{play_background_sound, Sounds};
use crate::animations::AnimationId;
use crate::game::{CurrentState, Game};
use super::loading::GameEntities;


pub struct GameState {
	entities: GameEntities,
}

impl GameState {
	pub fn new(entities: GameEntities) -> Self {
		GameState {
			entities,
		}
	}

	fn init_entities(&mut self, world: &mut World) {
		world.exec(
			|(nameds, mut transforms): (
				ReadStorage<Named>,
				WriteStorage<Transform>,
			)| {
				for (named, transform) in (&nameds, &mut transforms).join() {
					if named.name == "game_background" {
						transform.set_translation_z(-100.0);
					}
				}
			},
		);
	}

	fn start_fire(&mut self, world: &mut World) {
		// Execute a pass similar to a system
		world.exec(
			|(entities, game, animation_sets, mut control_sets): (
				Entities,
				Read<Game>,
				ReadStorage<AnimationSet<AnimationId, SpriteRender>>,
				WriteStorage<AnimationControlSet<AnimationId, SpriteRender>>,
			)| {
				// For each entity that has AnimationSet
				for (entity, animation_set) in (&entities, &animation_sets).join() {
					// Creates a new AnimationControlSet for the entity
					let control_set = get_animation_set(&mut control_sets, entity).unwrap();
					// Adds the `Fly` animation to AnimationControlSet and loops infinitely
					control_set.add_animation(
						game.animation_id,
						&animation_set.get(&game.animation_id).unwrap(),
						EndControl::Loop(None),
						1.0,
						AnimationCommand::Start,
					);
				}
			},
		);
	}
}

impl SimpleState for GameState {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		log::info!("GameState::on_start");

		let StateData { world, .. } = data;

		self.initialise_audio(world);
		self.init_entities(world);
		self.start_fire(world);
		*world.write_resource::<CurrentState>() = CurrentState::Running;
		play_background_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));

	}

	fn handle_event(&mut self,
		data: StateData<'_, GameData<'_, '_>>,
		event: StateEvent,
	) -> SimpleTrans {
		let StateData { world, .. } = data;

		if let StateEvent::Window(event) = &event {
			if is_key_down(&event, winit::VirtualKeyCode::Escape) {
				*world.write_resource::<CurrentState>() = CurrentState::Paused;

				Trans::Pop
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
