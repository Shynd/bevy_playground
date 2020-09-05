use bevy::prelude::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(add_people.system())
        .add_system(hello_world.system())
        .add_system(greet_people.system())
        .run();
}

fn hello_world() {
    println!("hello, world!")
}

// Person component
struct Person;

// Name component
struct Name(String);

fn add_people(mut commands: Commands) {
    commands
        .spawn((Person, Name("Amicus".to_string())))
        .spawn((Person, Name("Cassius".to_string())))
        .spawn((Person, Name("Alexios".to_string())));
}

fn greet_people(_person: &Person, name: &Name) {
    println!("hello {}!", name.0);
}
