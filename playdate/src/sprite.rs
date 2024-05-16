use crate::display::FlipState;
use playdate_sys::{
    playdate_sprite, LCDBitmap, LCDBitmapDrawMode_kDrawModeBlackTransparent,
    LCDBitmapDrawMode_kDrawModeCopy, LCDBitmapDrawMode_kDrawModeFillBlack,
    LCDBitmapDrawMode_kDrawModeFillWhite, LCDBitmapDrawMode_kDrawModeInverted,
    LCDBitmapDrawMode_kDrawModeNXOR, LCDBitmapDrawMode_kDrawModeWhiteTransparent,
    LCDBitmapDrawMode_kDrawModeXOR, LCDSprite,
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

    pub fn add_sprite(&mut self, sprite: &Sprite) {
        invoke_unsafe!(self.sprite_api.addSprite, sprite.ptr)
    }

    pub fn remove_sprite(&mut self, sprite: &Sprite) {
        invoke_unsafe!(self.sprite_api.removeSprite, sprite.ptr)
    }

    pub fn remove_sprites(&mut self, sprites: &[&Sprite]) {
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

    // TODO querySpritesAtPoint, querySpritesInRect, querySpritesAlongLine,
    // querySpriteInfoAlongLine, overlappingSprites, allOverlappingSprites
}

pub type Rect = playdate_sys::PDRect;
pub type IntRect = playdate_sys::LCDRect;

pub struct Sprite {
    sprite_api: &'static playdate_sprite,
    ptr: *mut LCDSprite,
}

impl Sprite {
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
        invoke_unsafe!(
            self.sprite_api.setImage,
            self.ptr,
            image.as_ptr() as *mut _,
            flip as u32
        )
    }

    pub fn image(&self) -> &Bitmap {
        todo!()
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
        invoke_unsafe!(
            self.sprite_api.setStencil,
            self.ptr,
            stencil.as_ptr() as *mut _
        )
    }

    pub fn set_stencil_image(&mut self, stencil: &Bitmap, tile: TileMode) {
        invoke_unsafe!(
            self.sprite_api.setStencilImage,
            self.ptr,
            stencil.as_ptr() as *mut _,
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

    // TODO setUpdateFunction, setDrawFunction, setUserData, getUserData

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

    // TODO setCollisionResponseFunction, checkCollisions, moveWithCollisions
}

impl Clone for Sprite {
    fn clone(&self) -> Self {
        let sprite_api = self.sprite_api;
        let ptr = invoke_unsafe!(self.sprite_api.copy, self.ptr);
        Self { sprite_api, ptr }
    }
}

impl Drop for Sprite {
    fn drop(&mut self) {
        invoke_unsafe!(self.sprite_api.freeSprite, self.ptr)
    }
}

pub struct Bitmap;

impl Bitmap {
    fn as_ptr(&self) -> *const LCDBitmap {
        todo!()
    }
}

pub struct Point {
    pub x: f32,
    pub y: f32,
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
