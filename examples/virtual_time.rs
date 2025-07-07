//! Shows how `Time<Virtual>` can be used to pause, resume, slow down
//! and speed up a game.

use std::time::Duration;

use bevy::{
    color::palettes::css::*, input::common_conditions::input_just_pressed, prelude::*,
    time::common_conditions::on_real_timer, time::Stopwatch,
};

#[derive(Component)]
pub struct VirtualClock {
    stopwatch: Stopwatch,
    speed: f32,
    paused: bool,
    delta_secs: f32,
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_clocks,
                move_scenario_time_sprites,
                move_real_time_sprites,
                move_life_time_sprites,
                toggle_scenario_pause.run_if(input_just_pressed(KeyCode::Space)),
                toggle_life_pause.run_if(input_just_pressed(KeyCode::Enter)),
                change_scenario_time_speed::<1>.run_if(input_just_pressed(KeyCode::ArrowUp)),
                change_scenario_time_speed::<-1>.run_if(input_just_pressed(KeyCode::ArrowDown)),
                change_life_time_speed::<1>.run_if(input_just_pressed(KeyCode::ArrowRight)),
                change_life_time_speed::<-1>.run_if(input_just_pressed(KeyCode::ArrowLeft)),
                (update_scenario_time_info_text, update_life_time_info_text, update_real_time_info_text)
                    // update the texts on a timer to make them more readable
                    // `on_timer` run condition uses `Virtual` time meaning it's scaled
                    // and would result in the UI updating at different intervals based
                    // on `Time<Virtual>::relative_speed` and `Time<Virtual>::is_paused()`
                    .run_if(on_real_timer(Duration::from_millis(250))),
            ),
        )
        .run();
}

/// `Real` time related marker
#[derive(Component)]
struct RealTime;

#[derive(Component)]
struct ScenarioTime;

#[derive(Component)]
struct LifeTime;

fn update_clocks(time: Res<Time>, mut queryy: Query<&mut VirtualClock>) {
    for mut clock in &mut queryy{
        if !clock.paused {
            let prev = clock.stopwatch.elapsed_secs();
            let scaled_delta = time.delta().as_secs_f32() * clock.speed;
            clock.stopwatch.tick(Duration::from_secs_f32(scaled_delta));
            let new = clock.stopwatch.elapsed_secs();
            clock.delta_secs = new - prev;
        } else {
            clock.delta_secs = 0.
        }
    }
}

/// Setup the example
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn(Camera2d);

    commands.spawn((VirtualClock {
        stopwatch: Stopwatch::new(),
        speed: 1.0,
        paused: false,
        delta_secs: 0.
    },
    LifeTime));

    commands.spawn((VirtualClock {
        stopwatch: Stopwatch::new(),
        speed: 1.0,
        paused: false,
        delta_secs: 0.
    },
    ScenarioTime));

    let virtual_color = GOLD.into();
    let sprite_scale = Vec2::splat(0.5).extend(1.);
    let texture_handle = asset_server.load("branding/icon.png");

    // the sprite moving based on real time
    commands.spawn((
        Sprite::from_image(texture_handle.clone()),
        Transform::from_scale(sprite_scale),
        RealTime,
        LifeTime
    ));

    // the sprite moving based on virtual time
    commands.spawn((
        Sprite {
            image: texture_handle,
            color: virtual_color,
            ..Default::default()
        },
        Transform {
            scale: sprite_scale,
            translation: Vec3::new(0., -160., 0.),
            ..default()
        },
        ScenarioTime,
        LifeTime
    ));

    // info UI
    let font_size = 33.;

    commands.spawn((
        Node {
            display: Display::Flex,
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Percent(100.),
            position_type: PositionType::Absolute,
            top: Val::Px(0.),
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        children![
            (
                Text::default(),
                TextFont {
                    font_size,
                    ..default()
                },
                RealTime,
            ),
            (
                Text::new("CONTROLS\nUn/Pause Scenario Time: Space\nScenario Speed+: Up\nScenario Speed-: Down\nLife Speed+: Right\nLife Speed-: Left\nPause Life Time: Enter"),
                TextFont {
                    font_size,
                    ..default()
                },
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
                TextLayout::new_with_justify(JustifyText::Center),
            ),
            (
                Text::default(),
                TextFont {
                    font_size,
                    ..default()
                },
                TextColor(virtual_color),
                TextLayout::new_with_justify(JustifyText::Right),
                ScenarioTime,
            ),
            (
                Text::default(),
                TextFont {
                    font_size,
                    ..default()
                },
                TextColor(virtual_color),
                TextLayout::new_with_justify(JustifyText::Right),
                LifeTime,
            ),
        ],
    ));
}

