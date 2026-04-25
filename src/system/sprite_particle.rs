use crate::graphics::sprite::Sprite;
use raylib::math::Vector2;

static DUMMY_SPRITE: Sprite = Sprite::new(0.0, 0.0, 0.0, 0.0);

pub struct SpriteParticle {
    pub sprite: &'static Sprite,
    pub position: Vector2,
    pub velocity: Vector2,
    pub acceleration: Vector2,
    pub lifetime: f32,
}

impl SpriteParticle {
    pub fn new_default() -> Self {
        SpriteParticle {
            sprite: &DUMMY_SPRITE,
            position: Default::default(),
            velocity: Default::default(),
            acceleration: Default::default(),
            lifetime: Default::default(),
        }
    }
}