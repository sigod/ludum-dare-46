use amethyst::{
	animation::{
		get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet,
		AnimationSetPrefab, EndControl,
	},
	assets::{PrefabData, ProgressCounter},
	core::{
		transform::Transform,
		Named
	},
	derive::PrefabData,
	ecs::{prelude::Entity, Entities, Join, ReadStorage, WriteStorage},
	error::Error,
	input::{is_key_down},
	prelude::*,
	renderer::{
		sprite::{prefab::SpriteScenePrefab, SpriteRender},
	},
	GameData, SimpleState, SimpleTrans, StateData, Trans, winit,
};
use serde::{Deserialize, Serialize};

use super::loading::GameEntities;


/// Animation ids used in a AnimationSet
#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AnimationId {
	BurnLow,
	BurnMedium,
	BurnHigh,
}

/// Loading data for one entity
#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct MyPrefabData {
	/// Information for rendering a scene with sprites
	sprite_scene: SpriteScenePrefab,
	/// Аll animations that can be run on the entity
	animation_set: AnimationSetPrefab<AnimationId, SpriteRender>,
}


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
			|(entities, animation_sets, mut control_sets): (
				Entities,
				ReadStorage<AnimationSet<AnimationId, SpriteRender>>,
				WriteStorage<AnimationControlSet<AnimationId, SpriteRender>>,
			)| {
				// For each entity that has AnimationSet
				for (entity, animation_set) in (&entities, &animation_sets).join() {
					// Creates a new AnimationControlSet for the entity
					let control_set = get_animation_set(&mut control_sets, entity).unwrap();
					// Adds the `Fly` animation to AnimationControlSet and loops infinitely
					control_set.add_animation(
						AnimationId::BurnLow,
						&animation_set.get(&AnimationId::BurnLow).unwrap(),
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

		self.init_entities(world);
		self.start_fire(world);
	}

	fn handle_event(&mut self,
		_data: StateData<'_, GameData<'_, '_>>,
		event: StateEvent,
	) -> SimpleTrans {
		if let StateEvent::Window(event) = &event {
			if is_key_down(&event, winit::VirtualKeyCode::Escape) {
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
