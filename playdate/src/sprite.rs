use crate::{
    bitmap::{Bitmap, BitmapFlip},
    gfx::{IntRect, Rect},
    libc, Playdate,
};
use alloc::{boxed::Box, rc::Rc, vec::Vec};
use core::{
    any::Any,
    ffi::c_void,
    marker::PhantomData,
    mem::{self, ManuallyDrop},
    ptr::null_mut,
};
use playdate_sys::{
    CollisionPoint, CollisionVector, LCDBitmapDrawMode_kDrawModeBlackTransparent,
    LCDBitmapDrawMode_kDrawModeCopy, LCDBitmapDrawMode_kDrawModeFillBlack,
    LCDBitmapDrawMode_kDrawModeFillWhite, LCDBitmapDrawMode_kDrawModeInverted,
    LCDBitmapDrawMode_kDrawModeNXOR, LCDBitmapDrawMode_kDrawModeWhiteTransparent,
    LCDBitmapDrawMode_kDrawModeXOR, LCDSprite, SpriteCollisionResponseType,
    SpriteCollisionResponseType_kCollisionTypeBounce,
    SpriteCollisionResponseType_kCollisionTypeFreeze,
    SpriteCollisionResponseType_kCollisionTypeOverlap,
    SpriteCollisionResponseType_kCollisionTypeSlide,
};

pub struct SpriteAPI<T>
where
    T: 'static,
{
    sprites: Vec<Sprite<T>>,
}

impl<T> SpriteAPI<T> {
    pub(crate) fn new() -> Self {
        let sprites = Vec::new();
        Self { sprites }
    }

    pub fn new_sprite(&mut self, mut game_object: Box<dyn GameObject<T>>) {
        let go_ptr = &mut *game_object as _;
        let builder = SpriteBuilder::new(go_ptr);
        let sprite = game_object.init(builder, &mut unsafe { Playdate::init() });
        self.sprites.push(sprite);

        mem::forget(game_object);
    }

    pub fn set_clip_rects_in_range(&mut self, rect: IntRect, start_z: i32, end_z: i32) {
        invoke_unsafe!(sprite.setClipRectsInRange, rect, start_z, end_z)
    }

    pub fn clear_clip_rects_in_range(&mut self, start_z: i32, end_z: i32) {
        invoke_unsafe!(sprite.clearClipRectsInRange, start_z, end_z)
    }

    pub fn set_always_redraw(&mut self, state: DrawTime) {
        invoke_unsafe!(sprite.setAlwaysRedraw, state as _)
    }

    pub fn add_dirty_rect(&mut self, rect: IntRect) {
        invoke_unsafe!(sprite.addDirtyRect, rect)
    }

    pub fn sprite_count(&self) -> i32 {
        invoke_unsafe!(sprite.getSpriteCount)
    }

    pub fn draw_sprites(&mut self) {
        invoke_unsafe!(sprite.drawSprites)
    }

    pub fn update_and_draw_sprites(&mut self) {
        invoke_unsafe!(sprite.updateAndDrawSprites)
    }

    pub fn reset_collision_world(&mut self) {
        invoke_unsafe!(sprite.resetCollisionWorld)
    }

    pub fn query_at_point<'a>(
        &self,
        x: f32,
        y: f32,
        sprites: &'a [Sprite<T>],
    ) -> Vec<&'a Sprite<T>> {
        // Do not use querySpritesAtPoint as it uses raw sprite pointers and we can't
        // make any guarantees here as to if they are mutably borrowed elsewhere
        sprites
            .iter()
            .filter(|sprite| {
                let bounds = sprite.bounds();
                let in_x = x >= bounds.x && x <= bounds.x + bounds.width;
                let in_y = y >= bounds.y && y <= bounds.y + bounds.height;
                in_x && in_y
            })
            .collect()
    }

    pub fn query_in_rect<'a>(&self, rect: Rect, sprites: &'a [Sprite<T>]) -> Vec<&'a Sprite<T>> {
        // Do not use querySpritesInRect as it uses raw sprite pointers and we can't
        // make any guarantees here as to if they are mutably borrowed elsewhere
        sprites
            .iter()
            .filter(|sprite| {
                let bounds = sprite.bounds();
                let in_x = bounds.x <= rect.x + rect.width && bounds.x + bounds.width >= rect.x;
                let in_y = bounds.y <= rect.y + rect.height && bounds.y + bounds.height >= rect.y;
                in_x && in_y
            })
            .collect()
    }

    // pub fn query_along_line<F>(&self, p1: Point, p2: Point, sprites: &[Sprite]) -> Vec<&Sprite> {
    //     // Do not use querySpritesAlongLine as it uses raw sprite pointers
    //     todo!()
    // }

    // pub fn query_sprite_info_along_line(&self, p1: Point, p2: Point) -> Vec<SpriteQueryInfo> {
    //     todo!()
    // }

    // pub fn all_overlapping_sprites<F>(&self, sprites: &[Sprite]) -> Vec<&Sprite> {
    //     // Do not use allOverlappingSprites as it uses raw sprite pointers
    //     todo!()
    // }
}

