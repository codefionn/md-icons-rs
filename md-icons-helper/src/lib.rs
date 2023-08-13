use std::path::Path;

use proc_macro::TokenStream;
use std::fs;

#[cfg(feature = "maud")]
extern crate maud;

#[cfg(feature = "leptos")]
extern crate leptos;

// Generates the procedural macros for the different styles
macro_rules! files_to_functions {
    ($id:ident, $dir:literal) => {
        #[proc_macro]
        pub fn $id(_item: TokenStream) -> TokenStream {
            let dir = Path::new(".");

            let path = dir.join($dir);
            if !path.is_dir() {
                panic!(
                    "Expected directory at {}, Current dir: {}",
                    path.to_string_lossy(),
                    dir.canonicalize().unwrap().to_string_lossy(),
                );
            }

            let mut code = String::new();
            if let Ok(dir) = path.read_dir() {
                for entry in dir {
                    if let Ok(entry) = entry {
                        let osname = entry.file_name();
                        let name = osname.to_string_lossy();
                        if name.ends_with(".svg") {
                            let name = &name[..name.len() - 4];
                            let re = regex::Regex::new("_([a-z])").unwrap();
                            let mut camel_name: Vec<char> = re.replace_all(name, |captures: &regex::Captures| {
                                captures[1].to_uppercase()
                            }).to_string().chars().collect();
                            camel_name[0] = camel_name[0].to_uppercase().nth(0).unwrap();
                            let camel_name: String = camel_name.into_iter().collect();

                            let upname = name.to_uppercase();
                            if let Ok(contents) = fs::read_to_string(entry.path()) {
                                let contents = contents.replace("\\", "\\\\");
                                let contents = contents.replace("\"", "\\\"");
                                code += format!(
                                    "pub const ICON_{}: &'static str = \"{}\";\n",
                                    upname, contents
                                )
                                .as_str();

                                if cfg!(feature = "maud") {
                                    code +=
                                        format!("#[inline] pub fn maud_icon_{}() -> maud::Markup {{ maud::html! {{ (maud::PreEscaped(ICON_{})) }} }} \n", name, upname)
                                            .as_str();
                                }

                                if cfg!(feature = "leptos") {
                                    code +=
                                        format!("#[leptos::component] #[inline] pub fn LeptosIcon{}(cx: leptos::Scope) -> impl leptos::IntoView {{ use leptos::IntoAttribute; leptos::view! {{ cx, <span inner_html=ICON_{}></span> }} }}", camel_name, upname)
                                            .as_str();
                                }
                            } else {
                                eprintln!("{}", entry.path().to_string_lossy());
                            }
                        }
                    }
                }
            } else {
                eprintln!("{}", path.to_string_lossy());
            }

            code.parse().unwrap()
        }
    };
}

files_to_functions!(generate_filled, "rsc/material-design-icons/svg/filled");
files_to_functions!(generate_outlined, "rsc/material-design-icons/svg/outlined");
files_to_functions!(generate_sharp, "rsc/material-design-icons/svg/sharp");
files_to_functions!(generate_two_tone, "rsc/material-design-icons/svg/two-tone");
