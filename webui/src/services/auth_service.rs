use std::future::Future;
use super::{request_post};
use crate::error::Error;
use crate::services::requests::{set_token};
use futures_signals::signal::{Mutable};
use futures_signals::signal::SignalExt;
use shared::model::{TokenResponse, UserCredential};

#[derive(Debug)]
pub struct AuthService {
    auth_channel: Mutable<bool>,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            auth_channel: Mutable::new(false),
        }
    }

    pub async fn auth_subscribe<F, U>(&self, callback: &mut F)
    where
        U: Future<Output = ()>,
        F: FnMut(bool) -> U
    {
        let fut = self.auth_channel.signal_cloned().for_each(callback);
        fut.await
    }

    pub fn logout(&self) {
        set_token(None);
        self.auth_channel.set(false);
    }

    pub async fn authenticate(&self, username: String, password: String) -> Result<TokenResponse, Error> {
        let credentials = UserCredential {
            username,
            password,
        };
        match request_post::<UserCredential, TokenResponse>("/auth/token", credentials).await {
            Ok(token) => {
                self.auth_channel.set(true);
                set_token(Some(&token.token));
                Ok(token)
            }
            Err(e) => {
                self.auth_channel.set(false);
                set_token(None);
                Err(e)
            }
        }
    }

    pub async fn refresh(&self) -> Result<TokenResponse, Error> {
        match request_post::<(), TokenResponse>("/auth/refresh", ()).await {
            Ok(token) => {
                self.auth_channel.set(true);
                set_token(Some(&token.token));
                Ok(token)
            }
            Err(e) => {
                self.auth_channel.set(false);
                set_token(None);
                Err(e)
            }
        }
    }

    // /// Get current user info
    // pub async fn get_current_user(&self) -> Result<UserCredentialWrapper, Error> {
    //     request_get_api::<UserCredentialWrapper>("/user".to_string()).await
    // }

    // /// Register a new user
    // pub async fn register(&self, register_info: UserCredentialWrapper) -> Result<UserCredentialWrapper, Error> {
    //     request_post::<UserCredentialWrapper, UserCredentialWrapper>("/users".to_string(), register_info).await
    // }
    //
    // /// Save info of current user
    // pub async fn save(&self, user_update_info: UserCredentialWrapper) -> Result<UserCredentialWrapper, Error> {
    //     request_put::<UserCredentialWrapper, UserCredentialWrapper>("/user".to_string(), user_update_info)
    //         .await
    // }
}
