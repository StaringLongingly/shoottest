use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(input_grabber)
        .run();
}

fn input_grabber(input: Res<Input<KeyCode>>)
{
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    if input.pressed(KeyCode::A)
    {
        x = -256
    }
    else if input.pressed(KeyCode::D)
    {
        x = 256
    }

    if input.pressed(KeyCode::W)
    {
        y = 256
    }
    else if input.pressed(KeyCode::S)
    {
        y = -256
    }

    if x.abs() == 256 && y.abs() == 256
    {
        x = (x as f32 * 0.70710677) as i32;
        y = (y as f32 * 0.70710677) as i32;	
    }

    println!("{}", x);
    print!("{}", y)
}
