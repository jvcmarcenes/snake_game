
pub(crate) mod world;
pub(crate) mod world_gen;
pub(crate) mod snake;
pub(crate) mod colors;
pub(crate) mod direction;
pub(crate) mod position;
pub(crate) mod input;
pub(crate) mod region;

pub(crate) mod prelude {
    pub(crate) use crate::{
		*,
		colors,
		world::*,
		world_gen::*,
		snake::*,
        direction::*,
        position::*,
		input::*,
		region::*,
    };
}

use std::time::Duration;

use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessFixedTimestepExt, ConditionSet};

use prelude::*;
#[allow(unused)] use prelude::Direction;

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct Collidable;

fn main() {
	App::new()

	.add_plugins(DefaultPlugins.set(WindowPlugin {
		window: WindowDescriptor {
			position: WindowPosition::Centered,
			..default()
		},
		..default()
	}))

	.insert_resource(ClearColor(colors::onyx()))
	.insert_resource(InputBuffer::new())
	.insert_resource(ActiveRegion::default())
	.insert_resource(WorldGen)

	.add_event::<MoveEvent>()
	.add_event::<EatEvent>()
	.add_event::<GameOverEvent>()

	.add_startup_system(world::spawn_camera)
	.add_startup_system(world::setup_world)

	.add_system(position::update_transform.after("postmove"))

	.add_startup_system(snake::setup)

	.add_fixed_timestep(Duration::from_millis(100), "step")
	.add_fixed_timestep_system_set(
		"step", 0,
		SystemSet::new()
		.with_system(snake::premove)

		.with_system(snake::consume_input.before(snake::premove))
	)

	.add_system(snake::move_.label("move"))
	.add_system(snake::move_body.after("move"))

	.add_system(snake::eat)

	.add_system(snake::game_over)

	.add_system_set(
		ConditionSet::new()
		.label("postmove")
		.after("move")
		.with_system(world::camera_follow)
		.with_system(region::track_region)
		.into()
	)

	.add_system(region::update_active_region)


	.add_system(input::gather)

	.run();
}
