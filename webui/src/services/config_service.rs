use crate::services::websocket_service::WebsocketService;
use futures::Stream;
use log::info;
use std::future;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct ConfigService {
    websocket_service: Rc<WebsocketService>,
    ws_connection: Rc<AtomicBool>
}

impl ConfigService {
    pub fn new(websocket_service: Rc<WebsocketService>) -> Self {
        let ws_service = websocket_service.clone();
        let ws_connection = Rc::new(AtomicBool::new(false));
        let flag = Rc::clone(&ws_connection);
        wasm_bindgen_futures::spawn_local(async move {
            ws_service.connection_subscribe(&mut |connected| {
                flag.store(connected, Ordering::SeqCst);
                future::ready(())
            }).await;
        });
        Self { websocket_service, ws_connection }
    }

    pub fn ready(&self) -> bool {
        self.ws_connection.load(Ordering::SeqCst)
    }

    pub async fn get_config(&self, offset: u32, count: u32) -> impl Stream<Item=ModelDocument> {
        info!("Requesting config");
        self.websocket_service.get_config(offset, count).await
    }
}