use crate::{
    bitmap::{Bitmap, BitmapFlip},
    gfx::{IntRect, Rect},
    libc,
};
use alloc::{rc::Rc, vec::Vec};
use core::{
    cell::RefCell,
    ffi::c_void,
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

pub struct SpriteAPI {
    _unused: [u8; 0],
}

impl SpriteAPI {
    pub(crate) fn new() -> Self {
        let _unused = Default::default();
        Self { _unused }
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

    pub fn query_at_point<'a>(&self, x: f32, y: f32, sprites: &'a [Sprite]) -> Vec<&'a Sprite> {
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

    pub fn query_in_rect<'a>(&self, rect: Rect, sprites: &'a [Sprite]) -> Vec<&'a Sprite> {
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

    pub fn query_along_line<F>(&self, p1: Point, p2: Point, sprites: &[Sprite]) -> Vec<&Sprite> {
        // Do not use querySpritesAlongLine as it uses raw sprite pointers
        todo!()
    }

    pub fn query_sprite_info_along_line(&self, p1: Point, p2: Point) -> Vec<SpriteQueryInfo> {
        todo!()
    }

    pub fn all_overlapping_sprites<F>(&self, sprites: &[Sprite]) -> Vec<&Sprite> {
        // Do not use allOverlappingSprites as it uses raw sprite pointers
        todo!()
    }
}

pub struct SpriteQueryInfo {
    pub ti1: f32,
    pub ti2: f32,
    pub entry_point: Point,
    pub exit_point: Point,
}

#[derive(Clone, Debug)]
enum SpriteStencil {
    Bitmap(Rc<Bitmap>),
    Pattern([u8; 8]),
}

#[derive(Default)]
struct SpriteData {
    displayed: bool,
    bitmap: Option<Rc<Bitmap>>,
    stencil: Option<SpriteStencil>,
}

pub struct Sprite {
    ptr: *mut LCDSprite,
    data: Rc<RefCell<SpriteData>>,
}

impl Sprite {
    pub fn new() -> Self {
        let data = Rc::new(RefCell::new(SpriteData::default()));
        let data_ptr = Rc::into_raw(data.clone()) as *mut c_void;

        let ptr = invoke_unsafe!(sprite.newSprite);
        invoke_unsafe!(sprite.setUserdata, ptr, data_ptr);

        Self { ptr, data }
    }

    pub(crate) fn from_ptr(ptr: *mut LCDSprite) -> Self {
        let data_ptr = invoke_unsafe!(sprite.getUserdata, ptr) as *const RefCell<SpriteData>;
        let owned_rc = unsafe { Rc::from_raw(data_ptr) };
        let data = owned_rc.clone();

        // Prevent the Sprite's stored Rc from being dropped as a result of this fn
        mem::forget(owned_rc);

        Self { ptr, data }
    }

    pub fn add(&mut self) {
        if self.data.borrow().displayed {
            return;
        }

        self.data.borrow_mut().displayed = true;
        invoke_unsafe!(sprite.addSprite, self.ptr)
    }

    pub fn remove(&mut self) {
        let mut data = self.data.borrow_mut();
        if !data.displayed {
            return;
        }

        data.displayed = false;
        invoke_unsafe!(sprite.removeSprite, self.ptr);
    }

    pub fn set_bounds(&mut self, bounds: Rect) {
        let _ = self.data.borrow_mut();
        invoke_unsafe!(sprite.setBounds, self.ptr, bounds)
    }

    pub fn bounds(&self) -> Rect {
        let _ = self.data.borrow();
        invoke_unsafe!(sprite.getBounds, self.ptr)
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        let _ = self.data.borrow_mut();
        invoke_unsafe!(sprite.moveTo, self.ptr, x, y)
    }

    pub fn move_by(&mut self, x: f32, y: f32) {
        let _ = self.data.borrow_mut();
        invoke_unsafe!(sprite.moveBy, self.ptr, x, y)
    }

    pub fn position(&self) -> Point {
        let _ = self.data.borrow();
        let mut x = 0.0;
        let mut y = 0.0;
        invoke_unsafe!(sprite.getPosition, self.ptr, &mut x, &mut y);

        Point { x, y }
    }

