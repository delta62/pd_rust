use crate::state::State;
use playdate::{
    rng, BitmapFlip, GameObject, Persistance, Playdate, Point, Sprite, SpriteBuilder, UpdateContext,
};

pub struct BackgroundPlane;

impl GameObject<State> for BackgroundPlane {
    fn init(&mut self, builder: SpriteBuilder<State>, pd: &mut Playdate<State>) -> Sprite<State> {
        let state = pd.data_mut();
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

    fn update(&mut self, ctx: UpdateContext<State>) -> Persistance {
        let state = ctx.pd.data_mut();
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
