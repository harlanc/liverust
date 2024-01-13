use super::errors::Gb28181Error;
use super::session::GB28181ServerSession;
use std::collections::HashMap;
use streamhub::define::StreamHubEventSender;
use tokio::sync::mpsc::UnboundedSender;

pub type SessionExistSender = UnboundedSender<()>;

pub struct GB28181Server {
    event_producer: StreamHubEventSender,
    session_name_2_exist_sender: HashMap<String, SessionExistSender>,
}

impl GB28181Server {
    pub fn new(event_producer: StreamHubEventSender) -> Self {
        Self {
            event_producer,
            session_name_2_exist_sender: HashMap::new(),
        }
    }

    pub async fn start_session(
        &mut self,
        local_port: u16,
        stream_name: String,
        need_dump: bool,
    ) -> Result<u16, Gb28181Error> {
        if self.session_name_2_exist_sender.contains_key(&stream_name) {
            return Err(Gb28181Error {
                value: crate::errors::Gb28181ErrorValue::SessionExists,
            });
        }

        if let Some(mut session) = GB28181ServerSession::new(
            local_port,
            self.event_producer.clone(),
            stream_name.clone(),
            need_dump,
        )
        .await
        {
            let local_port = session.local_port;
            let exist_sender = session.exit_sender.clone();

            log::info!("GB28181 server session listening on udp://{}", local_port);
            tokio::spawn(async move {
                if let Err(err) = session.run().await {
                    log::error!("session run error, err: {}", err);
                }
            });

            self.session_name_2_exist_sender
                .insert(stream_name, exist_sender);
            return Ok(local_port);
        }

        Err(Gb28181Error {
            value: crate::errors::Gb28181ErrorValue::NewSessionFailed,
        })
    }

    pub async fn stop_session(&mut self, stream_name: String) {
        if let Some(exist_sender) = self.session_name_2_exist_sender.get_mut(&stream_name) {
            if let Err(err) = exist_sender.send(()) {
                log::error!("exist sender send error: {:?}", err);
            }
            self.session_name_2_exist_sender.remove(&stream_name);
        } else {
            log::warn!("The session with stream name: {stream_name} does not exist.")
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn send_dump_file() {
        // let file_path = ""; // 替换为实际的文件路径
        // let mut file = File::open(file_path).unwrap();

        // // 创建 UDP socket
        // let socket = UdpSocket::bind("127.0.0.1:0").unwrap(); // 绑定到任意可用端口

        // loop {
        //     // let time_delta = match file.read_u16::<BigEndian>() {
        //     //     Ok(value) => value,
        //     //     Err(err) => {
        //     //         log::error!("file read error: {}", err);
        //     //         break;
        //     //     } // 文件已读取完毕或发生错误
        //     // };
        //     // sleep(Duration::from_millis(time_delta as u64));

        //     // 读取 10 个字节
        //     // 读取 4 个字节作为大端 u32
        //     let length = match file.read_u16::<BigEndian>() {
        //         Ok(value) => value,
        //         Err(err) => {
        //             log::error!("file read error: {}", err);
        //             break;
        //         } // 文件已读取完毕或发生错误
        //     };
        //     println!("length:{}", length);

        //     // 读取指定长度的字节
        //     let mut buffer = vec![0u8; length as usize];
        //     if let Err(err) = file.read_exact(&mut buffer) {
        //         log::error!("read file err: {err}");
        //     }

        //     // 发送数据到 UDP 端口
        //     let addr = "127.0.0.1:30000"; // UDP 目标地址
        //     let _sent_bytes = socket.send_to(&buffer, addr).unwrap();
        //     //  println!("Sent {} bytes to {}: {:?}", sent_bytes, addr, buffer);
        //     thread::sleep(Duration::from_millis(2));
        // }
    }
}
