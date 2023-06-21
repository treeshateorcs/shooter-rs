use bevy::{prelude::Image, render::render_resource::Extent3d};

pub trait ImageExt {
    fn blank(size_x: u32, size_y: u32) -> Self;

    fn for_each_pixel<F: FnMut(u32, u32, (u8, u8, u8, u8))>(&self, mut f: F) {
        let size = self.size_x() as usize;
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for (i, &v) in self.data().iter().enumerate() {
            match i % 4 {
                0 => {
                    r = v;
                }
                1 => {
                    g = v;
                }
                2 => {
                    b = v;
                }
                3 => {
                    let a = v;

                    if a != 0 {
                        let j = i / 4;
                        let x = (j % size) as u32;
                        let y = (j / size) as u32;
                        f(x, y, (r, g, b, a));
                    }
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: (u8, u8, u8, u8)) {
        if x >= self.size_x() {
            // TODO: panic on debug
            return;
        }

        if y >= self.size_y() {
            // TODO: panic on debug
            return;
        }

        if color.3 == 0 {
            return;
        }

        let i = (y as usize * self.size_x() as usize + x as usize) * 4;
        self.data_mut()[i] = color.0;
        self.data_mut()[i + 1] = color.1;
        self.data_mut()[i + 2] = color.2;
        self.data_mut()[i + 3] = color.3;
    }

    fn add_border(&mut self, color: (u8, u8, u8, u8)) {
        let size_x = self.size_x();
        let size_y = self.size_y();

        for x in 0..size_x {
            self.set_pixel(x, 0, color);
            self.set_pixel(x, size_y - 1, color);
        }

        for y in 0..size_y {
            self.set_pixel(0, y, color);
            self.set_pixel(size_x - 1, y, color);
        }
    }

    fn data(&self) -> &[u8];

    fn data_mut(&mut self) -> &mut [u8];

    fn size_x(&self) -> u32;

    fn size_y(&self) -> u32;
}

impl ImageExt for Image {
    fn blank(size_x: u32, size_y: u32) -> Self {
        let mut image = Self::default();

        image.resize(Extent3d {
            width: size_x,
            height: size_y,
            ..Default::default()
        });

        return image;
    }

    fn data(&self) -> &[u8] {
        return &self.data;
    }

    fn data_mut(&mut self) -> &mut [u8] {
        return &mut self.data;
    }

    fn size_x(&self) -> u32 {
        return self.texture_descriptor.size.width;
    }

    fn size_y(&self) -> u32 {
        return self.texture_descriptor.size.height;
    }
}
