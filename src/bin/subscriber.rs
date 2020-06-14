extern crate openweather;

use iota_streams::app_channels::{
    api::tangle::{Address, Transport, Subscriber}
    , message
};
use iota_lib_rs::prelude::iota_client;
use iota_streams::app::transport::tangle::client::SendTrytesOptions;
use failure::{Fallible, ensure, bail};
use std::env;
use iota_streams::protobuf3::types::Trytes;
use iota_streams::core::tbits::Tbits;
use std::str::FromStr;
use openweather::{LocationSpecifier, Settings};
use iota_conversion::trytes_converter::to_trytes;
use std::{thread, time};
static API_KEY: &str = "e85e0a3142231dab28a2611888e48f22";

fn get_signed_messages<T: Transport>(subscriber: &mut Subscriber, channel_address: &String, signed_message_identifier: &String, client: &mut T, recv_opt: T::RecvOptions) -> Fallible<()> {

    // Convert the channel address and message identifier to a link
    let message_link = match Address::from_str(&channel_address, &signed_message_identifier){
        Ok(message_link) => message_link,
        Err(()) => bail!("Failed to create Address from {}:{}", &channel_address, &signed_message_identifier),
    };

    println!("Receiving signed messages");

    // Use the IOTA client to find transactions with the corresponding channel address and tag
    let list = client.recv_messages_with_options(&message_link, recv_opt)?;

    // Iterate through all the transactions and stop at the first valid message
    for tx in list.iter() {
        let header = tx.parse_header()?;
        ensure!(header.check_content_type(message::signed_packet::TYPE));
        let (public_payload, masked_payload) = subscriber.unwrap_signed_packet(header.clone())?;
        println!("Found and verified messages");
        println!("Public message: {}, private message: {}", public_payload, masked_payload);
        break;
    }
    Ok(())
}

fn get_announcement<T: Transport>(subscriber: &mut Subscriber, channel_address: &String, announce_message_identifier: &String, client: &mut T, recv_opt: T::RecvOptions) -> Fallible<()> {

    // Convert the channel address and message identifier to a link
    let announcement_link = match Address::from_str(&channel_address, &announce_message_identifier){
        Ok(announcement_link) => announcement_link,
        Err(()) => bail!("Failed to create Address from {}:{}", &channel_address, &announce_message_identifier),
    };

    println!("Receiving announcement messages");

    // Use the IOTA client to find transactions with the corresponding channel address and tag
    let list = client.recv_messages_with_options(&announcement_link, recv_opt)?;
    // Iterate through all the transactions and stop at the first valid message
    for tx in list.iter() {
        let header = tx.parse_header()?;
        ensure!(header.check_content_type(message::announce::TYPE));
        subscriber.unwrap_announcement(header.clone())?;
        println!("Found and verified {} message", header.content_type());
        break;
    }
    Ok(())
}

fn get_keyload<T: Transport>(subscriber: &mut Subscriber, channel_address: &String, keyload_message_identifier: &String, client: &mut T, recv_opt: T::RecvOptions) -> Fallible<()> {

     // Convert the channel address and message identifier to an Address link type
     let keyload_link = match Address::from_str(&channel_address, &keyload_message_identifier) {
        Ok(keyload_link) => keyload_link,
        Err(()) => bail!("Failed to create Address from {}:{}", &channel_address, &keyload_message_identifier),
    };

    println!("Receiving keyload messages");

    // Use the IOTA client to find transactions with the corresponding channel address and tag
    let list = client.recv_messages_with_options(&keyload_link, recv_opt)?;

    // Iterate through all the transactions and stop at the first valid message
    for tx in list.iter() {
        let header = tx.parse_header()?;
        ensure!(header.check_content_type(message::keyload::TYPE));
        subscriber.unwrap_keyload(header.clone())?;
        println!("Found and verified {} message", header.content_type());
        break;
    }
    Ok(())
}


