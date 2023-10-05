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
                Velocity 
                {
                    linvel: Vec2::new(2.,2.),
                    angvel: 0.0,
                },
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
                },
                Character,
            ));
    commands.spawn((Camera2dBundle::default(),Character));

}
pub fn update_camera_position(mut query: Query<(&mut Camera2dBundle, RigidBody)> With<Character>)
{
    for mut (camera,rigidbody) in query.iter()
    {
        camera.transform.translation = rigidbody.transform.translation; 
    }
}
pub fn character_controller(mut query: Query<&mut Velocity>,buttons:Res<Input<KeyCode>>)
{
    for mut velocity in query.iter_mut()
    {
        if buttons.pressed(KeyCode::D) 
        {
            velocity.linvel = velocity.linvel + Vec2::new(1.,0.);
            println!("going right");
        }


        if buttons.pressed(KeyCode::A) 
        {
            velocity.linvel = velocity.linvel - Vec2::new(1.,0.);
            println!("going left");
        }
        if buttons.pressed(KeyCode::W) 
        {
            velocity.linvel = velocity.linvel + Vec2::new(0.,10.);
            println!("going left");
        }
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
