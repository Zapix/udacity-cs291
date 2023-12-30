use std::collections::{ HashMap };
use std::error::Error;

use web_sys::{console};

pub struct State {
    state: Option<String>,
    subscribed: Box<HashMap<String, Box<dyn Fn(&str)>>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            state: None,
            subscribed: Box::new(HashMap::new()),
        }
    }
}

pub trait Observer {
    fn value(&self) -> Option<&str>;
    fn set_value(&mut self, value: &str);

    fn subscribe(&mut self, subscriber_id: &str, notifier: Box<dyn Fn(&str)>) -> Result<(), Box<dyn Error>>;

    fn unsubscribe(&mut self, subscriber_id: &str) -> Result<(), Box<dyn Error>>;
}

impl Observer for State {
    fn value(&self) -> Option<&str> {
        match &self.state {
            Some(value) => Some(&*value.as_str()),
            None => None
        }
    }

    fn set_value(&mut self, value: &str) {
        if value.is_empty() {
            return;
        }
        console::log_1(&format!("Set value {}", value).as_str().into());
        self.state = Some(String::from(value));
        for (key, notify) in (*self.subscribed).iter() {
            console::log_1(
                &format!(
                    "Send new value {} to {}",
                    value,
                    key.as_str()
                ).as_str().into()
            );

            notify(value);
        }
    }

    fn subscribe(&mut self, subscriber_id: &str, notifier: Box<dyn Fn(&str)>) -> Result<(), Box<dyn Error>> {
        if !self.subscribed.contains_key(subscriber_id) {
            self.subscribed.insert(String::from(subscriber_id), notifier);
            console::log_1(&format!("{} subscribed to state", subscriber_id).as_str().into());
            Ok(())
        } else {
            Err(format!("Subscriber with id {} already exists", subscriber_id).as_str().into())
        }
    }

    fn unsubscribe(&mut self, subscriber_id: &str) -> Result<(), Box<dyn Error>> {
        if self.subscribed.contains_key(subscriber_id) {
            Ok(())
        } else {
            Err(format!("Subscriber with id {} is not registered", subscriber_id).as_str().into())
        }
    }
}