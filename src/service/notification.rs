use std::thread;

use bambangshop::{Result, compose_error_response};
use rocket::http::Status;
use crate::model::notification::Notificaiton;
use crate::model::product::Product;
use crate::model::subscriber::Subscriber;
use crate::repository::subscriber::SubscriberRepository;

pub struct NotificaitonService;

impl NotificaitonService {
    
}