pub struct SpriteQueryInfo {
    pub ti1: f32,
    pub ti2: f32,
    pub entry_point: Point,
    pub exit_point: Point,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum SpriteStencil {
    Bitmap(Rc<Bitmap>),
    Pattern([u8; 8]),
}

struct SpriteData<T> {
    displayed: bool,
    bitmap: Option<Rc<Bitmap>>,
    stencil: Option<SpriteStencil>,
    game_object: *mut dyn GameObject<T>,
}

pub struct Sprite<T> {
    unused: PhantomData<T>,
    ptr: *mut LCDSprite,
}

extern "C" fn update_callback<T: 'static>(ptr: *mut LCDSprite) {
    let mut pd = unsafe { Playdate::init() };
    let mut sprite = ManuallyDrop::new(Sprite::from_ptr(ptr));

    let data_ptr = invoke_unsafe!(sprite.getUserdata, ptr) as *mut SpriteData<T>;
    let go_ptr = unsafe { &*data_ptr }.game_object;
    let go = unsafe { &mut *go_ptr };

    let ctx = UpdateContext {
        sprite: &mut sprite,
        pd: &mut pd,
    };

    // todo: use persistance
    go.update(ctx);
}

extern "C" fn collide_callback<T: 'static>(sprite: *mut LCDSprite, other: *mut LCDSprite) -> u32 {
    let mut pd = unsafe { Playdate::init() };
    let mut self_sprite = ManuallyDrop::new(Sprite::from_ptr(sprite));
    let mut other_sprite = ManuallyDrop::new(Sprite::from_ptr(other));

    let data_ptr = invoke_unsafe!(sprite.getUserdata, sprite) as *mut SpriteData<T>;
    let go_ptr = unsafe { &*data_ptr }.game_object;
    let go = unsafe { &mut *go_ptr };

    let data_ptr = invoke_unsafe!(sprite.getUserdata, other) as *mut SpriteData<T>;
    let go_ptr = unsafe { &*data_ptr }.game_object;
    let other = unsafe { &mut *go_ptr };

    let ctx = CollisionContext {
        self_sprite: &mut self_sprite,
        other,
        other_sprite: &mut other_sprite,
        pd: &mut pd,
    };

    go.collide(ctx) as u32
}

extern "C" fn draw_callback<T: 'static>(ptr: *mut LCDSprite, bounds: Rect, draw_rect: Rect) {
    let mut pd = unsafe { Playdate::init() };
    let sprite = ManuallyDrop::new(Sprite::from_ptr(ptr));

    let data_ptr = invoke_unsafe!(sprite.getUserdata, ptr) as *mut SpriteData<T>;
    let go_ptr = unsafe { &*data_ptr }.game_object;
    let go = unsafe { &mut *go_ptr };

    let ctx = DrawContext {
        sprite: &sprite,
        bounds: &bounds,
        draw_rect: &draw_rect,
        pd: &mut pd,
    };

    // todo: use persistance
    go.draw(ctx);
}

