use leptos::*;

use random_color_generator::models::Color;

use web_sys::MouseEvent;

#[component]
fn RandomColorGenerator(cx: Scope) -> impl IntoView {
    let (color, set_color) = create_signal(cx, rand::random::<Color>());

    document()
        .body()
        .unwrap()
        .style()
        .set_property("background", color.get().to_string().as_str())
        .unwrap();

    view! { cx,
        <div class="container h-screen relative">
            <h2>{
                move|| view!{cx,
                    <span class="color">{color.get().to_string()}</span>
                }
            }</h2>
            <div class="flex justify-center space-x-4 absolute inset-x-1/4 bottom-1/4">
                <button class="btn btn-hero" on:click=move |_: MouseEvent| {
                    set_color.set(rand::random::<Color>());
                    document()
                        .body()
                        .unwrap()
                        .style()
                        .set_property("background", color.get().to_string().as_str())
                        .unwrap();
                }>"Generate"</button>
                <button class="btn btn-hero" on:click=move |_: MouseEvent| {
                    let navigator = window().navigator();

                    let _ = navigator
                        .clipboard()
                        .unwrap()
                        .write_text(color.get().to_string().as_str());
                }>"Copy"</button>
            </div>

        </div>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <RandomColorGenerator/>
        }
    })
}
