use serde::{Deserialize, Serialize};
use stylist::{css, yew::styled_component, Style};
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;
use web_sys::console;
use yew_router::hooks::use_navigator;
use yewdux::functional::use_store;
use crate::store::{self, Store};
use crate::api;
// use crate::store;
// use store;
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    pub user_name: String,
    pub location :String,
    pub data :String,
    pub name:String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TaskResponse {
    tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetResponse{
        pub id: i32,
    pub user_name: String,
    pub location :String,
    pub data :String,
    pub name:String,
}


#[function_component(Get_data)]
pub fn get_data() -> Html {
    let stylesheet = Style::new(css!(
        r#"
          section {
            display: flex;
            justify-content: center;
          }

          section > div {
            width: 60vw;
          }

          .message {
            color: white;
            font-size: 1em;
            text-align: center;
          }

          .error {
            color: red;
            font-size: 1em;
            text-align: center;
          }
        "#
    ))
    .unwrap();
let (store, dispatch) = use_store::<Store>();
    
    let tasks = use_state(|| Vec::new());
    let error = use_state(|| None::<String>);
    let fetch_tasks = {
        let tasks = tasks.clone();
        let error = error.clone();
        Callback::from(move |_| {
            let token = store.token.clone();
            let tasks = tasks.clone();
            let error = error.clone();
            spawn_local(async move {
                let client = Client::new();
                match client.get(&format!("{}/get/user", "http://localhost:3000"))  // Adjust BASE_URL as needed
                    .header("x-auth-token", token)
                    .send().await {
                        Ok(response) => {
                            if response.status().is_success() {
                                match response.json::<TaskResponse>().await {
                                    Ok(task_response) => {
                                        tasks.set(task_response.tasks);
                                    },
                                    Err(_) => {
                                        error.set(Some("Failed to parse tasks.".to_string()));
                                    }
                                }
                            } else {
                                error.set(Some(format!("HTTP Error: {}", response.status())));
                            }
                        }
                        Err(_) => {
                            error.set(Some("Network request failed.".to_string()));
                        }
                    }
            });
        })
    };

    use_effect_with_deps(
        move |_| {
            fetch_tasks.emit(());
            || ()
        },
        (),
    );

    html! {
        <>
            <h1>{"Tasks"}</h1>
            {
                if let Some(err) = (*error).as_ref() {
                    html! { <p>{ err }</p> }
                } else {
                    html! {
                        <ul>
                            { for (*tasks).iter().map(|task| {
                                html! { <li>{ format!("{} - {} - {} - {} - {}", task.name, task.user_name, task.location, task.data, task.id) }</li> }
                            })}
                        </ul>
                    }
                }
            }
        </>
    }
}








// #[styled_component(GetUser)]
// pub fn get_user() -> Html {
//     let stylesheet = Style::new(css!(
//         r#"
//           section {
//             display: flex;
//             justify-content: center;
//           }

//           section > div {
//             width: 75vw;
//           }

//           .message {
//             color: green;
//             font-size: 1em;
//             text-align: center;
//             margin-top: 20px;
//           }

//           .error {
//             color: red;
//             font-size: 1em;
//             text-align: center;
//             margin-top: 20px;
//           }
//         "#
//     ))
//     .unwrap();

//     let history = use_navigator().unwrap();
//     let (_store, store_dispatch) = use_store::<Store>();
//     let success_message = use_state(|| None::<String>);
//     let error_message = use_state(|| None::<String>);

//     let onsubmit = {
//         let store_dispatch = store_dispatch.clone();
//         let history = history.clone();
//         let success_message = success_message.clone();
//         let error_message = error_message.clone();

//         Callback::from(move |user_id: i32| {
//             let store_dispatch = store_dispatch.clone();
//             let history = history.clone();
//             let success_message = success_message.clone();
//             let error_message = error_message.clone();
            
//             spawn_local(async move {
//                 match api::get_tasks(token).await {
//                     Ok(user) => {
//                         // Process user data, maybe store in store_dispatch if needed
//                         // For example: store_dispatch(UserAction::SetUser(user));
//                         success_message.set(Some(format!("User data fetched: {:?}", user)));
//                     },
//                     Err(e) => {
//                         println!("{:?}", e);
//                         let error_message_str = e.as_string().unwrap_or_else(|| "Unknown error".to_string());
//                         error_message.set(Some(format!("Failed to fetch user data: {}", error_message_str)));
//                     }
//                 }
//             });
//         })
//     };

//     html! {
//       <div class={stylesheet}>
//         <h1>{"Get User"}</h1>
//         <section>
//           <div>
//             // Form or input for providing user ID
//             <UserForm {onsubmit} />
//             if let Some(message) = (*success_message).as_ref() {
//                 <div class={"message"}>{ message }</div>
//             }
//             if let Some(error) = (*error_message).as_ref() {
//                 <div class={"error"}>{ error }</div>
//             }
//           </div>
//         </section>
//       </div>
//     }
// }




