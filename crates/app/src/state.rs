use crate::services::user_service::UserService;

use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
}
