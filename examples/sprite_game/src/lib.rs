use playdate::{
    cstr, format_string, rng, Bitmap, BitmapFlip, ButtonState, FrameResult, Playdate, Rect, Sprite,
};
use playdate_init::pd_app;
use std::rc::Rc;

extern crate alloc;

enum Tag {
    Player,
    PlayerBullet,
    EnemyPlane,
}

impl From<u8> for Tag {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Player,
            2 => Self::PlayerBullet,
            3 => Self::EnemyPlane,
            _ => panic!(),
        }
    }
}

impl Into<u8> for Tag {
    fn into(self) -> u8 {
        match self {
            Self::Player => 1,
            Self::PlayerBullet => 2,
            Self::EnemyPlane => 3,
        }
    }
}

#[pd_app(init = "new", update = "update")]
struct Game {
    pd: Playdate,

    score: u32,
    max_background_planes: i32,
    background_plane_count: i32,
    bg_plane_height: f32,

    max_enemies: i32,
    enemy_count: i32,
    enemy_plane_height: f32,

    player: Rc<Sprite>,
    bullet_height: i32,

    bg_sprite: Sprite,
    bg_image: Bitmap,
    bg_h: i32,
    bg_y: i32,

    explosion_images: Vec<Rc<Bitmap>>,
    bullet_image: Rc<Bitmap>,
    enemy_plane_image: Rc<Bitmap>,
    background_plane_image: Rc<Bitmap>,
}

impl Game {
    fn new(mut pd: Playdate) -> Self {
        pd.display_mut().set_refresh_rate(20.0);

        let score = 0;
        let max_background_planes = 10;
        let bg_plane_height = 0.0;

        let max_enemies = 10;
        let enemy_count = 0;
        let enemy_plane_height = 0.0;

        let mut player: Sprite = Sprite::new();
        let player_image = Rc::new(Bitmap::load(cstr!("images/player")).unwrap());
        let data = player_image.data();
        player.set_image(player_image, BitmapFlip::Unflipped);

        let cr = Rect {
            x: 5.0,
            y: 5.0,
            width: data.width as f32 - 10.0,
            height: data.height as f32 - 10.0,
        };
        player.set_collide_rect(cr);
        player.move_to(200.0, 180.0);
        player.set_z_index(1_000);
        player.set_tag(Tag::Player as u8);
        player.add();

        let player = Rc::new(player);

        let background_plane_count = 1;
        let bullet_height = 0;

        let mut bg_sprite = Sprite::new();
        let bg_image = Bitmap::load(cstr!("images/background")).unwrap();
        let data = bg_image.data();
        let bg_h = data.height;
        let bg_y = 0;

        let bg_bounds = Rect {
            x: 0.0,
            y: 0.0,
            width: 400.0,
            height: 240.0,
        };

        bg_sprite.set_bounds(bg_bounds);
        bg_sprite.set_z_index(0);
        bg_sprite.add();

        let mut explosion_images = Vec::with_capacity(8);
        for i in 0..8 {
            let path = format_string!(cstr!("images/explosion/%d"), i + 1);
            let image = Bitmap::load(&path).unwrap();
            explosion_images.push(Rc::new(image));
        }

        let bullet_image = Rc::new(Bitmap::load(cstr!("images/doubleBullet")).unwrap());
        let enemy_plane_image = Rc::new(Bitmap::load(cstr!("images/plane1")).unwrap());
        let background_plane_image = Rc::new(Bitmap::load(cstr!("images/plane2")).unwrap());

        rng::set_seed(pd.system().seconds_since_epoch().seconds);

        Self {
            background_plane_image,
            background_plane_count,
            max_background_planes,
            max_enemies,
            explosion_images,
            bg_image,
            bg_sprite,
            player,
            bullet_image,
            bullet_height,
            pd,
            score,
            bg_h,
            bg_y,
            bg_plane_height,
            enemy_count,
            enemy_plane_image,
            enemy_plane_height,
        }
    }

    fn update(&mut self) -> FrameResult {
        self.check_buttons();
        self.check_crank();
        self.spawn_enemy_if_needed();
        self.spawn_background_plane_if_needed();
        self.pd.sprite_mut().update_and_draw_sprites();

        FrameResult::Update
    }

    fn check_crank(&mut self) {
        let change = self.pd.system().crank_change();

        if change > 1.0 {
            self.max_enemies += 1;
            if self.max_enemies > 119 {
                self.max_enemies = 119
            }
        } else if change < -1.0 {
            self.max_enemies -= 1;
            if self.max_enemies < 0 {
                self.max_enemies = 0
            }
        }
    }

    fn draw_background_sprite(&self) {
        self.bg_image.draw(0, self.bg_y, BitmapFlip::Unflipped);
        self.bg_image
            .draw(0, self.bg_y - self.bg_h, BitmapFlip::Unflipped);
    }

    fn update_explosion(&mut self, sprite: &mut Sprite) {
        let frame_number = sprite.tag::<u8>() + 1;

        if frame_number > 7 {
            sprite.remove();
        } else {
            sprite.set_tag(frame_number);
            let frame_image = self.explosion_images[frame_number as usize].clone();
            sprite.set_image(frame_image, BitmapFlip::Unflipped);
        }
    }

