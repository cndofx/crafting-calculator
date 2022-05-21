/// An `Item` fully defines any item involved in the crafting process,
/// whether it's craftable or not.
#[derive(Debug)]
pub struct Item {
    /// String used both for item display and identification.
    pub name: String,

    /// Optional recipe defining the process to craft the item.
    /// Set to `None` if the item is a raw material or can't be crafted.
    pub recipe: Option<Recipe>,
}

/// A `Recipe` is a vector of type `Ingredient` and other data defining a crafting recipe.
#[derive(Debug)]
pub struct Recipe {
    /// Vector of all the ingredients used in the recipe.
    /// ### Example:
    /// ```
    /// ingredients: vec![
    ///     Ingredient{ id: "iron_ore".to_string(), count: 1.0 },
    ///     Ingredient{ id: "coal".to_string(), count: 0.125 },
    /// ],
    /// ```
    pub ingredients: Vec<Ingredient>,

    /// Float specifying the time it takes to process/craft this item in seconds.
    pub processing_time: f32,

    /// Number of items produced per recipe.
    /// It allows floating point values in cases where the output isn't
    /// always the same. For example, if a recipe produces 1 item, but with
    /// a 50% chance to produce an additional item, `result` would be 1.5.
    pub result: f32,
}

/// An `Ingredient` is a reference to an `Item` and a count, used in a `Recipe`.
#[derive(Debug)]
pub struct Ingredient {
    /// Ingredient name corresponding to an instance of `Item`.
    pub name: String,
    /// Number of the specified item used the recipe.
    pub count: f32,
}