/// Move sprites using `Real` (unscaled) time
fn move_real_time_sprites(
    mut sprite_query: Query<&mut Transform, (With<Sprite>, With<RealTime>)>,
    // `Real` time which is not scaled or paused
    time: Res<Time<Real>>,
) {
    for mut transform in sprite_query.iter_mut() {
        // move roughly half the screen in a `Real` second
        // when the time is scaled the speed is going to change
        // and the sprite will stay still the time is paused
        transform.translation.x = get_sprite_translation_x(time.elapsed_secs());
    }
}


fn move_scenario_time_sprites(
    clock_query: Query<&VirtualClock, With<ScenarioTime>>,
    mut sprite_query: Query<&mut Transform, (With<Sprite>, With<ScenarioTime>)>,
) { if let Ok(clock) = clock_query.single(){
        for mut transform in sprite_query.iter_mut() {
            transform.translation.x = get_sprite_translation_x(clock.stopwatch.elapsed_secs());
        }
    }
}

/// Move sprites using `Life` (scaled) time
fn move_life_time_sprites(
    clock_query: Query<&VirtualClock, With<LifeTime>>,
    mut sprite_query: Query<&mut Transform, (With<Sprite>, With<LifeTime>)>,
) { if let Ok(clock) = clock_query.single(){
        for mut transform in sprite_query.iter_mut() {
            transform.rotate_z(clock.delta_secs);
        }
    }
}

fn get_sprite_translation_x(elapsed: f32) -> f32 {
    ops::sin(elapsed) * 500.
}

/// Update the speed of the ScenarioTime virtual clock by `DELTA`
fn change_scenario_time_speed<const DELTA: i8>(mut clock_query:Query<&mut VirtualClock, With<ScenarioTime>>) {
    if let Ok(mut clock) = clock_query.single_mut() {
        let time_speed = (clock.speed + DELTA as f32)
            .round()
            .clamp(0.25, 5.);

        clock.speed = time_speed;
    }
}

/// Update the speed of the ScenarioTime virtual clock by `DELTA`
fn change_life_time_speed<const DELTA: i8>(mut clock_query:Query<&mut VirtualClock, With<LifeTime>>) {
    if let Ok(mut clock) = clock_query.single_mut() {
        let time_speed = (clock.speed + DELTA as f32)
            .round()
            .clamp(0.25, 5.);

        clock.speed = time_speed;
    }
}

/// Pause or resume the ScenarioTime virtual clock
fn toggle_scenario_pause(
    mut query: Query<&mut VirtualClock, With<ScenarioTime>>,
) {
    if let Ok(mut clock) = query.single_mut() {
        clock.paused = !clock.paused;
    }
}

/// Pause or resume the LifeTime virtual clock
fn toggle_life_pause(
    mut query: Query<&mut VirtualClock, With<LifeTime>>,
) {
    if let Ok(mut clock) = query.single_mut() {
        clock.paused = !clock.paused;
    }
}

/// Update the `Real` time info text
fn update_real_time_info_text(time: Res<Time<Real>>, mut query: Query<&mut Text, With<RealTime>>) {
    for mut text in &mut query {
        **text = format!(
            "REAL TIME\nElapsed: {:.1}\nDelta: {:.5}\n",
            time.elapsed_secs(),
            time.delta_secs(),
        );
    }
}

/// Update the `Virtual` time info text
fn update_scenario_time_info_text(
    mut text_query: Query<&mut Text, With<ScenarioTime>>,
    clock_query: Query<&mut VirtualClock, With<ScenarioTime>>
) {
    if let Ok(clock) = clock_query.single() {
        for mut text in &mut text_query {
            **text = format!(
                "SCENARIO TIME\nElapsed: {:.1}\nDelta: {:.5}\nSpeed: {:.2}",
                clock.stopwatch.elapsed_secs(),
                clock.delta_secs,
                clock.speed
            );
        }
    }
}

/// Update the `Virtual` time info text
fn update_life_time_info_text(
    mut text_query: Query<&mut Text, With<LifeTime>>,
    clock_query: Query<&mut VirtualClock, With<LifeTime>>
) {
    if let Ok(clock) = clock_query.single() {
        for mut text in &mut text_query {
            **text = format!(
                "LIFE TIME\nElapsed: {:.1}\nDelta: {:.5}\nSpeed: {:.2}",
                clock.stopwatch.elapsed_secs(),
                clock.delta_secs,
                clock.speed
            );
        }
    }
}
