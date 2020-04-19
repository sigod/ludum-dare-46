use amethyst::{
	animation::{
		get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet,
		EndControl,
	},
	assets::AssetStorage,
	audio::{output::Output, Source},
	ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
	input::{InputHandler, StringBindings},
	renderer::{
		sprite::{SpriteRender},
	},
	winit::MouseButton,
};
use std::ops::Deref;

use crate::animations::AnimationId;
use crate::audio::{play_background_sound, Sounds};
use crate::game::Game;
use crate::systems::interaction::ClickedObject::{Fire, Man1, Man2, Girl1, Girl2, NoObject};

#[derive(Debug)]
enum ClickedObject {
	Fire,
	Man1,
	Man2,
	Girl1,
	Girl2,
	NoObject,
}

fn get_clicked_object(x: f32, y: f32) -> ClickedObject {

	let fire: (f32, f32, f32, f32) = (463.0, 695.0, 796.0, 644.0);
	let fire_2: (f32, f32, f32, f32) =  (559.0, 630.0, 669.0, 519.0);
	let man1: (f32, f32, f32, f32) = (330.0, 603.0, 506.0, 368.0);
	let man2: (f32, f32, f32, f32) = (915.0, 672.0, 990.0, 570.0);
	let man2_2: (f32, f32, f32, f32) = (1001.0, 688.0, 1148.0, 387.0);
	let girl1: (f32, f32, f32, f32) = (72.0, 719.0, 309.0, 437.0);
	let girl2: (f32, f32, f32, f32) = (752.0, 590.0, 885.0, 366.0);

	if x >= fire.0 && x <= fire.2 && y <= fire.1 && y >= fire.3 { Fire }
	else if x >= fire_2.0 && x <= fire_2.2 && y <= fire_2.1 && y >= fire_2.3 { Fire }
	else if x >= man1.0 && x <= man1.2 && y <= man1.1 && y >= man1.3 { Man1 }
	else if x >= man2.0 && x <= man2.2 && y <= man2.1 && y >= man2.3 { Man2 }
	else if x >= man2_2.0 && x <= man2_2.2 && y <= man2_2.1 && y >= man2_2.3 { Man2 }
	else if x >= girl1.0 && x <= girl1.2 && y <= girl1.1 && y >= girl1.3 { Girl1 }
	else if x >= girl2.0 && x <= girl2.2 && y <= girl2.1 && y >= girl2.3 { Girl2 }
	else {NoObject}
}

#[derive(Default)]
pub struct InteractionSystem;

impl<'s> System<'s> for InteractionSystem {
	type SystemData = (
		Entities<'s>,
		Read<'s, InputHandler<StringBindings>>,
		ReadStorage<'s, AnimationSet<AnimationId, SpriteRender>>,
		WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
		Write<'s, Game>,
		Read<'s, AssetStorage<Source>>,
		ReadExpect<'s, Sounds>,
		Option<Read<'s, Output>>,
		amethyst::ecs::WriteExpect<'s, amethyst::audio::AudioSink>,
	);

	fn run(&mut self, (
		entities,
		input,
		animation_sets,
		mut control_sets,
		mut game,
		storage,
		sounds,
		audio_output,
		mut audio_sink,
	): Self::SystemData) {
		if !game.click && input.mouse_button_is_down(MouseButton::Left) {
			log::debug!("left mouse button is down");

			if let Some(mouse_position) = input.mouse_position() {
				log::debug!("current mouse position: {:?}", mouse_position);

				let clicked_object: ClickedObject = get_clicked_object(mouse_position.0, mouse_position.1);
				log::debug!("current object is: {:?}", clicked_object);

				let previous_id = game.animation_id;

				game.animation_id = match game.animation_id {
					AnimationId::BurnLow => AnimationId::BurnMedium,
					AnimationId::BurnMedium => AnimationId::BurnHigh,
					AnimationId::BurnHigh => AnimationId::BurnLow,
				};

				if game.animation_id == AnimationId::BurnHigh {
					audio_sink.set_volume(0.1);
				}

				log::debug!("animation_id: {:?} -> {:?}", previous_id, game.animation_id);

				for (entity, animation_set) in (&entities, &animation_sets).join() {
					let control_set = get_animation_set(&mut control_sets, entity).unwrap();

					if control_set.has_animation(previous_id) {
						control_set.remove(previous_id);
					}

					control_set.add_animation(
						game.animation_id,
						&animation_set.get(&game.animation_id).unwrap(),
						EndControl::Loop(None),
						1.0,
						AnimationCommand::Start,
					);
				}

				play_background_sound(&*sounds, &storage, audio_output.as_deref());
			}
		}

		game.click = input.mouse_button_is_down(MouseButton::Left);
	}
}
