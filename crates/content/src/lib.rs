use serde::{Deserialize, Serialize};

pub const CONTENT_FORMAT_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrototypeRegistry {
    pub version: u32,
    pub items: Vec<ItemPrototype>,
    pub entities: Vec<EntityPrototype>,
    pub recipes: Vec<RecipePrototype>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPrototype {
    pub id: String,
    pub stack_size: u32,
    pub transport_speed_mod: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityPrototype {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipePrototype {
    pub id: String,
    pub inputs: Vec<RecipeIoEntry>,
    pub outputs: Vec<RecipeIoEntry>,
    pub crafting_time_ticks: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeIoEntry {
    pub item_id: String,
    pub amount: u32,
}

pub fn new_empty_registry() -> PrototypeRegistry {
    PrototypeRegistry {
        version: CONTENT_FORMAT_VERSION,
        items: vec![],
        entities: vec![],
        recipes: vec![],
    }
}

pub fn valid_recipe(recipe: &RecipePrototype) -> bool {
    recipe.crafting_time_ticks > 0
        && !recipe.inputs.is_empty()
        && !recipe.outputs.is_empty()
        && recipe.outputs.iter().all(|r| !r.item_id.trim().is_empty())
        && recipe.inputs.iter().all(|r| !r.item_id.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_registry_starts_with_current_version() {
        let registry = new_empty_registry();

        assert_eq!(registry.version, CONTENT_FORMAT_VERSION);
        assert!(registry.items.is_empty());
        assert!(registry.entities.is_empty());
        assert!(registry.recipes.is_empty());
    }

    #[test]
    fn valid_recipe_rejects_invalid_recipe() {
        let missing_crafting_time = RecipePrototype {
            id: "broken".to_string(),
            inputs: vec![RecipeIoEntry {
                item_id: "iron".to_string(),
                amount: 1,
            }],
            outputs: vec![RecipeIoEntry {
                item_id: "plate".to_string(),
                amount: 1,
            }],
            crafting_time_ticks: 0,
        };
        let empty_inputs = RecipePrototype {
            id: "broken".to_string(),
            inputs: vec![],
            outputs: vec![RecipeIoEntry {
                item_id: "plate".to_string(),
                amount: 1,
            }],
            crafting_time_ticks: 1,
        };
        let blank_outputs = RecipePrototype {
            id: "broken".to_string(),
            inputs: vec![RecipeIoEntry {
                item_id: "iron".to_string(),
                amount: 1,
            }],
            outputs: vec![RecipeIoEntry {
                item_id: "".to_string(),
                amount: 1,
            }],
            crafting_time_ticks: 1,
        };

        assert!(!valid_recipe(&missing_crafting_time));
        assert!(!valid_recipe(&empty_inputs));
        assert!(!valid_recipe(&blank_outputs));
    }

    #[test]
    fn valid_recipe_accepts_legit_data() {
        let recipe = RecipePrototype {
            id: "iron_plate".to_string(),
            inputs: vec![RecipeIoEntry {
                item_id: "iron_ore".to_string(),
                amount: 2,
            }],
            outputs: vec![RecipeIoEntry {
                item_id: "iron_plate".to_string(),
                amount: 1,
            }],
            crafting_time_ticks: 5,
        };

        assert!(valid_recipe(&recipe));
    }
}
