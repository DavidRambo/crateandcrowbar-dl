//! Downloads episodes of The Crate and Crowbar podcast from their website.
//! Since podcast catchers provide only the most recent 300 episodes, this is
//! intended to get earlier episodes. I don't know sort of loads their website
//! expects, so I opted to do it simply and synchronously with a five-second
//! sleep after every five downloads.
//!
//! Change the first_ep_no and last_ep_no values to select which episodes to download.
//!
//! A useful reference: https://github.com/Drew-Chase/parallel-downloads-with-events
use std::fs::File;

/// Represents an episode of the Crate and Crowbar.
///
/// The earliest episodes are hosted on AWS, while starting at episode 76, they
/// are hosted on Tom Francis's website.
///
/// The `uri` looks like either:
/// "http://s3-eu-west-1.amazonaws.com/crateandcrowbar/episodes/CCEp001.mp3?_=1"
/// or:
/// "https://www.pentadact.com/podcast/CCEp78.mp3"
/// or:
/// "https://www.pentadact.com/podcast/CCEp079.mp3?=_1"
///
/// Unfortunately, there isn't a clear cut-off.
///
/// The `ep_no` holds the episode number, which in this example is 1.
/// This allows for creating the file names for saving the data to disk.
#[derive(Clone)]
struct Episode {
    uri: String,
    ep_no: usize,
}

/// Given an episode number, returns the URI for downloading that episode.
fn format_aws_uri(ep_no: usize) -> String {
    let uri_base = "http://s3-eu-west-1.amazonaws.com/crateandcrowbar/episodes/CCEp";
    // Between these two strings goes the three-digit, left-padded with zeroes episode number.
    // For example: "001" or "122"
    let uri_ext = ".mp3?_=1";

    format!("{uri_base}{:<03}{uri_ext}", ep_no)
}

fn main() {
    std::env::set_current_dir("/var/home/david/crate_and_crowbar").expect("Change PWD");

    let first_ep_no = 61;
    let last_ep_no = 80;

    // Create a Vec of the file URIs (as Strings) to download.
    let episodes: Vec<Episode> = (first_ep_no..=last_ep_no)
        .map(|ep_no| Episode {
            uri: format_aws_uri(ep_no),
            ep_no,
        })
        .collect();

    let pause = std::time::Duration::new(5, 0);

    // Setup for threads.
    const THREADS: usize = 4;
    let episode_chunks = episodes.chunks(THREADS);
    let mut handles = Vec::with_capacity(THREADS);

    for chunk in episode_chunks {
        let chunk = chunk.to_vec();

        for episode in chunk {
            let handle = std::thread::spawn(move || {
                let fpath = format!("CC{}.mp3", episode.ep_no);
                let mut pod_file = File::create(fpath).expect("Create file for episode download");

                println!(
                    "Attempting to download Crate and Crowbar episode {}...",
                    episode.ep_no
                );

                let client = reqwest::blocking::Client::new();

                match client
                    .get(episode.uri)
                    .send()
                    .unwrap()
                    .copy_to(&mut pod_file)
                {
                    Ok(res) => println!("Downloaded episode #{}. File size: {res}", episode.ep_no),
                    Err(err) => {
                        eprintln!("Failed to download episode #{}", episode.ep_no);
                        eprintln!("Error: {err}");
                    }
                }
            });

            handles.push(handle);
        }

        for handle in handles.drain(..) {
            handle.join().unwrap();
        }

        println!("\n>>> Pausing for five seconds...\n");
        std::thread::sleep(pause);
    }

    println!("******** All done! ********");
}

#[cfg(test)]
mod tests {
    use crate::format_aws_uri;

    #[test]
    fn create_single_digit_episode_uri() {
        let expected = "http://s3-eu-west-1.amazonaws.com/crateandcrowbar/episodes/CCEp001.mp3?_=1";
        assert_eq!(format_aws_uri(1), expected);
    }

    #[test]
    fn create_double_digit_episode_uri() {
        let expected = "http://s3-eu-west-1.amazonaws.com/crateandcrowbar/episodes/CCEp021.mp3?_=1";
        assert_eq!(format_aws_uri(21), expected);
    }

    #[test]
    fn create_triple_digit_episode_uri() {
        let expected = "http://s3-eu-west-1.amazonaws.com/crateandcrowbar/episodes/CCEp121.mp3?_=1";
        assert_eq!(format_aws_uri(121), expected);
    }
}
