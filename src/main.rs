use leptos::{*, html::Input, ev::SubmitEvent};
use std::collections::HashMap;

// #[component]
// fn Tasks(cx: Scope, user: &str, tasks: HashMap<String, String>) -> impl IntroView {
// }

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let (name, set_name) = create_signal(cx, "Enter your username".to_string());
    let input_element: NodeRef<Input> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element()
            .expect("<input> to exist")
            .value();
        set_name(value);
    };

    view! { cx,
        <style>"h1 {text-align: center;}"</style>
        <style>"input {text-align: center;}"</style>
        <style>"p {text-align: center;}"</style>

        <h1>"This is a Todo App!"</h1>

        <br></br>

        <form on:submit=on_submit>
            <input type="text"
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Submit"/> 
            </form>
            <p>"Loged in as " {name}</p>

            // Triger tasks here
            // mount_to_body(|cx| view! { cx, <App/> })    
    }
}

fn main() {
    let users: Vec<String> = Vec::new();
    let tasks: HashMap<String, String> = HashMap::new();

    mount_to_body(|cx| view! { cx, <App/> })    
}
