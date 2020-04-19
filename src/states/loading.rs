use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::assets::{PrefabLoader, RonFormat};
use amethyst::assets::ProgressCounter;
use amethyst::core::transform::Transform;
use amethyst::ecs::Entity;
use amethyst::prelude::*;
use amethyst::renderer::ImageFormat;
use amethyst::renderer::SpriteRender;
use amethyst::renderer::SpriteSheet;
use amethyst::renderer::SpriteSheetFormat;
use amethyst::renderer::Texture;
use amethyst::renderer::Transparent;

use crate::audio::initialise_audio;
use crate::animations::{MyPrefabData};
use crate::states::{MenuState};
use crate::utils::{screen_dimensions};


#[derive(Clone)]
pub struct GameEntities {
	pub menu_background: Entity,
	pub game_background: Entity,
	pub camp_fire: Entity,
	pub game_background_shadows: Entity,
}


#[derive(Default)]
pub struct LoadingState {
	progress_counter: ProgressCounter,

	entities: Option<GameEntities>,
}

impl SimpleState for LoadingState {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let StateData { world, .. } = data;

		let menu_background = {
			let menu_background = load_sprite_sheet(world, &mut self.progress_counter, "menu.png", "menu.ron");
			load_background(world, menu_background, "menu")
		};
		let game_background = {
			let game_background = load_sprite_sheet(world, &mut self.progress_counter, "game_background.png", "game_background.ron");
			load_background(world, game_background, "game_background")
		};
		let camp_fire = {
			let fire_prefab = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
				loader.load(
					"fire_animation.ron",
					RonFormat,
					&mut self.progress_counter,
				)
			});

			world
				.create_entity()
				.with(fire_prefab)
				.named("camp_fire")
				.build()
		};
		let game_background_shadows = {
			let fire_prefab = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
				loader.load(
					"game_background_shadows.ron",
					RonFormat,
					&mut self.progress_counter,
				)
			});

			world
				.create_entity()
				.with(fire_prefab)
				.named("game_background_shadows")
				.with(Transparent)
				.build()
		};

		self.entities.replace(GameEntities {
			menu_background,
			game_background,
			camp_fire,
			game_background_shadows,
		});

		initialise_audio(world);
	}

	fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
		if self.progress_counter.is_complete() {
			Trans::Switch(Box::new(
				MenuState { entities: self.entities.clone().unwrap() },
			))
		}
		else {
			Trans::None
		}
	}
}

pub fn load_sprite_sheet(world: &mut World, counter: &mut ProgressCounter, png_path: &str, ron_path: &str) -> Handle<SpriteSheet> {
	let texture_handle = {
		let loader = world.read_resource::<Loader>();
		let texture_storage = world.read_resource::<AssetStorage<Texture>>();
		loader.load(png_path, ImageFormat::default(), &mut *counter, &texture_storage)
	};
	let loader = world.read_resource::<Loader>();
	let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
	loader.load(
		ron_path,
		SpriteSheetFormat(texture_handle),
		counter,
		&sprite_sheet_store,
	)
}

pub fn load_background(world: &mut World, sprite_sheet: Handle<SpriteSheet>, name: &'static str) -> Entity {
	let (width, height) = screen_dimensions(world);

	let mut transform = Transform::default();
	transform.set_translation_xyz(width * 0.5, height * 0.5, 100.0);

	let sprite = SpriteRender {
		sprite_sheet: sprite_sheet.clone(),
		sprite_number: 0,
	};

	world
		.create_entity()
		.with(transform)
		.with(sprite)
		.named(name)
		.build()
}
