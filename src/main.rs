mod configs;
mod template;
use crate::configs::PRETTIER_CONFIG;
use template::Template;

mod interactions;
use interactions::{ask_for_project_name, ask_for_template};

use spinners::{Spinner, Spinners};
use std::{env, process::Command};

struct Configuration {
    vite_cmd: String,
    deps_cmd: String,
    eslint_settings_raw: String,
    vite_config_raw: String,
    ts_config_raw: String,
}

impl Configuration {
    fn get_configuration(template: &Template, name: &String) -> Configuration {
        return Configuration {
            vite_cmd: Template::get_vite_cmd(&template, &name),
            deps_cmd: Template::get_deps(&template),
            eslint_settings_raw: Template::get_eslint_raw(&template).to_string(),
            vite_config_raw: Template::get_vite_config_raw(&template).to_string(),
            ts_config_raw: Template::get_ts_config_raw(&template).to_string(),
        };
    }
}

fn execute_npm_command(cmd: String, path: Option<&String>, spinner_message: &str) {
    // init loader
    let mut sp = Spinner::new(Spinners::Dots9, spinner_message.into());
    // Get the vec of commands
    let cmd: Vec<&str> = cmd.split_whitespace().collect();

    if let Some(path) = path {
        // Change directory
        std::env::set_current_dir(path).unwrap();
    }

    // Eseguo il comando
    let res = Command::new("npm")
        .args(cmd)
        .output()
        .expect("Failed to execute command");

    match res.status.success() {
        true => {
            if let Some(_) = path {
                // Change directory
                std::env::set_current_dir("..").unwrap();
            }
            // Stop the spinner
            sp.stop();
        }
        false => {
            // terminate program with error message
            let error_message = String::from_utf8_lossy(&res.stderr);
            eprintln!("Error: {}", error_message);
            std::process::exit(1);
        }
    }
}

fn copy_content(content: &String, to: &str) {
    // Create a copy in the new directory
    std::fs::write(to, content).unwrap();
}

fn clean_boilerplate(project_path: &str, template: &Template) {
    // navigate to directory
    std::env::set_current_dir(project_path).unwrap();

    let to_be_deleted = Template::get_boilerplate(template);
    let to_be_empty = vec!["src/main.ts"];

    for file in to_be_deleted {
        // if exists, delete
        if std::path::Path::new(file).exists() {
            std::fs::remove_file(file).unwrap();
        }
    }

    for file in to_be_empty {
        // if exists, empty
        if std::path::Path::new(file).exists() {
            std::fs::write(file, "").unwrap();
        }
    }
}

fn main() {
    let mut directory = String::from('.');

    // if a path is given as an argoument, it will be used
    let args: Vec<String> = env::args().collect();

    if args.len() >= 1 {
        if let Some(path) = args.get(1) {
            directory = String::from(path);
        }
    }

    // navigate to directory
    std::env::set_current_dir(directory).unwrap();

    let mut chosen_path = String::from(".");
    match env::current_dir() {
        Ok(path) => {
            chosen_path = path.display().to_string();
        }
        Err(e) => println!("Failed to get current directory. Error: {}", e),
    }

    let project_name = ask_for_project_name();

    let template = ask_for_template();

    println!(
        "Start creation of: {project_name}, in {chosen_path}, using {} template",
        template.to_string()
    );

    // extract the configuration
    let Configuration {
        vite_cmd,
        deps_cmd,
        eslint_settings_raw,
        vite_config_raw,
        ts_config_raw,
    } = Configuration::get_configuration(&template, &project_name);

    // install vite
    execute_npm_command(vite_cmd, None, "Installing vite template...");
    // install deps
    execute_npm_command(deps_cmd, Some(&project_name), "Installing deps...");

    // Copy eslint settings
    copy_content(
        &eslint_settings_raw,
        &format!("{}/.eslintrc.cjs", project_name),
    );

    // Copy prettier settings
    copy_content(
        &PRETTIER_CONFIG.to_string(),
        &format!("{}/.prettierrc", project_name),
    );

    // Copy vite settings
    copy_content(
        &vite_config_raw,
        &format!("{}/vite.config.ts", project_name),
    );

    // Copy ts settings
    copy_content(&ts_config_raw, &format!("{}/tsconfig.json", project_name));

    // Clean boilerplate
    clean_boilerplate(&project_name, &template);
}