fn subscribe<T: Transport>(subscriber: &mut Subscriber, channel_address: &String, announce_message_identifier: &String, client: &mut T, send_opt: T::SendOptions) -> Fallible<()> {

     // Convert the channel address and message identifier to a link
     let announcement_link = match Address::from_str(&channel_address, &announce_message_identifier){
        Ok(announcement_link) => announcement_link,
        Err(()) => bail!("Failed to create Address from {}:{}", &channel_address, &announce_message_identifier),
    };

    println!("Subscribing to channel");

    // Send a `Subscribe` message and link it to the message identifier
    //of the first valid `Announce` message that was found on the Tangle
    let subscription = subscriber.subscribe(&announcement_link)?;
    client.send_message_with_options(&subscription, send_opt)?;
    println!("Published `Subscribe` message");
    println!("Paste this `Subscribe` message identifier into your author's command prompt  {}", subscription.link.msgid);
    Ok(())
 }

 pub fn send_masked_payload<T: Transport>(subscriber: &mut Subscriber, channel_address: &String, keyload_message_identifier: &String, public_payload: &String, masked_payload: &String, client: &mut T, send_opt: T::SendOptions ) -> Fallible<Address> {

     // Convert the payloads to a Trytes type
     let public_payload = Trytes(Tbits::from_str(&public_payload).unwrap());
     let masked_payload = Trytes(Tbits::from_str(&to_trytes(&masked_payload).unwrap()).unwrap());
     println!("Masked {:?}",&masked_payload );

     // Convert the channel address and message identifier to an Address link type
     let keyload_link = match Address::from_str(&channel_address, &keyload_message_identifier) {
         Ok(keyload_link) => keyload_link,
         Err(()) => bail!("Failed to create Address from {}:{}", &channel_address, &keyload_message_identifier),
     };

     // Create a `TaggedPacket` message and link it to the message identifier of the `Keyload` message
     // whose session key is used to encrypt the masked payload
     let message = subscriber.tag_packet(&keyload_link, &public_payload, &masked_payload)?;

     // Convert the message to a bundle and send it to a node
     client.send_message_with_options(&message, send_opt)?;
     println!("Published private payload");
     Ok(message.link)
 }

 fn get_temperature() -> i64 {
     let loc = LocationSpecifier::CityAndCountryName{city:"Madrid".to_string(), country:"ES".to_string()};
     let weather = openweather::get_current_weather(&loc, API_KEY, &Settings::default()).unwrap();
     let temp = (weather.main.temp - 273.15).round() as i64;
     println!("Current temperature: {}", temp);
     return temp;
 }

 fn main() {

    // Create a new subscriber
    //Add random seed generation
    let mut subscriber = Subscriber::new("MYSUBSCRIBERSECRETSTRING", true);

    // Connect to an IOTA node
    let mut client = iota_client::Client::new("https://nodes.devnet.iota.org:443");

    let args: Vec<String> = env::args().collect();

    let channel_address = &args[1];
    let announce_message_identifier = &args[2];

    let recv_opt = ();

    match get_announcement(&mut subscriber, &channel_address.to_string(), &announce_message_identifier.to_string(), &mut client, recv_opt){
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }

    // Change the default settings to use a lower minimum weight magnitude for the Devnet
    let mut send_opt = SendTrytesOptions::default();
    send_opt.min_weight_magnitude = 9;// default is 14
    send_opt.local_pow = false;

    match subscribe(&mut subscriber, &channel_address.to_string(), &announce_message_identifier.to_string(), &mut client, send_opt) {
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }

    let mut keyload_message_identifier = String::new();
    println!("Enter the Keyload message identifier that was published by the author:");
    std::io::stdin().read_line(&mut keyload_message_identifier).unwrap();

    if keyload_message_identifier.ends_with('\n') {
        keyload_message_identifier.pop();
    }
    if keyload_message_identifier.ends_with('\r') {
        keyload_message_identifier.pop();
    }

    match get_keyload(&mut subscriber, &channel_address.to_string(), &keyload_message_identifier.to_string(), &mut client, recv_opt){
       Ok(()) => (),
       Err(error) => println!("Failed with error {}", error),
   }

    let keyload_message = Address::from_str(&channel_address, &keyload_message_identifier).unwrap();

    let mut send_opt = SendTrytesOptions::default();
    send_opt.min_weight_magnitude = 9;
    send_opt.local_pow = false; //IMPORTANT

    let public_payload = "TEMPERATURE";

    loop {
        let masked_payload = get_temperature().to_string();

        let signed_private_message = send_masked_payload(&mut subscriber, &channel_address, &keyload_message.msgid.to_string(), &public_payload.to_string(), &masked_payload.to_string(), &mut client, send_opt).unwrap();

        println!("Paste this `Tagged` message identifier into your author's command prompt  {}", signed_private_message.msgid);

        let ten_millis = time::Duration::from_millis(10000);
        thread::sleep(ten_millis);
    }
 }
