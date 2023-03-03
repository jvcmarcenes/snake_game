
use bevy::prelude::*;
use crate::prelude::Direction;

#[derive(Resource)]
pub struct InputBuffer(Option<Direction>, Option<Direction>);

impl InputBuffer {
    pub fn new() -> Self {
        Self(None, None)
    }

    pub fn push(&mut self, dir: Direction) {
        if self.0.is_none() {
            self.0.replace(dir);
        } else if self.1.is_none() {
            self.1.replace(dir);
        }
    }

    pub fn shift(&mut self) -> Option<Direction> {
        let dir = self.0;
        (self.0, self.1) = (self.1, None);
        dir
    }
}

pub fn gather(
    input: Res<Input<KeyCode>>,
    mut input_buffer: ResMut<InputBuffer>,
) {
    let pressed = input.get_just_pressed().collect::<Vec<_>>();
    match pressed[..] {
        [&key] => if let Ok(dir) = key.try_into() {
            input_buffer.push(dir);
        },
        _ => (),
    }
}
