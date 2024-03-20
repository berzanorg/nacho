use crate::{RpcMethod, RpcResponse, RPC_METHOD_SIZE_IN_BYTES};
use http_body_util::BodyExt;
use hyper::{body::Buf, server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use std::future::Future;
use tokio::net::{TcpListener, ToSocketAddrs};

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
/// start_rpc_server("127.0.0.1:2345", rpc_method_handler).await?;
/// ```
///
pub async fn start_rpc_server<A, F, Fut>(
    addr: A,
    rpc_method_handler: F,
) -> Result<(), std::io::Error>
where
    A: ToSocketAddrs,
    F: Fn(RpcMethod) -> Fut + Send + Sync + Copy + 'static,
    Fut: Future<Output = RpcResponse> + Send + 'static,
{
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (socket, _) = listener.accept().await?;

        let io = TokioIo::new(socket);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(|req| async {
                        let body = req.collect().await;

                        match body {
                            Ok(body) => {
                                let mut buf = body.aggregate();

                                if buf.remaining() != RPC_METHOD_SIZE_IN_BYTES {
                                    return RpcResponse::Mistake().into();
                                }

                                let mut body_bytes = [0_u8; RPC_METHOD_SIZE_IN_BYTES];
                                buf.copy_to_slice(body_bytes.as_mut_slice());

                                let rpc_method: Result<RpcMethod, _> = body_bytes.try_into();

                                match rpc_method {
                                    Ok(rpc_method) => {
                                        let rpc_response = rpc_method_handler(rpc_method).await;
                                        rpc_response.into()
                                    }
                                    Err(err) => {
                                        eprintln!("{err}");
                                        RpcResponse::Mistake().into()
                                    }
                                }
                            }
                            Err(err) => {
                                eprintln!("{err}");
                                RpcResponse::Mistake().into()
                            }
                        }
                    }),
                )
                .await
            {
                println!("{}", err)
            }
        });
    }
}
