use std::sync::Mutex;

use bytes::Bytes;
use clipboard_win::{formats, get_clipboard, set_clipboard};
use warp::Filter;

static PREV_INPUT: Mutex<String> = Mutex::new(String::new());
static PREV_OUTPUT: Mutex<String> = Mutex::new(String::new());
const HOST: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 11111;
const PATH: &str = "clip";

fn handle_clipboard(content: Bytes) -> String {
    let mut output = Vec::new();
    if let Ok(text) = get_clipboard::<String, _>(formats::Unicode) {
        // if current clipboard is different from previous input
        // meaning clipboard is updated by other program, send it
        let mut prev_input = PREV_INPUT.lock().unwrap();
        if *prev_input != text {
            println!("Send: {:?}", text);
            output.extend(text.bytes());
            *prev_input = text;
        }
    } else if let Ok(img) = get_clipboard::<Vec<u8>, _>(formats::Bitmap) {
        println!("Send: {:?} bytes", img.len());
        output.extend(img)
    }

    if !content.is_empty() {
        if let Ok(text) = String::from_utf8(content.clone().to_vec()) {
            // if input is different from previous output
            // meaning ios's clipboard is updated, set it
            let mut prev_output = PREV_OUTPUT.lock().unwrap();
            if *prev_output != text {
                let _ = set_clipboard(formats::Unicode, text.clone());
                println!("Set clipboard: {:?}", text);
                *prev_output = text;
            }
        } else if let Ok(_) = set_clipboard::<Vec<u8>, _>(formats::Bitmap, content.to_vec()) {
            println!("Set clipboard: {:?} bytes", content.len());
        }
    }
    String::from_utf8(output).unwrap_or_default()
}

fn endpoint() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(PATH)
        .and(warp::post())
        .and(warp::header::<String>("client"))
        .and(warp::body::bytes())
        .map(move |client, content: Bytes| {
            // Add 'move' keyword to capture the value
            if client != "shortcut" {
                println!("Error client: {:?}", client);
                return String::new();
            }
            handle_clipboard(content)
        })
}

#[tokio::main]
async fn main() {
    let endpoint = endpoint();
    let task = warp::serve(endpoint).run((HOST, PORT));
    println!(
        "Server started at {}:{PORT}/{PATH}",
        HOST.map(|n| n.to_string()).join(".")
    );
    task.await;
}