    fn create_explosion(&mut self, x: f32, y: f32) {
        let mut sprite = Sprite::new();
        sprite.set_image(self.explosion_images[0].clone(), BitmapFlip::Unflipped);
        sprite.move_to(x, y);
        sprite.set_z_index(2_000);
        sprite.set_tag(1);
        sprite.add();
    }

    fn create_enemy_plane(&mut self) -> Sprite {
        let mut plane = Sprite::new();
        let bitmap_data = self.enemy_plane_image.data();
        let cr = Rect {
            x: 0.0,
            y: 0.0,
            width: bitmap_data.width as _,
            height: self.enemy_plane_height,
        };
        //plane.set_update_fn(|sprite| self.update_enemy_plane(sprite));
        //plane.set_collision_response_fn(|_, _| CollisionResponse::Overlap);
        plane.set_image(self.enemy_plane_image.clone(), BitmapFlip::Unflipped);
        plane.set_collide_rect(cr);
        plane.set_z_index(500);
        plane.set_tag(Tag::EnemyPlane);
        plane.move_to(
            ((rng::rand() % 400) - bitmap_data.width / 2) as f32,
            -(rng::rand() as f32 % 30.0) - self.enemy_plane_height,
        );

        plane.add();

        self.enemy_count += 1;

        plane
    }

    fn update_enemy_plane(&mut self, sprite: &mut Sprite) {
        let position = sprite.position();
        let new_y = position.y + 4.0;

        if new_y > 400.0 + self.enemy_plane_height {
            sprite.remove();
            self.enemy_count -= 1;
        } else {
            sprite.move_to(position.x, new_y);
        }
    }

    fn update_background_sprite(&mut self) {
        self.bg_y += 1;
        if self.bg_y > self.bg_h {
            self.bg_y = 0;
        }

        self.bg_sprite.mark_dirty();
    }

    fn update_background_plane(&mut self, sprite: &mut Sprite) {
        let position = sprite.position();
        let new_y = position.y + 2.0;

        if new_y > 400.0 + self.bg_plane_height {
            sprite.remove();
            self.background_plane_count -= 1;
        } else {
            sprite.move_to(position.x, new_y);
        }
    }

    fn create_background_plane(&mut self) -> Sprite {
        let mut plane = Sprite::new();
        let data = self.background_plane_image.data();
        plane.set_image(self.background_plane_image.clone(), BitmapFlip::Unflipped);
        plane.move_to(
            ((rng::rand() % 400) - data.width / 2) as f32,
            -self.bg_plane_height,
        );
        plane.set_z_index(100);
        plane.add();

        self.background_plane_count += 1;

        plane
    }

    fn spawn_background_plane_if_needed(&mut self) {
        if self.background_plane_count >= self.max_background_planes {
            return;
        }

        if rng::rand() % (120 / self.max_background_planes) == 0 {
            self.create_background_plane();
        }
    }

    fn destroy_enemy_plane(&mut self, plane: &mut Sprite) {
        let position = plane.position();
        self.create_explosion(position.x, position.y);
        plane.remove();
        self.enemy_count -= 1;
    }

    fn spawn_enemy_if_needed(&mut self) {
        if self.enemy_count >= self.max_enemies {
            return;
        }

        if rng::rand() % (120 / self.max_enemies) == 0 {
            self.create_enemy_plane();
        }
    }

    fn check_buttons(&self) {
        let state = self.pd.system().button_state();
        let pushed = state.pushed;

        if pushed.contains(ButtonState::A) || pushed.contains(ButtonState::B) {
            self.player_fire();
        }
    }

    fn player_fire(&self) {
        let mut bullet = Sprite::new();
        bullet.set_update_fn(move |bullet| {
            let position = bullet.position();
            let new_y = position.y - 20;

            if new_y < -self.bullet_height {
                // todo: drop bullet
                return;
            }

            let mut hit = false;

            let collision_info = bullet.move_with_collisions(position.x, new_y);
            collision_info.into_iter().for_each(|collision| {
                let other = collision.other;
                if other.tag() == Tag::EnemyPlane {
                    self.score += 1;
                    hit = true;
                    // self.pd
                    //     .system()
                    //     .log_to_console(cstr!("Score: %d"), self.score);
                }
            });

            if hit {
                // delete bullet
            }
        });

        let bitmap_data = self.bullet_image.data();

        bullet.set_image(self.bullet_image, BitmapFlip::Unflipped);
        let cr = Rect {
            x: 0.0,
            y: 0.0,
            // todo
            width: bitmap_data.width as f32,
            height: self.bullet_height as f32,
        };

        bullet.set_collide_rect(cr);
        // collision response fn
        let bounds = self.player.bounds();
        bullet.move_to(bounds.x + bounds.width / 2.0, bounds.y);
        bullet.set_z_index(999);
        bullet.set_tag(Tag::PlayerBullet);
        bullet.add();
    }
}