impl<T> Sprite<T>
where
    T: 'static,
{
    pub(crate) fn from_ptr(ptr: *mut LCDSprite) -> Self {
        Self {
            ptr,
            unused: Default::default(),
        }
    }

    pub(crate) fn new(game_object: *mut dyn GameObject<T>) -> Self {
        let ptr = invoke_unsafe!(sprite.newSprite);

        let data = Box::new(SpriteData {
            displayed: false,
            stencil: None,
            bitmap: None,
            game_object,
        });

        let data_ptr = Box::into_raw(data) as *mut c_void;
        invoke_unsafe!(sprite.setUserdata, ptr, data_ptr);

        invoke_unsafe!(sprite.setUpdateFunction, ptr, Some(update_callback::<T>));
        invoke_unsafe!(sprite.setDrawFunction, ptr, Some(draw_callback::<T>));
        invoke_unsafe!(
            sprite.setCollisionResponseFunction,
            ptr,
            Some(collide_callback::<T>)
        );

        Self::from_ptr(ptr)
    }

    fn data(&self) -> &SpriteData<T> {
        let data_ptr = invoke_unsafe!(sprite.getUserdata, self.ptr) as *mut SpriteData<T>;
        unsafe { &*data_ptr }
    }

    fn data_mut(&mut self) -> &mut SpriteData<T> {
        let data_ptr = invoke_unsafe!(sprite.getUserdata, self.ptr) as *mut SpriteData<T>;
        unsafe { &mut *data_ptr }
    }

    pub fn add(&mut self) {
        let data = self.data_mut();
        if data.displayed {
            return;
        }

        data.displayed = true;
        invoke_unsafe!(sprite.addSprite, self.ptr)
    }

    pub fn remove(&mut self) {
        let data = self.data_mut();
        if !data.displayed {
            return;
        }

        data.displayed = false;
        invoke_unsafe!(sprite.removeSprite, self.ptr);
    }

    pub fn set_bounds(&mut self, bounds: Rect) {
        invoke_unsafe!(sprite.setBounds, self.ptr, bounds)
    }

    pub fn bounds(&self) -> Rect {
        invoke_unsafe!(sprite.getBounds, self.ptr)
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        invoke_unsafe!(sprite.moveTo, self.ptr, x, y)
    }

    pub fn move_by(&mut self, x: f32, y: f32) {
        invoke_unsafe!(sprite.moveBy, self.ptr, x, y)
    }

    pub fn position(&self) -> Point {
        let mut x = 0.0;
        let mut y = 0.0;
        invoke_unsafe!(sprite.getPosition, self.ptr, &mut x, &mut y);

        Point { x, y }
    }

    pub fn center(&self) -> Point {
        let mut x = 0.0;
        let mut y = 0.0;
        invoke_unsafe!(sprite.getCenter, self.ptr, &mut x, &mut y);

        Point { x, y }
    }

    pub fn set_center(&mut self, x: f32, y: f32) {
        invoke_unsafe!(sprite.setCenter, self.ptr, x, y)
    }

    pub fn set_image(&mut self, image: Rc<Bitmap>, flip: BitmapFlip) {
        let data = self.data_mut();
        let bmp = image.as_mut_ptr();
        data.bitmap = Some(image);
        invoke_unsafe!(sprite.setImage, self.ptr, bmp, flip as _);
    }

    pub fn clear_image(&mut self) {
        let data = self.data_mut();
        if data.bitmap.is_none() {
            return;
        }

        data.bitmap = None;
        invoke_unsafe!(sprite.setImage, self.ptr, null_mut(), Default::default());
    }

    pub fn image(&self) -> Option<Rc<Bitmap>> {
        let data = self.data();
        data.bitmap.as_ref().map(|bmp| bmp.clone())
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        invoke_unsafe!(sprite.setSize, self.ptr, width, height)
    }

    pub fn set_z_index(&mut self, z_index: i16) {
        invoke_unsafe!(sprite.setZIndex, self.ptr, z_index)
    }

    pub fn z_index(&self) -> i16 {
        invoke_unsafe!(sprite.getZIndex, self.ptr)
    }

    pub fn set_tag(&mut self, tag: impl Into<u8>) {
        invoke_unsafe!(sprite.setTag, self.ptr, tag.into())
    }

    pub fn tag<R: From<u8>>(&self) -> R {
        invoke_unsafe!(sprite.getTag, self.ptr).into()
    }

    pub fn set_draw_mode(&mut self, mode: DrawMode) {
        invoke_unsafe!(sprite.setDrawMode, self.ptr, mode as _)
    }

    pub fn set_image_flip(&mut self, flip: BitmapFlip) {
        invoke_unsafe!(sprite.setImageFlip, self.ptr, flip as _)
    }

    pub fn image_flip(&self) -> BitmapFlip {
        let value = invoke_unsafe!(sprite.getImageFlip, self.ptr);
        BitmapFlip::try_from(value).unwrap()
    }

    pub fn set_stencil(&mut self, stencil: Rc<Bitmap>) {
        let data = self.data_mut();
        let stencil_ptr = stencil.as_mut_ptr();
        data.stencil = Some(SpriteStencil::Bitmap(stencil));
        invoke_unsafe!(sprite.setStencil, self.ptr, stencil_ptr)
    }

    pub fn set_stencil_image(&mut self, stencil: Rc<Bitmap>, tile: TileMode) {
        let data = self.data_mut();
        let is_tilable = stencil.data().width % 32 != 0;
        debug_assert!(
            tile == TileMode::NoTile || is_tilable,
            "Tiled stencils must have a width divisible by 32"
        );

        let stencil_ptr = stencil.as_mut_ptr();
        data.stencil = Some(SpriteStencil::Bitmap(stencil));
        invoke_unsafe!(sprite.setStencilImage, self.ptr, stencil_ptr, tile as _)
    }

    pub fn set_stencil_pattern(&mut self, mut pattern: [u8; 8]) {
        let data = self.data_mut();
        let pattern_ptr = pattern.as_mut_ptr();
        data.stencil = Some(SpriteStencil::Pattern(pattern));
        invoke_unsafe!(sprite.setStencilPattern, self.ptr, pattern_ptr)
    }

    pub fn clear_stencil(&mut self) {
        let data = self.data_mut();
        if data.stencil.is_none() {
            return;
        }

        data.stencil = None;
        invoke_unsafe!(sprite.clearStencil, self.ptr);
    }

    pub fn set_clip_rect(&mut self, clip_rect: IntRect) {
        invoke_unsafe!(sprite.setClipRect, self.ptr, clip_rect)
    }

    pub fn clear_clip_rect(&mut self) {
        invoke_unsafe!(sprite.clearClipRect, self.ptr)
    }

    pub fn set_updates_enabled(&mut self, enabled: UpdatesState) {
        invoke_unsafe!(sprite.setUpdatesEnabled, self.ptr, enabled as _)
    }

    pub fn updates_enabled(&self) -> UpdatesState {
        let enabled = invoke_unsafe!(sprite.updatesEnabled, self.ptr);
        if enabled == 1 {
            UpdatesState::Enabled
        } else {
            UpdatesState::Disabled
        }
    }

    pub fn set_visible(&mut self, state: Visibility) {
        invoke_unsafe!(sprite.setVisible, self.ptr, state as _)
    }

    pub fn visible(&mut self) -> Visibility {
        let visible = invoke_unsafe!(sprite.isVisible, self.ptr);
        if visible == 1 {
            Visibility::Visible
        } else {
            Visibility::Invisible
        }
    }

    pub fn set_opaque(&mut self, state: Opaqueness) {
        invoke_unsafe!(sprite.setOpaque, self.ptr, state as _)
    }

    pub fn mark_dirty(&mut self) {
        invoke_unsafe!(sprite.markDirty, self.ptr)
    }

    pub fn set_ignores_draw_offset(&mut self, offset_behavior: OffsetBehavior) {
        invoke_unsafe!(sprite.setIgnoresDrawOffset, self.ptr, offset_behavior as _)
    }

    pub fn set_collisions_enabled(&mut self, enabled: CollisionState) {
        invoke_unsafe!(sprite.setCollisionsEnabled, self.ptr, enabled as _)
    }

    pub fn collisions_enabled(&self) -> CollisionState {
        let enabled = invoke_unsafe!(sprite.collisionsEnabled, self.ptr);
        if enabled == 1 {
            CollisionState::Enabled
        } else {
            CollisionState::Disabled
        }
    }

    pub fn set_collide_rect(&mut self, rect: Rect) {
        invoke_unsafe!(sprite.setCollideRect, self.ptr, rect)
    }

    pub fn collide_rect(&self) -> Rect {
        invoke_unsafe!(sprite.getCollideRect, self.ptr)
    }

    pub fn clear_collide_rect(&self) {
        invoke_unsafe!(sprite.clearCollideRect, self.ptr)
    }

    pub fn check_collisions(&self, goal_x: f32, goal_y: f32) -> Vec<SpriteCollisionInfo<T>> {
        let mut len = 0;
        let mut actual_x = 0.0;
        let mut actual_y = 0.0;
        let ptr = invoke_unsafe!(
            sprite.checkCollisions,
            self.ptr,
            goal_x,
            goal_y,
            &mut actual_x,
            &mut actual_y,
            &mut len
        );

        let mut vec = Vec::with_capacity(len as _);
        let len = len as isize;

        for i in 0..len {
            let val = &unsafe { *ptr.offset(i) };
            let other_sprite = ManuallyDrop::new(Sprite::from_ptr(val.other));
            let overlaps = if val.overlaps == 1 {
                SpriteOverlap::Overlapping
            } else {
                SpriteOverlap::TunneledThrough
            };

            let other_ptr = other_sprite.data().game_object;
            let other: Box<dyn Any> = unsafe { Box::from_raw(other_ptr) };
            let other = ManuallyDrop::new(other);

            vec.push(SpriteCollisionInfo {
                response_type: val.responseType.into(),
                other,
                other_sprite,
                overlaps,
                ti: val.ti,
                moved: val.move_.into(),
                normal: val.normal.into(),
                touch: val.touch.into(),
                sprite_rect: val.spriteRect.into(),
                other_rect: val.otherRect.into(),
            });
        }

        unsafe { libc::free(ptr as _) };

        vec
    }

    pub fn move_with_collisions<F>(&mut self, goal_x: f32, goal_y: f32, mut f: F)
    where
        F: FnMut(&mut Sprite<T>, &mut [SpriteCollisionInfo<T>]),
    {
        let mut len = 0;
        let mut actual_x = 0.0;
        let mut actual_y = 0.0;
        let ptr = invoke_unsafe!(
            sprite.moveWithCollisions,
            self.ptr,
            goal_x,
            goal_y,
            &mut actual_x,
            &mut actual_y,
            &mut len
        );

        let mut vec = Vec::with_capacity(len as _);
        let len = len as isize;

        for i in 0..len {
            let val = &unsafe { *ptr.offset(i) };
            let mut other_sprite = ManuallyDrop::new(Sprite::from_ptr(val.other));
            let overlaps = if val.overlaps == 1 {
                SpriteOverlap::Overlapping
            } else {
                SpriteOverlap::TunneledThrough
            };

            let other_ptr = other_sprite.data_mut().game_object;
            let other: Box<dyn Any> = unsafe { Box::from_raw(other_ptr) };
            let other = ManuallyDrop::new(other);

            vec.push(SpriteCollisionInfo {
                response_type: val.responseType.into(),
                other,
                other_sprite,
                overlaps,
                ti: val.ti,
                moved: val.move_.into(),
                normal: val.normal.into(),
                touch: val.touch.into(),
                sprite_rect: val.spriteRect.into(),
                other_rect: val.otherRect.into(),
            });
        }

        unsafe { libc::free(ptr as _) };

        f(self, &mut vec)
    }

    // pub fn overlapping_sprites(&self, sprites: &[Sprite]) -> Vec<&Sprite> {
    //     let _ = self.data.borrow();
    //     // do not use overlappingSprites as it returns raw pointers
    //     todo!()
    // }
}

