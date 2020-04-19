use amethyst::{
	assets::AssetStorage,
	audio::{output::Output, Source},
	ecs::{Read, ReadExpect, System},
};
use std::ops::Deref;

use crate::audio::{play_background_sound, Sounds};

#[derive(Default)]
pub struct SoundSystem;

impl<'s> System<'s> for SoundSystem {
	type SystemData = (
		ReadExpect<'s, Sounds>,
		Read<'s, AssetStorage<Source>>,
		Option<Read<'s, Output>>,
	);

	fn run(&mut self, (sounds, storage, audio_output): Self::SystemData){
		play_background_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
	}
}
