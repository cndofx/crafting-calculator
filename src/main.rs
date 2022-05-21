use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

mod item;
mod storage;
use item::{Item, Recipe, Ingredient};
use storage::{Config, RecipeBook};

fn main() -> Result<(), anyhow::Error> {
    let theme = ColorfulTheme::default();

    // let new_item = new_item()?;
    // dbg!(new_item);

    // load config file ==========
    let mut config: Config = Config::new();
    if let Ok(conf) = Config::from_file() {
        config = conf;
    }
    else {
        if Confirm::with_theme(&theme)
            .with_prompt("'config.json' cannot be read. Generate a new config?")
            .interact()?
        {
            config = Config::new();
            Config::save_file(&config)?;
        }
    }
    // load config file ==========

    // select recipe book ========
    let choice = Select::with_theme(&theme)
        .with_prompt("Select a recipe book")
        .default(0)
        .item("create new book")
        .interact()?;
    match choice {
        0 => new_recipe_book()?,
        _ => unreachable!()
    }
    // select recipe book ========

    Ok(())
}

fn new_recipe_book() -> Result<(), anyhow::Error> {
    let name = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Name")
        .interact()?;
    let book = RecipeBook::new(name);
    book.save()?;

    Ok(())
}

fn new_item() -> Result<Item, anyhow::Error> {
    let theme = ColorfulTheme::default();
    println!("Item Creation Wizard");

    let name = Input::with_theme(&theme)
        .with_prompt("Item Name")
        .interact()?;

    let has_recipe = Select::with_theme(&theme)
        .with_prompt("Has recipe?")
        .default(0)
        .item("no")
        .item("yes")
        .interact()?;

    let recipe: Option<Recipe> = match has_recipe {
        // set recipe to none
        0 => None,
        // define a new recipe
        1 => {
            let mut ingredients: Vec<Ingredient> = Vec::new();
            loop {
                let choice = Select::with_theme(&theme)
                    .with_prompt("Recipe Editor")
                    .default(0)
                    .item("add ingredient")
                    .item("list ingredients")
                    .item("confirm")
                    .interact()?;

                match choice {
                    // add ingredient
                    0 => {
                        loop {
                            let name = Input::with_theme(&theme)
                                .with_prompt("Ingredient Name")
                                .interact()?;
                            let count = Input::with_theme(&theme)
                                .with_prompt("Ingredient Count")
                                .interact()?;
                            if !Confirm::with_theme(&theme)
                                .with_prompt(format!("Add {} {} to the list?", count, name))
                                .interact()?
                            {
                                continue;
                            }
                            ingredients.push(Ingredient { name, count });
                            break;
                        }
                    },
                    // list ingredients
                    1 => {
                        for ingredient in ingredients.iter() {
                            println!("{} {}", ingredient.count, ingredient.name);
                        }
                    },
                    // confirm
                    2 => {
                        if Confirm::with_theme(&theme)
                            .with_prompt("Confirm ingredients?")
                            .interact()?
                        {
                            break;
                        }
                    },
                    _ => unreachable!(),
                }
            }
            
            let processing_time = Input::with_theme(&theme)
                .with_prompt("How many seconds does it take to craft?")
                .default("0.0".to_string())
                .interact()?;
            let result = Input::with_theme(&theme)
                .with_prompt("How many resulting items per craft?")
                .default("1.0".to_string())
                .interact()?;
            
            let processing_time = processing_time.parse::<f32>().unwrap();
            let result = result.parse::<f32>().unwrap();

            Some(Recipe {
                ingredients,
                processing_time,
                result,
            })
        }
        _ => unreachable!(),
    };

    Ok(Item {
        name,
        recipe,
    })
}
