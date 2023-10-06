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
pub struct Character;
#[derive(Component)]
pub struct MyCamera;
pub fn spawn_character(mut commands: Commands, asset_server: Res<AssetServer>)
{
    println!("player exists now!");
    let svg = asset_server.load("shapes/761.svg");
    commands
        .spawn(
            (
                RigidBody::Dynamic,
                GravityScale(5.),
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
    commands.spawn((Camera2dBundle::default(),Character, MyCamera));

}
pub fn update_camera_position(rigidbody_query:Query<&mut Transform, (With<Character>, Without<MyCamera>)>, mut camera_query: Query<&mut Transform, (With<Character>, With<MyCamera>)>)
//mut query: Query<(&mut Transform, &mut TextureAtlasSprite), (With<Character>, Without<Camera>)>,
    //mut camera_query: Query<&mut Transform, (Without<Character>, With<Camera>)>,
{
    let mut camera = camera_query.get_single_mut().unwrap();
    let character = rigidbody_query.get_single().unwrap();
    camera.translation = character.translation; 
}
pub fn character_controller(mut query: Query<&mut Velocity>,buttons:Res<Input<KeyCode>>)
{
    const controller_speed:f32 = 4.0;
    for mut velocity in query.iter_mut()
    {
        if buttons.pressed(KeyCode::D) 
        {
            velocity.linvel = velocity.linvel + Vec2::new(controller_speed,0.);
        }


        if buttons.pressed(KeyCode::A) 
        {
            velocity.linvel = velocity.linvel - Vec2::new(controller_speed,0.);
        }
        if buttons.pressed(KeyCode::W) 
        {
            velocity.linvel = velocity.linvel + Vec2::new(0.,10.);
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
