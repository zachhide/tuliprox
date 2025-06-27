use std::future::Future;
use std::rc::Rc;
use futures::lock::Mutex;
use futures::stream::{SplitSink, Stream, StreamExt};
use futures::SinkExt;
use futures_signals::signal::{Mutable};
use futures_signals::signal::SignalExt;
use log::{debug, error, info};
use prost::Message;
use reqwasm::websocket::{futures::WebSocket, Message as WasmMessage};
use crate::config::Config;
use shared::protocol_model;
use shared::protocol_model::Container;
use wasm_bindgen_futures::spawn_local;
use shared::model::protocol_message::create_document_request;

#[derive(Debug)]
enum ProtocolStep {
    Version,
    Default,
}

pub struct WebsocketService {
    connection_channel: Mutable<bool>,
    ws_url: String,
    protocol_version: u8,
    write: Rc<Mutex<Option<SplitSink<WebSocket, WasmMessage>>>>,
    tx_config: Rc<Mutex<Option<futures::channel::mpsc::UnboundedSender<ModelDocument>>>>,
}

impl WebsocketService {
    pub fn new(config: &Config) -> Self {
        Self {
            connection_channel: Mutable::new(false),
            ws_url: config.ws_url.to_string(),
            protocol_version: config.protocol_version,
            write: Rc::new(Mutex::new(None)),
            tx_config: Rc::new(Mutex::new(None)),
        }
    }

    pub async fn connection_subscribe<F, U>(&self, callback: &mut F)
    where
        U: Future<Output = ()>,
        F: FnMut(bool) -> U
    {
        let fut = self.connection_channel.signal_cloned().for_each(callback);
        fut.await
    }

    pub async fn get_documents(&self, offset: u32, count: u32) -> impl Stream<Item=ModelDocument> {
        let (in_tx, in_rx) = futures::channel::mpsc::unbounded::<ModelDocument>();
        *(self.tx_config.lock().await) = Some(in_tx);

        let mut lock = self.write.lock().await;
        let ws_write = lock.as_mut().unwrap();
        let request = create_document_request(offset, count);
        match ws_write.send(WasmMessage::Bytes(request.unwrap())).await {
            Err(e) => { error!("Document request failed {e}") }
            _ => {}
        };
        in_rx
    }

    pub async fn connect(&self) {
        match WebSocket::open(&self.ws_url) {
            Ok(ws) => {
                let (write, mut read) = ws.split();
                {
                    let mut lock = self.write.lock().await;
                    *lock = Some(write);
                }

                let protocol_version = self.protocol_version;
                let writer = self.write.clone();
                let tx_config = self.tx_config.clone();
                let con_chan = self.connection_channel.clone();
                spawn_local(async move {
                    let mut lock = writer.lock().await;
                    let ws_write = lock.as_mut().unwrap();
                    match ws_write.send(WasmMessage::Bytes(vec![protocol_version])).await {
                        Err(e) => { error!("Version mismatch {e}") }
                        _ => {
                            con_chan.set(true);
                        }
                    };

                    let mut protocol_step = ProtocolStep::Version;

                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(WasmMessage::Text(data)) => {
                                debug!("text from websocket: {}", data);
                            }
                            Ok(WasmMessage::Bytes(msg)) => {
                                match protocol_step {
                                    ProtocolStep::Version => {
                                        protocol_step = ProtocolStep::Default;
                                    }
                                    _ => {
                                        match Container::decode(&*msg) {
                                            Ok(container) => {
                                                for msg in container.message {
                                                    match msg.body {
                                                        Some(protocol_model::message::Body::DocumentResponse(response)) => {
                                                            match protocol_step {
                                                                ProtocolStep::Default => {
                                                                    let mut guard = tx_config.lock().await;
                                                                    let tx = guard.as_mut().unwrap();
                                                                    for doc in response.documents {
                                                                        tx.send(ModelDocument {
                                                                            path: doc.path,
                                                                            size: doc.size,
                                                                            created: doc.created,
                                                                        }).await.unwrap();
                                                                    }
                                                                }
                                                                _ => {}
                                                            }
                                                        }
                                                        _ => {
                                                            info!("other {:?}", msg);
                                                        }
                                                    }
                                                }
                                            }
                                            Err(err) => {
                                                error!("decode {err}")
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                error!("ws: {:?}", e)
                            }
                        }
                    }
                    log::debug!("WebSocket Closed");
                });
                // self.tx = Some(in_tx);
            }
            Err(e) => {
                self.connection_channel.set(false);
                error!("Failed to connect to websocket {e}")
            }
        }
    }
}