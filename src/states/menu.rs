use amethyst::assets::Handle;
use amethyst::core::transform::Transform;
use amethyst::ecs::Entity;
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::Camera;
use amethyst::renderer::SpriteRender;
use amethyst::renderer::SpriteSheet;
use amethyst::winit;

use crate::utils::{load_sprite_sheet, screen_dimensions};
use super::GameState;


#[derive(Default)]
pub struct MenuState {
	handle: Option<Handle<SpriteSheet>>,
	background_entity: Option<Entity>,
}

impl SimpleState for MenuState {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;

		self.handle.replace(load_sprite_sheet(world, "menu.png", "menu.ron"));

		initialize_camera(world);
		self.initialize_background(world);
	}

	fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		log::info!("on_pause!");
		self.delete_background(data.world);
	}

	fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		log::info!("on_resume!");
		self.initialize_background(data.world);
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
					GameState {},
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
	fn initialize_background(&mut self, world: &mut World) {
		if let Some(sprite_sheet) = &self.handle {
			let (width, height) = screen_dimensions(world);

			let mut transform = Transform::default();
			transform.set_translation_xyz(width * 0.5, height * 0.5, 0.0);

			let sprite = SpriteRender {
				sprite_sheet: sprite_sheet.clone(),
				sprite_number: 0,
			};

			let entity = world
				.create_entity()
				.with(transform)
				.with(sprite)
				.build();

			self.background_entity.replace(entity);
		}
	}

	fn delete_background(&mut self, world: &mut World) {
		if let Some(background) = self.background_entity {
			let _ = world.delete_entity(background);
		}
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
		.build();
}
