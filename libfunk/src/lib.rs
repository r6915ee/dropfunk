#[cfg(feature = "log")]
use log::*;
use serde::{Deserialize, Serialize};
use std::{io::Result as IoResult, path::PathBuf};

/// A basic representation of an engine's metadata.
#[derive(Default, Serialize, Deserialize)]
pub struct EngineMetadata {
    pub display_name: String,
    pub source_code: Option<String>,
    pub website: Option<String>,
    pub authors: Option<String>,
}

#[derive(Clone)]
pub struct Engine {
    pub root: PathBuf,
    pub versions: Vec<String>,
    pub modpacks: Vec<Modpack>,
}

pub struct EngineRoot {
    pub location: PathBuf,
    pub engines: Vec<Engine>,
    pub display_names: Vec<String>,
    pub source_codes: Vec<Option<String>>,
    pub websites: Vec<Option<String>>,
    pub authors: Vec<Option<String>>,
}

#[derive(Default, Clone)]
pub struct EngineRootBuilder {
    engines: Vec<Engine>,
    location: PathBuf,
}

impl EngineRootBuilder {
    pub fn location(mut self, location: PathBuf) -> EngineRootBuilder {
        self.location = location;
        self
    }

    pub fn build(&mut self) -> IoResult<EngineRoot> {
        if !self.location.try_exists()? {
            std::fs::create_dir_all(&*self.location)?;
        }
        let dirs = std::fs::read_dir(&*self.location)?;
        let mut count: usize = 0;
        let mut display_names: Vec<String> = Vec::new();
        let mut source_codes: Vec<Option<String>> = Vec::new();
        let mut websites: Vec<Option<String>> = Vec::new();
        let mut authors: Vec<Option<String>> = Vec::new();
        for entry in dirs {
            let dir = entry?;
            if dir.file_type()?.is_dir() {
                count += 1;
                let mut buf: PathBuf = dir.path();
                buf.push("meta.json");
                macro_rules! push {
                    ($x: expr) => {
                        display_names.push($x.display_name);
                        source_codes.push($x.source_code);
                        websites.push($x.website);
                        authors.push($x.authors);
                        self.engines.push(Engine {
                            root: dir.path(),
                            versions: Vec::new(),
                            modpacks: Vec::new(),
                        });
                    };
                }
                if !buf.try_exists()? {
                    let metadata: EngineMetadata = EngineMetadata {
                        display_name: "Template".into(),
                        source_code: None,
                        website: None,
                        authors: None,
                    };
                    let contents: String = serde_json::to_string(&metadata)?;
                    std::fs::write(buf, contents)?;
                    push!(metadata);
                } else {
                    let data: String = std::fs::read_to_string(buf)?;
                    let data_str: &str = data.as_str();
                    let metadata: EngineMetadata = serde_json::from_str(data_str)?;
                    push!(metadata);
                }
            }
        }
        #[cfg(feature = "log")]
        {
            let mut buf: itoa::Buffer = itoa::Buffer::new();
            info!("{} engine(s) were recorded", buf.format(count));
        }
        Ok(EngineRoot {
            location: self.location.clone(),
            engines: self.engines.clone(),
            display_names,
            source_codes,
            websites,
            authors,
        })
    }
}

impl EngineRoot {
    pub fn builder() -> EngineRootBuilder {
        EngineRootBuilder::default()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Modpack {
    pub display_name: String,
    pub version: String,
    pub brief: String,
}

#[cfg(test)]
mod tests {
    // use super::*;
}
