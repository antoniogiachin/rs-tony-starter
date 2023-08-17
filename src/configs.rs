pub const PRETTIER_CONFIG: &str = r#"{
  "printWidth": 80,
  "tabWidth": 2,
  "singleQuote": true,
}  
"#;

pub const VITE_CONFIG: &str = r#"import path from 'path';
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

pub const VITE_CONFIG_REACT: &str = r#"import { defineConfig } from 'vite'
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

pub const ESLINT_REACT: &str = r#"module.exports = {
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

pub const ESLINT_TS: &str = r#"module.exports = {
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

pub const TYPESCRIPT_CONFIG: &str = r#"{
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

pub const TYPESCRIPT_CONFIG_REACT: &str = r#""compilerOptions": {
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