pub struct SpriteBuilder<T> {
    sprite: Sprite<T>,
}

impl<T> SpriteBuilder<T>
where
    T: 'static,
{
    pub(crate) fn new(game_object: *mut dyn GameObject<T>) -> Self {
        let sprite = Sprite::new(game_object);
        Self { sprite }
    }

    pub fn build(self) -> Sprite<T> {
        self.sprite
    }

    pub fn image(mut self, image: Rc<Bitmap>, flip: BitmapFlip) -> Self {
        self.sprite.set_image(image, flip);
        self
    }

    pub fn move_to(mut self, x: f32, y: f32) -> Self {
        self.sprite.move_to(x, y);
        self
    }

    pub fn z_index(mut self, z_index: i16) -> Self {
        self.sprite.set_z_index(z_index);
        self
    }

    pub fn collide_rect(mut self, rect: Rect) -> Self {
        self.sprite.set_collide_rect(rect);
        self
    }

    pub fn tag(mut self, tag: impl Into<u8>) -> Self {
        self.sprite.set_tag(tag.into());
        self
    }

    pub fn add(mut self) -> Self {
        self.sprite.add();
        self
    }

    pub fn bounds(mut self, bounds: Rect) -> Self {
        self.sprite.set_bounds(bounds);
        self
    }
}

