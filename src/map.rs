use crate::city::City;
use crate::road::Road;
use bevy::prelude::{
    AssetServer, Camera2d, Color, Commands, Component, Entity, Quat, Sprite, Transform, Vec2,
};

pub const CITY_SIZE: f32 = 34.0;
const ROAD_WIDTH: f32 = 8.0;
const ROAD_Z: f32 = 0.0;
const CITY_Z: f32 = 1.0;

#[derive(Clone, Copy, Component, Debug, PartialEq)]
pub struct MapPosition(pub Vec2);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CityNode {
    pub entity: Entity,
    pub position: Vec2,
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn spawn_city(
    commands: &mut Commands,
    asset_server: &AssetServer,
    name: impl Into<String>,
    position: Vec2,
) -> CityNode {
    let entity = commands
        .spawn((
            City::new(name),
            MapPosition(position),
            Sprite {
                // image: asset_server.load("city.png"),
                color: Color::srgb(0.88, 0.68, 0.24),
                custom_size: Some(Vec2::splat(CITY_SIZE)),

                ..Default::default()
            },
            Transform::from_translation(position.extend(CITY_Z)),
        ))
        .id();

    commands.spawn((
        Sprite {
            image: asset_server.load("city.png"),
            custom_size: Some(Vec2::splat(CITY_SIZE)),
            ..Default::default()
        },
        Transform::from_translation(position.extend(CITY_Z + 1.0)),
    ));

    CityNode { entity, position }
}

pub fn spawn_road(commands: &mut Commands, city_a: CityNode, city_b: CityNode) -> Entity {
    let delta = city_b.position - city_a.position;
    let midpoint = city_a.position + delta / 2.0;

    commands
        .spawn((
            Road::new(city_a.entity, city_b.entity),
            Sprite::from_color(
                Color::srgb(0.44, 0.39, 0.33),
                Vec2::new(delta.length(), ROAD_WIDTH),
            ),
            Transform {
                translation: midpoint.extend(ROAD_Z),
                rotation: Quat::from_rotation_z(delta.y.atan2(delta.x)),
                ..Default::default()
            },
        ))
        .id()
}
