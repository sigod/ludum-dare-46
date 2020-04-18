use amethyst::input::{is_key_down};
use amethyst::prelude::*;
use amethyst::winit;


pub struct GameState {
}

impl SimpleState for GameState {
	fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
		log::info!("hello");
	}

	fn handle_event(&mut self,
		_data: StateData<'_, GameData<'_, '_>>,
		event: StateEvent,
	) -> SimpleTrans {
		if let StateEvent::Window(event) = &event {
			if is_key_down(&event, winit::VirtualKeyCode::Escape) {
				Trans::Pop
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
