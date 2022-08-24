use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(game)
        .run();
}

fn game(input: Res<Input<KeyCode>>)
{
    let mut moving_angle: f64 = 0.0;
    let mut is_moving = true;

    if input.pressed(KeyCode::A)
    {
        if input.pressed(KeyCode::W)
        {
            moving_angle = 45.0;
        }
        else if input.pressed(KeyCode::S)
        {
            moving_angle = 135.0;
        }
        else
        {
            moving_angle = 90.0;
        }   
    }
    else if input.pressed(KeyCode::D)
    {
        if input.pressed(KeyCode::W)
        {
            moving_angle = 315.0;
        }
        else if input.pressed(KeyCode::S)
        {
            moving_angle = 225.0;
        }
        else
        {
            moving_angle = 270.0;
        }
    }
    else
    {
        if input.pressed(KeyCode::W)
        {
            moving_angle = 0.0;
        }
        else if input.pressed(KeyCode::S)
        {
            moving_angle = 180.0;
        }
        else
        {
            is_moving = false
        }
    }

    let direction = moving_angle.to_radians().sin_cos();
    println!("{}", direction.0);
    print!("{}", direction.1);
    print!("              ");
}
