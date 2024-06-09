#![allow(internal_features)]
#![feature(core_intrinsics, lang_items)]
#![no_std]

extern crate alloc;

mod background;
mod background_plane;
mod bullet;
mod enemy_plane;
mod explosion;
mod player;
mod state;

use alloc::boxed::Box;
use background::Background;
use background_plane::BackgroundPlane;
use bullet::Bullet;
use core::cmp;
use enemy_plane::EnemyPlane;
use playdate::{rng, ButtonState, Buttons, FrameResult, Playdate};
use playdate_init::pd_app;
use player::Player;
use state::State;

#[pd_app(init = "new", update = "update", state = "State")]
struct Game;

impl Game {
    fn new(pd: &mut Playdate<State>) -> Self {
        pd.display_mut().set_refresh_rate(20.0);

        let now = pd.system().seconds_since_epoch().seconds;
        rng::set_seed(now);

        let player = Box::new(Player);
        let background = Box::new(Background::default());
        pd.sprite_mut().new_sprite(player);
        pd.sprite_mut().new_sprite(background);

        Self {}
    }

    fn update(&mut self, pd: &mut Playdate<State>) -> FrameResult {
        self.check_buttons(pd);
        self.check_crank(pd);
        self.spawn_enemy_if_needed(pd);
        self.spawn_background_plane_if_needed(pd);
        pd.sprite_mut().update_and_draw_sprites();

        FrameResult::Update
    }

    fn check_crank(&mut self, pd: &mut Playdate<State>) {
        let change = pd.system().crank_change();

        if change > 1.0 {
            pd.data_mut().max_enemies = cmp::min(pd.data().max_enemies, 119);
        } else if change < -1.0 {
            pd.data_mut().max_enemies = cmp::max(pd.data().max_enemies - 1, 0);
        }
    }

    fn spawn_background_plane_if_needed(&mut self, pd: &mut Playdate<State>) {
        let state = pd.data_mut();
        if state.background_plane_count >= state.max_background_planes {
            return;
        }

        let roll = rng::rand() % (120 / state.max_background_planes);

        if roll == 0 {
            let plane = Box::new(BackgroundPlane);
            pd.sprite_mut().new_sprite(plane)
        }
    }

    fn spawn_enemy_if_needed(&mut self, pd: &mut Playdate<State>) {
        let state = pd.data();
        let max_enemies = state.max_enemies;
        let enemy_count = state.enemy_count;

        if enemy_count >= max_enemies {
            return;
        }

        if rng::rand() % (120 / max_enemies) == 0 {
            let plane = Box::new(EnemyPlane::default());
            pd.sprite_mut().new_sprite(plane);
        }
    }

    fn check_buttons(&self, pd: &mut Playdate<State>) {
        let Buttons { pushed, .. } = pd.system().button_state();

        if pushed.contains(ButtonState::A) || pushed.contains(ButtonState::B) {
            let bullet = Box::new(Bullet);
            pd.sprite_mut().new_sprite(bullet);
        }
    }
}
