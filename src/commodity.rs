#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Commodity {
    Grain,
}

impl Commodity {
    pub fn name(self) -> &'static str {
        match self {
            Self::Grain => "Grain",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Extractor {
    pub name: &'static str,
    pub commodity: Commodity,
    pub output: u32,
}

impl Extractor {
    pub fn new(name: &'static str, commodity: Commodity, output: u32) -> Self {
        Self {
            name,
            commodity,
            output,
        }
    }

    pub fn farm(output: u32) -> Self {
        Self::new("Farm", Commodity::Grain, output)
    }
}

#[cfg(test)]
mod tests {
    use super::{Commodity, Extractor};

    #[test]
    fn farm_extracts_grain_with_output() {
        assert_eq!(
            Extractor::farm(5),
            Extractor {
                name: "Farm",
                commodity: Commodity::Grain,
                output: 5,
            }
        );
    }
}
