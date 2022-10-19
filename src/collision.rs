use std::ops::Range;
use crate::{Transform, Vec2};

pub trait Collidable {
    fn collides(&self, dimensions: Vec2, check_pos: &Transform, check_dimensions: Vec2) -> bool;
}

impl Collidable for Transform {
    fn collides(&self, dimensions: Vec2, check_pos: &Transform, check_dimensions: Vec2) -> bool {
        return ranges_overlap((self.translation.x - (dimensions.x/2.))..(self.translation.x + (dimensions.x/2.)),
                              (check_pos.translation.x - (check_dimensions.x/2.))..(check_pos.translation.x + (check_dimensions.x/2.))) &&
            ranges_overlap((self.translation.y - (dimensions.y/2.))..(self.translation.y + (dimensions.y/2.)),
                           (check_pos.translation.y - (check_dimensions.y/2.))..(check_pos.translation.y + (check_dimensions.y/2.)));
    }
}

fn ranges_overlap(r1: Range<f32>, r2: Range<f32>) -> bool {
    return r1.start <= r2.end && r1.end >= r2.start;
}