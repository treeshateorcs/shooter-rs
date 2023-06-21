use crate::{
    resource::{AssetStorage, Misc},
    util::create_blank_image,
};
use bevy::prelude::{AssetServer, Assets, Image, Res, ResMut};

pub fn on_enter(
    asset_server: Res<AssetServer>,
    mut asset_storage: ResMut<AssetStorage>,
    mut images: ResMut<Assets<Image>>,
    mut misc: ResMut<Misc>,
) {
    load_folder_or_log(&asset_server, &mut asset_storage, "actors");
    load_folder_or_log(&asset_server, &mut asset_storage, "fonts");
    load_folder_or_log(&asset_server, &mut asset_storage, "sounds");
    load_folder_or_log(&asset_server, &mut asset_storage, "terrain");
    load_folder_or_log(&asset_server, &mut asset_storage, "weapons");
    misc.dummy_image = Some(create_blank_image(1, 1, &mut images));
}

fn load_folder_or_log(
    asset_server: &Res<AssetServer>,
    asset_storage: &mut ResMut<AssetStorage>,
    path: &str,
) {
    match asset_server.load_folder(path) {
        Ok(assets) => {
            asset_storage.extend(assets);
        }
        Err(error) => {
            log::error!("Failed to load assets folder from {}: {:?}", path, error);
        }
    }
}
