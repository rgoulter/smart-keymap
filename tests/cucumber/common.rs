#[derive(Debug, Default, cucumber::Parameter)]
#[param(name = "deserializer", regex = "JSON|RON")]
pub enum Deserializer {
    #[default]
    JSON,
    RON,
}

impl std::str::FromStr for Deserializer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "JSON" => Self::JSON,
            "RON" => Self::RON,
            invalid => return Err(format!("Invalid `Deserializer`: {invalid}")),
        })
    }
}

impl Deserializer {
    pub fn from_str<'a, T>(&self, s: &'a str) -> Option<T>
    where
        T: serde::de::Deserialize<'a>,
    {
        match self {
            Self::JSON => serde_json::from_str(s).ok(),
            Self::RON => ron::from_str(s).ok(),
        }
    }
}
