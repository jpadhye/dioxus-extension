use ::web_sys::window;
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    launch(app);
}

pub fn app() -> Element {
    let mut count = use_signal(|| 0);
    let mut domain = use_signal(String::new);
    rsx! {
        // The Stylesheet component inserts a style link into the head of the document
        // But itdoesn't work in chrome extnesion as it violates CSP due to unsafe-eval processing in sdk crate
        // Stylesheet { href: asset!("/assets/tailwind.css") }


        div {
            class: "w-auto bg-gray-300 flex flex-col gap-2 items-center justify-center",
            div {
                h1 { "High-Five counter: {count}"}
                h1 { "Current domain: {domain}"}
            }


            div {
                button {
                    class: "bg-blue-500 hover:bg-blue-700 w-32 text-white font-bold py-2 px-4 rounded",
                    onclick: move |_| count += 1, "Up high!"
                }
            }

            div {             
                button {
                    class: "bg-blue-500 hover:bg-blue-700  w-32 text-white font-bold py-2 px-4 rounded",
                    onclick: move |_| count -= 1, "Down low!"
                } 
            }

            div {
                button {
                    class: "bg-blue-500 hover:bg-blue-700  w-32 text-white font-bold py-2 px-4 rounded",
                    // When you click the button, we use web-sys to read the domain and a signal
                    onclick: move |_| {
                        domain
                            .set(
                                window()
                                    .unwrap()
                                    .document()
                                    .unwrap()
                                    .dyn_into::<::web_sys::HtmlDocument>()
                                    .unwrap()
                                    .domain(),
                            );
                    },
                    "Read Domain"
                }
            }
        }
    }
}
