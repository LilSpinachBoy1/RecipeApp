use crate::recipe::Recipe;  // Recipe we accessed
use std::fs;  // File system manipulation

// Create a recipe manager struct to use from main
pub struct RecipeManager {
    recipes: vec<Recipe>,
    next_id: u32,
}

// Implimentation of recipe manager structure
impl RecipeManager {
    // Run when the impl is used (kinda like __init__)
    pub fn new() -> Self {
        RecipeManager {
            recipes: Vec::new(),
            next_id: 1,
        }
    }

    // Function to add a new recipe
    // Why "&mut self"? self is similar to in python, where it is the struct itself, and the members of it. & menas it is a reference. mut means it is safely referenced, and is not accessable from other locations
    pub fn add_recipe(&mut self, name: String, ingredients: Vec<String>, instructions: Vec<String>, servings: u32) {

    }
}
