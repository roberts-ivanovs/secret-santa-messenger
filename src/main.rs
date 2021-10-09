use std::collections::HashMap;

use aws_sdk_ses::Blob;
use aws_sdk_sns::{self, model::MessageAttributeValue, output::PublishOutput};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct PhoneNumber(String);

#[derive(Serialize, Deserialize, Clone)]
struct Name(String);

#[derive(Serialize, Deserialize, Clone)]
struct Entry {
    number: PhoneNumber,
    name: Name,
}

impl Entry {
    fn new(number: String, name: String) -> Self {
        Self {
            number: PhoneNumber(number),
            name: Name(name),
        }
    }
}

#[tokio::main]
async fn main() {
    let items = vec![
        Entry::new(
            "+37122222222".to_owned(),
            "Template".to_owned(),
        ),
        Entry::new(
            "+37122222222".to_owned(),
            "Template".to_owned(),
        ),
        Entry::new(
            "+37122222222".to_owned(),
            "Template".to_owned(),
        ),
        Entry::new(
            "+37122222222".to_owned(),
            "Template".to_owned(),
        ),
        Entry::new(
            "+37122222222".to_owned(),
            "Template".to_owned(),
        ),
        Entry::new(
            "+37122222222".to_owned(),
            "Template".to_owned(),
        ),
    ];
    let mut rng = rand::thread_rng();
    let shuffled_items = {
        let mut items = items.clone();
        items.shuffle(&mut rng);
        items
    };

    let items = [
        (shuffled_items.get(0).unwrap(), shuffled_items.get(5).unwrap()),
        (shuffled_items.get(1).unwrap(), shuffled_items.get(4).unwrap()),
        (shuffled_items.get(2).unwrap(), shuffled_items.get(3).unwrap()),
        (shuffled_items.get(3).unwrap(), shuffled_items.get(1).unwrap()),
        (shuffled_items.get(4).unwrap(), shuffled_items.get(0).unwrap()),
        (shuffled_items.get(5).unwrap(), shuffled_items.get(2).unwrap()),
    ];

    let shared_config = aws_config::load_from_env().await;
    let client = ClientWrapper(aws_sdk_sns::Client::new(&shared_config));
    let res = items.iter()
        .map(|(recipient, target)| {
            let msg = format!(
                "Hi, {}! You're Secret Santa this year for: {}",
                recipient.name.0, target.name.0
            );
            let res = client.construct_and_send(&recipient.number.0, msg);
            res
        })
        .collect::<Vec<_>>();
    let res = futures::future::join_all(res).await;
    println!("{:#?}", res);
}

struct ClientWrapper(aws_sdk_sns::Client);

impl ClientWrapper {
    async fn construct_and_send(&self, to: &str, message: String) -> PublishOutput {
        // NOTE: SenderID is broken!
        // let name = MessageAttributeValue::builder()
        //     .data_type("String")
        //     .string_value("Roberts")
        //     .binary_value(Blob::new("Roberts".as_bytes()))
        //     .build();
        // let mut hm = HashMap::new();
        // hm.insert(
        //     "AWS.SNS.SMS.SenderID".to_owned(),
        //     name.clone()
        // );

        let res = self
            .0
            .publish()
            .phone_number(to)
            .message(message)
            // .message_attributes("AWS.SNS.SMS.SenderID".to_owned(), name)
            // .set_subject(Some("Robertss".to_owned()))
            // .set_message_attributes(Some(hm))
            .send()
            .await
            .unwrap();

        res
    }
}
