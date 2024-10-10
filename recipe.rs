use serde::{Serialize, Deserialize};

// Creates a data structure to store recipes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: u32,
    pub name: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub servings: u32,
}

// Creates an implimentation of Recipe, that calls the new function when called, storing the variables to the Recipe
impl Recipe {
    pub fn new(id: u32, name: String, ingredients: Vec<String>, instructions: Vec<String>, servings: u32) {
        Recipe {
            id,
            name,
            ingredients,
            instructions,
            servings,
        }
    }
}