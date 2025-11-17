//! Downloads episodes of The Crate and Crowbar podcast from their website.
//! Since podcast catchers provide only the most recent 300 episodes, this is
//! intended to get earlier episodes. I don't know sort of loads their website
//! expects, so I opted to do it simply and synchronously with a five-second
//! sleep after every five downloads.
//!
//! Change the `first_ep_no` and `last_ep_no` values to select which episodes to download.
//! Change the `threads` value for how many threads you want to run. I have a 6C/12HT CPU, so I
//! just settled for four. Additionally, since many of the later episodes (starting in the late 70s)
//! are hosted on Tom Francis's web server, I did not want to slam it.
//!
//! The earliest episodes are hosted on AWS. Then, starting at episode 76, they
//! are mostly hosted on Tom Francis's website. At least two episodes (77 and 82) are linked
//! to AWS on the crateandcrowbar.com (the "Download" link beneath the embedded media player).
//! At least two episodes are available on AWS but not linked to from the podcast website (107 and
//! 108).
//!
//! The `url` looks like either:
//! "https://s3-eu-west-1.amazonaws.com/crateandcrowbar/episodes/CCEp001.mp3"
//! or:
//! "https://www.pentadact.com/podcast/CCEp78.mp3"
//! or:
//! "https://www.pentadact.com/podcast/CCEp079.mp3"
//!
//! Unfortunately, there isn't a clear cut-off. The zero prefix seems to be the standard.
//! But once ep 100 is hit, it stops being a concern.
//!
//! Hence the use of multiple request attempts.
//!
//! A useful reference: https://github.com/Drew-Chase/parallel-downloads-with-events

use std::fs::File;

/// Given an episode number, returns the URL for downloading that episode from the AWS server.
fn format_aws_url(ep_no: usize) -> String {
    let url_base = "https://s3-eu-west-1.amazonaws.com/crateandcrowbar/episodes/CCEp";
    // Between these two strings goes the three-digit, left-padded with zeroes episode number.
    // For example: "001" or "122"
    let url_ext = ".mp3";

    format!("{url_base}{ep_no:<03}{url_ext}")
}

/// Given an episode number, returns the URL for downloading that episode from Tom F's website.
fn format_pentadact_url_with_zero(ep_no: usize) -> String {
    let url_base = "https://www.pentadact.com/podcast/CCEp";
    // Between these two strings goes the three-digit, left-padded with zeroes episode number.
    // For example: "084" or "122"
    let url_ext = ".mp3";

    format!("{url_base}{ep_no:<03}{url_ext}")
}

/// Given an episode number, returns the URL for downloading that episode from Tom F's website.
/// For example: "https://www.pentadact.com/podcast/CCEp78.mp3"
fn format_pentadact_url_no_zero(ep_no: usize) -> String {
    let url_base = "https://www.pentadact.com/podcast/CCEp";
    let url_ext = ".mp3";

    format!("{url_base}{ep_no}{url_ext}")
}

/// Attempts to download the episode mp3 and save it to the specified file.
/// Returns an Option of the number of bytes if successful, otherwise None.
fn download_ep(url: &str, pod_file: &mut File, ep_no: usize) -> Option<()> {
    if let Ok(mut res) = reqwest::blocking::get(url) {
        if res.status().is_success() {
            match res.copy_to(pod_file) {
                Ok(res) => {
                    println!("Downloaded episode {ep_no}. File size: {res}");
                    return Some(());
                }
                Err(err) => {
                    eprintln!("Failed to download episode {ep_no} at {url}");
                    eprintln!("Error: {err}");
                    return None;
                }
            };
        } else {
            return None;
        }
    }

    None
}

fn main() {
    // NOTE: Be sure to update the path for your system!
    // This is where downloaded files will be saved.
    // In this example, make sure the crate_and_crowbar directory already exists.
    let dl_dir = std::path::Path::new("/var/home/{YOUR_HOME_HERE}/crate_and_crowbar");
    if !dl_dir.exists() {
        eprintln!("Invalid download directory. Be sure to set `dl_dir` to a valid directory.");
        return;
    }
    std::env::set_current_dir(dl_dir).expect("Change PWD");

    // NOTE: Set the range of eposides to download here.
    let first_ep_no = 1;
    let last_ep_no = 100;
    // Create list of episode numbers.
    let episodes_vec = (first_ep_no..=last_ep_no).collect::<Vec<usize>>();

    // Setup for threads.
    // NOTE: Set the number to whatever makes sense for your system.
    let threads: usize = 4;
    let episode_chunks = episodes_vec.chunks(threads);
    let mut handles = Vec::with_capacity(threads);

    for chunk in episode_chunks {
        // Create a Vec of the chunk so its items can be moved into the spawned threads.
        let chunk = chunk.to_vec();

        for ep_no in chunk {
            let handle = std::thread::spawn(move || {
                let fpath = format!("CC{ep_no}.mp3");
                let mut pod_file = File::create(fpath).expect("Create file for episode download");

                println!("Attempting to download Crate and Crowbar episode {ep_no}...");

                // Attempt to download from the various URL possibilities.
                // If there were more than three URL formatting options, then I'd create an enum
                // of those options and iterate through them. But with only three, the nesting is
                // not terrible.
                // First try the AWS server.
                let episode_url = format_aws_url(ep_no);
                if download_ep(&episode_url, &mut pod_file, ep_no).is_none() {
                    // Now try Tom F's web server using zero padding.
                    println!("AWS did not have episode {ep_no}, trying Tom F's web server...");
                    let episode_url = format_pentadact_url_with_zero(ep_no);

                    if download_ep(&episode_url, &mut pod_file, ep_no).is_none() {
                        // Finally, try Tom F's web server without zero padding.
                        let episode_url = format_pentadact_url_no_zero(ep_no);
                        if download_ep(&episode_url, &mut pod_file, ep_no).is_none() {
                            eprintln!(">>> Failed to download episode {ep_no}");
                        }
                    }
                }
            });

            handles.push(handle);
        }

        for handle in handles.drain(..) {
            handle.join().unwrap();
        }
    }

    println!("******** All done! ********");
}

#[cfg(test)]
mod tests {
    use crate::{format_aws_url, format_pentadact_url_no_zero, format_pentadact_url_with_zero};

    #[test]
    fn create_single_digit_episode_url() {
        let expected = "https://s3-eu-west-1.amazonaws.com/crateandcrowbar/episodes/CCEp001.mp3";
        assert_eq!(format_aws_url(1), expected);
    }

    #[test]
    fn create_double_digit_episode_url() {
        let expected = "https://s3-eu-west-1.amazonaws.com/crateandcrowbar/episodes/CCEp021.mp3";
        assert_eq!(format_aws_url(21), expected);
    }

    #[test]
    fn create_triple_digit_episode_url() {
        let expected = "https://s3-eu-west-1.amazonaws.com/crateandcrowbar/episodes/CCEp121.mp3";
        assert_eq!(format_aws_url(121), expected);
    }

    #[test]
    fn create_pentadact_two_digit_with_zero_episode_url() {
        let expected = "https://www.pentadact.com/podcast/CCEp085.mp3";
        assert_eq!(format_pentadact_url_with_zero(85), expected);
    }

    #[test]
    fn create_pentadact_two_digit_no_zero_episode_url() {
        let expected = "https://www.pentadact.com/podcast/CCEp78.mp3";
        assert_eq!(format_pentadact_url_no_zero(78), expected);
    }
}
