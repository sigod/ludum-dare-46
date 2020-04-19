use amethyst::{
	animation::AnimationBundle,
	assets::{PrefabLoaderSystemDesc},
	core::{TransformBundle},
	prelude::*,
	renderer::{
		plugins::{RenderFlat2D, RenderToWindow},
		types::DefaultBackend,
		RenderingBundle, SpriteRender,
	},
	utils::application_root_dir,
};


mod states;
mod utils;

use crate::states::{MenuState};
use crate::states::game::{AnimationId, MyPrefabData};


fn main() -> amethyst::Result<()> {
	amethyst::Logger::from_config(Default::default())
		.level_for("*", log::LevelFilter::Debug)
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
		.with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new(
			"sprite_animation_control",
			"sprite_sampler_interpolation",
		))?
		.with_bundle(TransformBundle::new())?
		.with_bundle(
			RenderingBundle::<DefaultBackend>::new()
				.with_plugin(
					RenderToWindow::from_config_path(display_config_path)?
						// NOTE: Deep pink.
						.with_clear([1.0, 0.078, 0.576, 1.0]),
				)
				.with_plugin(RenderFlat2D::default()),
		)?;

	let mut game = Application::build(assets_directory, MenuState::default())?.build(game_data)?;
	game.run();
	Ok(())
}
