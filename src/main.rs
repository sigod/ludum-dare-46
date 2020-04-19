use amethyst::{
	animation::AnimationBundle,
	assets::{PrefabLoaderSystemDesc},
	core::{TransformBundle},
	prelude::*,
	input::{InputBundle, StringBindings},
	renderer::{
		plugins::{RenderFlat2D, RenderToWindow},
		types::DefaultBackend,
		RenderingBundle, SpriteRender,
	},
	utils::application_root_dir,
};


mod animations;
mod game;
mod states;
mod systems;
mod utils;
mod audio;
use amethyst::audio::AudioBundle;

use crate::animations::{AnimationId, MyPrefabData};
use crate::game::{CurrentState};
use crate::states::{LoadingState};
use crate::states::game::{AnimationId, MyPrefabData};
use crate::audio::{play_background_sound, Sounds};
use crate::systems::InteractionSystem;


fn main() -> amethyst::Result<()> {
	amethyst::Logger::from_config(Default::default())
		.level_for("*", log::LevelFilter::Debug)
		.level_for("ludum_dare_46", log::LevelFilter::Debug)
		.start();

	let app_root = application_root_dir()?;
	let assets_directory = app_root.join("assets");
	let display_config_path = app_root.join("config").join("display.ron");

	let game_data = GameDataBuilder::default()
		.with_system_desc(
			PrefabLoaderSystemDesc::<MyPrefabData>::default(),
			"scene_loader",
			&[],
		)
		.with_bundle(AudioBundle::default())?
		.with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new(
			"sprite_animation_control",
			"sprite_sampler_interpolation",
		))?
		.with_bundle(TransformBundle::new())?
		.with_bundle(InputBundle::<StringBindings>::new())?
		.with_bundle(
			RenderingBundle::<DefaultBackend>::new()
				.with_plugin(
					RenderToWindow::from_config_path(display_config_path)?
						// NOTE: Deep pink.
						.with_clear([1.0, 0.078, 0.576, 1.0]),
				)
				.with_plugin(RenderFlat2D::default()),
		)?
		.with(
			InteractionSystem::default().pausable(CurrentState::Running),
			"interaction_system",
			&["input_system"]
		);

	let mut game = Application::build(assets_directory, LoadingState::default())?.build(game_data)?;
	game.run();
	Ok(())
}
