use crate::state::State;
use playdate::{
    BitmapFlip, GameObject, Persistance, Playdate, Sprite, SpriteBuilder, UpdateContext,
};

pub struct Explosion {
    frame: usize,
    x: f32,
    y: f32,
}

impl Explosion {
    pub fn new(x: f32, y: f32) -> Self {
        let frame = 0;
        Self { x, y, frame }
    }
}

impl GameObject<State> for Explosion {
    fn init(&mut self, builder: SpriteBuilder<State>, pd: &mut Playdate<State>) -> Sprite<State> {
        let state = pd.data();

        builder
            .image(state.explosion_images[0].clone(), BitmapFlip::Unflipped)
            .move_to(self.x, self.y)
            .z_index(2_000)
            .tag(1)
            .add()
            .build()
    }

    fn update(&mut self, ctx: UpdateContext<State>) -> Persistance {
        let state = ctx.pd.data();
        self.frame += 1;

        if self.frame > 7 {
            return Persistance::Destroy;
        }

        let frame_image = state.explosion_images[self.frame].clone();
        ctx.sprite.set_image(frame_image, BitmapFlip::Unflipped);

        Persistance::Keep
    }
}
