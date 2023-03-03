
use std::ops::Add;
use bevy::prelude::*;
use rand::{prelude::Distribution, distributions::Standard};
use crate::prelude::{*, Direction};

#[derive(Component, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_translation(trans: Vec3) -> Self {
        Self::new(
            (trans.x / TILE_SIZE) as i32,
            (trans.y / TILE_SIZE) as i32,
        )
    }

    pub fn distance(self, other: Position) -> f32 {
        f32::sqrt((
            self.x * other.x
            + self.y * other.y
        ) as f32)
    }

    pub fn as_vec2(self) -> Vec2 {
        Vec2::new(
            self.x as f32 * TILE_SIZE, 
            self.y as f32 * TILE_SIZE,
        )
    }

    pub fn within_range(self, other: Position, reach: f32) -> bool {
        self.as_vec2().distance(other.as_vec2()) <= reach
    }

    pub fn region(self) -> Region {
        Region(
            (self.x + 50) / 100,
            (self.y + 50) / 100,
        )
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, dir: Direction) -> Self::Output {
        use Direction::*;

        match dir {
            North => Position::new(self.x    , self.y + 1),
            East  => Position::new(self.x + 1, self.y    ),
            South => Position::new(self.x    , self.y - 1),
            West  => Position::new(self.x - 1, self.y    ),
        }
    }
}

impl Distribution<Position> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Position {
        let (x, y) = rng.gen();
        Position::new(x, y)
    }
}

pub fn update_transform(mut query: Query<(&mut Transform, &Position), Changed<Position>>) {
    query.for_each_mut(|(mut transform, pos)| {

        let Transform { translation: Vec3 { x, y, .. }, .. } = &mut *transform;

        if (*x / TILE_SIZE) as i32 != pos.x {
            *x = TILE_SIZE * pos.x as f32;
        }

        if (*y / TILE_SIZE) as i32 != pos.y {
            *y = TILE_SIZE * pos.y as f32;
        }
    });
}
