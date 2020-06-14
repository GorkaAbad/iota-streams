#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use iota_streams::app_channels::{
    api::tangle::{Address, Author, Transport}
    , message
};
use failure::{Fallible, ensure, bail};
use iota_conversion::trytes_converter::{to_string as trytes_to_string, to_trytes};


pub fn get_tagged_message<T: Transport>(author: &mut Author, channel_address: &String, tagged_message_identifier: &String, client: &mut T, recv_opt: T::RecvOptions) -> Fallible<()> {

    // Convert the channel address and message identifier to a link
    let message_link = match Address::from_str(&channel_address, &tagged_message_identifier){
        Ok(message_link) => message_link,
        Err(()) => bail!("Failed to create Address from {}:{}", &channel_address, &tagged_message_identifier),
    };

    println!("Receiving signed messages");

    // Use the IOTA client to find transactions with the corresponding channel address and tag
    let list = client.recv_messages_with_options(&message_link, recv_opt)?;

    // Iterate through all the transactions and stop at the first valid message
    for tx in list.iter() {
        let header = tx.parse_header()?;
        ensure!(header.check_content_type(message::tagged_packet::TYPE));
        let (public_payload, masked_payload) = author.unwrap_tagged_packet(header.clone())?;
        println!("Found and verified messages");
        println!("Public message: {}, private message: {}", public_payload, trytes_to_string(&masked_payload.to_string()).unwrap());
        break;
    }
    Ok(())
}
