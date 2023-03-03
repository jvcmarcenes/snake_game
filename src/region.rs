
use bevy::prelude::*;
use rand::Rng;
use crate::prelude::*;


#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Region(pub i32, pub i32);

impl Region {
    pub fn bounds(self) -> (i32, i32) {
        (
            self.0 * 100 - 50,
            self.1 * 100 - 50,
        )
    }

    pub fn for_every_cell(self, mut f: impl FnMut(i32, i32)) {
        let (x_min, y_min) = self.bounds();

        for y in y_min..y_min + 100 {
            for x in x_min..x_min + 100 {
                f(x, y);
            }
        }
    }

    pub fn random_pos(self, rng: &mut impl Rng) -> Position {
        let (x_min, y_min) = self.bounds();
        let x = rng.gen_range(x_min..x_min + 100);
        let y = rng.gen_range(y_min..y_min + 100);
        Position::new(x, y)
    }

    pub fn contains(self, pos: Position) -> bool {
        pos.region() == self
    }

    pub fn in_edge(self, pos: Position) -> bool {
        let Region(x, y) = pos.region();
        
        [x - 1, x, x + 1].contains(&self.0)
        && [y - 1, y, y + 1].contains(&self.1)
        && !(x == self.0 && y == self.1)
    }
}

#[derive(Resource, Default)]
pub struct ActiveRegion(pub Region);

pub fn track_region(
    mut com: Commands,
    mut current_region: ResMut<ActiveRegion>,
    world_gen: Res<WorldGen>,
    snake: Query<&Position, (With<Head>, Changed<Position>)>,
) {
    let Ok(snake_pos) = snake.get_single() else { return };
    let snake_region = snake_pos.region();

    let dx = snake_region.0 - current_region.0.0;
    let dy = snake_region.1 - current_region.0.1;

    if dx != 0 || dy != 0 {
        current_region.0 = snake_region;
    }

    if dx != 0 {
        for dy in -1..=1 {
            world_gen.spawn_chunk(&mut com, Region(snake_region.0 + dx, snake_region.1 + dy));
        }

    } else if dy != 0 {
        for dx in -1..=1 {
            world_gen.spawn_chunk(&mut com, Region(snake_region.0 + dx, snake_region.1 + dy));
        }
    }
}

#[derive(Component)]
pub struct Tracked;

#[derive(Component)]
pub struct InActiveRegion;

#[derive(Component)]
pub struct InEdgeRegion;

pub fn update_active_region(
    mut com: Commands,
    active_region: Res<ActiveRegion>,
    untracked: Query<(Entity, &Position), (With<Tracked>, Without<InActiveRegion>, Without<InEdgeRegion>)>,
    in_active: Query<(Entity, &Position), (With<Tracked>, With<InActiveRegion>, Without<InEdgeRegion>)>,
    in_edge: Query<(Entity, &Position), (With<Tracked>, With<InEdgeRegion>, Without<InActiveRegion>)>,
) {
    untracked.for_each(|(ent, &pos)| {
        let mut ent = com.entity(ent);
        
        if active_region.0.contains(pos) {
            ent.insert(InActiveRegion);
        
        } else if active_region.0.in_edge(pos) {
            ent.insert(InEdgeRegion);
        
        } else {
            ent.despawn()
        }
    });

    in_edge.for_each(|(ent, &pos)| {
        if active_region.0.contains(pos) {
            com.entity(ent)
            .remove::<InEdgeRegion>()
            .insert(InActiveRegion);
        
        } else if !active_region.0.in_edge(pos) {
            com.entity(ent).despawn();
        }
    });

    in_active.for_each(|(ent, &pos)| {
        if active_region.0.in_edge(pos) {
            com.entity(ent)
            .remove::<InActiveRegion>()
            .insert(InEdgeRegion);
        }
    });
}
