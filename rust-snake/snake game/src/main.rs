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