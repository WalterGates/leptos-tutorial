use leptos::{*, ev::MouseEvent};

#[derive(Copy, Clone)]
struct SmallcapsContext(WriteSignal<bool>);

#[component]
fn App() -> impl IntoView {
    let (red, set_red) = create_signal(false);
    let (right, set_right) = create_signal(false);
    let (italics, set_italics) = create_signal(false);
    let (smallcaps, set_smallcaps) = create_signal(false);

    provide_context(SmallcapsContext(set_smallcaps));

    view! {
        <main>
            <p
                class:red=red
                class:right=right
                class:italics=italics
                class:smallcaps=smallcaps
            >
                "Lorem ipsum sit dolor amet."
            </p>
            <ButtonA setter=set_red/>
            <ButtonB on_click=move |_| set_right.update(|val| *val = !*val)/>
            <ButtonC on:click=move |_| set_italics.update(|val| *val = !*val)/>
            <ButtonD/>
        </main>
    }
}

/// Button A recieves a signal setter and updates the signal itself
#[component]
fn ButtonA(
    /// Signal that will be toggled when the button is clicked.
    setter: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <button
            on:click=move |_| setter.update(|val| *val = !*val)
        >
            "Toggle Red"
        </button>
    }
}

/// Button B recieves a closure
#[component]
fn ButtonB<F>(
    /// Callback that will be invoked when the button is clicked.
    on_click: F,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! {
        <button
            on:click=on_click
        >
            "Toggle Right"
        </button>
    }
}

/// Button C is a dummy: it renders a button but doesn't handle
/// its click. Instead, the parent component adds an event listener.
#[component]
fn ButtonC() -> impl IntoView {
    view! {
        <button>
            "Toggle Italics"
        </button>
    }
}

/// Button D is very similar to Button A, but instead of passing the setter as a prop
/// we get it from the context
#[component]
fn ButtonD() -> impl IntoView {
    let setter = use_context::<SmallcapsContext>().expect("to have found the setter provided").0;

    view! {
        <button on:click = move |_| setter.update(|value| *value = !*value)>
            "Toggle"
        </button>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
