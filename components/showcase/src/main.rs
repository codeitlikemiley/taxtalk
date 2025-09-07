use leptos::prelude::*;
use taxtalk_showcase::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}