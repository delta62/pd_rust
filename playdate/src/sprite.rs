use crate::{display::FlipState, Playdate};
use playdate_sys::{
    playdate_sprite, LCDBitmap, LCDBitmapDrawMode_kDrawModeBlackTransparent,
    LCDBitmapDrawMode_kDrawModeCopy, LCDBitmapDrawMode_kDrawModeFillBlack,
    LCDBitmapDrawMode_kDrawModeFillWhite, LCDBitmapDrawMode_kDrawModeInverted,
    LCDBitmapDrawMode_kDrawModeNXOR, LCDBitmapDrawMode_kDrawModeWhiteTransparent,
    LCDBitmapDrawMode_kDrawModeXOR, LCDSprite, PlaydateAPI,
};

macro_rules! invoke_unsafe {
    ( $self:ident, $function:ident ) => {
        invoke_unsafe!($self, $function,)
    };
    ( $self:ident, $function:ident, $( $param:expr ),* $( , )? ) => {
        unsafe {
            let callable = $self.sprite().$function.unwrap();
            callable($( $param ),*)
        }
    };
}

macro_rules! invoke_sprite {
    ( $self:ident, $function:ident ) => {
        invoke_sprite!($self, $function,)
    };
    ( $self:ident, $function:ident, $( $param:expr ),* $( , )? ) => {
        unsafe {
            let callable = $self.sprite().$function.unwrap();
            callable($self.ptr as *mut _, $( $param ),*)
        }
    };
}

pub struct PlaydateSprite {
    ptr: *const playdate_sprite,
}

impl PlaydateSprite {
    pub(crate) fn from_ptr(ptr: *const playdate_sprite) -> Self {
        Self { ptr }
    }

    pub fn set_clip_rects_in_range(&mut self, rect: IntRect, start_z: i32, end_z: i32) {
        invoke_unsafe!(self, setClipRectsInRange, rect, start_z, end_z)
    }

    pub fn clear_clip_rects_in_range(&mut self, start_z: i32, end_z: i32) {
        invoke_unsafe!(self, clearClipRectsInRange, start_z, end_z)
    }

    pub fn set_always_redraw(&mut self, state: DrawTime) {
        invoke_unsafe!(self, setAlwaysRedraw, state as i32)
    }

    pub fn add_dirty_rect(&mut self, rect: IntRect) {
        invoke_unsafe!(self, addDirtyRect, rect)
    }

    pub fn add_sprite(&mut self, sprite: &Sprite) {
        invoke_unsafe!(self, addSprite, sprite.as_ptr() as *mut _)
    }

    pub fn remove_sprite(&mut self, sprite: &Sprite) {
        invoke_unsafe!(self, removeSprite, sprite.as_ptr() as *mut _)
    }

    pub fn remove_sprites(&mut self, sprites: &[&Sprite]) {
        invoke_unsafe!(
            self,
            removeSprites,
            sprites.as_ptr() as *mut _,
            sprites.len() as i32
        )
    }

    pub fn remove_all_sprites(&mut self) {
        invoke_unsafe!(self, removeAllSprites)
    }

    pub fn sprite_count(&self) -> i32 {
        invoke_unsafe!(self, getSpriteCount)
    }

    pub fn draw_sprites(&self) {
        invoke_unsafe!(self, drawSprites)
    }

    pub fn update_and_draw_sprites(&self) {
        invoke_unsafe!(self, updateAndDrawSprites)
    }

    pub fn reset_collision_world(&mut self) {
        invoke_unsafe!(self, resetCollisionWorld)
    }

    // TODO querySpritesAtPoint, querySpritesInRect, querySpritesAlongLine,
    // querySpriteInfoAlongLine, overlappingSprites, allOverlappingSprites

    unsafe fn sprite(&self) -> &playdate_sprite {
        self.ptr.as_ref().unwrap()
    }
}

pub type Rect = playdate_sys::PDRect;
pub type IntRect = playdate_sys::LCDRect;

pub struct Sprite {
    pd: *const PlaydateAPI,
    ptr: *const LCDSprite,
}

impl Sprite {
    pub fn new(api: &Playdate) -> Self {
        let pd = api.ptr();
        let ptr = unsafe {
            let sprite_api = pd.as_ref().unwrap().sprite.as_ref().unwrap();
            let new_sprite = sprite_api.newSprite.unwrap();
            new_sprite()
        };

        Self { pd, ptr }
    }

    pub fn as_ptr(&self) -> *const LCDSprite {
        self.ptr
    }

    pub fn set_bounds(&mut self, bounds: Rect) {
        invoke_sprite!(self, setBounds, bounds)
    }

    pub fn bounds(&self) -> Rect {
        invoke_sprite!(self, getBounds)
    }

    pub fn move_to(&self, x: f32, y: f32) {
        invoke_sprite!(self, moveTo, x, y)
    }

    pub fn move_by(&self, x: f32, y: f32) {
        invoke_sprite!(self, moveBy, x, y)
    }

    pub fn position(&self) -> Point {
        let mut x = 0.0;
        let mut y = 0.0;
        invoke_sprite!(self, getPosition, &mut x, &mut y);

        Point { x, y }
    }

