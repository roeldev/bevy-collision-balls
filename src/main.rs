#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ops::{Deref, RangeInclusive};

use bevy::core::FixedTimestep;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::math::*;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_prototype_debug_lines::*;
use bevy_prototype_lyon::prelude::*;
use rand::distributions::{Distribution, Uniform};

use crate::collision::*;
use crate::components::*;
use crate::debug::*;
use crate::quadtree::*;

mod collision;
mod components;
mod quadtree;
mod debug;

// pub const WIDTH: f32 = 1024.0;
// pub const HEIGHT: f32 = 768.0;

pub const WIDTH: f32 = 1024.0;
pub const HEIGHT: f32 = 768.0;

const BALLS: u64 = 200;

// Min/max radius range of balls.
const BALL_RADIUS: RangeInclusive<f32> = 3.0..=12.0;

// Possible ball colors.
const BALL_COLORS: [Color; 36] = [
    Color::ALICE_BLUE,
    Color::ANTIQUE_WHITE,
    Color::AQUAMARINE,
    Color::AZURE,
    Color::BEIGE,
    Color::BISQUE,
    Color::BLUE,
    Color::CRIMSON,
    Color::CYAN,
    Color::DARK_GRAY,
    Color::DARK_GREEN,
    Color::FUCHSIA,
    Color::GOLD,
    Color::GRAY,
    Color::GREEN,
    Color::INDIGO,
    Color::LIME_GREEN,
    Color::MAROON,
    Color::MIDNIGHT_BLUE,
    Color::NAVY,
    Color::OLIVE,
    Color::ORANGE,
    Color::ORANGE_RED,
    Color::PINK,
    Color::PURPLE,
    Color::RED,
    Color::SALMON,
    Color::SEA_GREEN,
    Color::SILVER,
    Color::TEAL,
    Color::TOMATO,
    Color::TURQUOISE,
    Color::VIOLET,
    Color::WHITE,
    Color::YELLOW,
    Color::YELLOW_GREEN,
];

// Initial random speed of ball is selected from -BALL_INIT_SPEED..=BALL_INIT_SPEED.
const BALL_INIT_SPEED: f32 = 50.0;

const APPLY_FRICTION: bool = false;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WindowDescriptor {
            title: "Bevy Balls".to_string(),
            width: WIDTH,
            height: HEIGHT,
            present_mode: PresentMode::Immediate,
            resizable: true,
            cursor_visible: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_startup_system(spawn_balls)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(2.0))
                .with_system(display_fps),
        )
        .add_system(check_collisions_quadtree.after(apply_velocity))
        // .add_system(check_collisions.after(apply_velocity))
        .add_system(apply_velocity)
        .run();
}

fn setup(mut cmd: Commands) {
    let center = Vec3::new(WIDTH * 0.5, HEIGHT * 0.5, 0.0);
    // let outline_padding = 1.0;
    // cmd.spawn_bundle(GeometryBuilder::build_as(
    //     &shapes::Rectangle {
    //         extents: Vec2::new(WIDTH - (outline_padding * 2.0), HEIGHT - (outline_padding * 2.0)),
    //         ..default()
    //     },
    //     DrawMode::Stroke(StrokeMode::color(Color::WHITE)),
    //     Transform {
    //         translation: center,
    //         ..default()
    //     },
    // ));

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.translation = center;
    cmd.spawn_bundle(camera);
}

fn display_fps(
    mut windows: ResMut<Windows>,
    windescr: Res<WindowDescriptor>,
    diagnostics: Res<Diagnostics>,
) {
    if let Some(fps) = diagnostics.get_measurement(FrameTimeDiagnosticsPlugin::FPS) {
        let window = windows.primary_mut();
        window.set_title(format!("{}: {}", windescr.title, fps.value.to_string()));
    }
}

