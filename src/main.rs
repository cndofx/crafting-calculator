use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

mod interface;
mod item;
mod storage;

use item::Recipe;
use storage::RecipeBook;

use crate::interface::new_item;

fn main() -> Result<(), anyhow::Error> {
    let theme = ColorfulTheme::default();

    // let new_item = new_item()?;
    // dbg!(new_item);

    // select recipe book ========
    let mut recipe_book: RecipeBook;
    loop {
        let recipe_books = RecipeBook::get_book_names()?;

        let choice = Select::with_theme(&theme)
            .with_prompt("Select a recipe book")
            .default(0)
            .item("-- create new book --")
            .items(&recipe_books)
            .interact()?;
        match choice {
            0 => interface::new_recipe_book()?,
            _ => {
                recipe_book = RecipeBook::load(&recipe_books[choice - 1]).unwrap();
                break;
            }
        }
    }
    // select recipe book ========

    println!("selected book: {:?}", recipe_book);

    let test_item = new_item()?;
    recipe_book.add_item(test_item);

    //dbg!(&recipe_book);
    recipe_book.save()?;

    Ok(())
}
