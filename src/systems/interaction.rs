use amethyst::{
	animation::{
		get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet,
		EndControl,
	},
	ecs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage},
	input::{InputHandler, StringBindings},
	renderer::{
		sprite::{SpriteRender},
	},
	winit::MouseButton,
};

use crate::animations::AnimationId;
use crate::game::Game;


#[derive(Default)]
pub struct InteractionSystem;

impl<'s> System<'s> for InteractionSystem {
	type SystemData = (
		Entities<'s>,
		Read<'s, InputHandler<StringBindings>>,
		ReadStorage<'s, AnimationSet<AnimationId, SpriteRender>>,
		WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
		Write<'s, Game>,
	);

	fn run(&mut self, (entities, input, animation_sets, mut control_sets, mut game): Self::SystemData) {
		if !game.click && input.mouse_button_is_down(MouseButton::Left) {
			log::info!("left mouse button is down");

			if let Some(mouse_position) = input.mouse_position() {
				log::info!("current mouse position: {:?}", mouse_position);

				for (entity, animation_set) in (&entities, &animation_sets).join() {
					let control_set = get_animation_set(&mut control_sets, entity).unwrap();

					if control_set.has_animation(game.animation_id) {
						control_set.remove(game.animation_id);
					}

					game.animation_id = match game.animation_id {
						AnimationId::BurnLow => AnimationId::BurnMedium,
						AnimationId::BurnMedium => AnimationId::BurnHigh,
						AnimationId::BurnHigh => AnimationId::BurnLow,
					};

					control_set.add_animation(
						// AnimationId::BurnLow,
						// &animation_set.get(&AnimationId::BurnLow).unwrap(),
						game.animation_id,
						&animation_set.get(&game.animation_id).unwrap(),
						EndControl::Loop(None),
						1.0,
						AnimationCommand::Start,
					);

					log::info!("animation_id: {:?}", game.animation_id);
				}
			}
		}

		game.click = input.mouse_button_is_down(MouseButton::Left);
	}
}
