use crate::display::FlipState;
use alloc::vec::Vec;
use playdate_alloc::libc::free;
use playdate_sys::{
    playdate_sprite, CollisionPoint, CollisionVector, LCDBitmap,
    LCDBitmapDrawMode_kDrawModeBlackTransparent, LCDBitmapDrawMode_kDrawModeCopy,
    LCDBitmapDrawMode_kDrawModeFillBlack, LCDBitmapDrawMode_kDrawModeFillWhite,
    LCDBitmapDrawMode_kDrawModeInverted, LCDBitmapDrawMode_kDrawModeNXOR,
    LCDBitmapDrawMode_kDrawModeWhiteTransparent, LCDBitmapDrawMode_kDrawModeXOR, LCDSprite, PDRect,
    SpriteCollisionResponseType, SpriteCollisionResponseType_kCollisionTypeBounce,
    SpriteCollisionResponseType_kCollisionTypeFreeze,
    SpriteCollisionResponseType_kCollisionTypeOverlap,
    SpriteCollisionResponseType_kCollisionTypeSlide,
};

pub struct PlaydateSprite {
    sprite_api: &'static playdate_sprite,
}

impl PlaydateSprite {
    pub(crate) fn from_ptr(sprite_api: &'static playdate_sprite) -> Self {
        Self { sprite_api }
    }

    pub fn set_clip_rects_in_range(&mut self, rect: IntRect, start_z: i32, end_z: i32) {
        invoke_unsafe!(self.sprite_api.setClipRectsInRange, rect, start_z, end_z)
    }

    pub fn clear_clip_rects_in_range(&mut self, start_z: i32, end_z: i32) {
        invoke_unsafe!(self.sprite_api.clearClipRectsInRange, start_z, end_z)
    }

    pub fn set_always_redraw(&mut self, state: DrawTime) {
        invoke_unsafe!(self.sprite_api.setAlwaysRedraw, state as i32)
    }

    pub fn add_dirty_rect(&mut self, rect: IntRect) {
        invoke_unsafe!(self.sprite_api.addDirtyRect, rect)
    }

    pub fn add_sprite<T>(&mut self, sprite: &Sprite<T>) {
        invoke_unsafe!(self.sprite_api.addSprite, sprite.ptr)
    }

    pub fn remove_sprite(&mut self, sprite: SpriteRef) {
        invoke_unsafe!(self.sprite_api.removeSprite, sprite.0 as *mut _)
    }

    pub fn remove_sprites(&mut self, sprites: &[SpriteRef]) {
        invoke_unsafe!(
            self.sprite_api.removeSprites,
            sprites.as_ptr() as *mut _,
            sprites.len() as i32
        )
    }

    pub fn remove_all_sprites(&mut self) {
        invoke_unsafe!(self.sprite_api.removeAllSprites)
    }

    pub fn sprite_count(&self) -> i32 {
        invoke_unsafe!(self.sprite_api.getSpriteCount)
    }

    pub fn draw_sprites(&self) {
        invoke_unsafe!(self.sprite_api.drawSprites)
    }

    pub fn update_and_draw_sprites(&self) {
        invoke_unsafe!(self.sprite_api.updateAndDrawSprites)
    }

    pub fn reset_collision_world(&mut self) {
        invoke_unsafe!(self.sprite_api.resetCollisionWorld)
    }

    pub fn query_sprites_at_point(&self, x: f32, y: f32) -> Vec<SpriteRef> {
        let mut len = 0;
        let ptr = invoke_unsafe!(self.sprite_api.querySpritesAtPoint, x, y, &mut len);
        Self::refs_from_raw_pointers(ptr, len)
    }

    pub fn query_sprites_in_rect(&self, x: f32, y: f32, width: f32, height: f32) -> Vec<SpriteRef> {
        let mut len = 0;
        let f = self.sprite_api.querySpritesInRect;
        let ptr = invoke_unsafe!(f, x, y, width, height, &mut len);
        Self::refs_from_raw_pointers(ptr, len)
    }

