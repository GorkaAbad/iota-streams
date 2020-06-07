use iota_streams::app_channels::api::tangle::{Author, Transport, Address};
use failure::Fallible;

// ? is for error handling
pub fn start_a_new_channel<T: Transport>(author: &mut Author, client: &mut T, send_opt: T::SendOptions) -> Fallible<Address> {
    //Send the Announce message
    let announcement = author.announce()?;
    client.send_message_with_options(&announcement, send_opt)?;
    println!("Channel published");

    Ok(announcement.link)
}
