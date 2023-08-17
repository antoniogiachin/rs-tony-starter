use crate::template::Template;
use dialoguer::{theme::ColorfulTheme, Select};
use std::io;

pub fn ask_for_project_name() -> String {
    fn fallback_to_def() -> String {
        return String::from("my-project");
    }

    println!("Enter your project name");

    let mut proj_name = String::new();

    match io::stdin().read_line(&mut proj_name) {
        Ok(_) => {
            if !proj_name.trim().is_empty() {
                return proj_name.trim().to_string();
            } else {
                return fallback_to_def();
            }
        }

        _ => {
            return fallback_to_def();
        }
    }
}

pub fn ask_for_template() -> Template {
    let options = vec![
        Template::React.to_string(),
        Template::Typescript.to_string(),
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&options)
        .default(0)
        .interact();

    match selection {
        Ok(idx) => Template::to_template(options[idx]),
        Err(_) => Template::to_template(options[0]),
    }
}
