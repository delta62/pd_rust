use crate::state::State;
use alloc::rc::Rc;
use core::cell::RefCell;
use playdate::{BitmapFlip, GameObject, Persistance, Sprite, SpriteBuilder, UpdateContext};

pub struct Explosion {
    state: Rc<RefCell<State>>,
    frame: usize,
    x: f32,
    y: f32,
}

impl Explosion {
    pub fn new(x: f32, y: f32, state: Rc<RefCell<State>>) -> Self {
        let frame = 0;
        Self { x, y, frame, state }
    }
}

impl GameObject for Explosion {
    fn init(&mut self, builder: SpriteBuilder) -> Sprite {
        let state = self.state.borrow();

        builder
            .image(state.explosion_images[0].clone(), BitmapFlip::Unflipped)
            .move_to(self.x, self.y)
            .z_index(2_000)
            .tag(1)
            .add()
            .build()
    }

    fn update(&mut self, ctx: UpdateContext) -> Persistance {
        let state = self.state.borrow();
        self.frame += 1;

        if self.frame > 7 {
            return Persistance::Destroy;
        }

        let frame_image = state.explosion_images[self.frame].clone();
        ctx.sprite.set_image(frame_image, BitmapFlip::Unflipped);

        Persistance::Keep
    }
}
