use bevy::{prelude::*, input::mouse::MouseMotion};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(game)
        .add_system(cursor_grab)
        .run();
}

pub struct PlayerStruct
{
    position_x : f64,
    position_y : f64,
    look_x : f64,
    look_y : f64
}



fn cursor_grab
(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
)
{
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }

}

fn game
(
    input: Res<Input<KeyCode>>, 
    mut mouse: EventReader<MouseMotion>,
)
{

    let mut moving_angle: f64 = 0.0;
    let mut _is_moving = true;

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
            _is_moving = false
        }
    }
    
    let mut position: (f64, f64)= (0.0, 0.0);
    let mut look: (f64, f64)= (0.0, 0.0);

    position = ( (position.0 + moving_angle.to_radians().sin_cos().1), (position.1 + moving_angle.to_radians().sin_cos().0) );
    
    for ev in mouse.iter() {
        look = ( (look.0 + ev.delta.x as f64), (look.1 + ev.delta.y as f64) )
    }

    let player: PlayerStruct = PlayerStruct
    {
        position_x: position.0,
        position_y: position.1,
        look_x: look.0,
        look_y: look.1
    };
   
    println!("{}", player.position_x);
    print!("         ");
    print!("{}", player.position_y);
    print!("         ");
    print!("{}", player.look_x);
    print!("         ");
    print!("{}", player.look_y);
    print!("         ");

}
