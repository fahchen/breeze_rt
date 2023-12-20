use bytes::Bytes;
use http_body_util::Full;
use hyper::{body, server::conn::http1, service::service_fn, Request, Response, Version};
use rustler::{resource, Binary, Env, LocalPid, ResourceArc, Term};

mod bridged_io;

use bridged_io::BridgedIo;

use crate::bridged_io::BridgedIoSender;

mod atoms {
    rustler::atoms! {
        ok,
        error,
        message_write,
        message_flush,
        message_shutdown,
    }
}

fn load(env: Env, _: Term) -> bool {
    resource!(BridgedIoSender, env);
    true
}

#[rustler::nif]
fn send_message(sender: ResourceArc<BridgedIoSender>, data: Binary) -> rustler::Atom {
    let result = sender.send(data.to_vec());
    if result.is_err() {
        eprintln!("Error sending message: {}", result.err().unwrap());
        return atoms::error();
    }
    atoms::ok()
}

#[rustler::nif]
fn start(pid: LocalPid) -> ResourceArc<BridgedIoSender> {
    let (sender, io) = BridgedIo::new(pid);

    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let local = tokio::task::LocalSet::new();

        local.block_on(&runtime, async {
            let conn = http1::Builder::new()
                .keep_alive(true)
                .half_close(true)
                .serve_connection(
                    io,
                    service_fn(|req: Request<body::Incoming>| async move {
                        if req.version() == Version::HTTP_11 {
                            Ok(Response::new(Full::<Bytes>::from("Hello World")))
                        } else {
                            // Note: it's usually better to return a Response
                            // with an appropriate StatusCode instead of an Err.
                            Err("not HTTP/1.1, abort connection")
                        }
                    }),
                );

            if let Err(err) = conn.await {
                println!("Error serving connection: {:?}", err);
            }
        })
    });

    ResourceArc::new(sender)
}

rustler::init!(
    "Elixir.BreezeRt.SocketMan",
    [start, send_message],
    load = load
);
