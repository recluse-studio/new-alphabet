use std::path::PathBuf;

use new_alphabet_demo_saas::write_site;

fn main() -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    write_site(&root.join("site"))
}
