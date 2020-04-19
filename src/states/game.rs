use amethyst::{
	animation::{
		get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet,
		AnimationSetPrefab, EndControl,
	},
	assets::{Handle, PrefabData, PrefabLoader, ProgressCounter, RonFormat},
	derive::PrefabData,
	ecs::{prelude::Entity, Entities, Join, ReadStorage, WriteStorage},
	error::Error,
	input::{is_key_down},
	prelude::*,
	renderer::{
		sprite::{prefab::SpriteScenePrefab, SpriteRender},
		SpriteSheet,
	},
	GameData, SimpleState, SimpleTrans, StateData, Trans, winit,
};
use serde::{Deserialize, Serialize};

use crate::utils::{load_background, load_sprite_sheet};


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


#[derive(Default)]
pub struct GameState {
	fire_loading_progress: Option<ProgressCounter>,
	fire_entity: Option<Entity>,

	background_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for GameState {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		log::info!("GameState::on_start");

		let StateData { world, .. } = data;

		self.fire_loading_progress = Some(Default::default());

		let fire_prefab = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
			loader.load(
				"fire_animation.ron",
				RonFormat,
				self.fire_loading_progress.as_mut().unwrap(),
			)
		});

		let fire_entity = world
			.create_entity()
			.with(fire_prefab)
			.build();

		self.fire_entity = Some(fire_entity);

		// TODO: Background can cover fire animation. Depends on order of loading.
		// self.background_handle.replace(load_sprite_sheet(world, "game_background.png", "game_background.ron"));
		// let _entity = load_background(world, self.background_handle.clone().unwrap());
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

		if let Some(fire_loading_progress) = &self.fire_loading_progress {
			if fire_loading_progress.is_complete() {
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

				// All data loaded
				self.fire_loading_progress = None;
			}
		}

		Trans::None
	}

	fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		log::info!("GameState::on_stop");

		let StateData { world, .. } = data;

		if let Some(fire_entity) = self.fire_entity.take() {
			let _ = world.delete_entity(fire_entity);
		}
	}
}
