/*
    local crate imports
*/
/// we are using bevy for the game
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, Window, WindowPlugin, WindowTheme}, //ecs::schedule,
    audio::{AudioSource, PlaybackSettings},
};

/// IDK where i want to put prompted stuff, probably will take it out
//use prompted::*;
/// want to have specific flags
/// -h help
/// -f freecam
//use std::env;
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
    Pause,
    Help,
    Lose,
}
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Medium,
}
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);
#[derive(Component)]
pub struct RotatableCamera {
    radius: f32,
    yaw: f32,
    pitch: f32,
}
#[derive(Resource, Default)]
pub struct PlayerState {
    position: Vec3,
    rotation: Quat,
}
#[derive(Resource, Default)]
pub struct CameraState {
    position: Vec3,
    rotation: Quat,
}
/// # main function
/// This function initializes the nannou framework app
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "The Future of Mail".into(),
                    name: Some("bevy.app".into()),
                    resolution: (1280., 720.).into(),
                    present_mode: PresentMode::AutoVsync,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            AudioPlugin::default(),
        ))
        //.add_plugins(DefaultPlugins)
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .insert_resource(PlayerState::default())
        .insert_resource(CameraState::default())
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((splash::splash_plugin, menu::menu_plugin, game::game_plugin))
        .run();
}
fn setup(mut commands: Commands, query: Query<Entity, With<Camera>>) {
    if query.is_empty() {
        commands.spawn((
            Camera2d,
            Camera {
                order: 0,
                ..default()
            },
        ));
    }
}
mod splash {
    use super::{despawn_screen, GameState};
    use bevy::prelude::*;
    pub fn splash_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), splash_setup)
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
            .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
    }
    #[derive(Component)]
    struct OnSplashScreen;
    #[derive(Resource, Deref, DerefMut)]
    struct SplashTimer(Timer);
    fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let icon = asset_server.load("Images/load_clock_1.png");
        commands
            .spawn((
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                OnSplashScreen,
            ))
            .with_children(|parent| {
                parent.spawn((
                    ImageNode::new(icon),
                    Node {
                        width: Val::Px(200.0),
                        ..default()
                    },
                ));
            });
        commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
    }
    fn countdown(
        mut game_state: ResMut<NextState<GameState>>,
        time: Res<Time>,
        mut timer: ResMut<SplashTimer>,
    ) {
        if timer.tick(time.delta()).finished() {
            game_state.set(GameState::Menu);
        }
    }
}
mod game {

