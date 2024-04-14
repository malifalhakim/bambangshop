use dashmap::Dashmap;
use lazy_static::lazy_static;
use create::model::subscriber::Subscriber;

use crate::model::subscriber;

// Singleton of database
lazy_static! {
    static ref SUBSCRIBER: Dashmap<String, Dashmap<String, Subscriber>> = Dashmap::new();
}

pub struct SubscriberRepository;

impl SubscriberRepository {
    pub fn add(product_type: &str, subscriber: Subscriber) -> Subscriber {
        let subscriber_value = subscriber.clone();
        if SUBSCRIBER.get(product_type).is_none() {
            SUBSCRIBER.insert(String::from(product_type),Dashmap::new());
        };

        SUBSCRIBER.get(product_type).unwrap()
            .insert(subscriber_value.url.clone(), subscriber_value);
        return subscriber;
    }
}

