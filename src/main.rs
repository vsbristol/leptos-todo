use leptos_router::*;
use leptos::{*};
use std::collections::HashMap;

#[component]
fn Todo(cx: Scope, ) -> impl IntoView { //tasks: HashMap<String, HashMap<String, String>>

    view! { cx, 
        <p>"Your loged in as  :)"</p>
    }
}

#[component]
fn Homepage(cx: Scope) -> impl IntoView {
    let username: String = String::from("");

    view! { cx,
        <style>"h1 {text-align: center;}"</style>
        <style>"input {text-align: center;}"</style>
        <style>"p {text-align: center;}"</style>

        <h1>"This is a Todo App!"</h1>

        <br></br>

        <form method="GET" action="/todo/">
            <input type="username" name="username" value=username/>
            <input type="submit"/>
        </form>

    }
}

fn main() {
    let _users: Vec<String> = Vec::new();
    let _tasks: HashMap<String, HashMap<String, String>> = HashMap::new();

    mount_to_body(|cx| view! { cx, 
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <Homepage/> }/>
                    <Route path="/todo" view=|cx| view! { cx, <Todo/> }/> //user: Stringuser: String
                </Routes>
            </main>
        </Router>
    })    
}
