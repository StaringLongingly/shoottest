use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(input_grabber)
        .run();
}

fn input_grabber(input: Res<Input<KeyCode>>)
{
    let mut x = 0;
    let mut y = 0;

    if input.pressed(KeyCode::A)
    {
        x = -1
    }
    else if input.pressed(KeyCode::D)
    {
        x = 1
    }

    if input.pressed(KeyCode::W)
    {
        y = 1
    }
    else if input.pressed(KeyCode::S)
    {
        y = -1
    }

    println!("{}", x);
    print!("{}", y)
}
