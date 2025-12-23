use std::{
    io::{Error, ErrorKind, Result as IoResult},
    path::{Path, PathBuf},
};

/// A basic representation of an engine's metadata.
#[derive(Default)]
pub struct EngineMetadata<'a> {
    pub display_name: &'a str,
    pub source_code: Option<&'a str>,
    pub website: Option<&'a str>,
    pub authors: Option<&'a str>,
}

pub struct Engine<'a> {
    pub root: &'a str,
    pub versions: Vec<&'a str>,
    pub modpacks: Vec<Modpack<'a>>,
    pub metadata: EngineMetadata<'a>,
}

pub struct EngineRoot<'a> {
    pub location: &'a Path,
    pub engines: Vec<Engine<'a>>,
    pub selected: usize,
}

#[derive(Default)]
pub struct EngineRootBuilder<'a> {
    location: &'a str,
    engines: Vec<Engine<'a>>,
    selected: usize,
}

impl<'a> EngineRootBuilder<'a> {
    pub fn location(mut self, location: &'a str) -> EngineRootBuilder<'a> {
        self.location = location;
        self
    }

    pub fn selected(mut self, selected: usize) -> EngineRootBuilder<'a> {
        self.selected = selected;
        self
    }

    pub fn build(self) -> EngineRoot<'a> {
        EngineRoot {
            location: Path::new(self.location),
            engines: self.engines,
            selected: self.selected,
        }
    }
}

impl<'a> EngineRoot<'a> {
    pub fn builder() -> EngineRootBuilder<'a> {
        EngineRootBuilder::default()
    }

    /// Get the path of of the selected engine.
    pub fn engine_path(&self) -> IoResult<PathBuf> {
        let container: &Engine = &self.engines[self.selected];
        let engine: &EngineMetadata = &container.metadata;
        let mut buf: PathBuf = PathBuf::from(self.location);
        buf.push(container.root);
        if buf.try_exists()? {
            Ok(buf)
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                format!("Engine {}'s path does not exist", engine.display_name),
            ))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Modpack<'a> {
    pub display_name: &'a str,
    pub version: &'a str,
    pub brief: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_create() {
        let data: EngineMetadata = EngineMetadata {
            display_name: "Codename Engine",
            source_code: Some("https://github.com/CodenameCrew/CodenameEngine/"),
            website: None,
            authors: Some("Codename Crew"),
        };
        assert_eq!(data.display_name, "Codename Engine");
        assert_eq!(data.authors, Some("Codename Crew"));
    }
}
