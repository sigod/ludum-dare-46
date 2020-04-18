use amethyst::assets::AssetStorage;
use amethyst::assets::Handle;
use amethyst::assets::Loader;
use amethyst::core::transform::Transform;
use amethyst::ecs::Entity;
use amethyst::prelude::*;
use amethyst::renderer::ImageFormat;
use amethyst::renderer::SpriteRender;
use amethyst::renderer::SpriteSheet;
use amethyst::renderer::SpriteSheetFormat;
use amethyst::renderer::Texture;
use amethyst::window::ScreenDimensions;


pub fn screen_dimensions(world: &World) -> (f32, f32) {
	let dim = world.read_resource::<ScreenDimensions>();
	(dim.width(), dim.height())
}

pub fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> Handle<SpriteSheet> {
	let texture_handle = {
		let loader = world.read_resource::<Loader>();
		let texture_storage = world.read_resource::<AssetStorage<Texture>>();
		loader.load(png_path, ImageFormat::default(), (), &texture_storage)
	};
	let loader = world.read_resource::<Loader>();
	let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
	loader.load(
		ron_path,
		SpriteSheetFormat(texture_handle),
		(),
		&sprite_sheet_store,
	)
}

pub fn load_background(world: &mut World, sprite_sheet: Handle<SpriteSheet>) -> Entity {
	let (width, height) = screen_dimensions(world);

	let mut transform = Transform::default();
	transform.set_translation_xyz(width * 0.5, height * 0.5, 0.0);

	let sprite = SpriteRender {
		sprite_sheet: sprite_sheet.clone(),
		sprite_number: 0,
	};

	world
		.create_entity()
		.with(transform)
		.with(sprite)
		.build()
}
