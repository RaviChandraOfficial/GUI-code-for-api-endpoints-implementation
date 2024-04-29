// use stylist::{yew::styled_component, Style};
// use yew::prelude::*;
// use wasm_bindgen_futures::spawn_local;
// use reqwest::{Client, Error};
// use serde::{Serialize, Deserialize};
// use web_sys::HtmlInputElement;

// #[derive(Default,Serialize, Deserialize, Clone, PartialEq)]
// struct OtpConfirmation {
//     pub username: String,
//     pub otp: String,
// }

// #[styled_component(OtpVerify)]
// pub fn otp_verify() -> Html {
//     let stylesheet = Style::new(css!(
//         r#"
//         .form-group {
//             margin-bottom: 1rem;
//         }

//         .input {
//             width: 100%;
//             padding: 0.5rem;
//             border: 1px solid #ccc;
//             border-radius: 0.25rem;
//         }

//         .button {
//             background-color: #007bff;
//             color: white;
//             padding: 0.5rem 1rem;
//             border: none;
//             border-radius: 0.25rem;
//             cursor: pointer;
//         }

//         .button:hover {
//             background-color: #0056b3;
//         }

//         .error-message {
//             color: red;
//             font-size: 1em;
//         }
//         "#
//     )).unwrap();

//     let username = use_state(String::default);
//     let otp = use_state(String::default);
//     let error_message = use_state(|| None::<String>);

//     let on_submit = {
//         let username = username.clone();
//         let otp = otp.clone();
//         let error_message = error_message.clone();
//         Callback::from(move |user:OtpConfirmation| {
//             // user.prevent_default();
//             let confirmation_data = OtpConfirmation {
//                 username: (*username).clone(),
//                 otp: (*otp).clone(),
//             };
//             spawn_local(async move {
//                 match confirm_otp(&confirmation_data).await {
//                     Ok(_) => error_message.set(Some("OTP verified successfully!".to_string())),
//                     Err(err) => error_message.set(Some(format!("OTP verification failed: {}", err))),
                
//                 }
//             });
//         })
//     };


// pub async fn confirm_otp(otp_data: &OtpConfirmation) -> Result<(), Error> {
//     let client = Client::new();
//     let response = client.post("http://localhost:3000/signup_confirm")
//                          .json(otp_data)
//                          .send().await?
//                          .error_for_status();
//     response.map(|_| ())
// }


//     html! {
//         <div>
//             // <form onsubmit={on_submit}>
//                 <div>
//                     <label>{"Username:"}</label>
//                     <input type="text"
//                            value={(*username).clone()}
//                            oninput={Callback::from(move |e: InputEvent| username.clone().set(e.target_unchecked_into::<HtmlInputElement>().value()))} />
//                 </div>
//                 <div>
//                     <label>{"OTP:"}</label>
//                     <input type="text"
//                            value={(*otp).clone()}
//                            oninput={Callback::from(move |e: InputEvent| otp.clone().set(e.target_unchecked_into::<HtmlInputElement>().value()))} />
//                 </div>
//                 <button type="submit">{"Verify OTP"}</button>
//                 { if let Some(message) = (*error_message).clone() {
//                     html! { <p class="error-message">{ message }</p> }
//                 } else {
//                     html! {}
//                 }}
//             // </form>
//         </div>
//     }
// }



use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use wasm_bindgen_futures::spawn_local;
use reqwest::{Client, Error};
use serde::{Serialize, Deserialize};
use web_sys::HtmlInputElement;
use yew_router::{history, hooks::use_navigator};
use yewdux::functional::use_store;
use crate::{components::{atoms::bb_link::_Props::route, molecules::otpverify::{AccountForm, Action, User}}, router::Route};
use crate::store::Store;

#[derive(Serialize, Deserialize, Clone, PartialEq, Default)]
struct OtpConfirmation {
    pub username: String,
    pub otp: String,
}

#[styled_component(ConfirmSignUp)]
pub fn confirm_sign_up() -> Html {
    let stylesheet = Style::new(css!(
        r#"
        .form-group {
            margin-bottom: 1rem;
        }
        .input {
            padding: 0.5rem;
            border: 1px solid #ccc;
            border-radius: 0.25rem;
            width: 75%;
        }
        .button {
            background-color: #007bff;
            color: white;
            padding: 0.5rem 1rem;
            border: none;
            border-radius: 0.25rem;
            cursor: pointer;
        }
        .button:hover {
            background-color: #0056b3;
        }
        .error-message {
            color: red;
            font-size: 1em;
        }
        "#
    )).unwrap();
// Create success and error messages using use_state
let success_message = use_state(|| None::<String>);
let error_message = use_state(|| None::<String>);

// Define the onsubmit callback with cloned references
let onsubmit = {
    // Clone references to avoid moving them into the closure
    // let history = use_navigator().unwrap();
    let success_message_clone = success_message.clone();
    let error_message_clone = error_message.clone();
    // let history_clone = history.clone(); 

    Callback::from(move |user: User| {
        // let history = history.clone();
        // Re-clone within the closure to avoid moving the original references
        let success_message_inner = success_message_clone.clone();
        let error_message_inner = error_message_clone.clone();

        let confirmation_data = OtpConfirmation {
            username: user.username,
            otp: user.otp,
        };

        // Asynchronous call with cloned references to avoid moving the original
        spawn_local(async move {
            match confirm_otp(&confirmation_data).await {
                Ok(_) => {
                    success_message_inner.set(Some("OTP verified successfully!".to_string()));
                    // history.push(&Route::Login);
                }
                Err(_) => {
                    error_message_inner.set(Some("OTP verification failed.".to_string()));
                }
            }
        });
    })
};

html! {
    <div class={stylesheet}>
      <h1>{"Sign Up Confirmation"}</h1>
      <section>
        <div>
          <AccountForm {onsubmit} action={Action::OtpVerify} />
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

pub async fn confirm_otp(otp_data: &OtpConfirmation) -> Result<(), Error> {
    let client = Client::new();
    let response = client.post("http://localhost:3000/signup_confirm")
                         .json(otp_data)
                         .send().await?
                         .error_for_status();
    response.map(|_| ())
}
