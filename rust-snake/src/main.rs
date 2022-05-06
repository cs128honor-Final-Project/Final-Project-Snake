use bevy::prelude::*;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 5.0));
    commands.spawn_bundle(camera);
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            //Window Title
            title: "snake".to_string(),
            //Window Size
            width: 300.,
            height: 200.,
        })
        //Window background color
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup.system())
        //Default Plugin
        .add_plugins(DefaultPlugins)
        .run();
}

//Number of grids (10 equal horizontal and 10 equal vertical, i.e. 10*10 grid)
const LENGTH: u32 = 10;
const WIDTH: u32 = 10;

//Position in the grid
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

//Size of the snake head in the grid

struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

//Drawing grid auxiliary lines
fn draw_grid(windows: Res<Windows>, mut lines: ResMut<DebugLines>) {
    let window = windows.get_primary().unwrap();
    let half_win_width = 0.5 * window.width();
    let half_win_height = 0.5 * window.height();
    let x_space = window.width() / CELL_X_COUNT as f32;
    let y_space = window.height() / CELL_Y_COUNT as f32;

    let mut i = -1. * half_win_height;
    while i < half_win_height {
        lines.line(
            Vec3::new(-1. * half_win_width, i, 0.0),
            Vec3::new(half_win_width, i, 0.0),
            0.0,
        );
        i += y_space;
    }

    i = -1. * half_win_width;
    while i < half_win_width {
        lines.line(
            Vec3::new(i, -1. * half_win_height, 0.0),
            Vec3::new(i, half_win_height, 0.0),
            0.0,
        );
        i += x_space;
    }

    //drwaing vertical lines
    lines.line(
        Vec3::new(0., -1. * half_win_height, 0.0),
        Vec3::new(0., half_win_height, 0.0),
        0.0,
    );
}

//Move commands
fn snake_movement( //<--
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<&mut Position, With<SnakeHead>>,
) {
    for mut pos in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            if pos.x > 0 {
                pos.x -= 1;
            }
        }
        if keyboard_input.pressed(KeyCode::Right) {
            if pos.x < CELL_X_COUNT as i32 - 1 {
                pos.x += 1;
            }
        }
        if keyboard_input.pressed(KeyCode::Down) {
            if pos.y > 0 {
                pos.y -= 1;
            }
        }
        if keyboard_input.pressed(KeyCode::Up) {
            if pos.y < CELL_Y_COUNT as i32 - 1 {
                pos.y += 1;
            }
        }
    }
}