use ggez::{self, *};
use ggez::graphics::Drawable;

mod resources;

use resources::Resources;


const GAME_ID: &str = "ludum-dare-46";
const GAME_TITLE: &str = "Ember Story";
const AUTHOR: &str = "sigod & co";

const DIMENSIONS: (f32, f32) = (1280.0, 800.0);
const DESIRED_FPS: u32 = 70;


struct MainState {
	resources: Resources,
}

impl MainState {
	pub fn new(_context: &mut Context, resources: Resources) -> Self {
		Self {
			resources,
		}
	}
}

impl event::EventHandler for MainState {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
		let mut has_updated = false;

		while timer::check_update_time(context, DESIRED_FPS) {
			if has_updated {
				continue;
			}

			// TODO: Update scenes.
			has_updated = true;
		}

		// TODO: input.update(timer::duration_to_f64(timer::delta(context)) as f32);

		Ok(())
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::Color::from((1.0, 0.078, 0.576, 0.0)));

		// TODO: Draw scenes.
		self.resources.menu.draw(context, graphics::DrawParam::default())?;
		self.resources.static_animations.draw(context);

		graphics::present(context)
	}

	fn resize_event(&mut self, context: &mut Context, width: f32, height: f32) {
		log::info!("received resize event: {}x{}", width, height);
		log::info!("screen_coordinates: {:?}", graphics::screen_coordinates(context));
	}
}


fn main() {
	flexi_logger::Logger::with_env_or_str("error, ludum_dare_46=debug")
		// .format(flexi_logger::detailed_format)
		.start()
		.unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

	std::panic::set_hook(Box::new(|panic_info| {
		log::error!("{}", panic_info);
		eprintln!("{}", panic_info);
	}));


	let resource_path = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
		let mut path = std::path::PathBuf::from(manifest_dir);
		path.push("resources");

		path
	}
	else {
		std::path::PathBuf::from("./resources")
	};
	log::info!("resource path: {:?}", resource_path);


	let cb = ContextBuilder::new(GAME_ID, AUTHOR)
		.window_setup(conf::WindowSetup::default().title(GAME_TITLE))
		.window_mode(conf::WindowMode::default()
			.dimensions(DIMENSIONS.0, DIMENSIONS.1)
		)
		.add_resource_path(&resource_path);
	let (context, ev) = &mut cb.build().unwrap();

	log::info!("main: screen_coordinates: {:?}", graphics::screen_coordinates(context));


	let resources = Resources::load(context, &resource_path).expect("Failed to load resources!");
	let state = &mut MainState::new(context, resources);

	if let Err(e) = event::run(context, ev, state) {
		log::error!("Error encountered: {}", e);
	}
	else {
		log::info!("Game exited cleanly.");
	}
}
