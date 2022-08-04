use super::DATA_FOLDER_NAME;
use flight_fusion_ipc::{area_source_reference::Table, AreaSourceReference, AreaTableLocation};
use object_store::path::{Path, PathPart};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AreaPath(Path);

impl AreaPath {
    pub fn is_data_root(&self) -> bool {
        let data_part = PathPart::from(DATA_FOLDER_NAME);
        self.0.parts().any(|p| p == data_part)
    }

    pub fn is_table_root(&self) -> bool {
        let data_part = PathPart::from(DATA_FOLDER_NAME);
        let parts = self.0.parts().collect::<Vec<_>>();
        if parts.len() < 2 {
            return false;
        }
        let idx = self
            .0
            .parts()
            .position(|part| part == data_part)
            .unwrap_or_default();
        idx == parts.len() - 2
    }
}

impl From<Path> for AreaPath {
    fn from(path: Path) -> Self {
        Self(path)
    }
}

impl From<&str> for AreaPath {
    fn from(path: &str) -> Self {
        Self(Path::from(path))
    }
}

impl From<String> for AreaPath {
    fn from(path: String) -> Self {
        Self(Path::from(path))
    }
}

impl From<AreaSourceReference> for AreaPath {
    fn from(source: AreaSourceReference) -> Self {
        match source.table {
            Some(Table::Location(loc)) => Path::from_iter(loc.areas)
                .child(DATA_FOLDER_NAME)
                .child(loc.name)
                .into(),
            _ => todo!(),
        }
    }
}

impl From<AreaPath> for AreaSourceReference {
    fn from(value: AreaPath) -> Self {
        let parts = value.0.parts().collect::<Vec<_>>();
        let mut spl = parts.split(|p| p.as_ref() == DATA_FOLDER_NAME);
        let areas = spl
            .next()
            .unwrap_or_default()
            .iter()
            .cloned()
            .map(|p| p.as_ref().to_string())
            .collect::<Vec<_>>();
        let name = spl
            .next()
            .unwrap_or_default()
            .to_vec()
            .first()
            .unwrap_or(&PathPart::from(""))
            .as_ref()
            .into();
        AreaSourceReference {
            table: Some(Table::Location(AreaTableLocation { areas, name })),
        }
    }
}

impl From<&AreaSourceReference> for AreaPath {
    fn from(source: &AreaSourceReference) -> Self {
        match source.table.clone() {
            Some(Table::Location(loc)) => Path::from_iter(loc.areas)
                .child(DATA_FOLDER_NAME)
                .child(loc.name)
                .into(),
            _ => todo!(),
        }
    }
}

impl From<AreaPath> for Path {
    fn from(path: AreaPath) -> Self {
        path.0
    }
}

impl From<&AreaPath> for Path {
    fn from(path: &AreaPath) -> Self {
        path.0.clone()
    }
}

impl AsRef<str> for AreaPath {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<AreaPath> for String {
    fn from(path: AreaPath) -> Self {
        path.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversions_with_path() {
        let path = Path::from(DATA_FOLDER_NAME);
        let area_path = AreaPath::from(path.clone());
        let into_path: Path = area_path.into();
        assert_eq!(path, into_path);
        assert_eq!(
            AreaPath::from(path.clone()),
            AreaPath::from(DATA_FOLDER_NAME)
        )
    }

    #[test]
    fn conversions_with_area_source() {
        let source = AreaSourceReference {
            table: Some(Table::Location(AreaTableLocation {
                areas: vec!["asd".to_string()],
                name: "table".to_string(),
            })),
        };
        let area_path: AreaPath = source.clone().into();
        assert_eq!(
            area_path,
            AreaPath::from(format!("asd/{}/table", DATA_FOLDER_NAME))
        );
        let source_ref: AreaSourceReference = area_path.into();
        assert_eq!(source_ref, source);

        let source = AreaSourceReference {
            table: Some(Table::Location(AreaTableLocation {
                areas: vec![],
                name: "table".to_string(),
            })),
        };
        let area_path: AreaPath = source.clone().into();
        assert_eq!(
            area_path,
            AreaPath::from(format!("{}/table", DATA_FOLDER_NAME))
        );
        let source_ref: AreaSourceReference = area_path.into();
        assert_eq!(source_ref, source);
    }

    #[test]
    fn check_data_root() {
        let path = Path::from(DATA_FOLDER_NAME);
        let area_path = AreaPath::from(path.clone());
        assert!(area_path.is_data_root());

        let path = Path::from("no_root");
        let area_path = AreaPath::from(path.clone());
        assert!(!area_path.is_data_root())
    }

    #[test]
    fn check_table_root() {
        let path = AreaPath::from(format!("{}/{}", DATA_FOLDER_NAME, "table"));
        assert!(path.is_table_root());

        let path = AreaPath::from(DATA_FOLDER_NAME);
        assert!(!path.is_table_root());

        let path = AreaPath::from(format!("foo/bar/{}/{}", DATA_FOLDER_NAME, "table"));
        assert!(path.is_table_root());

        let path = AreaPath::from(format!("foo/bar/{}", DATA_FOLDER_NAME));
        assert!(!path.is_table_root());
    }
}
