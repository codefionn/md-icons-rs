use std::path::Path;

use proc_macro::TokenStream;
use std::fs;

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
                            let name = &name[..name.len() - 4].to_uppercase();
                            if let Ok(contents) = fs::read_to_string(entry.path()) {
                                let contents = contents.replace("\\", "\\\\");
                                let contents = contents.replace("\"", "\\\"");
                                code += format!(
                                    "pub const ICON_{}: &'static str = \"{}\";\n",
                                    name, contents
                                )
                                .as_str();
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
