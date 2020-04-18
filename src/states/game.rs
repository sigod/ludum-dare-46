use amethyst::{
	animation::{
		get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet,
		AnimationSetPrefab, EndControl,
	},
	assets::{PrefabData, PrefabLoader, ProgressCounter, RonFormat},
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


/// Animation ids used in a AnimationSet
#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AnimationId {
	Burn,
}

/// Loading data for one entity
#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct MyPrefabData {
	/// Information for rendering a scene with sprites
	sprite_scene: SpriteScenePrefab,
	/// Аll animations that can be run on the entity
	animation_set: AnimationSetPrefab<AnimationId, SpriteRender>,
}


#[derive(Default)]
pub struct GameState {
	progress_counter: Option<ProgressCounter>,
}

impl SimpleState for GameState {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		log::info!("GameState::on_start");

		let world = data.world;

		self.progress_counter = Some(Default::default());

		let fire_prefab = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
			loader.load(
				"fire_animation.ron",
				RonFormat,
				self.progress_counter.as_mut().unwrap(),
			)
		});

		world
			.create_entity()
			.with(fire_prefab)
			.build();
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

	fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
		let StateData { world, .. } = data;

		if let Some(progress_counter) = &self.progress_counter {
			if progress_counter.is_complete() {
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
								AnimationId::Burn,
								&animation_set.get(&AnimationId::Burn).unwrap(),
								EndControl::Loop(None),
								1.0,
								AnimationCommand::Start,
							);
						}
					},
				);

				// All data loaded
				self.progress_counter = None;
			}
		}

		Trans::None
	}
}
