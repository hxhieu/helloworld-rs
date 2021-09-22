use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, InspectorPlugin, WorldInspectorPlugin};

#[derive(Inspectable)]
struct Person {
    text: String,
}
struct Name(String);
struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(greet_people.system());
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(HelloPlugin)
        .run();
}

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person {
            text: "Hugh Hoang".to_string(),
        })
        .insert(Name("Hugh Hoang".to_string()));

    commands
        .spawn()
        .insert(Person {
            text: "Hugh Hoang".to_string(),
        })
        .insert(Name("Andy Hoang".to_string()));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0)
        }
    }
}
