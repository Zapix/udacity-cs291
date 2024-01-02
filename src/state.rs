use std::collections::{ HashMap };
use std::error::Error;
use std::rc::Rc;

use web_sys::{console};
use crate::common::traits::UnitTrait;
use crate::unit0;
use crate::unit2;

pub struct State {
    state: Option<String>,
    subscribed: Box<HashMap<String, Box<dyn Fn(Rc<Box<dyn UnitTrait>>)>>>,
    _units: Rc<Vec<Rc<Box<dyn UnitTrait>>>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            state: None,
            subscribed: Box::new(HashMap::new()),
            _units: Rc::new(
                unit0::get_units().iter()
                    .chain(
                        unit2::get_units().iter()
                    )
                    .map(|x|(*x).clone())
                    .collect::<Vec<Rc<Box<dyn UnitTrait>>>>()
            ),
        }
    }

    pub fn units(&self) -> Rc<Vec<Rc<Box<dyn UnitTrait>>>> {
        self._units.clone()
    }
}

pub trait Observer {
    fn value(&self) -> Option<&str>;
    fn set_value(&mut self, value: &str);

    fn subscribe(&mut self, subscriber_id: &str, notifier: Box<dyn Fn(Rc<Box<dyn UnitTrait>>)>) -> Result<(), Box<dyn Error>>;

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

        match self._units.iter().find(|x| *x.identifier() == String::from(value)) {
            Some(unit) => {
                for (key, notify) in (*self.subscribed).iter() {
                    console::log_1(
                        &format!(
                            "Send new value {} to {}",
                            unit.identifier().as_str(),
                            key.as_str()
                        ).as_str().into()
                    );

                    notify(unit.clone());
                }
            },
            None => return()
        }
    }

    fn subscribe(&mut self, subscriber_id: &str, notifier: Box<dyn Fn(Rc<Box<dyn UnitTrait>>)>) -> Result<(), Box<dyn Error>> {
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