use amethyst::{
	animation::{AnimationSetPrefab},
	assets::{PrefabData, ProgressCounter},
	derive::PrefabData,
	ecs::{prelude::Entity},
	error::Error,
	renderer::{
		sprite::{prefab::SpriteScenePrefab, SpriteRender},
	},
};
use serde::{Deserialize, Serialize};

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