fn spawn_balls(mut cmd: Commands) {
    let rand_radius = Uniform::from(BALL_RADIUS);
    let rand_velocity = Uniform::from(-BALL_INIT_SPEED..=BALL_INIT_SPEED);
    let rand_pos_x =
        Uniform::from(*BALL_RADIUS.end()..=WIDTH - *BALL_RADIUS.end() - *BALL_RADIUS.end());
    let rand_pos_y =
        Uniform::from(*BALL_RADIUS.end()..=HEIGHT - *BALL_RADIUS.end() - *BALL_RADIUS.end());
    let mut rng = rand::thread_rng();

    let mut ball_color_index: usize = 0;
    for _ in 0..BALLS {
        let radius = rand_radius.sample(&mut rng);

        cmd.spawn()
            .insert(Ball {
                radius,
                mass: radius * radius,
            })
            .insert(Velocity {
                0: Vec2::new(
                    rand_velocity.sample(&mut rng),
                    rand_velocity.sample(&mut rng),
                ),
            })
            .insert_bundle(GeometryBuilder::build_as(
                &shapes::Circle {
                    radius,
                    ..default()
                },
                DrawMode::Fill(FillMode::color(BALL_COLORS[ball_color_index])),
                Transform {
                    translation: Vec3::new(
                        rand_pos_x.sample(&mut rng),
                        rand_pos_y.sample(&mut rng),
                        0.0,
                    ),
                    ..default()
                },
            ));

        ball_color_index += 1;
        if ball_color_index == BALL_COLORS.len() {
            ball_color_index = 0;
        }
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &mut Velocity)>, time: Res<Time>) {
    for (mut transform, mut velocity) in query.iter_mut() {
        if APPLY_FRICTION {
            velocity.0.x -= velocity.0.x * 0.03 * time.delta_seconds();
            velocity.0.y -= velocity.0.y * 0.03 * time.delta_seconds();
        }
        // apply velocity
        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();
    }
}

#[allow(dead_code)]
fn check_collisions(mut query: Query<(Entity, &mut Transform, &mut Velocity, &Ball)>) {
    for (_, mut transform, mut velocity, ball) in query.iter_mut() {
        let transform = &mut *transform;
        let velocity = &mut *velocity;

        let _ = ball_edge_top_collision(ball, transform, velocity)
            || ball_edge_bottom_collision(ball, transform, velocity);

        let _ = ball_edge_left_collision(ball, transform, velocity)
            || ball_edge_right_collision(ball, transform, velocity);
    }

    let mut collisions = Collisions::new(Some(BALLS as usize));

    let mut combinations = query.iter_combinations_mut();
    while let Some([a, b]) = combinations.fetch_next() {
        let (a, mut transform_a, _, ball_a) = a;
        let (b, mut transform_b, _, ball_b) = b;

        collisions.check([
            (a, &mut *transform_a, ball_a),
            (b, &mut *transform_b, ball_b),
        ]);
    }

    for balls in collisions {
        let [
        (_, transform_a, mut velocity_a, ball_a),
        (_, transform_b, mut velocity_b, ball_b)
        ] = query.many_mut(balls);

        balls_bounce_after_collision([
            (transform_a.deref(), &mut *velocity_a, ball_a),
            (transform_b.deref(), &mut *velocity_b, ball_b),
        ]);
    }
}

// const EDGE_AREA_TOP: Location = Location::Area(Bounds::new(Point::ZERO, WIDTH, *BALL_RADIUS.end()));
// const EDGE_AREA_LEFT: Location = Location::Area(Bounds::new(Point::ZERO, *BALL_RADIUS.end(), HEIGHT));
// const EDGE_AREA_BOTTOM: Location = Location::Area(Bounds::new(
//     Point::new(0.0, HEIGHT - *BALL_RADIUS.end()),
//     WIDTH,
//     *BALL_RADIUS.end(),
// ));
// const EDGE_AREA_RIGHT: Location = Location::Area(Bounds::new(
//     Point::new(WIDTH - *BALL_RADIUS.end(), 0.0),
//     *BALL_RADIUS.end(),
//     HEIGHT,
// ));

