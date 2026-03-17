mod new_alphabet;

use leptos::prelude::*;
use new_alphabet::recipes::blog_index::BlogIndexSurface;

fn main() {
    let _ = view! { <BlogIndexSurface /> };
}
