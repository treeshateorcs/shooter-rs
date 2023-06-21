mod debug_lines_static;
mod envelope;
pub mod ext;
pub mod macros;
pub mod math;
#[cfg(test)]
pub mod test;

use self::ext::ImageExt;
pub use self::{debug_lines_static::*, envelope::*};
use bevy::prelude::{Assets, Handle, Image};

pub fn create_blank_image(size_x: u32, size_y: u32, images: &mut Assets<Image>) -> Handle<Image> {
    return images.add(Image::blank(size_x, size_y));
}
