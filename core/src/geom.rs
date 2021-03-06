use cgmath::{Vector2};
use position::{MapPos};

pub const HEX_EX_RADIUS: f32 = 1.4;

// (pow(1.0, 2) - pow(0.5, 2)).sqrt()
pub const HEX_IN_RADIUS: f32 = 0.866025403784 * HEX_EX_RADIUS;

pub fn map_pos_to_world_pos(i: MapPos) -> Vector2<f32> {
    let v = Vector2 {
        x: (i.v.x as f32) * HEX_IN_RADIUS * 2.0,
        y: (i.v.y as f32) * HEX_EX_RADIUS * 1.5,
    };
    if i.v.y % 2 == 0 {
        Vector2{x: v.x + HEX_IN_RADIUS, y: v.y}
    } else {
        v
    }
}
