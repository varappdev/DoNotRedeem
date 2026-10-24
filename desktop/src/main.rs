use std::rc::Rc;

use dioxus::desktop::wry::dpi::Size;
use dioxus::html::object::width;
use dioxus::prelude::*;
use dioxus::desktop::{Config, LogicalSize, WindowBuilder, window};

use ui::Navbar;
use views::Home;

mod ui;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // #[layout(DesktopNavbar)]
    #[route("/")]
    Home {},
}

const WIN98_CSS: Asset = asset!("/assets/98.css"); 
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    let config = Config::new()
        .with_window(
            WindowBuilder::new()
                .with_title("Do Not Redeem")
                .with_decorations(false)
                .with_resizable(false)
                .with_inner_size(LogicalSize::new(800.0, 600.0))
                .with_max_inner_size(LogicalSize::new(1000.0, 800.0))
        );

    LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(App);
}

#[component]
fn Titlebar() -> Element {
    let window_local = dioxus::desktop::use_window();
    let mut is_max = use_signal(|| window_local.is_maximized());
    
    rsx! {
        div {
            class: "title-bar",
            
            // This event handler makes the entire area draggable
            onmousedown: move |_| { window().drag(); },

            // Title section
            div { class: "title-bar-text", "Do Not Redeem" }

            // Control Buttons
            div { class: "title-bar-controls",
                
                // Minimise Button
                button { 
                    aria_label: "Minimize",
                    onmousedown: move |event| event.stop_propagation(),
                    onclick: move |_| window().set_minimized(true),
                }

                /*
                    // Maximise / Restore Button
                    button { 
                        aria_label: if *is_max.read() {"Restore"} else{ "Maximize"},
                        onmousedown: move |event| event.stop_propagation(),
                        onclick: move |_| {
                            window().toggle_maximized();
                            is_max.toggle();
                        },
                        
                    }
                */

                // Close Button
                button { 
                    aria_label: "Close",
                    onmousedown: move |event| event.stop_propagation(),
                    onclick: move |_| window().close(), 
                }
            }
        }
    }
}


#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        document::Link { rel: "stylesheet", href: WIN98_CSS },
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div {
            // Global app resources
            class: "window app-window",
            Titlebar {},
            Router::<Route> {}
        }
    }
}

/// A desktop-specific Router around the shared `Navbar` component
/// which allows us to use the desktop-specific `Route` enum.
#[component]
fn DesktopNavbar() -> Element {
    rsx! {
        Navbar {
            Link {
                to: Route::Home {},
                "Home"
            }
        }

        Outlet::<Route> {}
    }
}
