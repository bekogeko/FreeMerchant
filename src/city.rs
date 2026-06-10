use crate::commodity::Extractor;
use bevy::prelude::{Component, Entity};

#[derive(Clone, Component, Debug, Eq, Hash, PartialEq)]
pub struct City {
    pub name: String,
    pub population: u32,
    pub extractors: Vec<Extractor>,
}

#[derive(Component)]
pub struct Player {
    pub position: Entity,
}

impl City {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            population: 1,
            extractors: Vec::new(),
        }
    }

    pub fn add_extractor(&mut self, extractor: Extractor) {
        self.extractors.push(extractor);
    }
}

#[cfg(test)]
mod tests {
    use super::City;
    use crate::commodity::{Commodity, Extractor};

    #[test]
    fn new_city_defaults_to_one_population_and_no_extractors() {
        let city = City::new("Zwolle");

        assert_eq!(city.population, 1);
        assert!(city.extractors.is_empty());
    }

    #[test]
    fn add_extractor_adds_extractor_to_city() {
        let mut city = City::new("Zwolle");

        city.add_extractor(Extractor::farm(5));

        assert_eq!(
            city.extractors,
            vec![Extractor {
                name: "Farm",
                commodity: Commodity::Grain,
                output: 5,
            }]
        );
    }
}