    pub fn query_sprites_along_line(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> Vec<SpriteRef> {
        let mut len = 0;
        let f = self.sprite_api.querySpritesAlongLine;
        let ptr = invoke_unsafe!(f, x1, y1, x2, y2, &mut len);
        Self::refs_from_raw_pointers(ptr, len)
    }

    pub fn query_sprite_info_along_line(
        &self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
    ) -> Vec<SpriteQueryInfo> {
        let mut len = 0;
        let f = self.sprite_api.querySpriteInfoAlongLine;
        let ptr = invoke_unsafe!(f, x1, y1, x2, y2, &mut len);
        Self::info_from_raw_pointers(ptr, len)
    }

    pub fn all_overlapping_sprites(&self) -> Vec<SpriteRef> {
        let mut len = 0;
        let ptr = invoke_unsafe!(self.sprite_api.allOverlappingSprites, &mut len);
        PlaydateSprite::refs_from_raw_pointers(ptr, len)
    }

    fn info_from_raw_pointers(
        ptr: *mut playdate_sys::SpriteQueryInfo,
        len: i32,
    ) -> Vec<SpriteQueryInfo> {
        let mut vec = Vec::with_capacity(len as usize);

        unsafe {
            for i in 0..len {
                let val = ptr.offset(i as isize);
                let val = &(*val);
                vec.push(SpriteQueryInfo {
                    sprite_ref: SpriteRef(val.sprite),
                    ti1: val.ti1,
                    ti2: val.ti2,
                    entry_point: val.entryPoint.into(),
                    exit_point: val.exitPoint.into(),
                });
            }

            free(ptr as _);
        }

        vec
    }

    fn refs_from_raw_pointers(ptr: *mut *mut LCDSprite, len: i32) -> Vec<SpriteRef> {
        let mut vec = Vec::with_capacity(len as usize);
        unsafe {
            for i in 0..len {
                let val = (*ptr).offset(i as isize);
                let sref = SpriteRef(val);
                vec.push(sref);
            }

            free(ptr as _);
        }

        vec
    }
}

pub struct SpriteQueryInfo {
    pub sprite_ref: SpriteRef,
    pub ti1: f32,
    pub ti2: f32,
    pub entry_point: Point,
    pub exit_point: Point,
}

pub struct SpriteRef(*const LCDSprite);
pub type Rect = playdate_sys::PDRect;
pub type IntRect = playdate_sys::LCDRect;

pub struct Sprite<T> {
    data: Option<T>,
    ptr: *mut LCDSprite,
    sprite_api: &'static playdate_sprite,
}

impl<T> Sprite<T> {
    pub fn set_bounds(&mut self, bounds: Rect) {
        invoke_unsafe!(self.sprite_api.setBounds, self.ptr, bounds)
    }

    pub fn bounds(&self) -> Rect {
        invoke_unsafe!(self.sprite_api.getBounds, self.ptr)
    }

    pub fn move_to(&self, x: f32, y: f32) {
        invoke_unsafe!(self.sprite_api.moveTo, self.ptr, x, y)
    }

    pub fn move_by(&self, x: f32, y: f32) {
        invoke_unsafe!(self.sprite_api.moveBy, self.ptr, x, y)
    }

    pub fn position(&self) -> Point {
        let mut x = 0.0;
        let mut y = 0.0;
        invoke_unsafe!(self.sprite_api.getPosition, self.ptr, &mut x, &mut y);

        Point { x, y }
    }

    pub fn center(&self) -> Point {
        let mut x = 0.0;
        let mut y = 0.0;
        invoke_unsafe!(self.sprite_api.getCenter, self.ptr, &mut x, &mut y);

        Point { x, y }
    }

    pub fn set_center(&self, x: f32, y: f32) {
        invoke_unsafe!(self.sprite_api.setCenter, self.ptr, x, y)
    }

    pub fn set_image(&mut self, image: &Bitmap, flip: FlipState) {
        let f = self.sprite_api.setImage;
        invoke_unsafe!(f, self.ptr, image.as_mut_ptr(), flip as u32)
    }

    pub fn image(&self) -> BitmapRef {
        let ptr = invoke_unsafe!(self.sprite_api.getImage, self.ptr);
        BitmapRef(ptr)
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        invoke_unsafe!(self.sprite_api.setSize, self.ptr, width, height)
    }

    pub fn set_z_index(&mut self, z_index: i16) {
        invoke_unsafe!(self.sprite_api.setZIndex, self.ptr, z_index)
    }

    pub fn z_index(&self) -> i16 {
        invoke_unsafe!(self.sprite_api.getZIndex, self.ptr)
    }

    pub fn set_tag(&self, tag: u8) {
        invoke_unsafe!(self.sprite_api.setTag, self.ptr, tag)
    }

    pub fn tag(&self) -> u8 {
        invoke_unsafe!(self.sprite_api.getTag, self.ptr)
    }

    pub fn set_draw_mode(&mut self, mode: DrawMode) {
        invoke_unsafe!(self.sprite_api.setDrawMode, self.ptr, mode as u32)
    }

    pub fn set_image_flip(&mut self, flip: FlipState) {
        invoke_unsafe!(self.sprite_api.setImageFlip, self.ptr, flip as u32)
    }

    pub fn image_flip(&self) -> FlipState {
        let value = invoke_unsafe!(self.sprite_api.getImageFlip, self.ptr);
        if value == 0 {
            FlipState::Normal
        } else {
            FlipState::Flipped
        }
    }

