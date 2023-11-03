use serde::{Deserialize, Serialize};

use crate::store::get_store_path;

const MATERIAL_STORE_PATH: &str = "materials.toml";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Material {
    pub(crate) name: String,
    pub(crate) value: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name: String::from(""),
            value: 0.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
struct StoredMaterial {
    materials: Vec<Material>,
}

impl Default for StoredMaterial {
    fn default() -> Self {
        Self {
            materials: vec![]
        }
    }
}

fn get_material_store_path() -> String {
    format!("{}/{}", get_store_path(), MATERIAL_STORE_PATH)
}

pub(crate) fn get_materials() -> Vec<Material> {
    let stored_config = confy::load_path::<StoredMaterial>(&get_material_store_path());
    if stored_config.is_ok() {
        let stored_materials = stored_config.unwrap();
        return stored_materials.materials;
    }
    store_materials(vec![]);
    vec![]
}

pub(crate) fn store_materials(materials: Vec<Material>) {
    let stored_material = StoredMaterial {
        materials
    };
    confy::store_path(&get_material_store_path(), stored_material)
        .expect("Unable to store updated materials");
}
