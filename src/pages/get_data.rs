
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yewdux::functional::use_store;
use crate::api::{get_tasks, Note};
use crate::store;
use crate::router::Route;
use yew_router::prelude::Link;

#[function_component(Getdata)]
pub fn get_data() -> Html {
    let (store, store_dispatch) = use_store::<store::Store>();

    let success_message = use_state(|| None::<Vec<Note>>);  // Change to Vec<Note>
    let error_message = use_state(|| None::<String>);

    let fetch_tasks = {

        let success_message = success_message.clone();
        let error_message = error_message.clone();
        let store = store.clone();  // Clone the store context
        Callback::from(move |_| {

            let success_message = success_message.clone();
            let error_message = error_message.clone();
            let token = store.token.clone(); // Clone the token inside the closure
            spawn_local(async move {
                match get_tasks(&token).await {
                    Ok(response) => {
                        success_message.set(Some(response.notes));  // Store the notes directly
                    },
                    Err(e) => {
                        error_message.set(Some(format!("Failed to load tasks: {:?}", e)));
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
        // <nav style="padding: 10px; background: #f0f0f0; border-bottom: 1px solid black;">
        // <Link<Route> to={Route::Getdata}>{ "GET" }</Link<Route>>
        // <Link<Route> to={Route::PostData}>{ "POST" }</Link<Route>>
        // <Link<Route> to={Route::PutData}>{ "PUT" }</Link<Route>>
        // <Link<Route> to={Route::DeleteData}>{ "DELETE" }</Link<Route>>
        // </nav>
        <table style="width: 100%; background: #f0f0f0; border-bottom: 1px solid black;">
        <tr>
            <td style="text-align: center; border-right: 1px solid black;">
                <Link<Route> to={Route::Getdata}>{ "GET" }</Link<Route>>
            </td>
            <td style="text-align: center; border-right: 1px solid black;">
                <Link<Route> to={Route::PostData}>{ "POST" }</Link<Route>>
            </td>
            <td style="text-align: center; border-right: 1px solid black;">
                <Link<Route> to={Route::PutData}>{ "PUT" }</Link<Route>>
            </td>
            <td style="text-align: center;">
                <Link<Route> to={Route::DeleteData}>{ "DELETE" }</Link<Route>>
            </td>
        </tr>
        </table>

            <div>
                <h1>{"Get Data:"}</h1>
                {
                    if let Some(notes) = (*success_message).as_ref() {
                        html! {
                            <table class={"tasks-table"} style="border-spacing: 10px; border-collapse: separate;">
                                <tr>
                                    <th style="padding: 8px;">{"ID"}</th>
                                    <th style="padding: 8px;">{"Name"}</th>
                                    <th style="padding: 8px;">{"Location"}</th>
                                    <th style="padding: 8px;">{"Data"}</th>
                                    <th Style="padding: 8px;">{"Username"}</th>
                                </tr>
                                { for notes.iter().map(|note| html! {
                                    <tr>
                                        <td style="padding: 8px; border: 1px solid gray;">{note.id.to_string()}</td>
                                        <td style="padding: 8px; border: 1px solid gray;">{&note.name}</td>
                                        <td style="padding: 8px; border: 1px solid gray;">{&note.location}</td>
                                        <td style="padding: 8px; border: 1px solid gray;">{&note.data}</td>
                                        <td style="padding: 8px; border: 1px solid gray;">{&note.user_name}</td>
                                    </tr>
                                })}
                            </table>
                        }
                    } else if let Some(err) = (*error_message).as_ref() {
                        html! { <div class={"error"}>{ err }</div> }
                    } else {
                        html! { <div>{"Data loading.........."}</div> }
                    }
                }
            </div>
        </>
    }
}




// use store;
// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Task {
//     id: u32,
//     pub user_name: String,
//     pub location :String,
//     pub data :String,
//     pub name:String,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct TaskResponse {
//     tasks: Vec<Task>,
// }

// #[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct GetResponse{
//     pub id: i32,
//     pub user_name: String,
//     pub location :String,
//     pub data :String,
//     pub name:String,
// }
// #[function_component(Getdata)]
// pub fn get_data() -> Html {
//     let stylesheet = Style::new(css!(
//         r#"
//         section {
//             display: flex;
//             justify-content: center;
//         }

//         section > div {
//             width: 60vw;
//         }

//         .message, .error {
//             font-size: 1em;
//             text-align: center;
//         }

//         .message {
//             color: white;
//         }

//         .error {
//             color: red;
//         }
//         "#
//     )).unwrap();

//     let store = use_context::<store::Store>().expect("store not found");
//     let tasks = use_state(|| Vec::<Note>::new()); // Corrected type to Vec<Note>
//     let success_message = use_state(|| None::<String>);
//     let error_message = use_state(|| None::<String>);

//     let fetch_tasks = {
//         let tasks = tasks.clone();
//         let success_message = success_message.clone();
//         let error_message = error_message.clone();
//         let token = store.token.clone(); // Assuming store.token does not change, capture once
//         Callback::from(move |_| {
//             spawn_local(async move {
//                 match get_tasks(&token).await {
//                     Ok(response) => {
//                         let formatted_notes = response.notes.iter()
//                             .map(|note| format!("ID: {}, Name: {}, Location: {}, Data: {}", note.id, note.name, note.location, note.data))
//                             .collect::<Vec<String>>()
//                             .join(", ");
//                         success_message.set(Some(format!("Notes data fetched: {}", formatted_notes)));
//                         tasks.set(response.notes.clone());  // Set tasks state with response notes
//                     }
//                     Err(e) => {
//                         error_message.set(Some(format!("Failed to load tasks: {:?}", e)));
//                     }
//                 }
//             });
//         })
//     };

//     use_effect_with_deps(
//         move |_| {
//             fetch_tasks.emit(());
//             || ()
//         },
//         (),
//     );

//     html! {
//         <>
           
//                 <div>
//                     <h1>{"Tasks"}</h1>
//                     {
//                         if let Some(msg) = (*success_message).as_ref() {
//                             html! { <div class={"message"}>{ msg }</div> }
//                         } else if let Some(err) = (*error_message).as_ref() {
//                             html! { <div class={"error"}>{ err }</div> }
//                         } else {
//                             html! { <div>{"No new messages"}</div> }
//                         }
//                     }
//                 </div>
        
//         </>
//     }
// } mainnnnnnnnn







// pub fn get_data() -> Html {
//     let (store, store_dispatch) = use_store::<store::Store>();
//     let tasks = use_state(|| Vec::<Note>::new());
//     let success_message = use_state(|| None::<String>);
//     let error_message = use_state(|| None::<String>);

//     let fetch_tasks = {
//         let tasks = tasks.clone();
//         let success_message = success_message.clone();
//         let error_message = error_message.clone();
//         let store = store.clone(); // Clone the store context for use in the closure

//         Callback::from(move |_| {
//             let tasks = tasks.clone();
//             let success_message = success_message.clone();
//             let error_message = error_message.clone();
//             let token = store.token.clone(); // Move the cloning of `token` inside the closure
//             spawn_local(async move {
//                 let success_message = success_message.clone();
//                 let error_message = error_message.clone();
//                 match get_tasks(&token).await {
//                     Ok(response) => {
//                         let formatted_notes = response.notes.iter()
//                             .map(|note| format!("ID: {}, Name: {}, Location: {}, Data: {}", note.id, note.name, note.location, note.data))
//                             .collect::<Vec<String>>()
//                             .join("|| ");
//                         success_message.set(Some(format!("Notes data fetched: {}", formatted_notes)));
//                         tasks.set(response.notes.clone()); // Set tasks state with response notes
//                     },
//                     Err(e) => {
//                         error_message.set(Some(format!("Failed to load tasks: {:?}", e)));
//                     }
//                 }
//             });
//         })
//     };

//     use_effect_with_deps(
//         move |_| {
//             fetch_tasks.emit(());
//             || ()
//         },
//         (),
//     );
//     html! {
//         <>
//             <div>
//                 <h1>{"Tasks"}</h1>
//                 {
//                     if let Some(notes) = (*success_message).as_ref() {
//                         html! {
//                             <table class={"tasks-table"}>
//                                 <tr>
//                                     <th>{"ID"}</th>
//                                     <th>{"Name"}</th>
//                                     <th>{"Location"}</th>
//                                     <th>{"Data"}</th>
//                                 </tr>
//                                 { for notes.iter().map(|note| html! {
//                                     <tr>
//                                         <td>{note.id.to_string()}</td>
//                                         <td>{&note.name}</td>
//                                         <td>{&note.location}</td>
//                                         <td>{&note.data}</td>
//                                     </tr>
//                                 })}
//                             </table>
//                         }
//                     } else if let Some(err) = (*error_message).as_ref() {
//                         html! { <div class={"error"}>{ err }</div> }
//                     } else {
//                         html! { <div>{"No new messages"}</div> }
//                     }
//                 }
//             </div>
//         </>
//     }
// }





// #[styled_component(Getdata)]
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
//     let (store, store_dispatch) = use_store::<store::Store>();
//     let success_message = use_state(|| None::<String>);
//     let error_message = use_state(|| None::<String>);
//     let onsubmit = {
//         let store_dispatch = store_dispatch.clone();
//         let history = history.clone();
//         let success_message = success_message.clone();
//         let error_message = error_message.clone();
    
//         Callback::from(move |user :GetResponse| {
//             let store_dispatch = store_dispatch.clone();
//             let history = history.clone();
//             let success_message = success_message.clone();
//             let error_message = error_message.clone();
//             let token = store.token.clone();
//             spawn_local(async move {
//                 match api::get_tasks(&token).await {
//                     Ok(response) => {
//                         let formatted_notes = response.notes.iter()
//                             .map(|note| format!("ID: {}, Name: {}, Location: {}, Data: {}", note.id, note.name, note.location, note.data))
//                             .collect::<Vec<String>>()
//                             .join(", ");
//                         success_message.set(Some(format!("Notes data fetched: {}", formatted_notes)));
//                     },
//                     Err(e) => {
//                         println!("{:?}", e);
//                         let error_message_str = e.to_string();
//                         error_message.set(Some(format!("Failed to fetch notes: {}", error_message_str)));
//                     }
//                 }
//             });
//         })
//     };
    
//     html! {
//         <div class={stylesheet}>
//             <h1>{"Get User Notes"}</h1>
//             <section>
//                 <div>
//                     if let Some(message) = (*success_message).as_ref() {
//                         <div class={"message"}>{ message }</div>
//                     }
//                     if let Some(error) = (*error_message).as_ref() {
//                         <div class={"error"}>{ error }</div>
//                     }
//                 </div>
//             </section>
//         </div>
//     }
// }
// //     html! {
// //         <div class={stylesheet}>
// //             <h1>{"Get User"}</h1>
// //             <section>
// //                 <div>
// //                     // Form or input for providing user ID
// //                     if let Some(message) = (*success_message).as_ref() {
// //                         <div class={"message"}>{ message }</div>
// //                     }
// //                     if let Some(error) = (*error_message).as_ref() {
// //                         <div class={"error"}>{ error }</div>
// //                     }
// //                 </div>
// //             </section>
// //         </div>
// //     }
// // }    



