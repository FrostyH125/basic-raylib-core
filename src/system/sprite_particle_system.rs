use crate::graphics::sprite::Sprite;
use raylib::{
    color::Color, math::{Rectangle, Vector2}, prelude::RaylibDrawHandle, texture::Texture2D,
};

use crate::graphics::sprite_particle::SpriteParticle;

pub struct SpriteParticleSystem {
    particles: Vec<SpriteParticle>,
    max_particles: usize,
    pub n_particles: usize,
}

impl SpriteParticleSystem {
    pub fn new(max_particles: usize) -> Self {
        let mut particles: Vec<SpriteParticle> = Vec::new();

        for _ in 0..max_particles {
            //initialize vector with dummy particles
            let p = SpriteParticle::new_default();
            particles.push(p);
        }

        SpriteParticleSystem { particles, n_particles: 0, max_particles }
    }

    pub fn emit(
        &mut self,
        sprite: &'static Sprite,
        position: Vector2,
        velocity: Vector2,
        acceleration: Vector2,
        lifetime: f32,
    ) {
        if self.n_particles >= self.max_particles {
            return;
        }

        let sprite_half_width = sprite.src_rect.width / 2.0;
        let sprite_half_height = sprite.src_rect.height / 2.0;

        // this is to make it spawn the particle with the center of the particle being the passed position
        // i did this so it would be easier to make things look more uniform, for example, now if particles
        // are emitted randomly along a straight edge, it'll not require any width or height math to account
        // for the width or height of the particle so that one side's particles doesn't extend further than the
        // other side
        let real_emit_pos = Vector2::new(position.x - sprite_half_width, position.y - sprite_half_height);

        self.particles[self.n_particles].sprite = sprite;
        self.particles[self.n_particles].position = real_emit_pos;
        self.particles[self.n_particles].origin = Vector2::zero();
        self.particles[self.n_particles].velocity = velocity;
        self.particles[self.n_particles].acceleration = acceleration;
        self.particles[self.n_particles].rotation_speed = 0.0;
        self.particles[self.n_particles].current_rotation = 0.0;
        self.particles[self.n_particles].lifetime = lifetime;

        self.n_particles += 1;
    }

    /// preserve_original_pos will prevent the object from shifting when the origin is set.
    /// use it if you need things to be at pixel perfect, 
    /// otherwise if you'd rather the object be centered
    /// like is automatic in the regular emit(), then leave it false
    pub fn emit_ex(
        &mut self,
        sprite: &'static Sprite,
        position: Vector2,
        velocity: Vector2,
        acceleration: Vector2,
        rotation_speed: f32,
        starting_rotation: f32,
        lifetime: f32,
        preserve_original_pos: bool,
    ) {
        if self.n_particles >= self.max_particles {
            return;
        }

        // clamp to 1.0 if its smaller, half pixel widths & heights are very hard to reason about
        let sprite_half_width = (sprite.src_rect.width / 2.0).max(1.0);
        let sprite_half_height = (sprite.src_rect.height / 2.0).max(1.0);
        let origin = Vector2::new(sprite_half_width, sprite_half_height);

        // remove the shift done by changing the origin
        let real_emit_pos = match preserve_original_pos {
            true => {
                position
                    + Vector2 {
                        x: sprite_half_width,
                        y: sprite_half_height,
                    }
            }
            false => position,
        };

        self.particles[self.n_particles].sprite = sprite;
        self.particles[self.n_particles].position = real_emit_pos;
        self.particles[self.n_particles].origin = origin;
        self.particles[self.n_particles].velocity = velocity;
        self.particles[self.n_particles].acceleration = acceleration;
        self.particles[self.n_particles].rotation_speed = rotation_speed;
        self.particles[self.n_particles].current_rotation = starting_rotation;
        self.particles[self.n_particles].lifetime = lifetime;

        self.n_particles += 1;
    }

    pub fn update(&mut self, dt: f32) {
        let mut i = 0;

        while i < self.n_particles {
            let particle = &mut self.particles[i];

            particle.lifetime -= dt;

            if particle.lifetime <= 0.0 {
                // move this particle to the absolute edge of the array
                // it doesnt matter what was there before, as it wasnt active,
                // and order doesnt matter
                // this also moves the last active particle into the inner array somewhere
                // which keeps it packed
                //
                // this is basic but just so i can keep things straight
                // if theres 10 particles, we're updating 0..=9
                // this means that swapping n_particles - 1 will swap with the idx 9 (the last one)
                // then decrementing n_particles means we now update 0..=8, which is great
                // because idx 9 is where it stops being active particles
                // (i am extremely dense and need this reminder)
                self.particles.swap(self.n_particles - 1, i);

                // THEN decrement n_particles as to not have it update the newly
                // expired particle
                self.n_particles -= 1;

                continue;
            }

            particle.position += particle.velocity * dt;
            particle.velocity += particle.acceleration * dt;
            particle.current_rotation += particle.rotation_speed * dt;

            i += 1;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        for i in 0..self.n_particles {
            let particle = &self.particles[i];

            particle.sprite.draw_pro(
                d,
                Rectangle::new(
                    particle.position.x,
                    particle.position.y,
                    particle.sprite.src_rect.width,
                    particle.sprite.src_rect.height,
                ),
                particle.origin,
                particle.current_rotation,
                texture,
                Color::WHITE
            );
        }
    }
}

#[test]
fn test_ps_emit_and_expire() {
    let mut ps = SpriteParticleSystem::new(1000);
    static NOTHING_SPRITE: Sprite = Sprite::new(0, 0, 0, 0);

    for _ in 0..=1500 {
        ps.emit(&NOTHING_SPRITE, Vector2::zero(), Vector2::zero(), Vector2::zero(), 1.0);
    }

    assert!(ps.n_particles == 1000);

    let dt = 1.1;

    ps.update(dt);

    assert!(ps.n_particles == 0);
}
