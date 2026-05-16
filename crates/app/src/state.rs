use crate::services::user_service::UserService;
use breachlet_domain::user::repo::UserRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState<U>
where
    U: UserRepository + 'static,
{
    pub user_service: Arc<UserService<U>>,
}
