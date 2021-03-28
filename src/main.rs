use warp::Filter;
use uwuifier::uwuify_str_sse;
use percent_encoding::percent_decode;
#[tokio::main]


async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("uwu" / String)
        .map(|name:String| format!("{}", uwuify_str_sse(percent_decode(name.as_bytes()).decode_utf8().unwrap().as_ref()    )
    ));

        
    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}