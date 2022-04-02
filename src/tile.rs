use std::io::empty;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use crate::{SpriteInfos, TILE_RADIUS, TILE_SPRITE, TileSet, WinSize};

#[derive(Component)]
pub struct Tile {
    pub x: f32,
    pub y: f32,
    pub is_alive: bool
}

impl Tile {
    pub fn change_status(&mut self, mut commands: Commands, materials: Res<SpriteInfos>, win_size: Res<WinSize>) {
        commands.spawn_bundle(SpriteBundle {
            texture: materials.tile.0.clone(),
            transform: Transform {
                translation: Vec3::new(-(win_size.w / 2. -( self.x - (TILE_RADIUS / 2.))), -(win_size.h / 2. -( self.y - (TILE_RADIUS / 2.))), 0.),
                scale: Vec3::new(0.5, 0.5, 0.1),
                ..Default::default()
            },
            ..Default::default()
        });
        self.is_alive = !self.is_alive;
    }
}

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_stage(
                "game_setup_actors",
                SystemStage::single(tile_mapping.system()),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.))
                    .with_system(tile_expand.system()),
            );

    }
}

fn tile_mapping(
    mut commands: Commands,
    materials: Res<SpriteInfos>,
    win_size: Res<WinSize>
) {
    let init_width = 0.;
    let init_height = 0.;

    let mut matrix: Vec<Vec<Tile>> = vec![];

    for i in 1..33 as i32 {
        let y_pos = init_height + (TILE_RADIUS * i as f32);
        let mut row: Vec<Tile> = vec![];
        for j in 1..33 as i32 {
            let x_pos = init_width + (TILE_RADIUS * j as f32);
            // info!("{}, {}", x_pos, y_pos);
            row.push(Tile {
                x: x_pos,
                y: y_pos,
                is_alive: false
            });

        }
        matrix.push(row);
    }
    commands.insert_resource(TileSet {
        set: matrix
    });
}

fn tile_expand(
    mut commands: Commands,
    tile_query: Query<&Transform, With<Tile>>
) {
    // for &tf in tile_query.iter() {
    //     if query.get(entity) {
    //         info!("hello");
    //     }
    // }

}

