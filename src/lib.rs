use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Debug, Deserialize, Serialize)]
struct GenericResponse {
    status: u16,
    message: String,
}

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    if let Some(webstr) = req.headers().get("Upgrade")? {
        // if webstr == "websocket" {
        //     console_log!("header is websocket!");
        // } else {
        //     console_log!("header is not websocket!");
        // }
        Response::ok("Hello, World! ".to_owned() + &*webstr)
    } else {
        Router::new()
            .get_async("/foo", handle_get)
            .run(req, env)
            .await
    }

    // let header = match req.headers().get("Upgrade")? {
    //     None => {return Err(Error::from("No header of Upgrade"))}
    //     Some(string) => {string}
    // };
    // if !"websocket".eq(&header) {
    //     Router::new()
    //         .get_async("/foo", handle_get)
    //         .run(req, env)
    //         .await
    // } else {
    //     let WebSocketPair { client, server } = match WebSocketPair::new() {
    //         Ok(pair) => { pair }
    //         Err(err) => {return Err(err)}
    //     };
    //     let _ = server.accept();
    //
    //     Response::ok("Hello, World!")
    // }

    // if let Result{Option:upgradeHeader, Err: err} = match req.headers().get("Upgrade") && !"websocket".eq(&upgradeHeader){
    //     Router::new()
    //         .get_async("/foo", todo!("todo"))
    //         .post_async("/bar", todo!("todo"))
    //         .delete_async("/baz", todo!("todo"))
    //         .run(req, env)
    //         .await
    // };
}

pub async fn handle_get(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    Response::from_json(&GenericResponse {
        status: 200,
        message: "You reached a GET route!".to_string(),
    })
}