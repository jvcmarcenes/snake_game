
pub mod world;
pub mod snake;
pub mod colors;
pub mod direction;
pub mod position;
pub mod input;

pub mod prelude {
    pub use crate::{
		colors,
		world::*,
		snake::*,
        direction::*,
        position::*,
		input::*,
    };
}

use std::time::Duration;

use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessFixedTimestepExt;

use prelude::*;

fn main() {
	App::new()

	.add_plugins(DefaultPlugins.set(WindowPlugin {
		window: WindowDescriptor {
			position: WindowPosition::Centered,
			..default()
		},
		..default()
	}))
	
	.insert_resource(ClearColor(colors::white()))
	.insert_resource(InputBuffer::new())

	.add_startup_system(world::spawn_camera)
	
	.add_system(position::update_transform)
	
	.add_startup_system(snake::setup)
	
	.add_fixed_timestep(Duration::from_millis(120), "step")
	.add_fixed_timestep_system_set(
		"step", 0,
		SystemSet::new()
		.with_system(snake::move_)
		
		.with_system(snake::consume_input.before(snake::move_))
		
		.with_system(world::camera_follow.after(snake::move_))
	)

	.add_system(input::gather)

	.run();
}