    pub fn center(&self) -> Point {
        let _ = self.data.borrow();
        let mut x = 0.0;
        let mut y = 0.0;
        invoke_unsafe!(sprite.getCenter, self.ptr, &mut x, &mut y);

        Point { x, y }
    }

    pub fn set_center(&mut self, x: f32, y: f32) {
        let _ = self.data.borrow_mut();
        invoke_unsafe!(sprite.setCenter, self.ptr, x, y)
    }

    pub fn set_image(&mut self, image: Rc<Bitmap>, flip: BitmapFlip) {
        let mut data = self.data.borrow_mut();
        let bmp = image.as_mut_ptr();
        data.bitmap = Some(image);
        invoke_unsafe!(sprite.setImage, self.ptr, bmp, flip as _);
    }

    pub fn clear_image(&mut self) {
        let mut data = self.data.borrow_mut();
        if data.bitmap.is_none() {
            return;
        }

        invoke_unsafe!(sprite.setImage, self.ptr, null_mut(), Default::default());
        data.bitmap = None;
    }

    pub fn image(&self) -> Option<Rc<Bitmap>> {
        let data = self.data.borrow();
        data.bitmap.as_ref().map(|bmp| bmp.clone())
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        let _ = self.data.borrow_mut();
        invoke_unsafe!(sprite.setSize, self.ptr, width, height)
    }

    pub fn set_z_index(&mut self, z_index: i16) {
        let _ = self.data.borrow_mut();
        invoke_unsafe!(sprite.setZIndex, self.ptr, z_index)
    }

    pub fn z_index(&self) -> i16 {
        let _ = self.data.borrow();
        invoke_unsafe!(sprite.getZIndex, self.ptr)
    }

    pub fn set_tag(&mut self, tag: impl Into<u8>) {
        let _ = self.data.borrow_mut();
        invoke_unsafe!(sprite.setTag, self.ptr, tag.into())
    }

    pub fn tag<R: From<u8>>(&self) -> R {
        let _ = self.data.borrow();
        invoke_unsafe!(sprite.getTag, self.ptr).into()
    }

    pub fn set_draw_mode(&mut self, mode: DrawMode) {
        let _ = self.data.borrow_mut();
        invoke_unsafe!(sprite.setDrawMode, self.ptr, mode as _)
    }

    pub fn set_image_flip(&mut self, flip: BitmapFlip) {
        let _ = self.data.borrow_mut();
        invoke_unsafe!(sprite.setImageFlip, self.ptr, flip as _)
    }

    pub fn image_flip(&self) -> BitmapFlip {
        let _ = self.data.borrow();
        let value = invoke_unsafe!(sprite.getImageFlip, self.ptr);
        BitmapFlip::try_from(value).unwrap()
    }

    pub fn set_stencil(&mut self, stencil: Rc<Bitmap>) {
        let mut data = self.data.borrow_mut();
        let stencil_ptr = stencil.as_mut_ptr();
        data.stencil = Some(SpriteStencil::Bitmap(stencil));
        invoke_unsafe!(sprite.setStencil, self.ptr, stencil_ptr)
    }

    pub fn set_stencil_image(&mut self, stencil: Rc<Bitmap>, tile: TileMode) {
        let mut data = self.data.borrow_mut();
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
        let mut data = self.data.borrow_mut();
        let pattern_ptr = pattern.as_mut_ptr();
        data.stencil = Some(SpriteStencil::Pattern(pattern));
        invoke_unsafe!(sprite.setStencilPattern, self.ptr, pattern_ptr)
    }

    pub fn clear_stencil(&mut self) {
        let mut data = self.data.borrow_mut();
        if data.stencil.is_none() {
            return;
        }

        invoke_unsafe!(sprite.clearStencil, self.ptr);
        data.stencil = None;
    }

    pub fn set_clip_rect(&mut self, clip_rect: IntRect) {
        let _ = self.data.borrow_mut();
        invoke_unsafe!(sprite.setClipRect, self.ptr, clip_rect)
    }

    pub fn clear_clip_rect(&mut self) {
        let _ = self.data.borrow_mut();
        invoke_unsafe!(sprite.clearClipRect, self.ptr)
    }

    pub fn set_updates_enabled(&mut self, enabled: UpdatesState) {
        let _ = self.data.borrow_mut();
        invoke_unsafe!(sprite.setUpdatesEnabled, self.ptr, enabled as _)
    }

