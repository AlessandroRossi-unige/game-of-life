use bevy::prelude::*;

struct WinSize {
    w: f32,
    h: f32,
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
        .add_plugins(DefaultPlugins)
        .run()
}

fn setup(

) {

}

