
use bevy::prelude::*;
use crate::prelude::*;

pub const TILE_SIZE: f32 = 26.0;

pub fn wall(pos: Position) -> impl Bundle {
    (
        Collidable,
        Tracked,
        tile(colors::white(), pos),
    )
}

pub fn food(pos: Position) -> impl Bundle {
    (
        Food,
        Tracked,
        tile(colors::blush(), pos),
    )
}

pub fn snake_head(body: &[Position]) -> impl Bundle {
    (
        Head::new(Segments(Vec::from(body))),
        tile(colors::munsell(), Position::new(0, 0)),
	)
}

pub fn snake_segment(pos: Position, i: usize) -> impl Bundle {
    (
        Segment(i),
        Collidable,
        Tracked,
        tile(colors::munsell(), pos),
    )
}

fn tile(color: Color, pos: Position) -> impl Bundle {
    (
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(pos.x as f32 * TILE_SIZE, pos.y as f32 * TILE_SIZE, 0.0),
            ..default()
        },
        pos
    )
}
