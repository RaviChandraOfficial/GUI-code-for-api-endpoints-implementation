use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;
use yew::html;

use yew::Callback;
use yew::use_state;
// use yew::services::ConsoleService;
use yew::Context;
use yew::function_component;
use wasm_bindgen::JsValue;
use crate::router::Route;
use crate::store::Store;
use yew_router::hooks::use_navigator;
use yewdux::functional::use_store;



#[styled_component(Logout)]
pub fn logout() -> Html {
    let stylesheet = Style::new(css!(
        r#"
          section {
            display: flex;
            justify-content: center;
          }

          section > div {
            width: 75vw;
          }

          .message {
            color: green;
            font-size: 1em;
            text-align: center;
            margin-top: 20px;
          }

          .error {
            color: red;
            font-size: 1em;
            text-align: center;
            margin-top: 20px;
          }
        "#
    ))
    .unwrap();

    let history = use_navigator().unwrap();
    let (_store, store_dispatch) = use_store::<Store>();
    let success_message = use_state(|| None::<String>);
    let error_message = use_state(|| None::<String>);

    let on_logout = {
        let store_dispatch = store_dispatch.clone();
        let history = history.clone();
        let success_message = success_message.clone();
        let error_message = error_message.clone();
        
        Callback::from(move |_| {
            // Clear any stored authentication data
            store_dispatch.set(Store::default()); // Update this based on your store logic
            
            // Update the state to reflect logout
            success_message.set(Some("Logout successful!".to_string()));
            
            // Redirect to login or home page
            history.push(&Route::Login);
        })
    };

    html! {
      <div class={stylesheet}>
        <h1>{"Logout"}</h1>
        <section>
          <div>
            <button onclick={on_logout}>{"Logout"}</button>
            if let Some(message) = (*success_message).as_ref() {
                <div class={"message"}>{ message }</div>
            }
            if let Some(error) = (*error_message).as_ref() {
                <div class={"error"}>{ error }</div>
            }
          </div>
        </section>
      </div>
    }
}
