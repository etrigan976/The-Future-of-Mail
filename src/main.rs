/*
    local crate imports
*/
use bevy::prelude::*;
/*
    global data
*/
const TXT_CLR: Color = Color::srgb(0.9, 0.9, 0.9);
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);
/// # main function
/// This function initializes the nannou framework app
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((splash::splash_plugin, menu::menu_plugin, game::game_plugin))
        .run();
}
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}