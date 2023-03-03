
use bevy::prelude::*;
use crate::prelude::{*, Direction};

#[derive(Component)]
pub struct Head {
    pub dir: Direction,
    pub body: Segments,
}

impl Head {
    fn new(body: Segments) -> Self {
        Self {
            dir: Direction::North,
            body,
        }
    }
}

#[derive(Default)]
pub struct Segments(pub Vec<Position>);

impl Segments {
    fn shift(&mut self, p: Position) {
        self.0.copy_within(1.., 0);
        let last = self.len() - 1;
        self.0[last] = p;
    }

    fn grow(&mut self, p: Position) {
        self.0.push(p);
    }

    fn len(&self) -> usize { self.0.len() }
}

#[derive(Component, Copy, Clone)]
pub struct Segment(usize);

pub fn setup(mut com: Commands) {
    let body = [
        Position::new(0, -3),
        Position::new(0, -2),
        Position::new(0, -1),
    ];

	com.spawn((
        Head::new(Segments(Vec::from(body))),

        tile(colors::munsell(), Position::new(0, 0)),
	));

    for (i, pos) in body.iter().enumerate() {
        spawn_segment(&mut com, *pos, i);
    }

}

fn spawn_segment(com: &mut Commands, pos: Position, i: usize) {
    com.spawn((
        Segment(i),

        Collidable,

        Tracked,

        tile(colors::munsell(), pos),
    ));
}

pub fn consume_input(
    mut head: Query<&mut Head>,
    mut input_buffer: ResMut<InputBuffer>,
) {
    let Ok(mut head) = head.get_single_mut() else { return };
    let Some(input) = input_buffer.shift() else { return };

    if head.dir != input.rev() {
        head.dir = input;
    }
}

pub fn premove(
    head_query: Query<(&Position, &Head)>,
    mut move_writer: EventWriter<MoveEvent>,
    mut eat_writer: EventWriter<EatEvent>,
    mut game_over_writer: EventWriter<GameOverEvent>,
    mut foods: Query<(&mut Visibility, Entity, &Position), (With<InActiveRegion>, With<Food>)>,
    mut collidables: Query<&Position, (With<InActiveRegion>, With<Collidable>)>,
) {
    let Ok((pos, head)) = head_query.get_single() else { return };

    let head_pos = *pos;
    let next_pos = head_pos + head.dir;

    for (mut vis, ent, &pos) in foods.iter_mut() {
        if pos == next_pos {
            *vis = Visibility::INVISIBLE;
            eat_writer.send(EatEvent(ent, pos));
            return;
        }
    }

    for &pos in collidables.iter_mut() {
        if pos == next_pos {
            game_over_writer.send(GameOverEvent);
            return;
        }
    }

    move_writer.send(MoveEvent(next_pos));
}

pub struct MoveEvent(Position);

pub fn move_(
    mut snake: Query<(&mut Position, &mut Head)>,
    mut move_reader: EventReader<MoveEvent>,
) {
    let Ok((mut snake_pos, mut head)) = snake.get_single_mut() else { return };
    for &MoveEvent(pos) in move_reader.iter() {
        
        let head_pos = *snake_pos;
        *snake_pos = pos;
        head.body.shift(head_pos);
    }
}

pub fn move_body(
    snake: Query<&Head>,
    mut body: Query<(&mut Position, &Segment)>,
) {
    let Ok(head) = snake.get_single() else { return };

    body.for_each_mut(|(mut pos, &Segment(i))| {
        *pos = head.body.0[i];
    });
}

pub struct EatEvent(Entity, Position);

pub fn eat(
    mut com: Commands,
    mut snake: Query<(&mut Position, &mut Head)>,
    mut eat_reader: EventReader<EatEvent>,
) {
    let Ok((mut snake_pos, mut head)) = snake.get_single_mut() else { return };

    for &EatEvent(ent, pos) in eat_reader.iter() {
        com.entity(ent).despawn();
        
        let head_pos = *snake_pos;
        *snake_pos = pos;
        head.body.grow(head_pos);

        spawn_segment(&mut com, head_pos, head.body.len() - 1);
    }
}

pub struct GameOverEvent;

pub fn game_over(
    mut com: Commands,
    snake: Query<(Entity, &Head)>,
    body: Query<Entity, With<Segment>>,
    mut game_over_reader: EventReader<GameOverEvent>,
) {
    for _ in game_over_reader.iter() {
        let Ok((ent, head)) = snake.get_single() else { return };
        let score = head.body.len();
    
        com.entity(ent).despawn();
        
        for ent in body.iter() {
            com.entity(ent).despawn();
        }
    
        println!("score: {score}");
    }
}
