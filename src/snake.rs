
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

//     fn grow(&mut self, p: Position) {
//         self.0.push(p);
//     }

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

        tile_sprite_bundle(colors::munsell()),

        Position::new(0, 0),
	));

    for (i, pos) in body.iter().enumerate() {
        spawn_segment(&mut com, *pos, i);
    }

}

fn spawn_segment(com: &mut Commands, pos: Position, i: usize) {
    com.spawn((
        Segment(i),
        
        pos,

        tile_sprite_bundle(colors::munsell()),
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

pub fn move_(
    mut head_query: Query<(&mut Position, &mut Head)>,
    mut body_query: Query<(&mut Position, &Segment), Without<Head>>,
) {
    let (mut pos, mut head) = head_query.single_mut();
    
    let head_pos = *pos;
    
    *pos = head_pos + head.dir;
    
    head.body.shift(head_pos);

    body_query.for_each_mut(|(mut pos, &Segment(i))| {
        *pos = head.body.0[i];
    });
}
