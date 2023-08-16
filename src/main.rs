const PRETTIER_CONFIG: &str = r#"{
  "printWidth": 80,
  "tabWidth": 2,
  "singleQuote": true,
}  
"#;

const VITE_CONFIG: &str = r#"import path from 'path';
import { defineConfig } from 'vite';
import dts from 'vite-plugin-dts';
// import pkg from './package.json' assert { type: 'json' };

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  build: {
    outDir: 'dist',
    lib: {
      entry: 'src/main.ts', // replace 'src/index.ts' with your library's entry point
      name: 'shaka-smartclip', // replace 'MyLibrary' with your library's name
      formats: ['es'], // choose your desired output formats
    },
    rollupOptions: {
      external: [
        // ...Object.keys(pkg.dependencies), // don't bundle dependencies
        /^node:.*/, // don't bundle built-in Node.js modules (use protocol imports!)
      ],
    },
  },
  plugins: [dts()], // emit TS declaration files
});
"#;

const VITE_CONFIG_REACT: &str = r#"import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  plugins: [react()],
})
"#;

const ESLINT_REACT: &str = r#"module.exports = {
  root: true,
  env: { browser: true, es2020: true },
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:react-hooks/recommended',
  ],
  ignorePatterns: ['dist', '.eslintrc.cjs'],
  parser: '@typescript-eslint/parser',
  plugins: ['react-refresh', "prettier"],
  rules: {
    'react-refresh/only-export-components': [
      'warn',
      { allowConstantExport: true },
    ],

    "prettier/prettier": "error",

    "quotes": ["error", "single"],
    "semi": ["error", "always"],
    "arrow-spacing": "error",
    "no-const-assign": "error",
    "no-empty": "off",

    "sort-imports": [
      "error",
      { "ignoreDeclarationSort": true, "ignoreCase": true }
    ],

    "@typescript-eslint/explicit-function-return-type": "warn",
    "@typescript-eslint/member-delimiter-style": "error",
    "@typescript-eslint/no-unused-vars": [
      "warn",
      { "ignoreRestSiblings": true }
    ],
    "@typescript-eslint/no-var-requires": "error",
    "@typescript-eslint/consistent-type-imports": [
      "warn",
      { "prefer": "type-imports" }
    ],
    "@typescript-eslint/explicit-function-return-type": "warn",
    "@typescript-eslint/no-empty-function": "off",
    "@typescript-eslint/explicit-member-accessibility": [
      "error",
      {
        "accessibility": "explicit",
        "overrides": {
          "constructors": "off"
        }
      }
    ]
  },
}
"#;

const ESLINT_TS: &str = r#"module.exports = {
  "env": {
    "browser": true,
    "es2021": true
  },
  "extends": ["eslint:recommended", "plugin:@typescript-eslint/recommended"],
  "root": true,
  "parser": "@typescript-eslint/parser",
  "parserOptions": {
    "ecmaVersion": "latest",
    "sourceType": "module"
  },
  "plugins": ["@typescript-eslint", "prettier"],
  "rules": {
    "prettier/prettier": "error",

    "quotes": ["error", "single"],
    "semi": ["error", "always"],
    "arrow-spacing": "error",
    "no-const-assign": "error",
    "no-empty": "off",

    "sort-imports": [
      "error",
      { "ignoreDeclarationSort": true, "ignoreCase": true }
    ],

    "@typescript-eslint/explicit-function-return-type": "warn",
    "@typescript-eslint/member-delimiter-style": "error",
    "@typescript-eslint/no-unused-vars": [
      "warn",
      { "ignoreRestSiblings": true }
    ],
    "@typescript-eslint/no-var-requires": "error",
    "@typescript-eslint/consistent-type-imports": [
      "warn",
      { "prefer": "type-imports" }
    ],
    "@typescript-eslint/explicit-function-return-type": "warn",
    "@typescript-eslint/no-empty-function": "off",
    "@typescript-eslint/explicit-member-accessibility": [
      "error",
      {
        "accessibility": "explicit",
        "overrides": {
          "constructors": "off"
        }
      }
    ]
  }
}
"#;

const TYPESCRIPT_CONFIG: &str = r#"{
  "compilerOptions": {
    "target": "ES5",
    "allowJs": true,
    "jsx": "react-jsx",
    // Paths
    "baseUrl": ".",
    "paths": {
      "@/*": ["src/*"]
    },

    "useDefineForClassFields": true,
    // "module": "ES5",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "skipLibCheck": true,

    /* Bundler mode */
    // "moduleResolution": "bundler",
    "declaration": true,
    "outDir": "dist/types",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,

    /* Linting */
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src"]
}
"#;

const TYPESCRIPT_CONFIG_REACT: &str = r#""compilerOptions": {
    "target": "ES5",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,

    /* Bundler mode */
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",

    /* Linting */
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }]
"#;

use dialoguer::{theme::ColorfulTheme, Select};
use spinners::{Spinner, Spinners};
use std::{env, io, process::Command};

fn ask_for_project_name() -> String {
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

enum Template {
    React,
    Typescript,
}

impl Template {
    fn to_string(&self) -> &str {
        match &self {
            Template::React => "React",
            Template::Typescript => "Typescript",
        }
    }

    fn to_template(input: &str) -> Template {
        match input {
            "React" => Template::React,
            "Typescript" => Template::Typescript,
            _ => Template::React,
        }
    }

    fn get_vite_cmd(&self, project_dir: &String) -> String {
        match &self {
            Template::React => format!("create vite@latest {} -- --template react-ts", project_dir),
            Template::Typescript => format!(
                "create vite@latest {} -- --template vanilla-ts",
                project_dir
            ),
        }
    }

    fn get_deps(&self) -> String {
        match &self {
            Template::React => String::from("i -D  eslint-plugin-prettier prettier"),
            Template::Typescript => String::from("i -D vite-plugin-dts @typescript-eslint/eslint-plugin @types/node @typescript-eslint/parser eslint eslint-plugin-prettier prettier"),
        }
    }

    fn get_eslint_raw(&self) -> &str {
        match &self {
            Template::React => ESLINT_REACT,
            Template::Typescript => ESLINT_TS,
        }
    }

    fn get_vite_config_raw(&self) -> &str {
        match &self {
            Template::React => VITE_CONFIG_REACT,
            Template::Typescript => VITE_CONFIG,
        }
    }

    fn get_ts_config_raw(&self) -> &str {
        match &self {
            Template::React => TYPESCRIPT_CONFIG_REACT,
            Template::Typescript => TYPESCRIPT_CONFIG,
        }
    }

    fn get_boilerplate(&self) -> Vec<&str> {
        match &self {
            Template::React => vec![""],
            Template::Typescript => vec!["src/counter.ts", "src/style.css", "src/typescript.svg"],
        }
    }
}

fn ask_for_template() -> Template {
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
