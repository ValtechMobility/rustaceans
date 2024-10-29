use core::time;
use gloo_timers::callback::Timeout;

use std::{thread::sleep, time::Duration};

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{js_sys, JsFuture};

// #[derive(Debug, Serialize, Deserialize)]
// #[wasm_bindgen]
// pub enum UserAccess {
//     User,
//     Admin,
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[wasm_bindgen]
// pub struct User {
//     username: String,
//     user_id: u32,
//     user_access: UserAccess,
// }
// #[wasm_bindgen]
// impl User {
//     #[wasm_bindgen(constructor)]
//     pub fn new(username: String, user_id: u32, user_access: UserAccess) -> Self {
//         User {
//             username,
//             user_id,
//             user_access,
//         }
//     }
// }

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);

    #[wasm_bindgen(js_namespace = setTimeout)]
    fn timeout(duration: u32);
}

// #[wasm_bindgen]
// pub fn check_first_user(user: User) -> String {
//     println!("{:?}", user);
//     user.username
// }

// #[wasm_bindgen(start)]
// async fn get_from_js() -> Result<(), JsValue> {
//     let promise = js_sys::Promise::resolve(&42.into());
//     let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
//     log(result.as_string().unwrap().as_str());
//     Ok(())
// }

#[wasm_bindgen]
pub async fn get_fourty_two() -> Result<JsValue, JsValue> {
    let promise = js_sys::Promise::resolve({ &42.into() });
    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(result)
}
