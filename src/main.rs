use log::Level;
use pushkind_store::app::App;

fn main() {
    _ = console_log::init_with_level(Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App)
}
