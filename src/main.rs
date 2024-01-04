use yew::prelude::*;

struct Model {
    value: i64,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model { value: 0 });

    let on_increment = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1,
            })
        })
    };

    let on_decrement = {
        let state = state.clone();

        Callback::from(move |_: MouseEvent| {
            state.set(Model {
                value: state.value - 1,
            })
        })
    };

    let on_clear = {
        let state = state.clone();

        Callback::from(move |_: MouseEvent| {
            state.set(Model {
                value: state.value - state.value,
            })
        })
    };

    html! {
        <div class={"container"}>
            <div class={"btn-rows"}>
                <button class={"btn"} onclick={on_decrement}>{ "- 1" }</button>
                <button class={"btn"} onclick={on_clear}>{ "clear" }</button>
                <button class={"btn"} onclick={on_increment}>{ "+ 1" }</button>
            </div>
            <p>{ state.value }</p>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
