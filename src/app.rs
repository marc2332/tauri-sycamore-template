use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {
    let name = create_signal(cx, String::new());
    let greet_msg = create_signal(cx, String::new());

    let greet = move |_| {
        spawn_local_scoped(cx, async move {
            let new_msg =
                invoke("greet", to_value(&GreetArgs { name: &name.get() }).unwrap()).await;

            log(&new_msg.as_string().unwrap());

            greet_msg.set(new_msg.as_string().unwrap());
        })
    };

    view! { cx,
        main(class="container") {
            div(class="row") {
                a(href="https://tauri.app",target="_blank") {
                    img(src="public/tauri.svg",class="logo tauri",alt="Tauri logo")
                }
                a(href="https://sycamore-rs.netlify.app",target="_blank") {
                    span(class="logo sycamore") {
                        "Sycamore"
                    }
                }
            }
            p {
                "Click on the Tauri and Sycamore logos to learn more."
            }
            p {
                "Recommended IDE setup: "
                a(href="https://code.visualstudio.com/",target="_blank") {
                    "VS Code"
                }
                " + "
                a(href="https://github.com/tauri-apps/tauri-vscode",target="_blank") {
                    "Tauri"
                }
                " + "
                a(href="https://github.com/rust-lang/rust-analyzer",target="_blank") {
                    "rust-analyzer"
                }
            }
            div(class="row") {
                input(id="greet-input",bind:value=name,placeholder="Enter a name...")
                button(type="button",on:click=greet) {
                    "Greet"
                }
            }
            p {
                b {
                    (greet_msg.get())
                }
            }
        }
    }
}
