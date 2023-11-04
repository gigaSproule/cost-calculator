use serde::{Deserialize, Serialize};

use crate::store::get_store_path;

const MATERIAL_STORE_PATH: &str = "materials.toml";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct StoredMaterial {
    pub(crate) name: String,
    pub(crate) quantity_per_pack: f64,
    pub(crate) price_per_pack: f64,
    pub(crate) value: f64,
}

impl Default for StoredMaterial {
    fn default() -> Self {
        Self {
            name: String::from(""),
            quantity_per_pack: 0.0,
            price_per_pack: 0.0,
            value: 0.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
struct MaterialStore {
    materials: Vec<StoredMaterial>,
}

impl Default for MaterialStore {
    fn default() -> Self {
        Self { materials: vec![] }
    }
}

fn get_material_store_path() -> String {
    format!("{}/{}", get_store_path(), MATERIAL_STORE_PATH)
}

pub(crate) fn get_materials() -> Vec<StoredMaterial> {
    let stored_config = confy::load_path::<MaterialStore>(&get_material_store_path());
    if stored_config.is_ok() {
        let stored_materials = stored_config.unwrap();
        return stored_materials.materials;
    }
    store_materials(vec![]);
    vec![]
}

pub(crate) fn store_materials(materials: Vec<StoredMaterial>) {
    let stored_material = MaterialStore { materials };
    confy::store_path(&get_material_store_path(), stored_material)
        .expect("Unable to store updated materials");
}
