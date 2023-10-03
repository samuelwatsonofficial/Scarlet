/*use bevy::ecs::system::Commands;
use bevy_rapier2d::prelude::RigidBody;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::Restitution;
use bevy::transform::TransformBundle;
use bevy::transform::components::Transform;*/
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_svg::prelude::*;
#[derive(Component)]
struct Character;
pub fn spawn_character(mut commands: Commands, asset_server: Res<AssetServer>)
{
    println!("player exists now!");
    let svg = asset_server.load("shapes/761.svg");
    commands
        .spawn(
            (
                RigidBody::Dynamic,
                Collider::ball(10.),
                Svg2dBundle
                {
                    svg,
                    origin: Origin::Center,
                    transform: Transform
                    {
                        scale: Vec3::new(4.0, 4.0, 1.0),
                        translation: Vec3::new(0.0,3.0,0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            ));

}

pub fn character_controller(buttons:Res<Input<KeyCode>>)
{
    if buttons.pressed(KeyCode::Right) 
    {
       println!("going right"); 
        // Right Button is being held down
    }
}
pub fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}
