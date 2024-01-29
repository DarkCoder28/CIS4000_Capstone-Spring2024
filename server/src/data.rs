use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;

use std::collections::hash_map::RandomState;
use tera::Tera;

pub static TEMPLATE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/templates");
pub static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/static");

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("*.ignoreallinputsthisisnotadrillpleaseshutupspellcheckthisisdefinatelyaword") {//Tera::new("src/templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        let _ = tera.add_raw_templates(vec![
            (
                "base.html",
                TEMPLATE_DIR
                    .get_file("base.html")
                    .expect("Error getting template file")
                    .contents_utf8()
                    .expect("Error converting template file to string"),
            ),
            (
                "navmenu.html",
                TEMPLATE_DIR
                    .get_file("navmenu.html")
                    .expect("Error getting template file")
                    .contents_utf8()
                    .expect("Error converting template file to string"),
            ),
            (
                "index.html",
                TEMPLATE_DIR
                    .get_file("index.html")
                    .expect("Error getting template file")
                    .contents_utf8()
                    .expect("Error converting template file to string"),
            ),
            (
                "auth.html",
                TEMPLATE_DIR
                    .get_file("auth.html")
                    .expect("Error getting template file")
                    .contents_utf8()
                    .expect("Error converting template file to string"),
            ),
        ]);
        tera
    };
    pub static ref RANDOM_STATE: RandomState = RandomState::new();
}
