use crate::recipe::Recipe;  // Recipe we accessed
use std::fs;  // File system manipulation

// Create a recipe manager struct to use from main
pub struct RecipeManager {
    recipes: vec<Recipe>,
    next_id: u32,
}

// Implementation of recipe manager structure
impl RecipeManager {
    // Run when the impl is used (kinda like __init__)
    pub fn new() -> Self {
        RecipeManager {
            recipes: Vec::new(),
            next_id: 1,
        }
    }

    // Function to add a new recipe
    // Why "&mut self"? self is similar to in python, where it is the struct itself, and the members of it. & means it is a reference. mut means it is safely referenced, and is not accessible from other locations
    pub fn add_recipe(&mut self, name: String, ingredients: Vec<String>, instructions: Vec<String>, servings: u32) -> u32 {
        // Get the id that this recipe will be, and add a new recipe to the recipes array
        let id = self.next_id;
        self.recipes.push(Recipe::new(id, name, ingredients, instructions, servings));

        // Increment next_id for the next recipe and then return id of new recipe
        self.next_id += 1;
        id
    }

    // Function to access all recipes (-> return type, like python funcs)
    pub fn get_all_recipes(&self) -> Vec<Recipe> {&self.recipes}

    // Function to get a specific recipe based on its id
    //TODO: Figure out what tf .iter().rust(|r| r.id == id) does
    pub fn get_recipe(&self, id: u32) -> Option<&Recipe> {self.recipes.iter().find(|r| r.id == id)}
}
