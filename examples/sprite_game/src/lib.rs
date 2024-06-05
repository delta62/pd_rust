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

use alloc::{boxed::Box, rc::Rc};
use background::Background;
use background_plane::BackgroundPlane;
use bullet::Bullet;
use core::{cell::RefCell, cmp};
use enemy_plane::EnemyPlane;
use playdate::{rng, ButtonState, Buttons, FrameResult, Playdate};
use playdate_init::pd_app;
use player::Player;
use state::State;

#[pd_app(init = "new", update = "update")]
struct Game {
    state: Rc<RefCell<State>>,
}

impl Game {
    fn new(pd: &mut Playdate) -> Self {
        pd.display_mut().set_refresh_rate(20.0);

        let now = pd.system().seconds_since_epoch().seconds;
        rng::set_seed(now);

        let state = State::new(pd);
        let state = Rc::new(RefCell::new(state));

        let player = Box::new(Player::new(state.clone()));
        let background = Box::new(Background::default());
        pd.sprite_mut().new_sprite(player);
        pd.sprite_mut().new_sprite(background);

        Self { state }
    }

    fn update(&mut self, pd: &mut Playdate) -> FrameResult {
        self.check_buttons(pd);
        self.check_crank(pd);
        self.spawn_enemy_if_needed(pd);
        self.spawn_background_plane_if_needed(pd);
        pd.sprite_mut().update_and_draw_sprites();

        FrameResult::Update
    }

    fn check_crank(&mut self, pd: &mut Playdate) {
        let change = pd.system().crank_change();
        let mut state = self.state.borrow_mut();

        if change > 1.0 {
            state.max_enemies = cmp::min(state.max_enemies, 119);
        } else if change < -1.0 {
            state.max_enemies = cmp::max(state.max_enemies - 1, 0);
        }
    }

    fn spawn_background_plane_if_needed(&mut self, pd: &mut Playdate) {
        let state = self.state.borrow();
        if state.background_plane_count >= state.max_background_planes {
            return;
        }

        let roll = rng::rand() % (120 / state.max_background_planes);

        if roll == 0 {
            let plane = Box::new(BackgroundPlane::new(self.state.clone()));
            pd.sprite_mut().new_sprite(plane)
        }
    }

    fn spawn_enemy_if_needed(&mut self, pd: &mut Playdate) {
        let state = self.state.borrow();
        let max_enemies = state.max_enemies;
        let enemy_count = state.enemy_count;

        if enemy_count >= max_enemies {
            return;
        }

        if rng::rand() % (120 / max_enemies) == 0 {
            let plane = Box::new(EnemyPlane::new(self.state.clone()));
            pd.sprite_mut().new_sprite(plane);
        }
    }

    fn check_buttons(&self, pd: &mut Playdate) {
        let Buttons { pushed, .. } = pd.system().button_state();

        if pushed.contains(ButtonState::A) || pushed.contains(ButtonState::B) {
            let bullet = Box::new(Bullet::new(self.state.clone()));
            pd.sprite_mut().new_sprite(bullet);
        }
    }
}
