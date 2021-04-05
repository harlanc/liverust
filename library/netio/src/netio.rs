use super::netio_errors::{NetIOError, NetIOErrorValue};

use bytes::Bytes;
use bytes::BytesMut;

use std::time::Duration;

use tokio::net::TcpStream;

use tokio_stream::StreamExt;

use futures::SinkExt;
use tokio_util::codec::BytesCodec;
use tokio_util::codec::Framed;

pub struct NetworkIO {
    stream: Framed<TcpStream, BytesCodec>,

    //timeout: Duration,
}

impl NetworkIO {
    pub fn new(stream: TcpStream, ms: Duration) -> Self {
        Self {
            stream: Framed::new(stream, BytesCodec::new()),
           // timeout: ms,
        }
    }

    pub async fn write(&mut self, bytes: Bytes) -> Result<(), NetIOError> {
        self.stream.send(bytes).await?;
        Ok(())
    }

    pub async fn read(&mut self) -> Result<BytesMut, NetIOError> {
        let message = self.stream.next().await;

        match message {
            Some(data) => match data {
                Ok(bytes) => {
                    // for k in bytes.clone(){
                    //     print!("{:02X} ",k);
                    // }
                    // print!("\n");
                    // print!("\n");
                    return Ok(bytes);
                }
                Err(err) => {
                    return Err(NetIOError {
                        value: NetIOErrorValue::IOError(err),
                    })
                }
            },
            None => {
                return Err(NetIOError {
                    value: NetIOErrorValue::NoneReturn,
                })
            }
        }

        // let data = self.framed_read.next().await;

        // match data {
        //     Some(result) => match result {
        //         Ok(bytes) => {
        //             return Ok(bytes);
        //         }
        //         Err(err) => {
        //             return Err(NetIOError {
        //                 value: NetIOErrorValue::IOError(err),
        //             })
        //         }
        //     },
        //     None => {
        //         return Err(NetIOError {
        //             value: NetIOErrorValue::NoneReturn,
        //         })
        //     }
        // }
    }
}
