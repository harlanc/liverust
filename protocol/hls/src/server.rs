use crate::hls_event_manager::DispatchEventProducer;
use {
    super::hls_event_manager::HlsEventManager,
    hyper::{
        service::{make_service_fn, service_fn},
        Body, Request, Response, Server, StatusCode,
    },
    std::collections::HashMap,
    tokio::fs::File,
    tokio_util::codec::{BytesCodec, FramedRead},
};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;
static NOTFOUND: &[u8] = b"Not Found";

async fn handle_connection(req: Request<Body>) -> Result<Response<Body>> {
    let path = req.uri().path();
    let directives = req
        .uri()
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new);

    let mut file_path: String = String::from("");

    if path.ends_with(".m3u8") {
        //http://127.0.0.1/app_name/stream_name/stream_name.m3u8
        let m3u8_index = path.find(".m3u8").unwrap();

        if m3u8_index > 0 {
            let (left, _) = path.split_at(m3u8_index);
            let rv: Vec<_> = left.split("/").collect();

            let app_name = String::from(rv[1]);
            let stream_name = String::from(rv[2]);

            file_path = format!("./{}/{}/{}.m3u8", app_name, stream_name, stream_name);
        }
    } else if path.ends_with(".ts") {
        //http://127.0.0.1/app_name/stream_name/ts_name.m3u8
        let ts_index = path.find(".ts").unwrap();

        if ts_index > 0 {
            let (left, _) = path.split_at(ts_index);

            let rv: Vec<_> = left.split("/").collect();
            println!("{:?}", rv);

            let app_name = String::from(rv[1]);
            let stream_name = String::from(rv[2]);
            let ts_name = String::from(rv[3]);

            file_path = format!("./{}/{}/{}.ts", app_name, stream_name, ts_name);
            println!("{}", file_path)
        }
    }

    return simple_file_send(file_path.as_str()).await;
}

/// HTTP status code 404
fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(NOTFOUND.into())
        .unwrap()
}

async fn simple_file_send(filename: &str) -> Result<Response<Body>> {
    // Serve a file by asynchronously reading it by chunks using tokio-util crate.

    if let Ok(file) = File::open(filename).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        let r = Response::builder()
            .status(200)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "*")
            .header("Access-Control-Allow-Headers", "*")
            .body(body);
        return Ok(r.unwrap());
    }

    Ok(not_found())
}

pub async fn run(port: u32, hls_dispatch: DispatchEventProducer) -> Result<()> {
    let listen_address = format!("0.0.0.0:{}", port);
    let sock_addr = listen_address.parse().unwrap();

    let new_service = make_service_fn(move |_| async {
        Ok::<_, GenericError>(service_fn(move |req| handle_connection(req)))
    });

    let server = Server::bind(&sock_addr).serve(new_service);
    log::info!("Hls server listening on http://{}", sock_addr);
    server.await?;

    Ok(())
}