    pub fn center(&self) -> Point {
        let mut x = 0.0;
        let mut y = 0.0;
        invoke_sprite!(self, getCenter, &mut x, &mut y);

        Point { x, y }
    }

    pub fn set_center(&self, x: f32, y: f32) {
        invoke_sprite!(self, setCenter, x, y)
    }

    pub fn set_image(&mut self, image: &Bitmap, flip: FlipState) {
        invoke_sprite!(self, setImage, image.as_ptr() as *mut _, flip as u32)
    }

    pub fn image(&self) -> &Bitmap {
        todo!()
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        invoke_sprite!(self, setSize, width, height)
    }

    pub fn set_z_index(&mut self, z_index: i16) {
        invoke_sprite!(self, setZIndex, z_index)
    }

    pub fn z_index(&self) -> i16 {
        invoke_sprite!(self, getZIndex)
    }

    pub fn set_tag(&self, tag: u8) {
        invoke_sprite!(self, setTag, tag)
    }

    pub fn tag(&self) -> u8 {
        invoke_sprite!(self, getTag)
    }

    pub fn set_draw_mode(&mut self, mode: DrawMode) {
        invoke_sprite!(self, setDrawMode, mode as u32)
    }

    pub fn set_image_flip(&mut self, flip: FlipState) {
        invoke_sprite!(self, setImageFlip, flip as u32)
    }

    pub fn image_flip(&self) -> FlipState {
        let value = invoke_sprite!(self, getImageFlip);
        if value == 0 {
            FlipState::Normal
        } else {
            FlipState::Flipped
        }
    }

    pub fn set_stencil(&mut self, stencil: &Bitmap) {
        invoke_sprite!(self, setStencil, stencil.as_ptr() as *mut _)
    }

    pub fn set_stencil_image(&mut self, stencil: &Bitmap, tile: TileMode) {
        invoke_sprite!(
            self,
            setStencilImage,
            stencil.as_ptr() as *mut _,
            tile as i32
        )
    }

    pub fn set_stencil_pattern(&mut self, pattern: [u8; 8]) {
        invoke_sprite!(self, setStencilPattern, pattern.as_ptr() as *mut _)
    }

    pub fn clear_stencil(&mut self) {
        invoke_sprite!(self, clearStencil)
    }

    pub fn set_clip_rect(&mut self, clip_rect: IntRect) {
        invoke_sprite!(self, setClipRect, clip_rect)
    }

    pub fn clear_clip_rect(&mut self) {
        invoke_sprite!(self, clearClipRect)
    }

    pub fn set_updates_enabled(&mut self, enabled: UpdatesState) {
        invoke_sprite!(self, setUpdatesEnabled, enabled as i32)
    }

    pub fn updates_enabled(&self) -> UpdatesState {
        let enabled = invoke_sprite!(self, updatesEnabled);
        if enabled == 1 {
            UpdatesState::Enabled
        } else {
            UpdatesState::Disabled
        }
    }

    pub fn set_visible(&mut self, state: VisibleState) {
        invoke_sprite!(self, setVisible, state as i32)
    }

    pub fn visible(&mut self) -> VisibleState {
        let visible = invoke_sprite!(self, isVisible);
        if visible == 1 {
            VisibleState::Visible
        } else {
            VisibleState::Invisible
        }
    }

    pub fn set_opaque(&mut self, state: Opaqueness) {
        invoke_sprite!(self, setOpaque, state as i32)
    }

    pub fn mark_dirty(&mut self) {
        invoke_sprite!(self, markDirty)
    }

    pub fn set_ignores_draw_offset(&mut self, offset_behavior: OffsetBehavior) {
        invoke_sprite!(self, setIgnoresDrawOffset, offset_behavior as i32)
    }

    // TODO setUpdateFunction, setDrawFunction, setUserData, getUserData

    pub fn set_collisions_enabled(&mut self, enabled: CollisionState) {
        invoke_sprite!(self, setCollisionsEnabled, enabled as i32)
    }

    pub fn collisions_enabled(&self) -> CollisionState {
        let enabled = invoke_sprite!(self, collisionsEnabled);
        if enabled == 1 {
            CollisionState::Enabled
        } else {
            CollisionState::Disabled
        }
    }

    pub fn set_collide_rect(&mut self, rect: Rect) {
        invoke_sprite!(self, setCollideRect, rect)
    }

    pub fn collide_rect(&self) -> Rect {
        invoke_sprite!(self, getCollideRect)
    }

    pub fn clear_collide_rect(&self) {
        invoke_sprite!(self, clearCollideRect)
    }

    // TODO setCollisionResponseFunction, checkCollisions, moveWithCollisions

    unsafe fn sprite(&self) -> &playdate_sprite {
        self.pd.as_ref().unwrap().sprite.as_ref().unwrap()
    }
}

impl Clone for Sprite {
    fn clone(&self) -> Self {
        let pd = self.pd;
        let ptr = unsafe {
            let copy_sprite = self.sprite().copy.unwrap();
            copy_sprite(self.ptr as *mut _)
        };

        Self { pd, ptr }
    }
}

impl Drop for Sprite {
    fn drop(&mut self) {
        unsafe {
            let free_sprite = self.sprite().freeSprite.unwrap();
            free_sprite(self.ptr as *mut _);
        }
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
