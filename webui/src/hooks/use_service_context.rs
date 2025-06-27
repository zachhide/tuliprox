use std::rc::Rc;
use yew::prelude::*;
use crate::config::Config;
use crate::services::auth_service::AuthService;
use crate::services::config_service::ConfigService;
use crate::services::websocket_service::WebsocketService;

pub struct Services {
    pub config: Rc<Config>,
    pub auth: Rc<AuthService>,
    pub ws: Rc<WebsocketService>,
    pub document: Rc<ConfigService>,
}

impl Services {
    pub fn new(config: &Config) -> Self {
        let auth = Rc::new(AuthService::new());
        let ws = Rc::new(WebsocketService::new(config));
        let document = Rc::new(ConfigService::new(ws.clone()));
        Self {
            config: Rc::new(config.clone()),
            auth,
            ws,
            document
        }
    }
}

impl PartialEq for Services {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for Services {}

#[derive(PartialEq, Eq, Clone)]
pub struct ServiceContext {
    services: Rc<Services>,
}

impl ServiceContext {
    pub fn new(config: &Config) -> Self {
        Self {
            services: Rc::new(Services::new(config))
        }
    }

    pub fn services(&self) ->  Rc<Services> {
            self.services.clone()
    }
}

#[hook]
pub fn use_service_context() -> Rc<Services> {
    use_context::<UseStateHandle<ServiceContext>>().unwrap().services()
}