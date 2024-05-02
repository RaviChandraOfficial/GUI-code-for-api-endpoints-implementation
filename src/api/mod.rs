//authresponse, user structure, task response, single task response, 
//create account, login, get_task, update_task, delete_task, create_task, complete_task, uncomplete_task, logout

pub mod api_errors;
pub mod patch_task;

use js_sys::JSON;
// use log::info;
use reqwasm::http::Request;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen::JsValue;
use web_sys::console;

use crate::store::{PostCredentials, Task};

use self::{api_errors::ApiError,
     patch_task::PatchTask
    };

// TODO refactor url to environment variable
const BASE_URL: &str = include_str!("api_base_uri.txt");

// #[derive(Serialize, Deserialize, Debug)]
// pub struct AuthResponse {
//     pub data: User,
// }

#[derive(Serialize, Deserialize,Debug)]
pub struct User {
    pub id_token:String,
    pub access_token: String,
    pub refesh_token:String
}

#[derive(Serialize, Deserialize)]
pub struct GetResponse {
    pub id: i32,
    pub user_name: String,
    pub location :String,
    pub data :String,
    pub name:String,
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct Task1 {
    pub id: i32,
    pub user_name: String,
    pub location :String,
    pub data :String,
    pub name:String,
}

#[derive(Serialize, Deserialize)]
pub struct SingleTaskResponse {
    pub data: Task,
}



#[derive(Serialize, Deserialize)]
pub struct SignupResponse {
    pub status:String,
    pub message:String,
}



#[derive(Debug,Serialize, Deserialize)]
pub struct SigninResponse {
    pub id_token:String,
    pub access_token:String,
    pub refresh_token: Option<String>, // Optional in case it's missing
}


pub async fn create_account(username: String, password: String, email: String) -> Result<SignupResponse, JsValue> {
    let response = Request::post(&format!("{}signup", BASE_URL))
        .header("Content-Type", "application/json")
        .body(
            json!({
                "username": username,
                "password": password,
                "email": email
            })
            .to_string(),
        )
        .send()
        .await;

    match response {
        Ok(http_response) => {

            if http_response.ok() {
                let auth_response = http_response.json::<SignupResponse>().await;
                match auth_response {
                    Ok(data) => Ok(data), 
                    Err(_) => Err(JsValue::from_str("Error parsing response JSON")),
                }
            } else {
                Err(JsValue::from_str(&format!("HTTP Error: {}", http_response.status())))
            }
        },
        Err(_) => Err(JsValue::from_str("Network request failed")),
    }
}

// pub async fn login(username: String, password: String) -> AuthResponse {
//     println!("{:?}",username);
//     println!("{:?}",password);
//     Request::post(&format!("{}/signin", BASE_URL))
//         .header("content-type", "application/json")
//         .body(
//             json! ({
//                 "username": username,
//                 "password": password
//             })
//             .to_string(),
//         )
//         .send()
//         .await
//         .unwrap()
//         .json::<AuthResponse>()
//         .await
//         .unwrap()
// }



// // Assuming the `login` function looks something like this:
// pub async fn login(username: String, password: String) -> Result<SigninResponse, JsValue> {
//     let response = Request::post(&format!("{}/signin", BASE_URL))
//         .header("Content-Type", "application/json")
//         .body(json!({
//             "username": username,
//             "password": password
//         }).to_string())
//         .send()
//         .await;


// pub async fn login(username: String, password: String) -> Result<SigninResponse, JsValue> {
//     let response = Request::post(&format!("{}/signin", BASE_URL))
//         .header("Content-Type", "application/json")
//         .body(json!({
//             "username": username,
//             "password": password,
//         })
//         .to_string()) // Make sure JSON is properly formatted
//         .send()
//         .await;

//     match response {
//         Ok(http_response) => {
//             if http_response.ok() {
//                 // info!("This is an info message");
//                 // console::log_1(&"http response ok lo checking:".into());
//                 // println!("{:?}",http_response.json::<AuthResponse>().await.map_err(|e| JsValue::from_str(&e.to_string())));
//                 // http_response.json::<SigninResponse>().await.map_err(|e| JsValue::from_str(&e.to_string()))
//                 let auth_response = http_response.json::<SigninResponse>().await;
//                 match auth_response {
//                     Ok(data) => Ok(data),
//                     Err(_) => Err(JsValue::from_str("response lo error vasthondi")),
//                 }
                
//             } else {
//                 // Handle non-OK HTTP responses
//                 Err(JsValue::from_str(&format!("HTTP Error: {}", http_response.status())))
//             }
//         },
//         Err(_) => Err(JsValue::from_str("Network request failed")),
//     }
// }




use gloo_net::http::{ Response};



pub async fn login(username: String, password: String) -> Result<SigninResponse, JsValue> {
    let response = gloo_net::http::Request::post(&format!("{}signin", BASE_URL))
        .header("Content-Type", "application/json")
        .body(json!({
            "username": username,
            "password": password,
        }).to_string()).unwrap()
        .send()
        .await;

        match response {
            Ok(http_response) => {
                if http_response.ok() {
                    // Only read the response body once
                    let json_result = http_response.json::<SigninResponse>().await;
    
                    match json_result {
                        Ok(data) => Ok(data),
                        Err(e) => Err(JsValue::from_str(&format!("Error parsing JSON: {:?}", e))),
                    }
                } else {
                    Err(JsValue::from_str(&format!("HTTP Error: {}", http_response.status())))
                }
            },
            Err(e) => {
                Err(JsValue::from_str(&format!("Network request failed: {:?}", e)))
            },
        }
    }




// // use gloo_net::http::Request;

// pub async fn login(username: String, password: String) -> Result<SigninResponse, JsValue> {
//     let response_result = Request::post(&format!("{}/signin", BASE_URL))
//         .header("Content-Type", "application/json")
//         .body(json!({
//             "username": username,
//             "password": password,
//         })
//         .to_string())
//         .send()
//         .await;

//     match response_result {
//         Ok(http_response) => {
//             if http_response.ok() {
//                 let json_result = http_response.json::<SigninResponse>().await;
//                 match json_result {
//                     Ok(auth_response) => {
//                         Ok(auth_response)
//                     }
//                     Err(e) => Err(JsValue::from_str(&format!("Error parsing JSON: {:?}", e))),
//                 }
//             } else {
//                 Err(JsValue::from_str(&format!("HTTP Error: {}", http_response.status())))
//             }
//         },
//         Err(e) => {
//             Err(JsValue::from_str(&format!("Network request failed: {:?}", e)))
//         },
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct SigninResponse {
//     pub id_token: String,
//     pub access_token: String,
//     // Optional field
//     pub refresh_token: Option<String>,
// }

// // use gloo_net::http::Request as GlooRequest;

// pub async fn login(username: String, password: String) -> Result<SigninResponse, JsValue> {
//     let response_result = Request::post(&format!("{}/signin", BASE_URL))
//         .header("Content-Type", "application/json")
//         .body(json!({
//             "username": username,
//             "password": password,
//         })
//         .to_string())
//         .send()
//         .await;

//     match response_result {
//         Ok(http_response) => {
//             if http_response.ok() {
//                 let json_result = http_response.json::<SigninResponse>().await;
//                 match json_result {
//                     Ok(auth_response) => {
//                         Ok(auth_response)
//                     }
//                     Err(e) => Err(JsValue::from_str(&format!("Error parsing JSON: {:?}", e))),
//                 }
//             } else {
//                 Err(JsValue::from_str(&format!("HTTP Error: {}", http_response.status())))
//             }
//         },
//         Err(e) => {
//             Err(JsValue::from_str(&format!("Network request failed: {:?}", e)))
//         },
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct SigninResponse {
//     pub id_token: String,
//     pub access_token: String,
//     // Optional field
//     pub refresh_token: Option<String>,
// }





// #[derive(Debug, Serialize, Deserialize)]
// pub struct SigninResponse {
//     pub id_token: String,
//     pub access_token: String,
//     pub refresh_token: String,
// }



// #[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
// // #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct GetTaskResponse{
//     pub data:Vec<data>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct data{
//     pub id: u32,

// }






// pub async fn get_tasks(token: &str) -> Result<GetResponse, ApiError> {
//     let request = Request::get(&format!("{}get/user", BASE_URL))
//         .header("AUTHORIZATION", token)
//         .send()
//         .await.expect("Fetching Failed");

//     if request.ok() {
//         Ok(request.json::<GetResponse>().await.unwrap())
//     } else {
//         Err(handle_errors(request.status()))
//     }
// }




// use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Note {
    pub data: String,
    pub id: u32,
    pub location: String,
    pub name: String,
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTasksResponse {
    pub notes: Vec<Note>,
    pub results: u32,
    pub status: String,
}


pub async fn get_tasks(token: &str) -> Result<GetTasksResponse, ApiError> {
    let client = reqwest::Client::new();
    let response = client.get(&format!("{}get/user", BASE_URL))
        .header("AUTHORIZATION", token)
        .send()
        .await.expect("failed");

    if response.status().is_success() {
        let result = response.json::<GetTasksResponse>().await.unwrap();
        Ok(result)
    } else {
        Err(handle_errors(response.status().into()))
    }
}


pub async fn update_task(task_id: u32, token: &str, task: PatchTask) -> Result<(), ApiError> {
    let request = Request::patch(&format!("{}put/user{}", BASE_URL, task_id))
        .header("x-auth-token", token)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&task).unwrap())
        .send()
        .await
        .unwrap();

    if request.ok() {
        Ok(())
    } else {
        Err(handle_errors(request.status()))
    }
}

pub async fn delete_task(task_id: u32, token: &str) -> Result<(), ApiError> {
    let request = Request::delete(&format!("{}delete/user{}", BASE_URL, task_id))
        .header("x-auth-token", token)
        .send()
        .await
        .unwrap();

    if request.ok() {
        Ok(())
    } else {
        Err(handle_errors(request.status()))
    }
}

// pub async fn Post_task(
//     token: &str,
//     title: String,
//     description: Option<String>,
//     priority: String,
// ) -> Result<SingleTaskResponse, ApiError> {
//     let new_task = PatchTask::new(Some(title), Some(priority), description, None);
//     let request = Request::post(&format!("{}/post/user", BASE_URL))
//         .header("x-auth-token", token)
//         .header("content-type", "application/json")
//         .body(serde_json::to_string(&new_task).unwrap())
//         .send()
//         .await
//         .unwrap();

//     if request.ok() {
//         Ok(request.json::<SingleTaskResponse>().await.unwrap())
//     } else {
//         Err(handle_errors(request.status()))
//     }
// }


pub async fn Post_task(
    token: &str,
    user:&PostCredentials,
) -> Result<(), ApiError> {
    let request = Request::post(&format!("{}/post/user", BASE_URL))
        .header("x-auth-token", token)
        .header("content-type", "application/json")
        .body(json!({
            "id": user.id,
            "name": user.name,
            "location":user.location,
            "data":user.data,
        }).to_string())
        .send()
        .await.expect("falied to send the request");

    if request.status()  == StatusCode::CREATED{
        Ok(())
    } else {
        Err(handle_errors(request.status()))
    }
}




pub async fn complete_task(task_id: u32, token: &str) -> Result<(), ApiError> {
    let request = Request::put(&format!("{}/tasks/{}/completed", BASE_URL, task_id))
        .header("x-auth-token", token)
        .send()
        .await
        .unwrap();

    if request.ok() {
        Ok(())
    } else {
        Err(handle_errors(request.status()))
    }
}

pub async fn uncomplete_task(task_id: u32, token: &str) -> Result<(), ApiError> {
    let request = Request::put(&format!("{}/tasks/{}/uncompleted", BASE_URL, task_id))
        .header("x-auth-token", token)
        .send()
        .await
        .unwrap();

    if request.ok() {
        Ok(())
    } else {
        Err(handle_errors(request.status()))
    }
}

pub async fn logout(token: &str) -> Result<(), ApiError> {
    let request = Request::post(&format!("{}signout", BASE_URL))
        .header("AUTHORIZATION", token)
        .send()
        .await
        .unwrap();
    if request.ok() {
        Ok(())
    } else {
        Err(handle_errors(request.status()))
    }
}

fn handle_errors(status: u16) -> ApiError {
    match status {
        401 => ApiError::NotAuthenticated,
        _ => ApiError::Unknown,
    }
}
