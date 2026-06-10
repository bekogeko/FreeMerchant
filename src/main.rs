use bevy::prelude::{AssetServer, Commands, DefaultPlugins, Res, Startup, Vec2};
use city::City;
use commodity::Extractor;

mod city;
mod city_ui;
mod commodity;
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
        City::new("Amsterdam"),
        Vec2::new(-260.0, 80.0),
    );
    let rotterdam = map::spawn_city(
        &mut commands,
        &asset_server,
        City::new("Rotterdam"),
        Vec2::new(-120.0, -130.0),
    );
    let zwolle = map::spawn_city(
        &mut commands,
        &asset_server,
        zwolle_city(),
        Vec2::new(240.0, 90.0),
    );

    map::spawn_road(&mut commands, amsterdam, rotterdam);
    map::spawn_road(&mut commands, amsterdam, zwolle);
    map::spawn_road(&mut commands, rotterdam, zwolle);
}

fn zwolle_city() -> City {
    let mut city = City::new("Zwolle");
    city.add_extractor(Extractor::farm(5));
    city.add_extractor(Extractor::farm(5));
    city
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commodity::{Commodity, Extractor};

    #[test]
    fn zwolle_starts_with_two_grain_farms() {
        let city = zwolle_city();

        assert_eq!(city.name, "Zwolle");
        assert_eq!(city.population, 1);
        assert_eq!(
            city.extractors,
            vec![
                Extractor {
                    name: "Farm",
                    commodity: Commodity::Grain,
                    output: 5,
                },
                Extractor {
                    name: "Farm",
                    commodity: Commodity::Grain,
                    output: 5,
                },
            ]
        );
    }
}
