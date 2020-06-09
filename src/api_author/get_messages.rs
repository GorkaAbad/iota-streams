#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use iota_streams::app_channels::{
    api::tangle::{Address, Author, Transport}
    , message
};
use failure::{Fallible, ensure, bail};


pub fn get_messages<T: Transport>(author: &mut Author, channel_address: &String, subscribe_message_identifier: &String, client: &mut T, keyload:  Address, announce:  Address,recv_opt: T::RecvOptions, recv_opt2: T::RecvOptions) -> Fallible<Address> {
    //
    // print!("1 {:?}",announce);
    // print!("2 {:?}",keyload);

     let announceMsg = client.recv_messages_with_options(&announce, recv_opt)?;
     let keyloadMsg = client.recv_messages_with_options(&keyload, recv_opt2)?;
    //
     println!("3 {:?}",announceMsg.len());
     println!("4 {:?}",keyloadMsg.len());

     println!("Announce",);
     for msg in announceMsg.iter() {
         let preparsed = msg.parse_header()?;
         println!("Content type {:?}", preparsed.content_type());
         if preparsed.check_content_type(message::tagged_packet::TYPE) {
             let rs = author.unwrap_tagged_packet(preparsed.clone());
             ensure!(rs.is_err());
             match rs {
                 Ok((unwrapped_public, unwrapped_masked)) => {
                     println!("Tagged Public Packet: {}", unwrapped_public.to_string());
                     println!("Tagged Masked Packet: {}", unwrapped_masked.to_string());
                 }
                 Err(e) => println!("Tagged Packet Error: {}", e),
             }
             continue;
         }

         if preparsed.check_content_type(message::subscribe::TYPE) {
             println!("subscribe", );
             let rs = author.unwrap_subscribe(preparsed.clone());
             ensure!(rs.is_err());
             match rs {
                 Ok(()) => {

                 }
                 Err(e) => println!("Tagged Packet Error: {}", e),
             }
             continue;
         }
         if preparsed.check_content_type(message::keyload::TYPE) {
             println!("Keyload", );
         }
         if preparsed.check_content_type(message::signed_packet::TYPE) {
             println!("Signed", );
         }
         if preparsed.check_content_type(message::announce::TYPE) {
             println!("Announce", );
         }
     }
     println!("Keyload", );
     for msg in keyloadMsg.iter() {
         let preparsed = msg.parse_header()?;

         if preparsed.check_content_type(message::tagged_packet::TYPE) {
             let rs = author.unwrap_tagged_packet(preparsed.clone());
             ensure!(rs.is_err());
             match rs {
                 Ok((unwrapped_public, unwrapped_masked)) => {
                     println!("Tagged Public Packet: {}", unwrapped_public.to_string());
                     println!("Tagged Masked Packet: {}", unwrapped_masked.to_string());
                 }
                 Err(e) => println!("Tagged Packet Error: {}", e),
             }
             continue;
         }
         if preparsed.check_content_type(message::subscribe::TYPE) {
             println!("suscribe", );
             let rs = author.unwrap_subscribe(preparsed.clone());
             ensure!(rs.is_err());
             match rs {
                 Ok(()) => {

                 }
                 Err(e) => println!("Tagged Packet Error: {}", e),
             }
             continue;
         }
         if preparsed.check_content_type(message::keyload::TYPE) {
             println!("Keyload", );
         }
         if preparsed.check_content_type(message::signed_packet::TYPE) {
             println!("Signed", );
         }
     }


    Ok(keyload)
}