    use super::{despawn_screen, GameState};
    use crate::{CameraState, PlayerState, RotatableCamera};
    use bevy::{input::ButtonInput, prelude::*, audio::AudioPlugin};
    use rand::{prelude::SliceRandom, thread_rng};
    pub fn game_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(
                Update,
                (
                    game,
                    rotate_camera,
                    move_player,
                    return_to_main,
                    detect_collisions,
                )
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(
                OnExit(GameState::Game),
                (despawn_screen::<OnGameScreen>, despawn_models),
            );
    }
    #[derive(Component)]
    struct OnGameScreen;
    #[derive(Resource, Deref, DerefMut)]
    struct GameTimer(Timer);
    #[derive(Component, Default)]
    pub struct AtmosphereCamera;
    #[derive(Resource, Default)]
    pub struct AtmosphereModel;
    #[derive(Component)]
    struct SpawnedModel;
    #[derive(Component)]
    pub struct PlayerModel;
    #[derive(Component)]
    pub struct PeopleModel;
    #[derive(Component)]
    struct BuildingModel;
    #[derive(Component)]
    struct PlatformModel;
    #[derive(Resource, Deref, DerefMut)]
    struct PauseTimer(Timer);
    #[derive(Resource, Default)]
    struct PlayerPoints(u32);
    #[derive(Component)]
    struct CheckPointCube;
    fn rotate_camera(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut param_set: ParamSet<(
            Query<(&mut Transform, &mut RotatableCamera)>,
            Query<&Transform, With<PlayerModel>>,
        )>,
        time: Res<Time>,
    ) {
        let player_transform = param_set.p1().get_single().ok().cloned(); // Fetch player transform first

        let mut camera_query = param_set.p0();
        for (mut transform, mut camera) in camera_query.iter_mut() {
            let speed = 1.5 * time.delta_secs();

            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                camera.yaw += speed;
            }
            if keyboard_input.pressed(KeyCode::ArrowRight) {
                camera.yaw -= speed;
            }
            if keyboard_input.pressed(KeyCode::ArrowUp) {
                camera.pitch += speed;
            }
            if keyboard_input.pressed(KeyCode::ArrowDown) {
                camera.pitch -= speed;
            }

            camera.pitch = camera.pitch.clamp(
                -std::f32::consts::FRAC_PI_2 + 0.1,
                std::f32::consts::FRAC_PI_2 - 0.1,
            );

            if let Some(player_transform) = player_transform {
                let x = player_transform.translation.x
                    + camera.radius * camera.yaw.cos() * camera.pitch.cos();
                let y = player_transform.translation.y + camera.radius * camera.pitch.sin();
                let z = player_transform.translation.z
                    + camera.radius * camera.yaw.sin() * camera.pitch.cos();

                transform.translation = Vec3::new(x, y, z);
                transform.look_at(player_transform.translation, Vec3::Y);
            }
        }
    }
    fn move_player(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut query: Query<(&mut Transform, &mut PlayerModel)>,
        time: Res<Time>,
    ) {
        let speed = 50.0;
        let rotation_speed = 3.0;
        for (mut transform, _player_model) in query.iter_mut() {
            let mut direction = Vec3::ZERO;
            if keyboard_input.pressed(KeyCode::KeyW) {
                direction.x += 1.0;
            }
            if keyboard_input.pressed(KeyCode::KeyS) {
                direction.x -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::KeyA) {
                direction.z -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                direction.z += 1.0;
            }
            if direction != Vec3::ZERO {
                direction = direction.normalize();
                transform.translation += direction * speed * time.delta_secs();
                if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::KeyD) {
                    let target_rotation = Quat::from_rotation_arc(Vec3::Z, direction);
                    transform.rotation = transform
                        .rotation
                        .slerp(target_rotation, rotation_speed * time.delta_secs());
                }
            }
        }
    }
    fn detect_collisions(
        mut commands: Commands,
        mut game_state: ResMut<NextState<GameState>>,
        current_state: Res<State<GameState>>,
        mut player_points: ResMut<PlayerPoints>,
        player_query: Query<&Transform, With<PlayerModel>>,
        person_query: Query<(Entity, &Transform), With<SceneRoot>>,
        cube_query: Query<(Entity, &Transform), With<CheckPointCube>>,
        building_query: Query<&Transform, With<BuildingModel>>,
        platform_query: Query<&Transform, With<PlatformModel>>,
        time: Res<Time>,
        audio: Res<Audio>,
        asset_server: Res<AssetServer>,
        mut pause_timer: ResMut<PauseTimer>,
    ) {
        if let Ok(player_transform) = player_query.get_single() {
            for (entity, person_transform) in person_query.iter() {
                let distance = player_transform
                    .translation
                    .distance(person_transform.translation);
                if distance < 20.0 {
                    // Collision with person model
                    commands.entity(entity).despawn_recursive();
                    commands.spawn(AudioSourceBundle {
                        source: asset_server.load("audio/person_collision.ogg"),
                        settings: PlaybackSettings::ONCE,
                    });
                    pause_timer.0.reset();
                    game_state.set(GameState::Pause);
                    spawn_light_blue_cube(&mut commands);
                    break;
                }
            }
    
            for (entity, cube_transform) in cube_query.iter() {
                let distance = player_transform
                    .translation
                    .distance(cube_transform.translation);
                if distance < 20.0 {
                    // Collision with light-blue cube
                    commands.entity(entity).despawn_recursive();
                    commands.spawn(AudioSourceBundle {
                        source: asset_server.load("audio/person_collision.ogg"),
                        settings: PlaybackSettings::ONCE,
                    });
                    player_points.0 += 1;
                    spawn_person_model(&mut commands, &asset_server);
                    break;
                }
            }
        }
    
        if current_state.get() == &GameState::Pause && pause_timer.tick(time.delta()).finished() {
            game_state.set(GameState::Game);
        }
    }
    /// this is where the magic happens
    fn game_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        query: Query<Entity, With<PlayerModel>>,
        query2: Query<Entity, With<RotatableCamera>>,
    ) {
        // Spawn the atmosphere camera component
        if query2.is_empty() {
            commands
                .spawn((
                    Camera3d::default(),
                    Transform::from_xyz(-300.0, 300.0, -100.0).looking_at(Vec3::ZERO, Vec3::Y),
                    Camera {
                        order: 1,
                        ..default()
                    },
                    RotatableCamera {
                        radius: 350.0,
                        yaw: std::f32::consts::PI,
                        pitch: 1.0,
                    },
                ))
                .insert(AtmosphereCamera);
        }
        // Insert the default atmosphere model
        commands.insert_resource(AtmosphereModel);
        // Load and spawn the 3D model
        let building_list = [
            ("Models/building1.glb#Scene0"),
            ("Models/building1.glb#Scene0"),
            ("Models/building1.glb#Scene0"),
            ("Models/building1.glb#Scene0"),
            ("Models/building1.glb#Scene0"),
            ("Models/building1.glb#Scene0"),
            ("Models/building1.glb#Scene0"),
            ("Models/building1.glb#Scene0"),
            ("Models/building1.glb#Scene0"),
            ("Models/building2.glb#Scene0"),
            ("Models/building2.glb#Scene0"),
            ("Models/building2.glb#Scene0"),
            ("Models/building2.glb#Scene0"),
            ("Models/building2.glb#Scene0"),
            ("Models/building2.glb#Scene0"),
            ("Models/building2.glb#Scene0"),
            ("Models/building2.glb#Scene0"),
            ("Models/building3.glb#Scene0"),
            ("Models/building3.glb#Scene0"),
            ("Models/building3.glb#Scene0"),
            ("Models/building3.glb#Scene0"),
            ("Models/building3.glb#Scene0"),
            ("Models/building3.glb#Scene0"),
            ("Models/building3.glb#Scene0"),
            ("Models/building3.glb#Scene0"),
            ("Models/building3.glb#Scene0"),
            ("Models/building4.glb#Scene0"),
            ("Models/building4.glb#Scene0"),
            ("Models/building4.glb#Scene0"),
            ("Models/building4.glb#Scene0"),
            ("Models/building4.glb#Scene0"),
            ("Models/building4.glb#Scene0"),
            ("Models/building4.glb#Scene0"),
            ("Models/building4.glb#Scene0"),
            ("Models/building4.glb#Scene0"),
            ("Models/building5.glb#Scene0"),
            ("Models/building5.glb#Scene0"),
            ("Models/building5.glb#Scene0"),
            ("Models/building5.glb#Scene0"),
            ("Models/building5.glb#Scene0"),
            ("Models/building5.glb#Scene0"),
            ("Models/building5.glb#Scene0"),
            ("Models/building5.glb#Scene0"),
        ];
        let building_coords = [
            // coordinates for building type 1
            (25.0, 1.1, 25.0, 0.0, 0.0, 0.0),
            (70.0, 1.1, 25.0, 0.0, 0.0, 0.0),
            (-70.0, 1.1, -25.0, 0.0, 0.0, 0.0),
            (-25.0, 1.1, -170.0, 0.0, 0.0, 0.0),
            (-70.0, 1.1, 75.0, 0.0, 0.0, 0.0),
            (-75.0, 1.1, -125.0, 0.0, 0.0, 0.0),
            (-165.0, 1.1, -25.0, 0.0, 0.0, 0.0),
            (-165.0, 1.1, 75.0, 0.0, 0.0, 0.0),
            (120.0, 1.1, -125.0, 0.0, 0.0, 0.0),
            // coordinates for building type 2
            (70.0, 1.1, -40.0, 0.0, 0.0, 0.0),
            (25.0, 1.1, 90.0, 0.0, 0.0, 0.0),
            (-120.0, 1.1, -40.0, 0.0, 0.0, 0.0),
            (-25.0, 1.1, 90.0, 0.0, 0.0, 0.0),
            (70.0, 1.1, -120.0, 0.0, 0.0, 0.0),
            (-120.0, 1.1, 90.0, 0.0, 0.0, 0.0),
            (120.0, 1.1, 90.0, 0.0, 0.0, 0.0),
            (165.0, 1.1, 45.0, 0.0, 0.0, 0.0),
            // coordinates for building type 3
            (-25.0, 1.1, 25.0, 0.0, 0.0, 0.0),
            (25.0, 1.1, 155.0, 0.0, 0.0, 0.0),
            (25.0, 1.1, -170.0, 0.0, 0.0, 0.0),
            (-25.0, 1.1, 155.0, 0.0, 0.0, 0.0),
            (-70.0, 1.1, -75.0, 0.0, 0.0, 0.0),
            (-120.0, 1.1, -105.0, 0.0, 0.0, 0.0),
            (160.0, 1.1, -25.0, 0.0, 0.0, 0.0),
            (120.0, 1.1, 25.0, 0.0, 0.0, 0.0),
            (160.0, 1.1, -75.0, 0.0, 0.0, 0.0),
            // coordinates for building type 4
            (25.0, 1.1, -25.0, 0.0, 0.0, 0.0),
            (25.0, 1.1, -75.0, 0.0, 0.0, 0.0),
            (-25.0, 1.1, -75.0, 0.0, 0.0, 0.0),
            (-70.0, 1.1, 25.0, 0.0, 0.0, 0.0),
            (70.0, 1.1, 75.0, 0.0, 0.0, 0.0),
            (-70.0, 1.1, 120.0, 0.0, 0.0, 0.0),
            (120.0, 1.1, -25.0, 0.0, 0.0, 0.0),
            (-165.0, 1.1, 25.0, 0.0, 0.0, 0.0),
            (-70.0, 1.1, 165.0, 0.0, 0.0, 0.0),
            //coordinates for building type 5
            (-25.0, 1.1, -25.0, 0.0, 0.0, 0.0),
            (25.0, 1.1, -125.0, 0.0, 0.0, 0.0),
            (-25.0, 1.1, -125.0, 0.0, 0.0, 0.0),
            (-120.0, 1.1, 25.0, 0.0, 0.0, 0.0),
            (70.0, 1.1, 120.0, 0.0, 0.0, 0.0),
            (120.0, 1.1, -75.0, 0.0, 0.0, 0.0),
            (-160.0, 1.1, -75.0, 0.0, 0.0, 0.0),
            (70.0, 1.1, 165.0, 0.0, 0.0, 0.0),
        ];
        
        let model_handle = asset_server.load("Models/island.glb#Scene0");
        commands.spawn((
            SceneRoot(model_handle),
            Transform::from_xyz(0.0, 0.0, 0.0),
            PlatformModel,
        ));
        if query.is_empty() {
            let player_model = asset_server.load("Models/bot_main.glb#Scene0");
            commands.spawn((
                SceneRoot(player_model),
                Transform::from_xyz(0.0, 1.1, 0.0),
                PlayerModel,
            ));
        }
        for (model, coords) in building_list.iter().zip(building_coords.iter()) {
            let building = asset_server.load(*model);
            let rotation = Quat::from_euler(EulerRot::XYZ, coords.3, coords.4, coords.5);
            commands.spawn((
                SceneRoot(building),
                Transform {
                    translation: Vec3::new(coords.0, coords.1, coords.2),
                    rotation,
                    ..default()
                },
                SpawnedModel,
                BuildingModel,
            ));
        }
        
        commands.insert_resource(PlayerPoints::default());
        spawn_person_model(&mut commands, &asset_server);
        commands.insert_resource(GameTimer(Timer::from_seconds(60.0, TimerMode::Repeating)));
    }
    fn despawn_models(
        mut commands: Commands,
        query: Query<(Entity, Option<&PlayerModel>), With<SpawnedModel>>,
    ) {
        for (entity, is_player) in query.iter() {
            if is_player.is_none() {
                // Only despawn non-player models
                commands.entity(entity).despawn_recursive();
            }
        }
    }
    fn spawn_person_model(commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let people_coords = [
            (165.0, 1.1, 3.0),
            (145.0, 1.1, 110.0),
            (145.0, 1.1, -100.0),
            (95.0, 1.1, 155.0),
            (95.0, 1.1, -150.0),
            (45.0, 1.1, 180.0),
            (50.0, 1.1, -180.0),
            (0.0, 1.1, -190.0),
            (0.0, 1.1, 190.0),
            (-50.0, 1.1, 180.0),
            (-45.0, 1.1, -180.0),
            (-95.0, 1.1, 150.0),
            (-95.0, 1.1, -155.0),
            (-145.0, 1.1, 100.0),
            (-145.0, 1.1, -110.0),
            (-165.0, 1.1, 0.0),
        ];
        let person_model = asset_server.load("Models/person.glb#Scene0");
        let mut rng = thread_rng();
        let random_coord = people_coords.choose(&mut rng).unwrap();
        commands.spawn((
            SceneRoot(person_model),
            Transform::from_xyz(random_coord.0, random_coord.1, random_coord.2),
            PeopleModel,
            SpawnedModel,
        ));
    }
    fn spawn_light_blue_cube(commands: &mut Commands) {
        let people_coords = [
            (165.0, 1.1, 3.0),
            (145.0, 1.1, 110.0),
            (145.0, 1.1, -100.0),
            (95.0, 1.1, 155.0),
            (95.0, 1.1, -150.0),
            (45.0, 1.1, 180.0),
            (50.0, 1.1, -180.0),
            (0.0, 1.1, -190.0),
            (0.0, 1.1, 190.0),
            (-50.0, 1.1, 180.0),
            (-45.0, 1.1, -180.0),
            (-95.0, 1.1, 150.0),
            (-95.0, 1.1, -155.0),
            (-145.0, 1.1, 100.0),
            (-145.0, 1.1, -110.0),
            (-165.0, 1.1, 0.0),
        ];
        let mut rng = thread_rng();
        let random_coord = people_coords.choose(&mut rng).unwrap();
        commands.spawn((
            Transform::from_xyz(random_coord.0, random_coord.1, random_coord.2),
            CheckPointCube,
            SpawnedModel,
        ));
    }

    fn game(
        time: Res<Time>,
        mut game_state: ResMut<NextState<GameState>>,
        mut timer: ResMut<GameTimer>,
    ) {
        if timer.tick(time.delta()).finished() {
            game_state.set(GameState::Menu);
        }
    }
    fn return_to_main(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut game_state: ResMut<NextState<GameState>>,
        player_query: Query<&Transform, With<PlayerModel>>,
        camera_query: Query<&Transform, With<RotatableCamera>>,
        mut player_state: ResMut<PlayerState>,
        mut camera_state: ResMut<CameraState>,
        mut is_resuming: Local<bool>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            // Save player position and rotation
            if let Ok(player_transform) = player_query.get_single() {
                player_state.position = player_transform.translation;
                player_state.rotation = player_transform.rotation;
            }

            // Save camera position and rotation
            if let Ok(camera_transform) = camera_query.get_single() {
                camera_state.position = camera_transform.translation;
                camera_state.rotation = camera_transform.rotation;
            }

            // Pause the game
            game_state.set(GameState::Pause);
            *is_resuming = true;
        }
    }
}

