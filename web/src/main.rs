use leptos::*;

use random_color_generator::models::Color;

use web_sys::MouseEvent;

#[component]
fn RandomColorGenerator(cx: Scope) -> impl IntoView {
    let (color, set_color) = create_signal(cx, rand::random::<Color>());

    let onclick_generate_button = move |_: MouseEvent| {
        set_color(rand::random::<Color>());
        document()
            .body()
            .unwrap()
            .style()
            .set_property("background", color().to_string().as_str())
            .unwrap();
    };

    document()
        .body()
        .unwrap()
        .style()
        .set_property("background", color().to_string().as_str())
        .unwrap();

    view! { cx,
        <div class="container">
            <h2>{
                move|| view!{cx,
                    <span class="color">{color().to_string()}</span>
                }
            }</h2>
            <button class="btn btn-hero" on:click=onclick_generate_button>"Generate"</button>
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
