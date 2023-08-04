use dialoguer::{theme::ColorfulTheme, MultiSelect};

pub fn multi_selection() {
    let multiselected = vec![
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];

    let defaults = vec![false; 5];

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your foods")
        .items(&multiselected[..])
        .defaults(&defaults)
        .interact_opt()
        .unwrap();

    if let Some(selections) = selections {
        if selections.is_empty() {
            println!("You did not select anything :(");
        } else {
            println!("You selected these things:");
            for s in selections {
                println!("  {}", multiselected[s]);
            }
        }
    } else {
        println!("You did not select anything :(");
    }
}
