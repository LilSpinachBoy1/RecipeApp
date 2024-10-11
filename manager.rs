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
    pub fn get_recipe(&self, id: u32) -> Option<&Recipe> {
        // .iter() --> Creates an iterator over the recipes vec.
        // .find(|r| r.id == id) --> Finds based on a condition, the condition takes r as the current item in recipes (Recipe struct) and checks if it's id matches the id passed in
        self.recipes.iter().find(|r| r.id == id)
    }

    // Function to change the details of a recipe, returns true if changes made, or false if no recipe found
    pub fn update_recipe(&mut self, id: u32, name: String, ingredients: Vec<String>, instructions: Vec<String>, servings: u32) -> bool {
        // If the .find() returns a value, recipe is given the value of the .find() result, and the if let statement executes, otherwise jumps
        if let Some(recipe) = self.recipes.iter_mut().find(|r| r.id == id) {
            // recipe is now the value of the found recipe, so we can edit values (due to it being mutable thanks to .iter_mut())
            recipe.name = name;
            recipe.ingredients = ingredients;
            recipe.instructions = instructions;
            recipe.servings = servings;
            true  // Return true
        }
        else {false}  // Return false
    }

    // Function to delete a specified recipe
    pub fn delete_recipe(&mut self, id: u32) -> bool {
        let initial_len = self.recipes.len();
        self.recipes.retain(|r| r.id != id);  // Retains values that don't match the given id in the list
        self.recipes.len() < initial_len  // Return true if a value is returned, or false if not
    }

    // Function to save recipes to a file
    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let json = serde_json::to_string(&self.recipes)?;  // The ? means that if the string cannot be converted, the function automatically returns an error
        fs::write(filename, json)  // Write the data to the file, and then return weather the result was successful
    }

    // Function to load recipes from a file
    pub fn load_from_file(&self ,filename: &str) -> std::io::Result<()> {
        let json = fs::read_to_string(filename)?;  // Reads contents of file and saves as a string
        self.recipes = serde_json::from_str(&json)?;  // Convert json str and store as recipes

        // There's a lot here so here we go...
        self.next_id = self.recipes.iter()
            .map(|r| r.id)  // Gets each item's id
            .max()  // Finds the largest value, returns Option<u32>, as can return Some(largest_id) if there is a value, or None if the list is empty
            .unwrap_or(0) + 1;  // If max returns Some(), unwrap_or will extract the value, if it returns None, the default value of 0 will be returned

        Ok(())  // Func has run successfully, as no errors have been raised yet
    }
}
