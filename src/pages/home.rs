use std::ops::Deref;

use crate::{
    components::atoms::bb_select::BBSelect,
    components::{atoms::bb_select::SelectOption, organisms::tasks::Tasks},
    store::{self, Store, Task},
};
// use yew_router::components::RouterAnchor;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::Link;
use crate::router::Route;
#[styled_component(Home)]
pub fn component() -> Html {
    // let stylesheet = Style::new(css!(
    //     r#"
    //     section {
    //         display: flex;
    //         justify-content: space-around;
    //         padding: 20px;
            
    //     }
        
    //     a {
    //         padding: 10px 20px;
    //         background-color: yellow;
    //         color: white;
    //         text-decoration: none;
    //         border-radius: 5px;
    //     }
        
    //     a:hover {
    //         background-color: #0056b3;
    //     }
        
    //     "#
    // ))
    let stylesheet = Style::new(css!(
        r#"
        .section-container {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            width: 90%;
            height: 70vh;
        }

        .link-section {
            display: flex;
            justify-content: center;
            align-items: center;
            padding: 20px;
            margin: 10px;
            width: 50%;
            border-radius: 30px;
        }
        
        a {
            font-size: 24px; 
            padding: 15px 30px; 
            background-color: yellow;
            color: black; 
            text-decoration: none;
            border-radius: 5px;
            width: 100%;
            text-align: center;
        }
        
        a:hover {
            background-color: #0056b3;
            color: white;
        }
        "#
    ))
    .unwrap();

    let (store, dispatch) = use_store::<Store>();

    let tasks = store.tasks.clone();
    let filter_options = store.filter_options.clone();
    let sort_options = store.sort_options.clone();
    let filter_onchange = {
        let dispatch = dispatch.clone();
        Callback::from(move |filter_value| {
            store::select_filter(dispatch.clone(), filter_value);
        })
    };
    let sort_onchange = {
        let dispatch = dispatch.clone();
        Callback::from(move |sort_value| {
            store::select_sort(dispatch.clone(), sort_value);
        })
    };
    let token = store.token.clone();

    html! {
        <>
        if is_logged_in(&token){
        <div class={stylesheet}>
            <div class="section-container">
                <section class="link-section" style="background-color: #f0f8ff;"> // Light blue background
                    <Link<Route> to={Route::Getdata}>{ "GET" }</Link<Route>>
                </section>
                <section class="link-section" style="background-color: #f5fffa;"> // Mint cream background
                    <Link<Route> to={Route::PostData}>{ "POST" }</Link<Route>>
                </section>
                <section class="link-section" style="background-color: #fffacd;"> // Lemon chiffon background
                    <Link<Route> to={Route::PutData}>{ "PUT" }</Link<Route>>
                </section>
                <section class="link-section" style="background-color: #ffe4e1;"> // Misty rose background
                    <Link<Route> to={Route::DeleteData}>{ "DELETE" }</Link<Route>>
                </section>
            </div>
        </div>
        }
        </>
    }
}





fn is_logged_in(token: &str) -> bool {
    !token.deref().is_empty()
}









// use crate::{
//     api::{self, api_errors::ApiError}, store::{self, PostCredentials, Store, Task}
// };
// use stylist::{yew::styled_component, Style};
// use yew::prelude::*;
// use yewdux::prelude::*;
// use wasm_bindgen_futures::{spawn_local, JsFuture};
// #[styled_component(Home)]
// pub fn component() -> Html {
//     let stylesheet = Style::new(css!(
//         r#"
//           display: flex;
//           flex-direction: column;
//           align-items: center;
//         "#
//     ))
//     .unwrap();

//     let (store, dispatch) = use_store::<Store>();
//     let tasks = store.tasks.clone();
//     let token = store.token.clone();

//     let get_tasks = {
//         let dispatch = dispatch.clone();
//         Callback::from(move |_| {
//             spawn_local(async move {
//                 match api::get_tasks(&token).await { 
//                     Ok(tasks) => {
//                         // store::set_tasks(dispatch.clone(), tasks);
//                         store::set_tasks(tasks, dispatch.clone());
//                     },
//                     Err(ApiError::NotAuthenticated) => store::logout(dispatch),
//                     Err(error) => {
//                         log::error!("Error fetching tasks: {:?}", error);
//                     }
//                 }
//             });
//         })
//     };

//     let add_task = {
//         let dispatch = dispatch.clone();
//         let token = store.token.clone();
//         Callback::from(move |user:PostCredentials| {
//             spawn_local(async move {
//                 // Replace Default::default() with the appropriate task data
//                 let task = PostCredentials {
//                     id: user.id, // Replace with the appropriate ID
//                     name: user.name, // Replace with the task name
//                     location: user.location, // Replace with the task location
//                     data:user.data, // Replace with the task data
//                 };
    
//                 match api::Post_task(&token, &task).await {
//                     Ok(task) => {
//                         store::add_task(dispatch, task);
//                     },
//                     Err(ApiError::NotAuthenticated) => store::logout(dispatch),
//                     Err(error) => {
//                         log::error!("Error adding task: {:?}", error);
//                     }
//                 }
//             });
//         })
//     };
    
//     let update_task = {
//         let dispatch = dispatch.clone();
//         Callback::from(move |_| {
//             spawn_local(async move {
//                 match api::update_task(Default::default()).await {
//                     Ok(task) => {
//                         store::update_task(dispatch, task);
//                     },
//                     Err(ApiError::NotAuthenticated) => store::logout(dispatch),
//                     Err(error) => {
//                         log::error!("Error updating task: {:?}", error);
//                     }
//                 }
//             });
//         })
//     };

//     let delete_task = {
//         let dispatch = dispatch.clone();
//         Callback::from(move |_| {
//             spawn_local(async move {
//                 match api::delete_task(Default::default()).await {
//                     Ok(()) => {
//                         store::delete_task(dispatch, task_id);
//                     },
//                     Err(ApiError::NotAuthenticated) => store::logout(dispatch),
//                     Err(error) => {
//                         log::error!("Error deleting task: {:?}", error);
//                     }
//                 }
//             });
//         })
//     };

//     html! {
//         <section class={stylesheet}>
//             <div>
//                 {if !token.is_empty() {
//                     html! {
//                         <>
//                             <button onclick=get_tasks>{"Get Tasks"}</button>
//                             <button onclick=add_task>{"Add Task"}</button>
//                             <button onclick=update_task>{"Update Task"}</button>
//                             <button onclick=delete_task>{"Delete Task"}</button>
//                         </>
//                     }
//                 } else {
//                     html! {}
//                 }}
//             </div>
//             {if !token.is_empty() {
//                 html! {
//                     <Tasks tasks={tasks} />
//                 }
//             } else {
//                 html! {}
//             }}
//         </section>
//     }
// }
