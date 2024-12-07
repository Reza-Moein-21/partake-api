#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    web_server::WebServer::run().await
}

mod web_server {
    use std::{convert::Infallible, net::SocketAddr};

    use http_body_util::Full;
    use hyper::{body::Bytes, server::conn::http1, service::service_fn, Request, Response};
    use hyper_util::rt::TokioIo;
    use tokio::net::TcpListener;

    pub struct WebServer {}
    impl WebServer {
        pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            // TODO must read from configuration
            let port = 3000;
            let addr = SocketAddr::from(([127, 0, 0, 1], port));
            println!("Server started on port {}", port);
            let listener = TcpListener::bind(addr).await?;

            loop {
                let (stream, _) = listener.accept().await?;

                let io = TokioIo::new(stream);

                tokio::task::spawn(async move {
                    if let Err(err) = http1::Builder::new()
                        .serve_connection(io, service_fn(Self::hello))
                        .await
                    {
                        eprintln!("Error serving connection: {:?}", err);
                    }
                });
            }
        }
        async fn hello(
            _: Request<hyper::body::Incoming>,
        ) -> Result<Response<Full<Bytes>>, Infallible> {
            Ok(Response::new(Full::new(Bytes::from("Hello, World!\n"))))
        }
    }
}

mod domain {

    use chrono::{DateTime, Local};
    struct Bill {
        order: Order,
        bill_type: BillType,
        total_amount: Amount,
        payors: Vec<Participant>,
    }

    struct Order {
        items: Vec<OrderItem>,
        where_shit_happend: Place,
        when_shit_happend: DateTime<Local>,
        participants: Vec<Participant>,
    }

    struct OrderItem {
        title: String,
        item_amount: Amount,
    }

    enum BillType {
        FOOD,
        DRINK,
        TRIP,
        OTHER,
    }

    struct Place {
        title: String,
    }

    struct Amount {
        value: f64,
        ccy: String,
    }

    struct Participant {
        name: String,
    }
}
