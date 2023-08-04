use dialoguer::{theme::ColorfulTheme, Select};
pub fn tab_selection() {
    let selections = vec![
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("Enjoy your {}!\n\n\n\n", selections[selection]);

    //? with Optional
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Optionally pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact_opt()
        .unwrap();
    if let Some(_selection) = selection {
        println!("Enjoy your {}!\n\n\n\n", selections[_selection]);
    } else {
        println!("You didn't select anything!\n\n\n\n");
    }

    //? with Optional. and max 2
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Optionally pick your flavor, hint it might be on the second page")
        .default(0)
        .max_length(2)
        .items(&selections[..])
        .interact_opt()
        .unwrap();
    if let Some(_selection) = selection {
        println!("Enjoy your {}!\n\n\n\n", selections[_selection]);
    } else {
        println!("You didn't select anything!\n\n\n\n");
    }
}
