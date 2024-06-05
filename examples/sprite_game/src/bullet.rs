use crate::{enemy_plane::EnemyPlane, state::State};
use alloc::rc::Rc;
use core::cell::RefCell;
use playdate::{
    BitmapData, BitmapFlip, GameObject, Persistance, Point, Rect, Sprite, SpriteBuilder,
    UpdateContext,
};

pub struct Bullet {
    state: Rc<RefCell<State>>,
}

impl Bullet {
    pub fn new(state: Rc<RefCell<State>>) -> Self {
        Self { state }
    }
}

impl GameObject for Bullet {
    fn init(&mut self, builder: SpriteBuilder) -> Sprite {
        let state = self.state.borrow();

        let BitmapData { width, .. } = state.bullet_image.data();

        let player_bounds = &state.player_bounds;
        let collide_rect = Rect {
            x: player_bounds.x,
            y: player_bounds.y,
            width: width as f32,
            height: state.bullet_height,
        };

        let image = state.bullet_image.clone();

        builder
            .image(image, BitmapFlip::Unflipped)
            .collide_rect(collide_rect)
            .move_to(player_bounds.x + player_bounds.width / 2.0, player_bounds.y)
            .z_index(999)
            .add()
            .build()
    }

    fn update(&mut self, ctx: UpdateContext) -> Persistance {
        let bullet_height = self.state.borrow().bullet_height;
        let Point { x, y } = ctx.sprite.position();
        let new_y = y - 20.0;

        if new_y < -bullet_height {
            return Persistance::Destroy;
        }

        let mut hit = false;
        ctx.sprite
            .move_with_collisions(x, new_y, |_sprite, collisions| {
                for collision in collisions {
                    if let Some(enemy) = collision.other.downcast_mut::<EnemyPlane>() {
                        let mut state = self.state.borrow_mut();
                        hit = true;
                        state.score += 1;
                        enemy.set_hit();
                    }
                }
            });

        if hit {
            Persistance::Destroy
        } else {
            Persistance::Keep
        }
    }
}
