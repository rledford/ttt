use raylib::prelude::*;

pub struct Player {
    pub position: Vector2,
    pub aabb: Rectangle,
}

impl Player {
    pub fn collider(&self) -> Rectangle {
        Rectangle {
            x: self.position.x - self.aabb.width * 0.5,
            y: self.position.y - self.aabb.height * 0.5,
            width: self.aabb.width,
            height: self.aabb.height,
        }
    }
}
