use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    pub root: bool,
    pub charset: String,
    pub end_of_line: String,
    pub insert_final_newline: bool,
    pub trim_trailing_whitespace: bool,
    pub sections: HashMap<String, EditorConfigSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfigSection {
    pub indent_style: String,
    pub indent_size: String,
    pub end_of_line: Option<String>,
    pub charset: Option<String>,
    pub trim_trailing_whitespace: Option<bool>,
    pub insert_final_newline: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrettierConfig {
    pub semi: bool,
    pub single_quote: bool,
    pub tab_width: u8,
    pub trailing_comma: String,
    pub print_width: u8,
    pub plugins: Option<Vec<String>>,
    /// Emits `rubySingleQuote` when set. Only meaningful with `@prettier/plugin-ruby`.
    pub ruby_single_quote: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageJson {
    pub name: String,
    pub version: String,
    pub description: String,
    pub dev_dependencies: HashMap<String, String>,
}

impl fmt::Display for EditorConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "root = {}", self.root)?;
        writeln!(f, "charset = {}", self.charset)?;
        writeln!(f, "end_of_line = {}", self.end_of_line)?;
        writeln!(f, "insert_final_newline = {}", self.insert_final_newline)?;
        writeln!(
            f,
            "trim_trailing_whitespace = {}",
            self.trim_trailing_whitespace
        )?;
        writeln!(f)?;

        for (pattern, section) in &self.sections {
            writeln!(f, "[{}]", pattern)?;
            writeln!(f, "indent_style = {}", section.indent_style)?;
            writeln!(f, "indent_size = {}", section.indent_size)?;

            if let Some(ref end_of_line) = section.end_of_line {
                writeln!(f, "end_of_line = {}", end_of_line)?;
            }

            if let Some(ref charset) = section.charset {
                writeln!(f, "charset = {}", charset)?;
            }

            if let Some(trim_trailing_whitespace) = section.trim_trailing_whitespace {
                writeln!(f, "trim_trailing_whitespace = {}", trim_trailing_whitespace)?;
            }

            if let Some(insert_final_newline) = section.insert_final_newline {
                writeln!(f, "insert_final_newline = {}", insert_final_newline)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl Default for EditorConfig {
    fn default() -> Self {
        let mut sections = HashMap::new();

        // Default section
        sections.insert(
            "*".to_string(),
            EditorConfigSection {
                indent_style: "space".to_string(),
                indent_size: "2".to_string(),
                end_of_line: Some("lf".to_string()),
                charset: Some("utf-8".to_string()),
                trim_trailing_whitespace: Some(true),
                insert_final_newline: Some(true),
            },
        );

        // Ruby files
        sections.insert(
            "*.{rb,erb,ru,rake,gemspec}".to_string(),
            EditorConfigSection {
                indent_style: "space".to_string(),
                indent_size: "2".to_string(),
                end_of_line: Some("lf".to_string()),
                charset: Some("utf-8".to_string()),
                trim_trailing_whitespace: Some(true),
                insert_final_newline: Some(true),
            },
        );

        // JavaScript/TypeScript and other web files
        sections.insert(
            "*.{yml,yaml,haml,jbuilder,jsx,html,sls,tf}".to_string(),
            EditorConfigSection {
                indent_style: "space".to_string(),
                indent_size: "2".to_string(),
                end_of_line: Some("lf".to_string()),
                charset: Some("utf-8".to_string()),
                trim_trailing_whitespace: Some(true),
                insert_final_newline: Some(true),
            },
        );

        // Makefiles
        sections.insert(
            "{*[Mm]akefile*,*.mak,*.mk,depend}".to_string(),
            EditorConfigSection {
                indent_style: "tab".to_string(),
                indent_size: "4".to_string(),
                end_of_line: None,
                charset: None,
                trim_trailing_whitespace: None,
                insert_final_newline: None,
            },
        );

        // enc/ directory
        sections.insert(
            "enc/*".to_string(),
            EditorConfigSection {
                indent_style: "tab".to_string(),
                indent_size: "8".to_string(),
                end_of_line: None,
                charset: None,
                trim_trailing_whitespace: None,
                insert_final_newline: None,
            },
        );

        // reg files with C headers
        sections.insert(
            "reg*.[ch]".to_string(),
            EditorConfigSection {
                indent_style: "tab".to_string(),
                indent_size: "8".to_string(),
                end_of_line: None,
                charset: None,
                trim_trailing_whitespace: None,
                insert_final_newline: None,
            },
        );

        Self {
            root: true,
            charset: "utf-8".to_string(),
            end_of_line: "lf".to_string(),
            insert_final_newline: true,
            trim_trailing_whitespace: true,
            sections,
        }
    }
}

impl Default for PrettierConfig {
    fn default() -> Self {
        Self {
            semi: true,
            single_quote: true,
            tab_width: 2,
            trailing_comma: "es5".to_string(),
            print_width: 80,
            plugins: None,
            ruby_single_quote: None,
        }
    }
}

impl PrettierConfig {
    /// Configure Prettier as the Ruby formatting source of truth.
    ///
    /// `@prettier/plugin-ruby` (npm) drives Ruby style; RuboCop cops that fight
    /// it are disabled in `.rubocop.yml`. Both JS and Ruby use single quotes.
    /// When `erb` is set, ERB templates are formatted via
    /// `@4az/prettier-plugin-html-erb`.
    fn with_ruby_plugin(mut self, erb: bool) -> Self {
        self.single_quote = true;
        self.ruby_single_quote = Some(true);
        let mut plugins = vec!["@prettier/plugin-ruby".to_string()];
        if erb {
            plugins.push("@4az/prettier-plugin-html-erb".to_string());
        }
        self.plugins = Some(plugins);
        self
    }

    pub fn from_template(template: &str) -> Self {
        match template {
            "google" => Self {
                semi: false,
                single_quote: true,
                tab_width: 2,
                trailing_comma: "es5".to_string(),
                print_width: 80,
                plugins: None,
                ruby_single_quote: None,
            },
            "airbnb" => Self {
                semi: true,
                single_quote: true,
                tab_width: 2,
                trailing_comma: "es5".to_string(),
                print_width: 100,
                plugins: None,
                ruby_single_quote: None,
            },
            // Rails renders ERB views, so it also gets the HTML+ERB plugin.
            "rails" => Self::default().with_ruby_plugin(true),
            "sinatra" | "gem" | "ruby" => Self::default().with_ruby_plugin(false),
            _ => Self::default(),
        }
    }
}

impl fmt::Display for PrettierConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{{"semi": {}, "singleQuote": {}, "tabWidth": {}, "trailingComma": "{}", "printWidth": {}"#,
            self.semi, self.single_quote, self.tab_width, self.trailing_comma, self.print_width
        )?;

        if let Some(plugins) = &self.plugins {
            write!(f, r#", "plugins": ["#)?;
            for (index, plugin) in plugins.iter().enumerate() {
                if index > 0 {
                    write!(f, ", ")?;
                }
                write!(f, r#""{}""#, plugin)?;
            }
            write!(f, "]")?;
        }

        if let Some(ruby_single_quote) = self.ruby_single_quote {
            write!(f, r#", "rubySingleQuote": {}"#, ruby_single_quote)?;
        }

        write!(f, "}}")
    }
}

/// Pinned npm version for `@prettier/plugin-ruby` (the Ruby formatter plugin).
const PRETTIER_PLUGIN_RUBY_VERSION: &str = "^4.0.4";
/// Pinned npm version for `@4az/prettier-plugin-html-erb` (ERB view formatter).
const PRETTIER_PLUGIN_HTML_ERB_VERSION: &str = "^0.0.7";

/// Base Prettier dev-dependencies shared by every Ruby package.json.
///
/// Uses the published, scoped `@prettier/plugin-ruby` package — NOT the
/// unscoped `prettier-plugin-ruby` GitHub tarball, which does not match the
/// `.prettierrc` plugin id and leaves Prettier unable to format Ruby.
fn ruby_prettier_deps() -> HashMap<String, String> {
    let mut deps = HashMap::new();
    deps.insert("prettier".to_string(), "^3.0.0".to_string());
    deps.insert(
        "@prettier/plugin-ruby".to_string(),
        PRETTIER_PLUGIN_RUBY_VERSION.to_string(),
    );
    deps
}

impl Default for PackageJson {
    fn default() -> Self {
        Self {
            name: "project".to_string(),
            version: "0.1.0".to_string(),
            description: "A Ruby project".to_string(),
            dev_dependencies: ruby_prettier_deps(),
        }
    }
}

impl PackageJson {
    pub fn from_template(template: &str) -> Self {
        match template {
            "rails" => Self {
                name: "rails-project".to_string(),
                version: "0.1.0".to_string(),
                description: "A Rails web application".to_string(),
                dev_dependencies: {
                    let mut deps = ruby_prettier_deps();
                    // Rails renders ERB views; format them with the HTML+ERB plugin.
                    deps.insert(
                        "@4az/prettier-plugin-html-erb".to_string(),
                        PRETTIER_PLUGIN_HTML_ERB_VERSION.to_string(),
                    );
                    deps.insert("eslint".to_string(), "^8.0.0".to_string());
                    deps
                },
            },
            "sinatra" => Self {
                name: "sinatra-project".to_string(),
                version: "0.1.0".to_string(),
                description: "A Sinatra web application".to_string(),
                dev_dependencies: ruby_prettier_deps(),
            },
            "gem" => Self {
                name: "ruby-gem".to_string(),
                version: "0.1.0".to_string(),
                description: "A Ruby gem".to_string(),
                dev_dependencies: {
                    let mut deps = ruby_prettier_deps();
                    deps.insert("rspec".to_string(), "^3.12.0".to_string());
                    deps
                },
            },
            "express" => Self {
                name: "express-project".to_string(),
                version: "0.1.0".to_string(),
                description: "An Express.js web application".to_string(),
                dev_dependencies: {
                    let mut deps = HashMap::new();
                    deps.insert("prettier".to_string(), "^3.0.0".to_string());
                    deps.insert("eslint".to_string(), "^8.0.0".to_string());
                    deps.insert(
                        "@typescript-eslint/eslint-plugin".to_string(),
                        "^6.0.0".to_string(),
                    );
                    deps.insert(
                        "@typescript-eslint/parser".to_string(),
                        "^6.0.0".to_string(),
                    );
                    deps
                },
            },
            "react" => Self {
                name: "react-project".to_string(),
                version: "0.1.0".to_string(),
                description: "A React application".to_string(),
                dev_dependencies: {
                    let mut deps = HashMap::new();
                    deps.insert("prettier".to_string(), "^3.0.0".to_string());
                    deps.insert("eslint".to_string(), "^8.0.0".to_string());
                    deps.insert("eslint-plugin-react".to_string(), "^7.33.0".to_string());
                    deps.insert(
                        "eslint-plugin-react-hooks".to_string(),
                        "^4.6.0".to_string(),
                    );
                    deps.insert(
                        "@typescript-eslint/eslint-plugin".to_string(),
                        "^6.0.0".to_string(),
                    );
                    deps.insert(
                        "@typescript-eslint/parser".to_string(),
                        "^6.0.0".to_string(),
                    );
                    deps
                },
            },
            "default" => Self {
                name: "node-app".to_string(),
                version: "0.1.0".to_string(),
                description: "A Node.js project".to_string(),
                dev_dependencies: {
                    let mut deps = HashMap::new();
                    deps.insert("prettier".to_string(), "^3.0.0".to_string());
                    deps.insert("eslint".to_string(), "^8.0.0".to_string());
                    deps
                },
            },
            _ => Self::default(),
        }
    }
}

impl fmt::Display for PackageJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{{"name":"{}","version":"{}","description":"{}","devDependencies":{{"#,
            self.name, self.version, self.description
        )?;

        let mut first = true;
        for (key, value) in &self.dev_dependencies {
            if !first {
                write!(f, ",")?;
            }
            write!(f, r#""{}":"{}""#, key, value)?;
            first = false;
        }

        write!(f, "}}}}")?;
        Ok(())
    }
}