    pub fn updates_enabled(&self) -> UpdatesState {
        let _ = self.data.borrow();
        let enabled = invoke_unsafe!(sprite.updatesEnabled, self.ptr);
        if enabled == 1 {
            UpdatesState::Enabled
        } else {
            UpdatesState::Disabled
        }
    }

    pub fn set_visible(&mut self, state: Visibility) {
        let _ = self.data.borrow_mut();
        invoke_unsafe!(sprite.setVisible, self.ptr, state as _)
    }

    pub fn visible(&mut self) -> Visibility {
        let _ = self.data.borrow();
        let visible = invoke_unsafe!(sprite.isVisible, self.ptr);
        if visible == 1 {
            Visibility::Visible
        } else {
            Visibility::Invisible
        }
    }

    pub fn set_opaque(&mut self, state: Opaqueness) {
        let _ = self.data.borrow_mut();
        invoke_unsafe!(sprite.setOpaque, self.ptr, state as _)
    }

    pub fn mark_dirty(&mut self) {
        let _ = self.data.borrow_mut();
        invoke_unsafe!(sprite.markDirty, self.ptr)
    }

    pub fn set_ignores_draw_offset(&mut self, offset_behavior: OffsetBehavior) {
        let _ = self.data.borrow_mut();
        invoke_unsafe!(sprite.setIgnoresDrawOffset, self.ptr, offset_behavior as _)
    }

    pub fn set_collisions_enabled(&mut self, enabled: CollisionState) {
        let _ = self.data.borrow_mut();
        invoke_unsafe!(sprite.setCollisionsEnabled, self.ptr, enabled as _)
    }

    pub fn collisions_enabled(&self) -> CollisionState {
        let _ = self.data.borrow();
        let enabled = invoke_unsafe!(sprite.collisionsEnabled, self.ptr);
        if enabled == 1 {
            CollisionState::Enabled
        } else {
            CollisionState::Disabled
        }
    }

    pub fn set_collide_rect(&mut self, rect: Rect) {
        let _ = self.data.borrow_mut();
        invoke_unsafe!(sprite.setCollideRect, self.ptr, rect)
    }

    pub fn collide_rect(&self) -> Rect {
        let _ = self.data.borrow();
        invoke_unsafe!(sprite.getCollideRect, self.ptr)
    }

    pub fn clear_collide_rect(&self) {
        let _ = self.data.borrow_mut();
        invoke_unsafe!(sprite.clearCollideRect, self.ptr)
    }

    pub fn check_collisions(&self, goal_x: f32, goal_y: f32) -> Vec<SpriteCollisionInfo> {
        let _ = self.data.borrow();
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

            let overlaps = if val.overlaps == 1 {
                SpriteOverlap::Overlapping
            } else {
                SpriteOverlap::TunneledThrough
            };

            vec.push(SpriteCollisionInfo {
                response_type: val.responseType.into(),
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

    pub fn move_with_collisions(&mut self, goal_x: f32, goal_y: f32) -> Vec<SpriteCollisionInfo> {
        let _ = self.data.borrow_mut();
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

            let overlaps = if val.overlaps == 1 {
                SpriteOverlap::Overlapping
            } else {
                SpriteOverlap::TunneledThrough
            };

            vec.push(SpriteCollisionInfo {
                response_type: val.responseType.into(),
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

    pub fn overlapping_sprites(&self, sprites: &[Sprite]) -> Vec<&Sprite> {
        let _ = self.data.borrow();
        // do not use overlappingSprites as it returns raw pointers
        todo!()
    }
}

pub struct SpriteCollisionInfo {
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

impl Drop for Sprite {
    fn drop(&mut self) {
        // Remove from display list if we're in it
        {
            let data = self.data.borrow_mut();
            if data.displayed {
                invoke_unsafe!(sprite.removeSprite, self.ptr);
            }
        }

        // Drop the last Rc to data which is stored in user data
        let data_ptr = invoke_unsafe!(sprite.getUserdata, self.ptr) as *const RefCell<SpriteData>;

        invoke_unsafe!(sprite.setUserdata, self.ptr, null_mut());
        unsafe { Rc::from_raw(data_ptr) };
        invoke_unsafe!(sprite.freeSprite, self.ptr)
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