#[allow(dead_code)]
fn check_collisions_quadtree(mut debug_lines: ResMut<DebugLines>, mut query: Query<(Entity, &mut Transform, &mut Velocity, &Ball)>) {
    let debug_lines = &mut *debug_lines;

    for (_, mut transform, mut velocity, ball) in query.iter_mut() {
        let transform = &mut *transform;
        let velocity = &mut *velocity;

        let _ = ball_edge_top_collision(ball, transform, velocity)
            || ball_edge_bottom_collision(ball, transform, velocity);

        let _ = ball_edge_left_collision(ball, transform, velocity)
            || ball_edge_right_collision(ball, transform, velocity);
    }

    let mut tree = QuadTree::new(Bounds::new(Point::ZERO, WIDTH, HEIGHT), 4, None);

    for (entity, transform, _, ball) in query.iter() {
        if let Err(err) = tree.insert(
            Location::from_center(transform.translation.into(), ball.radius * 2.0, ball.radius * 2.0),
            Entity::from(entity),
        ) {
            match err {
                ErrorKind::OutOfBounds(bounds, location) => {
                    println!("err: {}: {err}, {:?} not in {:?}", entity.id(), location, bounds)
                }
            }
        }
    }

    for (bounds, part) in tree {
        bounds.debug_draw_lines(debug_lines, None);
        if part.len() < 2 {
            continue;
        }

        let mut collisions = Collisions::new(Some(part.capacity() * 2));
        for (a, loc_a) in part.clone() {
            loc_a.debug_draw_lines(debug_lines, None);

            for (b, _) in part.clone() {
                if a == b {
                    continue;
                }

                let [
                (a, mut transform_a, _, ball_a),
                (b, mut transform_b, _, ball_b)
                ] = query.many_mut([a, b]);

                debug_lines.line(transform_a.translation, transform_b.translation, 0.0);

                collisions.check([
                    (a, &mut *transform_a, ball_a),
                    (b, &mut *transform_b, ball_b),
                ]);
            }
        }

        for balls in collisions {
            let [
            (_, transform_a, mut velocity_a, ball_a),
            (_, transform_b, mut velocity_b, ball_b)
            ] = query.many_mut(balls);

            balls_bounce_after_collision([
                (transform_a.deref(), &mut *velocity_a, ball_a),
                (transform_b.deref(), &mut *velocity_b, ball_b),
            ]);
        }
    }

    // for (_, transform, velocity, ball) in query.many_mut(tree.query_entities(edge.left)) {
    //     if transform.translation.x <= ball.radius {
    //         transform.translation.x = ball.radius + (ball.radius - transform.translation.x);
    //         velocity.0.x *= -1.0;
    //     }
    // }
    // for (_, transform, velocity, ball) in query.many_mut(tree.query_entities(edge.right)) {
    //     let max_x = WIDTH - ball.radius;
    //     // transform.translation.x + ball.radius >= WIDTH
    //     if transform.translation.x >= max_x {
    //         transform.translation.x = max_x - (transform.translation.x - max_x);
    //         velocity.0.x *= -1.0;
    //     }
    // }
    // for (_, transform, velocity, ball) in query.many_mut(tree.query_entities(edge.top)) {
    //     if transform.translation.y <= ball.radius {
    //         transform.translation.y = ball.radius + (ball.radius - transform.translation.y);
    //         velocity.0.y *= -1.0;
    //     }
    // }
    // for (_, transform, velocity, ball) in query.many_mut(tree.query_entities(edge.bottom)) {
    //     let max_y = HEIGHT - ball.radius;
    //     if transform.translation.y >= max_y {
    //         transform.translation.y = max_y - (transform.translation.y - max_y);
    //         velocity.0.y *= -1.0;
    //     }
    // }

    // query.iter_combinations();
    // for (a, b) in tree.iter_combinations() {
    //     query.
    // }

    // for q in query.iter_mut() {}
    // for partitions in qt.iter() {
    //     let mut combinations = .iter_combinations_mut();
    //     for q in partitions.iter_combinations_mut() {}
    // }
    // print!("w:{}, h:{}, l:{}\n", qt.width(), qt.height(), qt.len())
}
