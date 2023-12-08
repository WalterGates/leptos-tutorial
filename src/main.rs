use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <button
            on:click=move |_| set_count.update(|n| *n += 1)
            class:red=move || count() % 2 == 1
        >
            "Click me"
        </button>
        <br/>
        <ProgressBar progress=count/>
        <br/>
        <ProgressBar max=100 progress=double_count/>
        <p>"Count: " {count}</p>
        <p>"Double count: " {double_count}</p>
    }
}

/// Shows progress toward a goal.
#[component]
fn ProgressBar<F>(
    /// The maximum value of the progress bar.
    #[prop(default = 50)]
    max: u16,
    /// How much progress should be displayed.
    progress: F,
) -> impl IntoView
where
    F: Fn() -> i32 + 'static,
{
    view! {
        <progress
            max=max
            value=progress
        />
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
