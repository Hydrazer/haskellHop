use bevy::prelude::*;
use itertools::Itertools;
use rand::{prelude::SliceRandom, thread_rng, Rng};
// use bevy::prelude::stage::*;
use bevy::time::{FixedTimestep, FixedTimesteps};
use bevy::window::PresentMode;

const BACKGROUND_COLOR: Color = Color::rgb(0.7, 0.3, 0.3);
const PLAYER_SCALE: f32 = 0.15;
const WINDOW_HEIGHT: f32 = 500.0;
const WINDOW_WIDTH: f32 = 1000.0;
const GRAVITY: f32 = 9.81;
const FRICTION: f32 = 0.7;

#[derive(PartialEq, Debug)]
enum State {
  NICE,
  DEFAULT,
  CORRUPT_B,
  CORRUPT_A,
  JAVA,
}

impl Default for State {
  fn default() -> State {
    State::DEFAULT
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
struct Texter {
  time_save: usize,
  state: State,
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
  // state: State,
}

#[derive(Default, PartialEq, Clone)]
struct Obj {
  entity: Option<Entity>,
  i: f32,
  j: f32,
  vel_i: f32,
  vel_j: f32,
  scale: f32,
}

#[derive(Default, PartialEq)]
struct Java {
  entity: Option<Entity>,
  colour: Color,
  i: f32,
  j: f32,
  vel_i: f32,
  vel_j: f32,
  scale: f32,
  dir: Dir,
  time_save: usize,
  obj_vec: Vec<Obj>,
  // time_save: usize,
  // state: State,
}

#[derive(Component)]
struct ScoreRotate;

fn main() {
  App::new()
    .init_resource::<Player>()
    .init_resource::<Java>()
    .init_resource::<Texter>()
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
    .add_system(texter_state)
    .add_stage_after(
      CoreStage::Update,
      "score_corrupt",
      SystemStage::parallel()
        .with_run_criteria(FixedTimestep::step(1.0 / 2.0))
        .with_system(score_corrupt),
    )
    .add_stage_after(
      CoreStage::Update,
      "java_move",
      SystemStage::parallel()
        .with_run_criteria(FixedTimestep::step(1.0 / 60.0))
        .with_system(java_move),
    )
    .insert_resource(ClearColor(BACKGROUND_COLOR))
    .run();
}

fn texter_state(
  mut player: ResMut<Player>,
  time: Res<Time>,
  mut texter: ResMut<Texter>,

  mut transform_q: Query<&mut Transform>,
) {
  if player.jump_count == 69 {
    if texter.state != State::NICE {
      texter.state = State::NICE;
      texter.time_save = time.time_since_startup().as_millis() as usize;
    }
  } else if texter.state == State::NICE &&
    time.time_since_startup().as_millis() as usize - texter.time_save >= 1000
  {
    texter.state = State::DEFAULT;
  }

  if player.jump_count >= 80 && texter.state == State::DEFAULT {
    texter.state = State::CORRUPT_A;
    texter.time_save = time.time_since_startup().as_millis() as usize;
  }
}

fn java_move(
  mut java: ResMut<Java>,
  mut texter: ResMut<Texter>,
  time: Res<Time>,
  mut transform_q: Query<&mut Transform>,

  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  if texter.state == State::JAVA {
    let mut rng = thread_rng();
    let tc = time.time_since_startup().as_millis() as usize;
    if tc - java.time_save >= 2000 {
      java.vel_j = vec![-5.0, 5.0].choose(&mut rng).unwrap() * 1.0;
      java.time_save = tc;
      let obj_scale = java.scale / 5.0;
      let obj_i = java.i;
      let obj_j = java.j;

      let obj = Obj {
        i: obj_i,
        j: obj_j,
        vel_i: -1.0,
        vel_j: 0.0,
        scale: obj_scale,
        entity: Some(
          commands
            .spawn_bundle(SpriteBundle {
              texture: asset_server.load("textures/apple.png"),
              transform: Transform {
                scale: Vec3::new(obj_scale, obj_scale, 0.0),
                translation: Vec3::new(obj_j, obj_i, 0.0),
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
        ),
      };
      java.obj_vec.push(obj);
    }

    for obj in &mut java.obj_vec {
      if transform_q.get_mut(obj.entity.unwrap()).is_ok() {
        obj.i += obj.vel_i;
        *transform_q.get_mut(obj.entity.unwrap()).unwrap() = Transform {
          translation: Vec3::new(obj.j, obj.i, 0.0),
          scale: Vec3::new(obj.scale, obj.scale, 0.0),
          ..default()
        };
      }
    }

    if java.j <= -(WINDOW_WIDTH / 2.0) + (WINDOW_WIDTH * 0.1) {
      java.vel_j = java.vel_j.abs();
    } else if java.j >= (WINDOW_WIDTH / 2.0) - (WINDOW_WIDTH * 0.1) {
      java.vel_j = -(java.vel_j.abs());
    }

    if texter.state == State::JAVA {
      java.j += java.vel_j;
      *transform_q.get_mut(java.entity.unwrap()).unwrap() = Transform {
        translation: Vec3::new(java.j, java.i, 0.0),

        scale: Vec3::new(java.scale, java.scale, 0.0),
        ..default()
      };
    }
  }
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

fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut player: ResMut<Player>,
  mut java: ResMut<Java>,
) {
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

  java.entity = None;
  java.colour = Color::rgba(1.0, 1.0, 1.0, 0.0);
  java.i = (WINDOW_HEIGHT / 2.0) - (WINDOW_HEIGHT * 0.1);
  java.j = 0.0;
  java.vel_i = 0.0;
  java.vel_j = 0.0;
  java.scale = PLAYER_SCALE;
}

fn score_update(
  time: Res<Time>,
  mut transform_q: Query<&mut Transform, (With<Text>, With<ScoreRotate>)>,
  mut text_q: Query<&mut Text>,
  mut sprite_q: Query<&mut Sprite>,
  mut texter: ResMut<Texter>,
  mut player: ResMut<Player>,
  mut java: ResMut<Java>,
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut background_colour: ResMut<ClearColor>,
) {
  let mut rot = 5_f32;
  // let td = time.delta().as_millis() as f32 / 60.0;
  for mut text in &mut text_q {
    match texter.state {
      State::DEFAULT => {
        text.sections[0].value = player.jump_count.to_string();
        text.sections[0].style.color = Color::WHITE;
      }
      State::NICE => {
        text.sections[0].value = "haha funny number".to_string();
        text.sections[0].style.color = Color::CYAN;
      }

      State::CORRUPT_A => {
        *background_colour = ClearColor(Color::rgb(
          background_colour.r() * 0.99,
          background_colour.b() * 0.99,
          background_colour.g() * 0.99,
        ));
        text.sections[0].value = player.jump_count.to_string();
        text.sections[0].style.color = Color::WHITE;
        rot = 2_f32;
      }

      State::CORRUPT_B => {
        rot = 0_f32;
        *background_colour = ClearColor(Color::rgb(
          background_colour.r() * 0.99,
          background_colour.b() * 0.99,
          background_colour.g() * 0.99,
        ));

        let tc = text.sections[0].style.color;
        text.sections[0].style.color = Color::rgba(tc.r(), tc.g(), tc.b(), tc.a() * 0.98);
        println!("{}", tc.a());

        if java.entity.is_none() {
          for mut transform in &mut transform_q {
            transform.rotation = Quat::from_rotation_z(0.0_f32.to_radians());
          }
          // let mut bc = background_colour.clone();
          // *background_colour = ClearColor(Color::BLACK);
          java.entity = Some(
            commands
              .spawn_bundle(SpriteBundle {
                texture: asset_server.load("textures/java.png"),
                transform: Transform {
                  scale: Vec3::new(java.scale, java.scale, 0.0),
                  translation: Vec3::new(java.j, java.i, 0.0),
                  ..default()
                },
                sprite: Sprite {
                  flip_x: true,
                  flip_y: false,
                  color: java.colour,
                  ..default()
                },
                ..default()
              })
              .id(),
          );
        }

        if sprite_q.get_mut(java.entity.unwrap()).is_ok() {
          *background_colour = ClearColor(Color::rgb(
            background_colour.r() * 0.99,
            background_colour.b() * 0.99,
            background_colour.g() * 0.99,
          ));
          let jc = java.colour;
          java.colour = Color::rgba(jc.r(), jc.g(), jc.b(), jc.a() + 0.001);
          *sprite_q.get_mut(java.entity.unwrap()).unwrap() = Sprite {
            color: java.colour,
            ..default()
          };
        }
      }

      State::JAVA => {
        let jc = java.colour;
        java.colour = Color::rgba(jc.r(), jc.g(), jc.b(), jc.a() + 0.001);
        *sprite_q.get_mut(java.entity.unwrap()).unwrap() = Sprite {
          color: java.colour,
          ..default()
        };
      }
    }
  }

  for mut transform in &mut transform_q {
    transform.rotate_z(rot.to_radians());
    let r = transform.rotation.to_axis_angle().1.to_degrees();
    if texter.state == State::CORRUPT_A &&
      (r <= 10.0 || 360.0 - r <= 10.0) &&
      time.time_since_startup().as_millis() as usize - texter.time_save >= 2000
    {
      for mut text in &mut text_q {
        texter.state = State::CORRUPT_B;
        let mut rng = thread_rng();
        let c = Color::GREEN;
        let tc = text.sections[0].style.color;

        text.sections[0].value = (0..5)
          .map(|_| {
            ((33..=126)
              .into_iter()
              .collect::<Vec<_>>()
              .choose(&mut rng)
              .unwrap() *
              1) as u8 as char
          })
          .collect::<String>();
        let c = Color::GREEN;
        let tan = text.sections[0].style.color.a();
        text.sections[0].style.color = Color::rgba(c.r(), c.g(), c.b(), tan);
      }
    }
  }
}

fn score_corrupt(
  mut java: ResMut<Java>,
  mut transform_q: Query<&mut Transform, (With<Text>, With<ScoreRotate>)>,
  mut texter: ResMut<Texter>,
  time: Res<Time>,
  mut text_q: Query<&mut Text>,
) {
  if texter.state == State::CORRUPT_B {
    for mut text in &mut text_q {
      let mut rng = thread_rng();

      text.sections[0].value = (0..5)
        .map(|_| {
          ((33..=126)
            .into_iter()
            .collect::<Vec<_>>()
            .choose(&mut rng)
            .unwrap() *
            1) as u8 as char
        })
        .collect::<String>();

      if text.sections[0].style.color.a() <= 0.001 {
        java.time_save = time.time_since_startup().as_millis() as usize;
        texter.state = State::JAVA;
      }
    }
  }
}
