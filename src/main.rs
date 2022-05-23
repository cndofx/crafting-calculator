use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

mod interface;
mod item;
mod storage;

fn main() -> Result<(), anyhow::Error> {
    let theme = ColorfulTheme::default();

    // let new_item = new_item()?;
    // dbg!(new_item);

    // select recipe book ========
    let choice = Select::with_theme(&theme)
        .with_prompt("Select a recipe book")
        .default(0)
        .item("create new book")
        .interact()?;
    match choice {
        0 => interface::new_recipe_book()?,
        _ => unreachable!(),
    }
    // select recipe book ========

    Ok(())
}
