use std::time::Duration;

use bevy::{
    app::{ScheduleRunnerPlugin, ScheduleRunnerSettings},
    prelude::*,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
enum MySet {
    BeforeRound,
    AfterRound,
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
enum MyState {
    #[default]
    Small,
    Big,
}

#[derive(Resource, Default)]
struct Counter(u32);

fn main() {
    App::new()
        .init_resource::<Counter>()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs(10)))
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_state::<MyState>()
        .configure_set(MySet::BeforeRound.before(CoreSet::Update))
        .configure_set(MySet::AfterRound.after(CoreSet::Update))
        .add_system(print1.in_base_set(MySet::BeforeRound))
        .add_system(print2.in_base_set(MySet::AfterRound))
        .add_system(print_small_enter.in_schedule(OnEnter(MyState::Small)))
        .add_system(print_small_exit.in_schedule(OnExit(MyState::Small)))
        .add_system(print_big_enter.in_schedule(OnEnter(MyState::Big)))
        .add_system(print_big_exit.in_schedule(OnExit(MyState::Big)))
        .run();
}

fn print1(mut counter: ResMut<Counter>) {
    println!("Before: {}", counter.0);
    counter.0 += 1;
}

fn print2(counter: Res<Counter>, mut state: ResMut<NextState<MyState>>) {
    println!("After: {}\n", counter.0);
    if counter.0 % 2 == 1 {
        state.set(MyState::Big)
    } else {
        state.set(MyState::Small)
    }
}

fn print_small_enter() {
    println!("{}", "print_small_enter");
}

fn print_small_exit() {
    println!("{}", "print_small_exit");
}

fn print_big_enter() {
    println!("{}", "print_big_enter");
}

fn print_big_exit() {
    println!("{}", "print_big_exit");
}
