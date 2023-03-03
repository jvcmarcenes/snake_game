
use bevy::prelude::*;
use rand::{Rng, thread_rng};

use crate::prelude::{*, Direction};

#[derive(Resource)]
pub struct WorldGen;

impl WorldGen {
    pub fn spawn_chunk(&self, com: &mut Commands, region: Region) {
        let chunk = self.gen_chunk(region);

        for pos in chunk {
            spawn_wall(com, pos);
        }

        for _ in 0..15 {
            let pos = region.random_pos(&mut thread_rng());

            com.spawn((
                Food,

                Tracked,

                tile(colors::blush(), pos),
            ));
        }
    }

    fn gen_chunk(&self, region: Region) -> Vec<Position> {
        let mut positions = Vec::new();

        let rng = &mut thread_rng();

        let (x_min, y_min) = region.bounds();

        for x in x_min..x_min + 100 {
            if rng.gen::<f32>() < 0.8 {
                positions.push(Position::new(x, y_min));
            }
        }

        for y in y_min..y_min + 100 {
            if rng.gen::<f32>() < 0.8 {
                positions.push(Position::new(x_min, y));
            }
        }

        for _ in 0..40 {
            let pos = region.random_pos(&mut thread_rng());
            let mut wall_cluster = self.make_wall_cluster(pos);
            positions.append(&mut wall_cluster);
        }

        positions
        .into_iter()
        .filter(|pos| !pos.within_range(Position::new(0, 0), 10.0))
        .collect()
    }

    fn make_wall_cluster(&self, mut pos: Position) -> Vec<Position> {
        let rng = &mut thread_rng();

        let mut positions = Vec::from([pos]);
    
        let amount = rng.gen_range(0..3);
        let dir = rng.gen::<Direction>();
    
        for _ in 0..4 {
            pos = pos + dir;
            positions.push(pos);
        }
    
        for _ in 0..amount {
            pos = pos + dir;
            positions.push(pos);
    
            if rng.gen::<f32>() < 0.25 {
                positions.append(
                    &mut self.make_wall_cluster(pos)
                );
            }
        }
    
        positions
    }
}

pub fn spawn_wall(com: &mut Commands, pos: Position) {
    com.spawn((
        Tracked,

        Collidable,

        tile(colors::white(), pos),
    ));
}
