mod new_alphabet;

use leptos::prelude::*;
use new_alphabet::recipes::review_queue::ReviewQueueSurface;

fn main() {
    let _ = view! { <ReviewQueueSurface /> };
}
