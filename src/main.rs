use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_player)
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Update, greet_people)
        .run();
}

#[derive(Component)]
pub struct PersonName(String);

#[derive(Component)]
pub struct Person;

fn add_player(mut commands: Commands) {
    commands.spawn((Person, PersonName("Test".to_string())));
    commands.spawn((Person, PersonName("Renzo Hume".to_string())));
    commands.spawn((Person, PersonName("Zayna Nieves".to_string())));
}
#[derive(Resource)]
pub struct GreetTimer(Timer);

// TODO: Try to implement a ball struct
// TODO: Try to print it
// TODO: Move it

// NOTE: TEST

// WARNING:
// TODO

// FIXME:

// [ ] Petit test
// [x] Petit test
// BUG:
pub fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&PersonName, With<Person>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}
