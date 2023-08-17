use crate::configs::ESLINT_REACT;
use crate::configs::ESLINT_TS;
use crate::configs::TYPESCRIPT_CONFIG;
use crate::configs::TYPESCRIPT_CONFIG_REACT;
use crate::configs::VITE_CONFIG;
use crate::configs::VITE_CONFIG_REACT;

pub enum Template {
    React,
    Typescript,
}

impl Template {
    pub fn to_string(&self) -> &str {
        match &self {
            Template::React => "React",
            Template::Typescript => "Typescript",
        }
    }

    pub fn to_template(input: &str) -> Template {
        match input {
            "React" => Template::React,
            "Typescript" => Template::Typescript,
            _ => Template::React,
        }
    }

    pub fn get_vite_cmd(&self, project_dir: &String) -> String {
        match &self {
            Template::React => format!("create vite@latest {} -- --template react-ts", project_dir),
            Template::Typescript => format!(
                "create vite@latest {} -- --template vanilla-ts",
                project_dir
            ),
        }
    }

    pub fn get_deps(&self) -> String {
        match &self {
            Template::React => String::from("i -D  eslint-plugin-prettier prettier"),
            Template::Typescript => String::from("i -D vite-plugin-dts @typescript-eslint/eslint-plugin @types/node @typescript-eslint/parser eslint eslint-plugin-prettier prettier"),
        }
    }

    pub fn get_eslint_raw(&self) -> &str {
        match &self {
            Template::React => ESLINT_REACT,
            Template::Typescript => ESLINT_TS,
        }
    }

    pub fn get_vite_config_raw(&self) -> &str {
        match &self {
            Template::React => VITE_CONFIG_REACT,
            Template::Typescript => VITE_CONFIG,
        }
    }

    pub fn get_ts_config_raw(&self) -> &str {
        match &self {
            Template::React => TYPESCRIPT_CONFIG_REACT,
            Template::Typescript => TYPESCRIPT_CONFIG,
        }
    }

    pub fn get_boilerplate(&self) -> Vec<&str> {
        match &self {
            Template::React => vec![""],
            Template::Typescript => vec!["src/counter.ts", "src/style.css", "src/typescript.svg"],
        }
    }
}
