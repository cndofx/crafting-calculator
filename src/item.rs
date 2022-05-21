/// Fully defines any item involved in the crafting process.
#[derive(Debug)]
pub struct Item {
    /// Internal item name used for identification.
    pub id: String,

    /// Item name displayed to the user.
    pub name: String,

    /// An optional recipe defining the process to craft the item.
    /// Set to *None* if the item is a raw material or can't be crafted.
    pub recipe: Option<Recipe>,
}

/// A vector of *Ingredients* and other data defining a crafting recipe.
#[derive(Debug)]
pub struct Recipe {
    /// A vector of all the ingredients used to craft this item.
    /// # Example:
    /// ```
    /// ingredients: vec![
    ///     Ingredient{ id: "iron_ore".to_string(), count: 1.0 },
    ///     Ingredient{ id: "coal".to_string(), count: 0.125 },
    /// ],
    /// ```
    pub ingredients: Vec<Ingredient>,

    /// A float specifying the time it takes to process/craft this item in seconds.
    pub processing_time: f32,

    /// The number of items produced per recipe on average.
    pub result: f32,
}

/// A reference to an *Item* and a count, used in *Recipes*.
#[derive(Debug)]
pub struct Ingredient {
    /// ID corresponding to an instance of *Item*.
    pub id: String,
    /// The number of the specified item used the recipe.
    pub count: f32,
}
