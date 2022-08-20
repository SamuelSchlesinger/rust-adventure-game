use bevy::{prelude::*, render::texture::ImageSettings, time::FixedTimestep};

const ARENA_WIDTH: i32 = 20;
const ARENA_HEIGHT: i32 = 20;

#[derive(Component)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Component, Clone, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Moving(bool, bool);

#[derive(Component, Debug)]
struct Tile;

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct AdventureTitle;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Adventure".to_string(),
            width: 1500.,
            height: 1500.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(animate_player_sprite)
        .add_system(animate_tiles)
        .add_system(change_player_direction)
        .add_system(move_player)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.3))
                .with_system(entity_walk),
        )
        .run();
}

fn change_player_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Direction, With<Player>>,
) {
    if let Some(mut direction) = query.iter_mut().next() {
        if keyboard_input.pressed(KeyCode::W) {
            *direction = Direction::North;
        }
        if keyboard_input.pressed(KeyCode::A) {
            *direction = Direction::West;
        }
        if keyboard_input.pressed(KeyCode::S) {
            *direction = Direction::South;
        }
        if keyboard_input.pressed(KeyCode::D) {
            *direction = Direction::East;
        }
    }
}

fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Moving, With<Player>>) {
    if keyboard_input.just_released(KeyCode::Space) {
        if let Some(mut moving) = query.iter_mut().next() {
            moving.0 = !moving.0;
        }
    }
}

fn entity_walk(mut query: Query<(&Direction, &mut Moving, &mut Position)>) {
    for (direction, mut moving, mut position) in query.iter_mut() {
        if moving.0 {
            moving.1 = !moving.1;
            let previous_position = position.clone();
            match direction {
                Direction::North => {
                    position.y = std::cmp::min(position.y + 1, ARENA_HEIGHT - 1);
                }
                Direction::South => {
                    position.y = std::cmp::max(position.y - 1, 0);
                }
                Direction::East => {
                    position.x = std::cmp::min(position.x + 1, ARENA_WIDTH - 1);
                }
                Direction::West => {
                    position.x = std::cmp::max(position.x - 1, 0);
                }
            }
            if *position == previous_position {
                moving.0 = false;
            }
        }
    }
}

const PLAYER_SPRITE_NORTH: usize = 40;
const PLAYER_SPRITE_SOUTH: usize = 4;
const PLAYER_SPRITE_EAST: usize = 28;
const PLAYER_SPRITE_WEST: usize = 16;

fn center_sprite_for(direction: &Direction) -> usize {
    match direction {
        Direction::North => PLAYER_SPRITE_NORTH,
        Direction::South => PLAYER_SPRITE_SOUTH,
        Direction::East => PLAYER_SPRITE_EAST,
        Direction::West => PLAYER_SPRITE_WEST,
    }
}

fn body_sprite_for(direction: &Direction, moving: &Moving) -> usize {
    let center_sprite_index = center_sprite_for(direction);
    if moving.0 == true {
        if moving.1 == true {
            center_sprite_index + 1
        } else {
            center_sprite_index - 1
        }
    } else {
        center_sprite_index
    }
}

fn animate_player_sprite(
    windows: Res<Windows>,
    mut query: Query<
        (
            &Direction,
            &Moving,
            &Position,
            &mut TextureAtlasSprite,
            &mut Transform,
        ),
        With<Player>,
    >,
) {
    if let Some((direction, moving, position, mut sprite, mut transform)) = query.iter_mut().next()
    {
        if let Some(window) = windows.get_primary() {
            sprite.index = body_sprite_for(direction, moving);
            transform.translation = Vec3::new(
                convert(position.x as f32, window.width() as f32, ARENA_WIDTH as f32),
                convert(
                    position.y as f32,
                    window.height() as f32,
                    ARENA_HEIGHT as f32,
                ),
                0.0,
            );
        }
    }
}

fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
    let tile_size = bound_window / bound_game;
    pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
}

fn animate_tiles(
    windows: Res<Windows>,
    mut query: Query<(&Position, &mut TextureAtlasSprite, &mut Transform), With<Tile>>,
) {
    for (position, mut sprite, mut transform) in query.iter_mut() {
        sprite.index = 5;
        if let Some(window) = windows.get_primary() {
            transform.translation = Vec3::new(
                convert(position.x as f32, window.width() as f32, ARENA_WIDTH as f32),
                convert(
                    position.y as f32,
                    window.height() as f32,
                    ARENA_HEIGHT as f32,
                ),
                0.0,
            );
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let characters_texture_handle = asset_server.load("characters.png");
    let characters_texture_atlas =
        TextureAtlas::from_grid(characters_texture_handle, Vec2::new(16.0, 16.0), 12, 8);
    let characters_texture_atlas_handle = texture_atlases.add(characters_texture_atlas);

    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: characters_texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        })
        .insert(Direction::North)
        .insert(Position::new(0, 0))
        .insert(Moving(false, true))
        .insert(Player);
    let basictiles_texture_handle = asset_server.load("basictiles.png");
    let basictiles_texture_atlas =
        TextureAtlas::from_grid(basictiles_texture_handle, Vec2::new(16.0, 16.0), 8, 4);
    let basictiles_texture_atlas_handle = texture_atlases.add(basictiles_texture_atlas);
    for y in 0..ARENA_HEIGHT {
        for x in 0..ARENA_WIDTH {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: basictiles_texture_atlas_handle.clone(),
                    transform: Transform::from_scale(Vec3::splat(6.0)),
                    ..default()
                })
                .insert(Position { x, y })
                .insert(Tile);
        }
    }
    commands
        .spawn_bundle(
            TextBundle::from_section(
                "Adventure!",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(50.0),
                    right: Val::Px(50.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(AdventureTitle);
}
