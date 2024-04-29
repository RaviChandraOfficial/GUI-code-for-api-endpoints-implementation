use crate::{
    components::atoms::bb_select::BBSelect,
    components::{atoms::bb_select::SelectOption, organisms::tasks::Tasks},
    store::{self, Store, Task},
};
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yewdux::prelude::*;

#[styled_component(Home)]
pub fn component() -> Html {
    let stylesheet = Style::new(css!(
        r#"
          display: flex;
          flex-direction: column;
          align-items: center;
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
      <section class={stylesheet}>
        if !token.is_empty() {
            <div>
            <div class="filter">
                <BBSelect
                data_test="filter"
                id="filter"
                label="Filter Tasks"
                options={filter_options.clone()}
                onchange={filter_onchange}
                />
            </div>
            <div class="sort">
                <BBSelect
                data_test="sort"
                id="sort"
                label="Sort Tasks"
                options={sort_options.clone()}
                onchange={sort_onchange}
                />
            </div>
            </div>
            <Tasks tasks={sort_tasks(filter_tasks(tasks, filter_options), sort_options)} />
        }
      </section>
    }
}

fn filter_tasks(tasks: Vec<Task>, filter_options: Vec<SelectOption>) -> Vec<Task> {
    let selected_filter_option = filter_options
        .into_iter()
        .find(|filter_option| filter_option.is_selected)
        .unwrap_or_else(|| SelectOption::new("none", "None", true));

    tasks
        .into_iter()
        .filter(|task| match selected_filter_option.value.as_str() {
            "none" => true,
            "completed" => task.completed_at.is_some(),
            "uncompleted" => task.completed_at.is_none(),
            "priority_a" => task.priority.is_some() && task.priority.as_ref().unwrap() == "A",
            "priority_b" => task.priority.is_some() && task.priority.as_ref().unwrap() == "B",
            "priority_c" => task.priority.is_some() && task.priority.as_ref().unwrap() == "C",
            _ => true,
        })
        .collect()
}

fn sort_tasks(mut tasks: Vec<Task>, sort_options: Vec<SelectOption>) -> Vec<Task> {
    let selected_sort_option = sort_options
        .into_iter()
        .find(|sort_option| sort_option.is_selected)
        .unwrap_or_else(|| SelectOption::new("created_order", "Created Order", true));
    tasks.sort_by(|a, b| match selected_sort_option.value.as_str() {
        "priority" => a
            .priority
            .as_ref()
            .unwrap_or(&"A".to_owned())
            .partial_cmp(b.priority.as_ref().unwrap_or(&"A".to_owned()))
            .unwrap(),
        "name" => a.title.partial_cmp(&b.title).unwrap(),
        _ => a.id.partial_cmp(&b.id).unwrap(),
    });
    tasks
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
