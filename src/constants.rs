use crate::resources::AnimationId;

pub const GAME_ID: &str = "ludum-dare-46";
pub const GAME_TITLE: &str = "Ember Story";
pub const AUTHOR: &str = "sigod & co";

pub const DIMENSIONS: (f32, f32) = (1280.0, 800.0);
pub const DESIRED_FPS: u32 = 70;

pub const GUITAR_VOLUME: f32 = 0.55;
pub const MAN_VOLUME: f32 = 0.28;
pub const GIRL1_VOLUME: f32 = 0.36;
pub const GIRL2_VOLUME: f32 = 0.36;
pub const FIRE_VOLUME: f32 = 0.30;
pub const OWL_VOLUME: f32 = 0.30;
pub const STORY_VOLUME: f32 = 0.22;
pub const FIREWOOD_THROW_VOLUME: f32 = 0.22;
pub const BACKGROUND_MUSIC_VOLUME: f32 = 0.30;

pub const STORY_DELAY: f64 = 3.0;
pub const STARTING_ANIMATION: AnimationId = AnimationId::BurnMedium;
pub const FIRE_STARTING_INTENSITY: f64 = 0.60;
pub const FIRE_DROP_OFF_RATE: f64 = 0.03;
pub const WOOD_INCREASE: f64 = 0.20;

pub const ANIMATION_LOW_IS_BELOW: f64 = 0.33;
pub const ANIMATION_MEDIUM_IS_BELOW: f64 = 0.66;
