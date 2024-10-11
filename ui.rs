use iced::widget::{Button, Column, Container, Row, Scrollable, Text, TextInput};  // Import UI widget components
use iced::{Element, Length, Sandbox, Settings};
use iced::theme;
use iced::Color;
use crate::manager::RecipeManager;  // Import the RecipeManager we made (cus it serves so much)
use crate::recipe::Recipe;  // Import the Recipe struct and its implementation

// Create the structure for the GUI
pub struct RecipeManagerGUI {
    recipe_manager: RecipeManager,
    recipe_name: String,
    recipe_ingredients: String,
    recipe_instructions: String,
    recipe_servings: String,
    selected_recipe: Option<String>,  // May return none, so has to be Option<String>
    error_message: Option<String>,
    editing: bool,
}

// enum can represent data of multiple forms, in this case the different options for what can happen in the app
#[derive(Debug, Clone)]
pub enum Message {
    AddRecipe,
    EditRecipe(u32),
    UpdateRecipe,
    CancelEdit,
    RecipeNameChanged(String),
    RecipeIngredientsChanged(String),
    RecipeInstructionsChanged(String),
    RecipeServingsChanged(String),
    RecipeSelected(Recipe),
    DeleteRecipe(u32),
    SaveRecipes,
    LoadRecipes,
}

impl Sandbox for RecipeManagerGUI {
    type Message = Message;

    // Called on creation
    fn new() -> Self {
        // Defines a bunch of attributes to self
        Self {
            recipe_manager: RecipeManager::new(),
            recipe_name: String::new(),
            recipe_ingredients: String::new(),
            recipe_instructions: String::new(),
            recipe_servings: String::new(),
            selected_recipe: None,
            error_message: None,
            editing: false,
        }
    }

    // Sets the title of the page (I think)
    fn title(&self) -> String {
        String::from("RecipeManager")  // Return the string given from the literal string "RecipeManager" (essentially just set the title of the page as RecipeManager)
    }

    fn update(&mut self, message: Message) {
        // Match each possible action to a process to do as a result
        match message {
            Message::AddRecipe => {
                if !self.recipe_name.is_empty() {  // If the recipe name is something
                    let servings = self.recipe_servings.parse().unwrap_or(1);  // Try to get an int32 from servings string, and just return 1 if it cant
                    self.recipe_manager.add_recipe(self.recipe_name.clone(), self.recipe_ingredients.split(',').map(String::from).collect(), self.recipe_instructions.split('\n').map(String::from).collect(), servings);

                    // Clear the values from self, ready for next time round!
                    self.recipe_name.clear();
                    self.recipe_ingredients.clear();
                    self.recipe_instructions.clear();
                    self.recipe_servings.clear();
                }
            }

            Message::EditRecipe(id) => {
                if let Some(recipe) = self.recipe_manager.get_recipe(id) {
                    self.recipe_name = recipe.name.clone();
                    self.recipe_ingredients = recipe.ingredients.join(", ");  // Join array elements and store as a string in the object
                    self.recipe_instructions = recipe.instructions.join("\n");
                    self.recipe_servings = recipe.servings.to_string();  // Convert the servings to string
                    self.selected_recipe = Some(recipe.clone());  // Covers if there is no currently selected recipe
                    self.editing = true;
                }
            }
        }
    }
}