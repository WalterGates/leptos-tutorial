use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! { <NumericInput/> }
}

#[component]
fn NumericInput() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(42));

    let on_input = move |event| set_value(event_target_value(&event).parse::<i32>());

    let fallback = |errors: RwSignal<Errors>| {
        view! {
            <div class="error">
                <p>"Not a number! Errors: "</p>
                <ul>{ move || errors()
                    .into_iter()
                    .map(|(_, err)| view! { <li>{err.to_string()}</li> })
                    .collect_view()
                }</ul>
            </div>
        }
    };

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or not!) "
            <input type="number" on:input=on_input/>
            <ErrorBoundary fallback=fallback>
                <p>"You entered " <strong>{value}</strong></p>
            </ErrorBoundary>
        </label>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
