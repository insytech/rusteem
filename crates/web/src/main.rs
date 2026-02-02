mod api;
mod app;
mod auth;
mod components;
mod pages;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