    pub fn set_stencil(&mut self, stencil: &Bitmap) {
        invoke_unsafe!(self.sprite_api.setStencil, self.ptr, stencil.as_mut_ptr())
    }

    pub fn set_stencil_image(&mut self, stencil: &Bitmap, tile: TileMode) {
        invoke_unsafe!(
            self.sprite_api.setStencilImage,
            self.ptr,
            stencil.as_mut_ptr(),
            tile as i32
        )
    }

    pub fn set_stencil_pattern(&mut self, pattern: [u8; 8]) {
        invoke_unsafe!(
            self.sprite_api.setStencilPattern,
            self.ptr,
            pattern.as_ptr() as *mut _
        )
    }

    pub fn clear_stencil(&mut self) {
        invoke_unsafe!(self.sprite_api.clearStencil, self.ptr)
    }

    pub fn set_clip_rect(&mut self, clip_rect: IntRect) {
        invoke_unsafe!(self.sprite_api.setClipRect, self.ptr, clip_rect)
    }

    pub fn clear_clip_rect(&mut self) {
        invoke_unsafe!(self.sprite_api.clearClipRect, self.ptr)
    }

    pub fn set_updates_enabled(&mut self, enabled: UpdatesState) {
        invoke_unsafe!(self.sprite_api.setUpdatesEnabled, self.ptr, enabled as i32)
    }

    pub fn updates_enabled(&self) -> UpdatesState {
        let enabled = invoke_unsafe!(self.sprite_api.updatesEnabled, self.ptr);
        if enabled == 1 {
            UpdatesState::Enabled
        } else {
            UpdatesState::Disabled
        }
    }

    pub fn set_visible(&mut self, state: VisibleState) {
        invoke_unsafe!(self.sprite_api.setVisible, self.ptr, state as i32)
    }

    pub fn visible(&mut self) -> VisibleState {
        let visible = invoke_unsafe!(self.sprite_api.isVisible, self.ptr);
        if visible == 1 {
            VisibleState::Visible
        } else {
            VisibleState::Invisible
        }
    }

    pub fn set_opaque(&mut self, state: Opaqueness) {
        invoke_unsafe!(self.sprite_api.setOpaque, self.ptr, state as i32)
    }

    pub fn mark_dirty(&mut self) {
        invoke_unsafe!(self.sprite_api.markDirty, self.ptr)
    }

    pub fn set_ignores_draw_offset(&mut self, offset_behavior: OffsetBehavior) {
        invoke_unsafe!(
            self.sprite_api.setIgnoresDrawOffset,
            self.ptr,
            offset_behavior as i32
        )
    }

    pub fn set_update_fn(&mut self, update_fn: extern "C" fn(*mut LCDSprite)) {
        invoke_unsafe!(self.sprite_api.setUpdateFunction, self.ptr, Some(update_fn))
    }

    pub fn set_draw_fn(
        &mut self,
        draw_fn: extern "C" fn(*mut LCDSprite, bounds: PDRect, drawrect: PDRect),
    ) {
        invoke_unsafe!(self.sprite_api.setDrawFunction, self.ptr, Some(draw_fn))
    }

    pub fn set_user_data(&mut self, data: Option<T>) {
        self.data = data;
    }

    pub fn user_data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    pub fn set_collisions_enabled(&mut self, enabled: CollisionState) {
        invoke_unsafe!(
            self.sprite_api.setCollisionsEnabled,
            self.ptr,
            enabled as i32
        )
    }

    pub fn collisions_enabled(&self) -> CollisionState {
        let enabled = invoke_unsafe!(self.sprite_api.collisionsEnabled, self.ptr);
        if enabled == 1 {
            CollisionState::Enabled
        } else {
            CollisionState::Disabled
        }
    }

    pub fn set_collide_rect(&mut self, rect: Rect) {
        invoke_unsafe!(self.sprite_api.setCollideRect, self.ptr, rect)
    }

    pub fn collide_rect(&self) -> Rect {
        invoke_unsafe!(self.sprite_api.getCollideRect, self.ptr)
    }

    pub fn clear_collide_rect(&self) {
        invoke_unsafe!(self.sprite_api.clearCollideRect, self.ptr)
    }

    pub fn set_collision_response_fn(
        &mut self,
        func: extern "C" fn(*mut LCDSprite, *mut LCDSprite) -> u32,
    ) {
        invoke_unsafe!(
            self.sprite_api.setCollisionResponseFunction,
            self.ptr,
            Some(func)
        )
    }

