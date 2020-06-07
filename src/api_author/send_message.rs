use iota_streams::app_channels::api::tangle::{Author, Address, Transport};
use iota_streams::protobuf3::types::Trytes;
use iota_streams::core::tbits::Tbits;
use std::str::FromStr;
use failure::{Fallible, bail};

pub fn send_signed_message<T: Transport>(author: &mut Author, channel_address: &String, announce_message_identifier: &String, public_payload: &String, client: &mut T, send_opt: T::SendOptions ) -> Fallible<Address> {
    let public_payload = Trytes(Tbits::from_str(&public_payload).unwrap());

    //In the from parameter, is defined to whos packet is going to be linked to
    let announcement_link = match Address::from_str(&channel_address, &announce_message_identifier){
        Ok(announcement_link) => announcement_link,
        Err(()) => bail!("Failed to create Address from {}:{}", &channel_address, &announce_message_identifier),
    };

    //Sending a signed packet. The message if linked to the Annnounce won't be masked
    let empty_masked_payload = Trytes(Tbits::from_str("").unwrap());
    let message = author.sign_packet(&announcement_link, &public_payload, &empty_masked_payload)?;
    println!("Sending signed message");

    client.send_message_with_options(&message, send_opt)?;
    println!("Published signed message");

    Ok(message.link)
}