pub trait GameObject<T>: Any {
    fn init(&mut self, builder: SpriteBuilder<T>, pd: &mut Playdate<T>) -> Sprite<T>;

    #[allow(unused_variables)]
    fn update(&mut self, ctx: UpdateContext<T>) -> Persistance {
        Persistance::Keep
    }

    #[allow(unused_variables)]
    fn collide(&mut self, ctx: CollisionContext<T>) -> CollisionResponse {
        CollisionResponse::Overlap
    }

    #[allow(unused_variables)]
    fn draw(&mut self, ctx: DrawContext<T>) {}

    fn destroy(&mut self) {}
}

pub enum Persistance {
    Keep,
    Destroy,
}

pub struct UpdateContext<'a, T>
where
    T: 'static,
{
    pub sprite: &'a mut Sprite<T>,
    pub pd: &'a mut Playdate<T>,
}

pub struct CollisionContext<'a, T>
where
    T: 'static,
{
    pub self_sprite: &'a mut Sprite<T>,
    pub other: &'a mut dyn GameObject<T>,
    pub other_sprite: &'a mut Sprite<T>,
    pub pd: &'a mut Playdate<T>,
}

pub struct DrawContext<'a, T>
where
    T: 'static,
{
    pub sprite: &'a Sprite<T>,
    pub bounds: &'a Rect,
    pub draw_rect: &'a Rect,
    pub pd: &'a mut Playdate<T>,
}