    pub fn check_collisions(&self, goal_x: f32, goal_y: f32) -> Vec<SpriteCollisionInfo> {
        let mut len = 0;
        let mut actual_x = 0.0;
        let mut actual_y = 0.0;
        let f = self.sprite_api.checkCollisions;
        let ptr = invoke_unsafe!(
            f,
            self.ptr,
            goal_x,
            goal_y,
            &mut actual_x,
            &mut actual_y,
            &mut len
        );
        let mut vec = Vec::with_capacity(len as usize);

        unsafe {
            for i in 0..len {
                let val = ptr.offset(i as isize);
                let val = &(*val);

                vec.push(SpriteCollisionInfo {
                    sprite: SpriteRef(val.sprite),
                    other: SpriteRef(val.other),
                    response_type: val.responseType.into(),
                    overlaps: val.overlaps.into(),
                    ti: val.ti,
                    moved: val.move_.into(),
                    normal: val.normal.into(),
                    touch: val.touch.into(),
                    sprite_rect: val.spriteRect.into(),
                    other_rect: val.otherRect.into(),
                });
            }

            free(ptr as _);
        }

        vec
    }

    pub fn move_with_collisions(&mut self, goal_x: f32, goal_y: f32) -> Vec<SpriteCollisionInfo> {
        let mut len = 0;
        let mut actual_x = 0.0;
        let mut actual_y = 0.0;
        let f = self.sprite_api.moveWithCollisions;
        let ptr = invoke_unsafe!(
            f,
            self.ptr,
            goal_x,
            goal_y,
            &mut actual_x,
            &mut actual_y,
            &mut len
        );
        let mut vec = Vec::with_capacity(len as usize);

        unsafe {
            for i in 0..len {
                let val = ptr.offset(i as isize);
                let val = &(*val);

                vec.push(SpriteCollisionInfo {
                    sprite: SpriteRef(val.sprite),
                    other: SpriteRef(val.other),
                    response_type: val.responseType.into(),
                    overlaps: val.overlaps.into(),
                    ti: val.ti,
                    moved: val.move_.into(),
                    normal: val.normal.into(),
                    touch: val.touch.into(),
                    sprite_rect: val.spriteRect.into(),
                    other_rect: val.otherRect.into(),
                });
            }

            free(ptr as _);
        }

        vec
    }

    pub fn overlapping_sprites(&self) -> Vec<SpriteRef> {
        let mut len = 0;
        let ptr = invoke_unsafe!(self.sprite_api.overlappingSprites, self.ptr, &mut len);
        PlaydateSprite::refs_from_raw_pointers(ptr, len)
    }
}

pub struct SpriteCollisionInfo {
    pub sprite: SpriteRef,
    pub other: SpriteRef,
    pub response_type: CollisionResponse,
    pub overlaps: SpriteOverlap,
    pub ti: f32,
    pub moved: Point,
    pub normal: IntPoint,
    pub touch: Point,
    pub sprite_rect: Rect,
    pub other_rect: Rect,
}

impl<T> Clone for Sprite<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        let sprite_api = self.sprite_api;
        let ptr = invoke_unsafe!(self.sprite_api.copy, self.ptr);
        let data = self.data.clone();
        Self {
            sprite_api,
            ptr,
            data,
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CollisionResponse {
    Slide = SpriteCollisionResponseType_kCollisionTypeSlide,
    Freeze = SpriteCollisionResponseType_kCollisionTypeFreeze,
    Overlap = SpriteCollisionResponseType_kCollisionTypeOverlap,
    Bounce = SpriteCollisionResponseType_kCollisionTypeBounce,
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
        invoke_unsafe!(self.sprite_api.freeSprite, self.ptr)
    }
}

pub struct Bitmap(*mut LCDBitmap);

impl Bitmap {
    pub(crate) fn as_mut_ptr(&self) -> *mut LCDBitmap {
        self.0
    }
}

pub struct BitmapRef(*const LCDBitmap);

pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl From<CollisionPoint> for Point {
    fn from(value: CollisionPoint) -> Self {
        Point {
            x: value.x,
            y: value.y,
        }
    }
}

pub struct IntPoint {
    pub x: i32,
    pub y: i32,
}

impl From<CollisionVector> for IntPoint {
    fn from(value: CollisionVector) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Eq, PartialEq)]
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
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TileMode {
    NoTile = 0,
    Tile = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum UpdatesState {
    Disabled = 0,
    Enabled = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum VisibleState {
    Invisible = 0,
    Visible = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Opaqueness {
    Translucent = 0,
    Opaque = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum DrawTime {
    WhenNeeded = 0,
    Always = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum OffsetBehavior {
    DrawOffset = 0,
    ScreenCoordinates = 1,
}

#[repr(i32)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum CollisionState {
    Disabled = 0,
    Enabled = 1,
}

pub enum SpriteOverlap {
    TunneledThrough,
    Overlapping,
}

impl From<u8> for SpriteOverlap {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::TunneledThrough,
            1 => Self::Overlapping,
            _ => unreachable!(),
        }
    }
}
