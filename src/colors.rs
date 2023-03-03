
use bevy::prelude::Color;

pub fn white() -> Color { color(0xFDFDFF) }
pub fn onyx() -> Color { color(0x393D3F) }
pub fn munsell() -> Color { color(0x62929e) }
// pub fn blush() -> Color { color(0xDE4D86) }

fn color(c: u32) -> Color {
	let [_, r, g, b] = c.to_be_bytes();
	Color::rgb_u8(r, g, b)
}
