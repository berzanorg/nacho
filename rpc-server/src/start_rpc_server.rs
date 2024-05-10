use crate::{RpcMethod, RpcResponse};
use http_body_util::BodyExt;
use hyper::{body::Buf, server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use nacho_data_structures::ByteConversion;
use std::{
    future::Future,
    net::{Ipv4Addr, SocketAddrV4},
};
use tokio::net::TcpListener;

/// Starts an RPC Server at the given socket address and uses the given handler function to handle RPC requests.
///
/// The handler function takes an `RpcMethod` and returns an `RpcResponse` asynchronously.
///
/// It automatically responds with `RpcResponse::Mistake` for invalid requests.
///
/// It runs the handler function for each valid request and responds with the handler's result.
///
/// # Examples
///
/// Define a handler:
/// ```rs
/// async fn rpc_handler(rpc_method: RpcMethod) -> RpcResponse {
///     match rpc_method {
///         // ...
///     }
/// }
/// ```
///
/// Start an RPC server:
/// ```rs
/// start_rpc_server(rpc_method_handler).await?;
/// ```
///
pub async fn start_rpc_server<F, Fut>(rpc_method_handler: F) -> Result<(), std::io::Error>
where
    F: Fn(RpcMethod) -> Fut + Send + Sync + Copy + 'static,
    Fut: Future<Output = RpcResponse> + Send + 'static,
{
    let port = std::env::var("NACHO_RPC_SERVER_PORT")
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "NACHO_RPC_SERVER_PORT environment variable is not set",
            )
        })?
        .parse()
        .unwrap();

    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port)).await?;

    loop {
        let (socket, _) = listener.accept().await?;

        let io = TokioIo::new(socket);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(|req| async {
                        let body = req.collect().await;

                        let rpc_method = parse_body(body);

                        let rpc_response = rpc_method_handler(rpc_method).await;
                        rpc_response.into()
                    }),
                )
                .await
            {
                println!("{}", err)
            }
        });
    }
}

fn parse_body(
    body: Result<http_body_util::Collected<hyper::body::Bytes>, hyper::Error>,
) -> RpcMethod {
    let body = match body {
        Ok(body) => body,
        Err(_) => return RpcMethod::Unknown,
    };

    let mut buf = body.aggregate();

    if buf.remaining() != RpcMethod::SIZE_IN_BYTES {
        return RpcMethod::Unknown;
    }

    let mut body_bytes = [0_u8; RpcMethod::SIZE_IN_BYTES];
    buf.copy_to_slice(body_bytes.as_mut_slice());

    let rpc_method = RpcMethod::from_bytes(&body_bytes);

    rpc_method
}
