
use bevy::prelude::*;
use crate::{prelude::*, snake::Head};

pub fn spawn_camera(mut com: Commands) {
	com.spawn((
		Camera2dBundle::default(),
        CameraFollow,
        Position::new(0, 0),
	));
}

#[derive(Component)]
pub struct CameraFollow;

pub fn camera_follow(
    // windows: Res<Windows>,
    mut cam: Query<&mut Position, (With<CameraFollow>, Without<Head>)>,
    snake_query: Query<&Position, With<Head>>,
) {
    let Ok(mut cam_pos) = cam.get_single_mut() else { return };
    let Ok(snake_pos) = snake_query.get_single() else { return };

    // let win = windows.get_primary().unwrap();

    // let hor_lim = (win.width() / TILE_SIZE * 0.6) as i32;
    let dx = snake_pos.x - cam_pos.x;
    if dx.abs() >= 10 { cam_pos.x += dx.signum(); }

    // let vert_lim = (win.height() / TILE_SIZE * 0.6) as i32;
    let dy = snake_pos.y - cam_pos.y;
    if dy.abs() >= 6 { cam_pos.y += dy.signum(); }
}

pub fn setup_world(
    mut com: Commands,
    world_gen: Res<WorldGen>,
) {
    for y in -1..=1 {
        for x in -1..=1 {
            world_gen.spawn_chunk(&mut com, Region(x, y));
        }
    }
}
