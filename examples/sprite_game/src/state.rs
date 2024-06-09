use alloc::{rc::Rc, vec::Vec};
use playdate::{cstr, format_string, Bitmap, Playdate, PlaydateState, Rect};

pub struct State {
    pub score: u32,
    pub background_plane_count: i32,
    pub max_background_planes: i32,
    pub enemy_count: i32,
    pub max_enemies: i32,
    pub player_bounds: Rect,

    pub bullet_height: f32,
    pub enemy_plane_height: f32,
    pub bg_plane_height: f32,

    pub explosion_images: Vec<Rc<Bitmap>>,
    pub bullet_image: Rc<Bitmap>,
    pub enemy_plane_image: Rc<Bitmap>,
    pub background_plane_image: Rc<Bitmap>,
}

impl PlaydateState for State {
    fn init(pd: &mut Playdate<()>) -> Self {
        let gfx = pd.graphics();
        let mut explosion_images = Vec::with_capacity(8);
        for i in 0..8 {
            let path = format_string!(cstr!("images/explosion/%d"), i + 1);
            let image = gfx.load_bitmap(&path).unwrap();
            explosion_images.push(Rc::new(image));
        }

        let enemy_plane_image = Rc::new(gfx.load_bitmap(cstr!("images/plane1")).unwrap());
        let background_plane_image = Rc::new(gfx.load_bitmap(cstr!("images/plane2")).unwrap());
        let bullet_image = Rc::new(gfx.load_bitmap(cstr!("images/doubleBullet")).unwrap());

        let bullet_height = bullet_image.data().height as f32;
        let enemy_plane_height = enemy_plane_image.data().height as f32;
        let bg_plane_height = background_plane_image.data().height as f32;

        Self {
            score: 0,
            background_plane_count: 1,
            max_background_planes: 10,
            player_bounds: Rect {
                width: 0.0,
                height: 0.0,
                x: 0.0,
                y: 0.0,
            },

            max_enemies: 10,
            enemy_count: 0,

            bg_plane_height,
            enemy_plane_height,
            bullet_height,

            bullet_image,
            enemy_plane_image,
            explosion_images,
            background_plane_image,
        }
    }
}