pub struct SpriteCollisionInfo<T> {
    pub other_sprite: ManuallyDrop<Sprite<T>>,
    pub other: ManuallyDrop<Box<dyn Any>>,
    pub response_type: CollisionResponse,
    pub overlaps: SpriteOverlap,
    pub ti: f32,
    pub moved: Point,
    pub normal: IntPoint,
    pub touch: Point,
    pub sprite_rect: Rect,
    pub other_rect: Rect,
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CollisionResponse {
    Slide = SpriteCollisionResponseType_kCollisionTypeSlide,
    Freeze = SpriteCollisionResponseType_kCollisionTypeFreeze,
    Overlap = SpriteCollisionResponseType_kCollisionTypeOverlap,
    Bounce = SpriteCollisionResponseType_kCollisionTypeBounce,
}

impl Default for CollisionResponse {
    fn default() -> Self {
        Self::Bounce
    }
}

impl From<SpriteCollisionResponseType> for CollisionResponse {
    #[allow(non_upper_case_globals)]
    fn from(value: SpriteCollisionResponseType) -> Self {
        match value {
            SpriteCollisionResponseType_kCollisionTypeSlide => Self::Slide,
            SpriteCollisionResponseType_kCollisionTypeFreeze => Self::Freeze,
            SpriteCollisionResponseType_kCollisionTypeOverlap => Self::Overlap,
            SpriteCollisionResponseType_kCollisionTypeBounce => Self::Bounce,
            _ => unreachable!(),
        }
    }
}

impl<T> Drop for Sprite<T> {
    fn drop(&mut self) {
        let data_ptr = invoke_unsafe!(sprite.getUserdata, self.ptr) as *mut SpriteData<T>;
        let data = unsafe { Box::from_raw(data_ptr) };

        // Remove from display list if we're in it
        if data.displayed {
            invoke_unsafe!(sprite.removeSprite, self.ptr);
        }

        // drop the game object
        drop(unsafe { Box::from_raw(data.game_object) });

        invoke_unsafe!(sprite.setUserdata, self.ptr, null_mut());
        invoke_unsafe!(sprite.freeSprite, self.ptr);

        // drop the user data
        // it would be done automatically anyway, but make it explicit to be safe
        drop(data);
    }
}

pub type Point = CollisionPoint;
pub type IntPoint = CollisionVector;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DrawMode {
    Copy = LCDBitmapDrawMode_kDrawModeCopy,
    WhiteTransparent = LCDBitmapDrawMode_kDrawModeWhiteTransparent,
    BlackTransparent = LCDBitmapDrawMode_kDrawModeBlackTransparent,
    FillWhite = LCDBitmapDrawMode_kDrawModeFillWhite,
    FillBlack = LCDBitmapDrawMode_kDrawModeFillBlack,
    Xor = LCDBitmapDrawMode_kDrawModeXOR,
    NotXor = LCDBitmapDrawMode_kDrawModeNXOR,
    Inverted = LCDBitmapDrawMode_kDrawModeInverted,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TileMode {
    NoTile = 0,
    Tile = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UpdatesState {
    Disabled = 0,
    Enabled = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Visibility {
    Invisible = 0,
    Visible = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Opaqueness {
    Translucent = 0,
    Opaque = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DrawTime {
    WhenNeeded = 0,
    Always = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OffsetBehavior {
    DrawOffset = 0,
    ScreenCoordinates = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollisionState {
    Disabled = 0,
    Enabled = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpriteOverlap {
    TunneledThrough = 0,
    Overlapping = 1,
}
