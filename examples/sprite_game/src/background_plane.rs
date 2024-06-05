use crate::state::State;
use alloc::rc::Rc;
use core::cell::RefCell;
use playdate::{
    rng, BitmapFlip, GameObject, Persistance, Point, Sprite, SpriteBuilder, UpdateContext,
};

pub struct BackgroundPlane {
    state: Rc<RefCell<State>>,
}

impl BackgroundPlane {
    pub fn new(state: Rc<RefCell<State>>) -> Self {
        Self { state }
    }
}

impl GameObject for BackgroundPlane {
    fn init(&mut self, builder: SpriteBuilder) -> Sprite {
        let mut state = self.state.borrow_mut();
        let data = state.background_plane_image.data();

        state.background_plane_count += 1;

        let image = state.background_plane_image.clone();
        let bg_plane_height = state.bg_plane_height;

        builder
            .image(image, BitmapFlip::Unflipped)
            .move_to(
                ((rng::rand() % 400) - data.width / 2) as f32,
                -bg_plane_height,
            )
            .z_index(100)
            .add()
            .build()
    }

    fn update(&mut self, ctx: UpdateContext) -> Persistance {
        let mut state = self.state.borrow_mut();
        let Point { x, y } = ctx.sprite.position();
        let new_y = y + 2.0;

        if new_y > 400.0 + state.bg_plane_height {
            state.background_plane_count -= 1;
            return Persistance::Destroy;
        }

        ctx.sprite.move_to(x, new_y);
        Persistance::Keep
    }
}
