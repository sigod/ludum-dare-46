use crate::animations::AnimationId;


pub struct Game {
	pub click: bool,
	pub animation_id: AnimationId,
}

impl Default for Game {
	fn default() -> Self {
		Game {
			click: false,
			animation_id: AnimationId::BurnLow,
		}
	}
}


#[derive(PartialEq)]
pub enum CurrentState {
	Running,
	Paused,
}

impl Default for CurrentState {
	fn default() -> Self {
		CurrentState::Paused
	}
}
