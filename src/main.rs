use bevy::prelude::{AssetServer, Commands, DefaultPlugins, Res, Startup, Vec2};

mod city;
mod city_ui;
mod map;
mod road;

fn main() {
    bevy::prelude::App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(city_ui::CityUIPlugin)
        .add_systems(Startup, (map::setup_camera, add_cities_and_roads))
        .run();
}

fn add_cities_and_roads(mut commands: Commands, asset_server: Res<AssetServer>) {
    let amsterdam = map::spawn_city(
        &mut commands,
        &asset_server,
        "Amsterdam",
        Vec2::new(-260.0, 80.0),
    );
    let rotterdam = map::spawn_city(
        &mut commands,
        &asset_server,
        "Rotterdam",
        Vec2::new(-120.0, -130.0),
    );
    let zwolle = map::spawn_city(
        &mut commands,
        &asset_server,
        "Zwolle",
        Vec2::new(240.0, 90.0),
    );

    map::spawn_road(&mut commands, amsterdam, rotterdam);
    map::spawn_road(&mut commands, amsterdam, zwolle);
    map::spawn_road(&mut commands, rotterdam, zwolle);
}
