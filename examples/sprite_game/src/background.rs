use crate::state::State;
use alloc::rc::Rc;
use playdate::{
    cstr, BitmapFlip, DrawContext, GameObject, Persistance, Playdate, Rect, Sprite, SpriteBuilder,
    UpdateContext,
};

#[derive(Default)]
pub struct Background {
    height: i32,
    y: i32,
}

impl GameObject<State> for Background {
    fn init(&mut self, builder: SpriteBuilder<State>, pd: &mut Playdate<State>) -> Sprite<State> {
        let bg_image = pd
            .graphics()
            .load_bitmap(cstr!("images/background"))
            .unwrap();
        let bg_image = Rc::new(bg_image);
        let bitmap_data = bg_image.data();

        self.height = bitmap_data.height;

        builder
            .bounds(Rect {
                x: 0.0,
                y: 0.0,
                width: 400.0,
                height: 240.0,
            })
            .image(bg_image, BitmapFlip::Unflipped)
            .z_index(0)
            .add()
            .build()
    }

    fn draw(&mut self, ctx: DrawContext<State>) {
        let bg_image = ctx.sprite.image().unwrap();
        bg_image.draw(0, self.y, BitmapFlip::Unflipped);
        bg_image.draw(0, self.y - self.height, BitmapFlip::Unflipped);
    }

    fn update(&mut self, ctx: UpdateContext<State>) -> Persistance {
        self.y += 1;
        if self.y > self.height {
            self.y = 0;
        }

        ctx.sprite.mark_dirty();

        Persistance::Keep
    }
}
