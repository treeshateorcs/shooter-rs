use crate::util::ext::WorldExt;
use bevy::{
    ecs::system::Command,
    prelude::{With, World},
    window::{CursorGrabMode, PrimaryWindow, Window},
};

pub struct CursorGrab(pub bool);

impl CursorGrab {
    pub fn get_mode(&self) -> CursorGrabMode {
        if self.0 {
            return CursorGrabMode::Confined;
        } else {
            return CursorGrabMode::None;
        }
    }
}

impl Command for CursorGrab {
    fn write(self, world: &mut World) {
        if !self.0 || world.config().misc.grab_cursor {
            for mut window in world
                .query_filtered::<&mut Window, With<PrimaryWindow>>()
                .iter_mut(world)
            {
                window.cursor.grab_mode = self.get_mode();
                window.cursor.visible = !self.0;
            }
        }
    }
}
