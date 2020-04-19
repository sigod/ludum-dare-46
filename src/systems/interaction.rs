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
