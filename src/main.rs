use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <button
            on:click = move |_| set_count.update(|n| *n += 1)
            class:red = move || count() % 2 == 1
        >
            "Click me"
        </button>
        <br/>
        <progress
            max="50"
            value=count
        />
        <br/>
        <progress
            max="50"
            value=double_count
        />
        <p>"Count: " {count}</p>
        <p>"Double count: " {double_count}</p>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
