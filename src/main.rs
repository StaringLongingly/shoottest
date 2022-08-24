use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        /*.insert_resource(MovementSettings
        {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0, // default: 12.0
        })*/
        .add_system(game)
        .run();
}

fn game(input: Res<Input<KeyCode>>)
{

}
