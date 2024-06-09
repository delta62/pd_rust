use crate::{enemy_plane::EnemyPlane, state::State};
use alloc::rc::Rc;
use playdate::{
    cstr, BitmapFlip, ButtonState, Buttons, GameObject, Persistance, Playdate, Point, Rect, Sprite,
    SpriteBuilder, UpdateContext,
};

pub struct Player;

impl GameObject<State> for Player {
    fn init(&mut self, builder: SpriteBuilder<State>, pd: &mut Playdate<State>) -> Sprite<State> {
        let player_image = pd.graphics().load_bitmap(cstr!("images/player")).unwrap();
        let player_image = Rc::new(player_image);
        let bitmap_data = player_image.data();

        builder
            .image(player_image, BitmapFlip::Unflipped)
            .move_to(200.0, 180.0)
            .z_index(1_000)
            .collide_rect(Rect {
                x: 5.0,
                y: 5.0,
                width: bitmap_data.width as f32 - 10.0,
                height: bitmap_data.height as f32 - 10.0,
            })
            .add()
            .build()
    }

    fn update(&mut self, ctx: UpdateContext<State>) -> Persistance {
        let UpdateContext { pd, sprite } = ctx;
        let Buttons { current, .. } = pd.system().button_state();

        let mut dx = 0.0;
        let mut dy = 0.0;

        if current.intersects(ButtonState::UP) {
            dy = -4.0
        } else if current.intersects(ButtonState::DOWN) {
            dy = 4.0
        }

        if current.intersects(ButtonState::LEFT) {
            dx = -4.0
        } else if current.intersects(ButtonState::RIGHT) {
            dx = 4.0
        }

        let Point { x, y } = sprite.position();
        sprite.move_with_collisions(x + dx, y + dy, |_sprite, collisions| {
            for collision in collisions {
                if let Some(enemy) = collision.other.downcast_mut::<EnemyPlane>() {
                    pd.data_mut().score -= 1;
                    enemy.set_hit();
                }
            }
        });

        Persistance::Keep
    }
}
