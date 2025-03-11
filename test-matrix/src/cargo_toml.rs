use semver::VersionReq;

#[derive(serde::Deserialize)]
pub struct CargoToml {
    package: Package,
    dependencies: Dependencies,
}

#[derive(serde::Deserialize)]
pub struct Package {
    name: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Dependencies {
    grafbase_sdk: Option<SdkDependency>,
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum SdkDependency {
    Simple(VersionReq),
    Structured(StructuredDependency),
}

#[derive(serde::Deserialize)]
pub struct StructuredDependency {
    version: Option<VersionReq>,
}

impl CargoToml {
    pub fn name(&self) -> &str {
        &self.package.name
    }

    pub fn grafbase_sdk_version(&self) -> Option<&VersionReq> {
        self.dependencies.grafbase_sdk.as_ref().and_then(|dep| match dep {
            SdkDependency::Simple(req) => Some(req),
            SdkDependency::Structured(structured) => structured.version.as_ref(),
        })
    }
}

pub fn parse(content: &str) -> Result<CargoToml, toml::de::Error> {
    toml::from_str(content)
}
