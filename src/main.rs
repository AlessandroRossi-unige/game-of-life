#![allow(unused)]

mod tile;

use std::borrow::BorrowMut;
use std::path::Path;
use bevy::app::AppExit;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::render::texture::ImageType;
use bevy::sprite::collide_aabb::collide;
use crate::tile::{Tile, TilePlugin};

const SPRITE_DIR: &str = "assets";
const TILE_SPRITE: &str = "tile.png";
const TILE_RADIUS: f32 = 32.;



pub struct WinSize {
    w: f32,
    h: f32,
}

#[derive(Component)]
struct TileSet {
    set: Vec<Vec<Tile>>
}

pub struct SpriteInfos {
    tile: (Handle<Image>, Vec2),
    // background: Handle<TextureAtlas>
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
        .insert_resource(WindowDescriptor {
            title: "Conway's Game Of Life!".to_string(),
            width: 1024.,
            height: 1024.,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_system(close_game)
        .add_system(player_click_tile.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(TilePlugin)
        .run()
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut windows: ResMut<Windows>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let mut window = windows.get_primary_mut().unwrap();

    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height()
    });

    window.set_position(IVec2::new(200, -5));

    commands.insert_resource(SpriteInfos {
        tile: load_image(&mut images, TILE_SPRITE)
    });
}

fn load_image(images: &mut ResMut<Assets<Image>>, path: &str) -> (Handle<Image>, Vec2) {
    let path = Path::new(SPRITE_DIR).join(path);
    let bytes = std::fs::read(&path).expect(&format!("Cannot find {}", path.display()));
    let image = Image::from_buffer(&bytes, ImageType::MimeType("image/png")).unwrap();
    let size = image.texture_descriptor.size;
    let size = Vec2::new(size.width as f32, size.height as f32);
    let image_handle = images.add(image);
    (image_handle, size)
}

fn close_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn player_click_tile(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut tile_set: ResMut<TileSet>,
    materials: Res<SpriteInfos>,
    win_size: Res<WinSize>
) {
    let window = windows.get_primary().unwrap();

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            info!("Position: {}", position);
            let i_index = (position.y / 32.) as usize ;
            let j_index = (position.x / 32.) as usize ;
            let tile = &mut tile_set.set[i_index][j_index];
            info!("{}, {}", &(i_index as usize), &(j_index as usize));
            info!("{}", Vec3::new(-(win_size.w / 2. -( tile.x - (TILE_RADIUS / 2.))), -(win_size.h / 2. -( tile.y - (TILE_RADIUS / 2.))), 0.));

            tile.change_status(commands, materials, win_size);
        } else {
            info!("outside")
        }
    }

}