mod menu {
    use super::CameraState;
    use super::PlayerState;
    use super::RotatableCamera;
    use crate::game::PlayerModel;
    use bevy::{app::AppExit, color::palettes::css::CRIMSON, prelude::*};

    use super::{despawn_screen, GameState, TXT_CLR};

    // This plugin manages the menu, with 5 different screens:
    // - a main menu with "New Game", "Settings", "Quit"
    // - a settings menu with two submenus and a back button
    // - two settings screen with a setting that can be set and a back button
    pub fn menu_plugin(app: &mut App) {
        app.init_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), menu_setup)
            // Systems to handle the main menu screen
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
            // Systems to handle the help menu screen
            .add_systems(OnEnter(MenuState::Help), help_menu_setup)
            .add_systems(OnExit(MenuState::Help), despawn_screen::<OnHelpMenuScreen>)
            // Systems to handle the pause menu screen
            .add_systems(OnEnter(GameState::Pause), pause_menu_setup)
            .add_systems(
                OnExit(GameState::Pause),
                despawn_screen::<OnPauseMenuScreen>,
            )
            .add_systems(OnEnter(GameState::Lose), lose_menu_setup)
            .add_systems(OnExit(GameState::Lose), despawn_screen::<OnLoseMenuScreen>)
            // Common systems to all screens that handles buttons behavior
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(GameState::Menu)),
            )
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(GameState::Pause)),
            )
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(GameState::Lose)),
            );
    }

    // State used for the current menu screen
    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    enum MenuState {
        Main,
        Help,
        #[default]
        Disabled,
    }

    // Tag component used to tag entities added on the main menu screen
    #[derive(Component)]
    struct OnMainMenuScreen;
    #[derive(Component)]
    struct OnHelpMenuScreen;
    #[derive(Component)]
    struct OnPauseMenuScreen;
    #[derive(Component)]
    struct OnLoseMenuScreen;

    const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
    const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
    const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

    // Tag component used to mark which setting is currently selected
    #[derive(Component)]
    struct SelectedOption;

    // All actions that can be triggered from a button click
    #[derive(Component)]
    enum MenuButtonAction {
        Play,
        Help,
        BackToMainMenu,
        Quit,
    }

    // This system handles changing all buttons color based on mouse interaction
    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut background_color, selected) in &mut interaction_query {
            *background_color = match (*interaction, selected) {
                (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
                (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
                (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
                (Interaction::None, None) => NORMAL_BUTTON.into(),
            }
        }
    }

    fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
        menu_state.set(MenuState::Main);
    }

    fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        // Common style for all buttons on the screen
        let button_node = Node {
            width: Val::Px(300.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_icon_node = Node {
            width: Val::Px(30.0),
            // This takes the icons out of the flexbox flow, to be positioned exactly
            position_type: PositionType::Absolute,
            // The icon will be close to the left border of the button
            left: Val::Px(10.0),
            ..default()
        };
        let button_text_font = TextFont {
            font_size: 33.0,
            ..default()
        };

        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                OnMainMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(CRIMSON.into()),
                    ))
                    .with_children(|parent| {
                        // Display the game name
                        parent.spawn((
                            Text::new("THE FUTURE OF MAIL"),
                            TextFont {
                                font_size: 67.0,
                                ..default()
                            },
                            TextColor(TXT_CLR),
                            Node {
                                margin: UiRect::all(Val::Px(50.0)),
                                ..default()
                            },
                        ));

                        // Display three buttons for each action available from the main menu:
                        // - new game
                        // - settings
                        // - Help
                        // - quit
                        parent
                            .spawn((
                                Button,
                                button_node.clone(),
                                BackgroundColor(NORMAL_BUTTON),
                                MenuButtonAction::Play,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("Images/new_game.png");
                                parent.spawn((ImageNode::new(icon), button_icon_node.clone()));
                                parent.spawn((
                                    Text::new("New Game"),
                                    button_text_font.clone(),
                                    TextColor(TXT_CLR),
                                ));
                            });
                        parent
                            .spawn((
                                Button,
                                button_node.clone(),
                                BackgroundColor(NORMAL_BUTTON),
                                MenuButtonAction::Help,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("Images/Help.png");
                                parent.spawn((ImageNode::new(icon), button_icon_node.clone()));
                                parent.spawn((
                                    Text::new("Help"),
                                    button_text_font.clone(),
                                    TextColor(TXT_CLR),
                                ));
                            });
                        parent
                            .spawn((
                                Button,
                                button_node.clone(),
                                BackgroundColor(NORMAL_BUTTON),
                                MenuButtonAction::Quit,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("Images/exit.png");
                                parent.spawn((ImageNode::new(icon), button_icon_node.clone()));
                                parent.spawn((
                                    Text::new("Quit"),
                                    button_text_font.clone(),
                                    TextColor(TXT_CLR),
                                ));
                            });
                    });
            });
    }
    fn help_menu_setup(mut commands: Commands) {
        let button_node = Node {
            width: Val::Px(200.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        let button_text_style = (
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(TXT_CLR),
        );

        let controls_text_style = (
            TextFont {
                font_size: 25.0,
                ..default()
            },
            TextColor(TXT_CLR),
        );

        let controls = [
            ("esc", "return to main menu"),
            ("w", "forward"),
            ("s", "backward"),
            ("a", "left"),
            ("d", "right"),
            ("up arrow", "camera angle down"),
            ("down arrow", "camera angle up"),
            ("left arrow", "camera angle right"),
            ("right arrow", "camera angle left"),
        ];

        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                OnHelpMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(CRIMSON.into()),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Help Menu"),
                            TextFont {
                                font_size: 67.0,
                                ..default()
                            },
                            TextColor(TXT_CLR),
                            Node {
                                margin: UiRect::all(Val::Px(50.0)),
                                ..default()
                            },
                        ));

                        for (key, action) in controls.iter() {
                            parent
                                .spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::all(Val::Px(10.0)),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new(*key),
                                        controls_text_style.clone(),
                                        Node {
                                            width: Val::Px(150.0),
                                            ..default()
                                        },
                                    ));
                                    parent.spawn((
                                        Text::new("\t"),
                                        controls_text_style.clone(),
                                        Node {
                                            width: Val::Px(10.0),
                                            ..default()
                                        },
                                    ));
                                    parent.spawn((
                                        Text::new(*action),
                                        controls_text_style.clone(),
                                        Node {
                                            width: Val::Px(300.0),
                                            ..default()
                                        },
                                    ));
                                });
                        }

                        parent
                            .spawn((
                                Button,
                                button_node.clone(),
                                BackgroundColor(NORMAL_BUTTON),
                                MenuButtonAction::BackToMainMenu,
                            ))
                            .with_children(|parent| {
                                parent.spawn((Text::new("Back"), button_text_style.clone()));
                            });
                    });
            });
    }
    fn pause_menu_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        query: Query<Entity, With<Camera>>,
    ) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        commands.spawn((
            Camera2d,
            Camera {
                order: 0, // Ensure unique order
                ..default()
            },
        ));
        let button_node = Node {
            width: Val::Px(300.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_icon_node = Node {
            width: Val::Px(30.0),
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            ..default()
        };
        let button_text_font = TextFont {
            font_size: 33.0,
            ..default()
        };

        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                OnPauseMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(CRIMSON.into()),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Pause Menu"),
                            TextFont {
                                font_size: 67.0,
                                ..default()
                            },
                            TextColor(TXT_CLR),
                            Node {
                                margin: UiRect::all(Val::Px(50.0)),
                                ..default()
                            },
                        ));

                        parent
                            .spawn((
                                Button,
                                button_node.clone(),
                                BackgroundColor(NORMAL_BUTTON),
                                MenuButtonAction::Play,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("Images/new_game.png");
                                parent.spawn((ImageNode::new(icon), button_icon_node.clone()));
                                parent.spawn((
                                    Text::new("Resume"),
                                    button_text_font.clone(),
                                    TextColor(TXT_CLR),
                                ));
                            });

                        parent
                            .spawn((
                                Button,
                                button_node.clone(),
                                BackgroundColor(NORMAL_BUTTON),
                                MenuButtonAction::Help,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("Images/Help.png");
                                parent.spawn((ImageNode::new(icon), button_icon_node.clone()));
                                parent.spawn((
                                    Text::new("Help"),
                                    button_text_font.clone(),
                                    TextColor(TXT_CLR),
                                ));
                            });

                        parent
                            .spawn((
                                Button,
                                button_node.clone(),
                                BackgroundColor(NORMAL_BUTTON),
                                MenuButtonAction::Quit,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("Images/exit.png");
                                parent.spawn((ImageNode::new(icon), button_icon_node.clone()));
                                parent.spawn((
                                    Text::new("Quit"),
                                    button_text_font.clone(),
                                    TextColor(TXT_CLR),
                                ));
                            });
                    });
            });
    }
    fn lose_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let button_node = Node {
            width: Val::Px(300.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_text_style = (
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(TXT_CLR),
        );

        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                OnLoseMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(CRIMSON.into()),
                    ))
                    .with_children(|parent| {
                        let icon = asset_server.load("Images/Lose_Icon.png");
                        parent.spawn((
                            ImageNode::new(icon),
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(200.0),
                                ..default()
                            },
                        ));

                        parent
                            .spawn((
                                Button,
                                button_node.clone(),
                                BackgroundColor(NORMAL_BUTTON),
                                MenuButtonAction::Quit,
                            ))
                            .with_children(|parent| {
                                parent.spawn((Text::new("Exit"), button_text_style.clone()));
                            });
                    });
            });
    }
    fn menu_action(
        interaction_query: Query<
            (&Interaction, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_events: EventWriter<AppExit>,
        mut menu_state: ResMut<NextState<MenuState>>,
        mut game_state: ResMut<NextState<GameState>>,
        mut param_set: ParamSet<(
            Query<&mut Transform, With<PlayerModel>>,
            Query<&mut Transform, With<RotatableCamera>>,
        )>,
        player_state: Res<PlayerState>,
        camera_state: Res<CameraState>,
        mut is_resuming: Local<bool>,
    ) {
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    MenuButtonAction::Quit => {
                        app_exit_events.send(AppExit::Success);
                    }
                    MenuButtonAction::Play => {
                        if *is_resuming {
                            // Restore saved positions
                            if let Ok(mut player_transform) = param_set.p0().get_single_mut() {
                                player_transform.translation = player_state.position;
                                player_transform.rotation = player_state.rotation;
                            }

                            if let Ok(mut camera_transform) = param_set.p1().get_single_mut() {
                                camera_transform.translation = camera_state.position;
                                camera_transform.rotation = camera_state.rotation;
                            }
                        }
                        game_state.set(GameState::Game);
                        menu_state.set(MenuState::Disabled);
                        *is_resuming = false;
                    }
                    MenuButtonAction::BackToMainMenu => {
                        game_state.set(GameState::Menu);
                        menu_state.set(MenuState::Main);
                    }
                    MenuButtonAction::Help => {
                        game_state.set(GameState::Help);
                        menu_state.set(MenuState::Help);
                    }
                }
            }
        }
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
