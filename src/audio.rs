use amethyst::{
    assets::{Loader, AssetStorage},
    audio::{output::Output, Source, OggFormat, SourceHandle},
    ecs::{World, WorldExt},
};


const BACKGROUND_SOUND: &str = "audio/demo_1.2.ogg";

pub struct Sounds {
    pub background_sfx: SourceHandle,
}

/// Loads an ogg audio track.
fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

/// Initialise audio in the world. This will eventually include
/// the background tracks as well as the sound effects, but for now
/// we'll just work on sound effects.
pub fn initialise_audio(world: &mut World) {
    let sound_effects = {
        let loader = world.read_resource::<Loader>();

        let sound = Sounds {
            background_sfx: load_audio_track(&loader, &world, BACKGROUND_SOUND),
        };

        sound
    };

    // Add sound effects to the world. We have to do this in another scope because
    // world won't let us insert new resources as long as `Loader` is borrowed.
    world.insert(sound_effects);
}

pub fn play_background_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.background_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}