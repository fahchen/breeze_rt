use std::{
    io::Error,
    ops::Deref,
    pin::Pin,
    sync::mpsc,
    task::{Context, Poll},
};

use bytes::{BufMut, BytesMut};
use hyper::rt::{Read, ReadBufCursor, Write};
use rustler::{types::tuple::make_tuple, Encoder, LocalPid, OwnedEnv};

pub struct BridgedIoSender(mpsc::Sender<Vec<u8>>);

impl Deref for BridgedIoSender {
    type Target = mpsc::Sender<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct BridgedIo {
    pub rx: mpsc::Receiver<Vec<u8>>,
    pub buf: BytesMut,
    pub pid: LocalPid,
}

impl BridgedIo {
    pub fn new(pid: LocalPid) -> (BridgedIoSender, Self) {
        let (tx, rx) = mpsc::channel();
        (
            BridgedIoSender(tx),
            Self {
                rx,
                pid,
                buf: BytesMut::new(),
            },
        )
    }
}

impl Read for BridgedIo {
    fn poll_read(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        mut buf: ReadBufCursor<'_>,
    ) -> Poll<Result<(), Error>> {
        let mut dbuf = unsafe { buf.as_mut() };
        let n_dbuf = dbuf.len();

        match self.buf.len() {
            0 => match self.rx.recv() {
                Ok(data) if data.is_empty() => Poll::Ready(Ok(())),
                Ok(data) if data.len() > n_dbuf => {
                    dbuf.put_slice(&data[..n_dbuf]);
                    unsafe {
                        buf.advance(n_dbuf);
                    };
                    self.buf.put_slice(&data[n_dbuf..]);
                    Poll::Ready(Ok(()))
                }
                Ok(data) if data.len() <= n_dbuf => {
                    dbuf.put_slice(&data);
                    unsafe {
                        buf.advance(data.len());
                    };
                    Poll::Ready(Ok(()))
                }
                Err(mpsc::RecvError) => Poll::Ready(Ok(())),
                _ => unreachable!(),
            },
            n_buf => {
                let n_data = if n_buf >= n_dbuf { n_dbuf } else { n_buf };

                let data = self.buf.split_to(n_data);
                dbuf.put_slice(data.as_ref());
                unsafe {
                    buf.advance(n_data);
                };

                Poll::Ready(Ok(()))
            }
        }
    }
}

impl Write for BridgedIo {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        let mut msg_env = OwnedEnv::new();
        let result = msg_env.send_and_clear(&self.pid, |env| {
            make_tuple(
                env,
                &[crate::atoms::message_write().encode(env), buf.encode(env)],
            )
        });

        if result.is_err() {
            return Poll::Pending;
        }

        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        let mut msg_env = OwnedEnv::new();
        let result =
            msg_env.send_and_clear(&self.pid, |env| crate::atoms::message_flush().encode(env));

        if result.is_err() {
            return Poll::Pending;
        }

        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        let mut msg_env = OwnedEnv::new();
        let result = msg_env.send_and_clear(&self.pid, |env| {
            crate::atoms::message_shutdown().encode(env)
        });

        if result.is_err() {
            return Poll::Pending;
        }

        Poll::Ready(Ok(()))
    }
}
