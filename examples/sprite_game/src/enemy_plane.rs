use crate::{explosion::Explosion, state::State};
use alloc::{boxed::Box, rc::Rc};
use core::cell::RefCell;
use playdate::{
    rng, BitmapFlip, GameObject, Persistance, Point, Rect, Sprite, SpriteBuilder, UpdateContext,
};

pub struct EnemyPlane {
    state: Rc<RefCell<State>>,
    is_hit: bool,
}

impl EnemyPlane {
    pub fn new(state: Rc<RefCell<State>>) -> Self {
        let is_hit = false;
        Self { state, is_hit }
    }

    pub fn set_hit(&mut self) {
        self.is_hit = true;
    }
}

impl GameObject for EnemyPlane {
    fn init(&mut self, builder: SpriteBuilder) -> Sprite {
        let mut state = self.state.borrow_mut();

        let bitmap_data = state.enemy_plane_image.data();
        let x = ((rng::rand() % 400) - bitmap_data.width / 2) as f32;
        let y = -(rng::rand() as f32 % 30.0) - bitmap_data.height as f32;
        let collide_rect = Rect {
            x: 0.0,
            y: 0.0,
            width: bitmap_data.width as _,
            height: state.enemy_plane_height,
        };

        state.enemy_count += 1;
        let image = state.enemy_plane_image.clone();

        builder
            .image(image, BitmapFlip::Unflipped)
            .collide_rect(collide_rect)
            .move_to(x, y)
            .z_index(500)
            .build()
    }

    fn update(&mut self, ctx: UpdateContext) -> Persistance {
        if self.is_hit {
            {
                let mut state = self.state.borrow_mut();
                state.enemy_count -= 1;
            }

            let state = self.state.clone();
            let Point { x, y } = ctx.sprite.position();
            ctx.pd
                .sprite_mut()
                .new_sprite(Box::new(Explosion::new(x, y, state)));
            return Persistance::Destroy;
        }

        let mut state = self.state.borrow_mut();
        let position = ctx.sprite.position();
        let new_y = position.y + 4.0;

        if new_y > 400.0 + state.enemy_plane_height {
            state.enemy_count -= 1;
            return Persistance::Destroy;
        } else {
            ctx.sprite.move_to(position.x, new_y);
        }

        Persistance::Keep
    }
}
