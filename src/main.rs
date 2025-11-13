//! Downloads episodes of The Crate and Crowbar podcast from their website.
//! Since podcast catchers provide only the most recent 300 episodes, this is
//! intended to get earlier episodes. I don't know sort of loads their website
//! expects, so I opted to do it simply and synchronously with a five-second
//! sleep after every five downloads.
//!
//! Change the first_ep_no and last_ep_no values to select which episodes to download.
use std::fs::File;

use reqwest;

fn format_uri(ep_no: u8) -> String {
    let uri_base = "http://s3-eu-west-1.amazonaws.com/crateandcrowbar/episodes/CCEp";
    // Between these two strings goes the three-digit, left-padded with zeroes episode number.
    // For example: "001" or "122"
    let uri_ext = ".mp3?_=1";

    format!("{uri_base}{:<03}{uri_ext}", ep_no)
}

fn main() {
    std::env::set_current_dir("/var/home/david/crate_and_crowbar").expect("Change PWD");

    let client = reqwest::blocking::Client::new();

    let pause = std::time::Duration::new(5, 0);

    let first_ep_no = 61;
    let last_ep_no = 120;

    for episode_no in first_ep_no..=last_ep_no {
        let uri_full = format_uri(episode_no);

        let fpath = format!("CC{episode_no}.mp3");
        let mut pod_file = File::create(fpath).expect("Create file for episode download");

        println!("Attempting to download Crate and Crowbar episode {episode_no}...");

        let Ok(res) = client.get(uri_full).send().unwrap().copy_to(&mut pod_file) else {
            eprintln!("Failed to download episode #{episode_no}");
            return;
        };

        println!("Downloaded episode #{episode_no}. File size: {res}");

        if episode_no % 5 == 0 && episode_no != last_ep_no {
            println!("\n>>> Pausing for five seconds...\n");
            std::thread::sleep(pause);
        }
    }

    println!("******** All done! ********");
}

#[cfg(test)]
mod tests {
    use crate::format_uri;

    #[test]
    fn create_single_digit_episode_uri() {
        let expected = "http://s3-eu-west-1.amazonaws.com/crateandcrowbar/episodes/CCEp001.mp3?_=1";
        assert_eq!(format_uri(1_u8), expected);
    }

    #[test]
    fn create_double_digit_episode_uri() {
        let expected = "http://s3-eu-west-1.amazonaws.com/crateandcrowbar/episodes/CCEp021.mp3?_=1";
        assert_eq!(format_uri(21_u8), expected);
    }

    #[test]
    fn create_triple_digit_episode_uri() {
        let expected = "http://s3-eu-west-1.amazonaws.com/crateandcrowbar/episodes/CCEp121.mp3?_=1";
        assert_eq!(format_uri(121_u8), expected);
    }
}
