use iota_streams::app_channels::api::tangle::Author;
use iota_lib_rs::prelude::iota_client;
use iota_streams::app::transport::tangle::client::SendTrytesOptions;
mod api_author;
use crate::api_author::announce::start_a_new_channel;
use crate::api_author::get_subscribers::get_subscriptions_and_share_keyload;
use crate::api_author::get_messages::get_tagged_message;


 fn main() {
    let mut client = iota_client::Client::new("https://nodes.devnet.iota.org:443");

    let mut send_opt = SendTrytesOptions::default();
    send_opt.min_weight_magnitude = 9;
    send_opt.local_pow = false; //IMPORTANT

    //Ad random seed generation
    let mut author = Author::new("SHALKHDEIULSDADAAERAFHSQWSJFHSKEUFHEESDDAADDJFHSFKDDDSHADF", 3, true);

    let channel_address = author.channel_address().to_string();

    let announce_message = start_a_new_channel(&mut author, &mut client, send_opt).unwrap();

    println!("");
    println!("Now, in a new terminal window, use the subscriber to publish a `Subscribe` message on the channel");
    println!("");
    println!("cargo run --release --bin subscriber {} {}", channel_address, announce_message.msgid);
    println!("");

    let mut subscribe_message_identifier = String::new();
    println!("Enter the message identifier of the `Subscribe` message that was published by the subscriber:");
    std::io::stdin().read_line(&mut subscribe_message_identifier).unwrap();

    if subscribe_message_identifier.ends_with('\n') {
        subscribe_message_identifier.pop();
    }
    if subscribe_message_identifier.ends_with('\r') {
        subscribe_message_identifier.pop();
    }

    let recv_opt = ();
    let keyload_message = get_subscriptions_and_share_keyload(&mut author, &channel_address, &mut subscribe_message_identifier, &mut client, send_opt, recv_opt).unwrap();

    println!("Paste this `Keyload` message identifier in the subscriber's command prompt: {}", keyload_message.msgid);

    loop {
        let mut tagged_message_identifier = String::new();
        println!("Enter the Taggedmessage identifier that was published by the subscriber:");
        std::io::stdin().read_line(&mut tagged_message_identifier).unwrap();

        if tagged_message_identifier.ends_with('\n') {
            tagged_message_identifier.pop();
        }
        if tagged_message_identifier.ends_with('\r') {
            tagged_message_identifier.pop();
        }

        match get_tagged_message(&mut author, &channel_address.to_string(), &tagged_message_identifier.to_string(), &mut client, recv_opt){
            Ok(()) => (),
            Err(error) => println!("Failed with error {}", error),
        }
    }
}
