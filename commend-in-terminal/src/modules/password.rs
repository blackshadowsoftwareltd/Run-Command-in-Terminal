use dialoguer::{theme::ColorfulTheme, Password};
pub fn password_taking() {
    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your Password :")
        .with_confirmation("Repeat Password", "Error : the password don't match.")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.len() > 3 {
                Ok(())
            } else {
                Err("Password must be longer than 3")
            }
        })
        .interact()
        .unwrap();
    println!("Your password is {}", password);
}
