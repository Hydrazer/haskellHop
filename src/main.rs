use bevy::prelude::*;
// use bevy::prelude::stage::*;
use bevy::time::{FixedTimestep, FixedTimesteps};
use bevy::window::PresentMode;

const BACKGROUND_COLOR: Color = Color::rgb(0.7, 0.3, 0.3);
const PLAYER_SCALE: f32 = 0.15;
const WINDOW_HEIGHT: f32 = 500.0;
const WINDOW_WIDTH: f32 = 1000.0;
const GRAVITY: f32 = 9.81;
const FRICTION: f32 = 0.7;

#[derive(PartialEq)]
enum ScoreState {
  NICE,
  DEFAULT,
}

impl Default for ScoreState {
  fn default() -> ScoreState {
    ScoreState::DEFAULT
  }
}

#[derive(PartialEq, Eq)]
enum Dir {
  LEFT,
  RIGHT,
}

impl Default for Dir {
  fn default() -> Dir {
    Dir::RIGHT
  }
}

#[derive(Default, PartialEq)]
struct Player {
  entity: Option<Entity>,
  i: f32,
  j: f32,
  vel_i: f32,
  vel_j: f32,
  scale: f32,
  jump_count: usize,
  dir: Dir,
  score_state: ScoreState,
  time_save: usize,
}

#[derive(Component)]
struct ScoreRotate;

fn main() {
  App::new()
    .init_resource::<Player>()
    .insert_resource(WindowDescriptor {
      title: "haskellHop".to_string(),
      width: WINDOW_WIDTH,
      height: WINDOW_HEIGHT,
      present_mode: PresentMode::AutoVsync,
      ..default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_stage_after(
      CoreStage::Update,
      "player_move",
      SystemStage::parallel()
        .with_run_criteria(FixedTimestep::step(1.0 / 60.0))
        .with_system(player_move),
    )
    .add_stage_after(
      CoreStage::Update,
      "score_update",
      SystemStage::parallel()
        .with_run_criteria(FixedTimestep::step(1.0 / 60.0))
        .with_system(score_update),
    )
    /* .add_system(player_move)
    .add_system(score_update) */
    .insert_resource(ClearColor(BACKGROUND_COLOR))
    .run();
}

fn player_move(
  time: Res<Time>,
  keyboard_input: Res<Input<KeyCode>>,
  mut player: ResMut<Player>,
  mut transform_q: Query<&mut Transform>,
  mut sprite_q: Query<&mut Sprite>,
) {
  // let td = time.delta().as_millis() as f32 / 60.0;

  if (keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Space)) &&
    player.vel_i == 0.0
  {
    player.jump_count += 1;
    player.vel_i = 10.0;
  }

  if keyboard_input.pressed(KeyCode::D) {
    player.vel_j += 0.4;
    player.dir = Dir::RIGHT;
  }

  if keyboard_input.pressed(KeyCode::A) {
    player.vel_j -= 0.4;
    player.dir = Dir::LEFT;
  }

  let floor = -(WINDOW_HEIGHT / 2.0) + (WINDOW_HEIGHT * 0.1);

  player.j += player.vel_j;

  if player.i + player.vel_i < floor {
    player.i = floor;
    player.vel_i = 0.0;

    player.vel_j = player.vel_j * FRICTION;
  } else {
    player.i = player.i + player.vel_i;
    player.vel_i -= GRAVITY / 10.0;
  }

  if player.jump_count == 69 {
    if player.score_state != ScoreState::NICE {
      player.score_state = ScoreState::NICE;
      player.time_save = time.time_since_startup().as_millis() as usize;
    }
  } else if time.time_since_startup().as_millis() as usize - player.time_save >= 1000 {
    player.score_state = ScoreState::DEFAULT;
  }

  *transform_q.get_mut(player.entity.unwrap()).unwrap() = Transform {
    translation: Vec3::new(player.j, player.i, 0.0),
    scale: Vec3::new(player.scale, player.scale, 0.0),

    ..default()
  };

  *sprite_q.get_mut(player.entity.unwrap()).unwrap() = Sprite {
    flip_x: match player.dir {
      Dir::LEFT => true,
      Dir::RIGHT => false,
    },
    flip_y: false,
    ..default()
  };
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut player: ResMut<Player>) {
  commands.spawn_bundle(Camera2dBundle::default());
  let font = asset_server.load("fonts/Monocraft.ttf");
  let text_style = TextStyle {
    font,
    font_size: 60.0,
    color: Color::WHITE,
  };
  let text_alignment = TextAlignment::CENTER;

  player.jump_count = 0;

  commands
    .spawn_bundle(Text2dBundle {
      text: Text::from_section(player.jump_count.to_string().as_str(), text_style.clone())
        .with_alignment(text_alignment),
      ..default()
    })
    .insert(ScoreRotate);

  player.i = -(WINDOW_HEIGHT / 2.0) + (WINDOW_HEIGHT * 0.1);
  player.j = -(WINDOW_WIDTH / 2.0) + (WINDOW_HEIGHT * 0.1);
  player.vel_i = 0.0;
  player.vel_j = 0.0;
  player.scale = PLAYER_SCALE;

  player.entity = Some(
    commands
      .spawn_bundle(SpriteBundle {
        texture: asset_server.load("textures/haskell.png"),
        transform: Transform {
          scale: Vec3::new(player.scale, player.scale, 0.0),
          translation: Vec3::new(player.j, player.i, 0.0),
          ..default()
        },
        sprite: Sprite {
          flip_x: true,
          flip_y: false,
          ..default()
        },
        ..default()
      })
      .id(),
  );
}

fn score_update(
  time: Res<Time>,
  mut transform_q: Query<&mut Transform, (With<Text>, With<ScoreRotate>)>,
  mut text_q: Query<&mut Text>,
  mut player: ResMut<Player>,
) {
  // let td = time.delta().as_millis() as f32 / 60.0;
  for mut text in &mut text_q {
    match player.score_state {
      ScoreState::DEFAULT => {
        text.sections[0].value = player.jump_count.to_string();
        text.sections[0].style.color = Color::WHITE;
      }
      ScoreState::NICE => {
        text.sections[0].value = "haha funny number".to_string();
        text.sections[0].style.color = Color::CYAN;
      }
    }
  }

  for mut transform in &mut transform_q {
    transform.rotate_z(5_f32.to_radians());
  }
}
