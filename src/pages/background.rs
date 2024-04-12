use yew::{function_component, html, use_state, Callback, Html, Properties, TargetCast};

// Define properties for your component, if needed
#[derive(Properties, PartialEq)]
struct TextInputProps {
    on_change: Callback<String>,
}

// A simple text input component
#[function_component(TextInput)]
fn text_input(props: &TextInputProps) -> Html {
    let oninput = {
        let on_change = props.on_change.clone();
        move |e: yew::events::InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            on_change.emit(input.value());
        }
    };

    html! {
        <input
            class="input"
            type="text"
            placeholder="What username do you want?"
            oninput={oninput}
        />
    }
}

// Main component that includes the TextInput and sets the background
#[function_component(App)]
pub fn app() -> Html {
    let username = use_state(|| "".to_string());
    
    let on_username_change = {
        let username = username.clone();
        move |new_username: String| username.set(new_username)
    };

    html! {
        <div class="app" style="background-image: url('/home/vboxuser/Downloads/bg.png'); background-size: cover; background-repeat: no-repeat; background-attachment: fixed;">
            <h1>{ "Sign Up" }</h1>
            <TextInput on_change={on_username_change} />
            <p>{ format!("Username: {}", *username) }</p>
        </div>
    }
}


