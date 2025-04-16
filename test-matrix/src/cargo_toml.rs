#[derive(serde::Deserialize)]
pub struct CargoToml {
    package: Package,
}

#[derive(serde::Deserialize)]
pub struct Package {
    name: String,
}

impl CargoToml {
    pub fn name(&self) -> &str {
        &self.package.name
    }
}

pub fn parse(content: &str) -> Result<CargoToml, toml::de::Error> {
    toml::from_str(content)
}
