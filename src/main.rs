use std::convert::Infallible;
use std::time::Duration;
use async_std::task::block_on;
use salvo::prelude::*;
use salvo::{Depot, Request, Response, Router, Server, sse};
use salvo::sse::SseEvent;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use uuid::Uuid;
use async_std::stream::StreamExt;
use rand::Rng;


#[tokio::main]
async fn main() {

    let router = Router::with_hoop(Logger::new())
        .hoop(Compression::new().enable_gzip(CompressionLevel::Fastest))
        .push(Router::new().path("sse").get(sse_stream));

    Server::new(TcpListener::new("0.0.0.0:9090").bind().await).serve(router).await;
}

#[handler]
pub async fn sse_stream(
    _req: &mut Request,
    res: &mut Response,
    _depot: &mut Depot) {

    let (sender_response, receiver_response) = mpsc::channel::<SseEvent>(1000);

    let data_string = r#"{"aId":"12345678","bId":"987654321","cId":"abcd","mTNMoCqZU1":"ZlrWCKb059","3yP7piSDG7":"GtfCgYLgH2","3nuoqDxyRz":{"OucP7ijiVU":0.14,"NWn5eWTRaI":0.0,"r4IOJ70eyL":"jf7NIgxIps"},"z1B2mZkewj":{"cqK8P5PywK":0.0,"WmYFwzxjiY":0.0,"ubsmYvJ5Hg":"IWxVYO9JKf"},"dYHmrI5Y9z":{"I8rQVmnQeb":0.0,"NalXDhKiIa":0.0,"d8PTvwdTWr":"eI35Jtaypy"},"0B95i170aM":{"ZOOqle02qc":0.14,"NIFqhnXQPF":"IzefB2DUMD"},"eI35Jtaypy":{"K25PRv3UQ0":0.14,"crqvveK9et":"vk5tc1h5rV"},"K25PRv3UQ0":{"kOaKR8HL4O":0.14,"IjAsfT3tvI":"jDxH8kt2V9"},"lZThnkPs5H":{"IXbxrs8ogd":{"QEnsb0MHBa":"200","v6RNnCGvSa":"Kt8FMmMm5w"}},"EJSK7OWYCY":{"ghbtUQUWZD":1.0,"rLuBOaDwNU":0.0,"5ZVLA2cudi":0.0,"vo0Kl4voMe":0.0}}"#;
    let _spawn_handle = std::thread::spawn(move || block_on(async {
        let mut rng = rand::thread_rng();
        loop {

            let sleep_duration = Duration::from_millis(rng.gen_range(0..1000));
            println!("sleep duration: {:?}", sleep_duration);
            std::thread::sleep(sleep_duration);

            let send_result = sender_response.send(sse::SseEvent::default().data(data_string).name("dummy_data").id(Uuid::new_v4().to_string())
            ).await;

            if let Err(_) = send_result {
                break;
            }
        }

    }));

    let response_stream = ReceiverStream::new(receiver_response);
    let stream = response_stream.map(|sse_event| {
        Ok::<SseEvent, Infallible>(sse_event)
    });

    SseKeepAlive::new(stream).streaming(res).ok();

}
