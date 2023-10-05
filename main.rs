use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_svg::prelude::*;
use std::time::Instant;

mod character;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default()) 
        .add_plugin(bevy_svg::prelude::SvgPlugin)
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_startup_system(character::spawn_character)
        .add_system(print_ball_altitude)
        .add_system(character::character_controller)
        .add_system(timer_update)
        .add_system(character::display_events)
        .run();
}
#[derive(Component)]
struct Timer;
fn setup_graphics(mut commands: Commands, asset_server: Res<AssetServer>) 
{
   
    
    let text_alignment = TextAlignment::Center;
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "hello\nbevy!",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/UbuntuMono-Regular.ttf"),
                font_size: 100.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),
        Timer,
    ));
}
fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));
}
fn timer_update(time: Res<Time>, mut query: Query<&mut Text, With<Timer>> )
{
    for mut text in &mut query
    {

        let value=time.elapsed().as_millis() as f64/1000.;
        text.sections[0].value=format!("{value:.2}");

    }
}


fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        //println!("Ball altitude: {}", transform.translation.y);
    }
}
