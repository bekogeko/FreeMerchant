use bevy::prelude::{Component, Entity};

#[derive(Clone, Copy, Component, Debug, Eq, Hash, PartialEq)]
pub struct Road {
    pub city_a: Entity,
    pub city_b: Entity,
}

impl Road {
    pub fn new(city_a: Entity, city_b: Entity) -> Self {
        Self { city_a, city_b }
    }

    pub fn connects(self, city_a: Entity, city_b: Entity) -> bool {
        (self.city_a == city_a && self.city_b == city_b)
            || (self.city_a == city_b && self.city_b == city_a)
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use super::Road;
    use crate::city::City;

    #[test]
    fn road_connects_cities_in_both_directions() {
        let mut world = World::new();
        let amsterdam = world.spawn(City::new("Amsterdam")).id();
        let rotterdam = world.spawn(City::new("Rotterdam")).id();

        let road = Road::new(amsterdam, rotterdam);

        assert!(road.connects(amsterdam, rotterdam));
        assert!(road.connects(rotterdam, amsterdam));
    }
}
