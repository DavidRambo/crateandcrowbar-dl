# Crate and Crowbar Episode Downloader

This Rust crate downloads episodes of The Crate and Crowbar podcast from the download links provided on their website: [crateandcrowbar.com](crateandcrowbar.com).
I wrote this because podcast archives on, say, Apple Podcasts, only have the most recent 300 episodes available for a podcast, and I wanted to listen to older episodes.

## Usage

Since the number of episodes you want will likely differ, I'm not releasing compiled binaries.
Instead, get Rust on your system and clone the repository.

There are a few values to set in the `main` function.

1. First, decide where you want to save the files and assign the absolute path to the `dl_dir` name.
2. Then change the `first_ep_no` and `last_ep_no` values to select which episodes to download.
3. Finally, change the `threads` value for how many threads you want to run. I have a 6C/12HT CPU, so I just settled for four. Additionally, since many of the later episodes (starting in the late 70s) are hosted on Tom Francis's web server, I did not want to slam it.

### Sample Output

```
Attempting to download Crate and Crowbar episode 84...
Attempting to download Crate and Crowbar episode 82...
Attempting to download Crate and Crowbar episode 85...
Attempting to download Crate and Crowbar episode 83...
AWS did not have episode 85, trying Tom F's web server...
AWS did not have episode 84, trying Tom F's web server...
Downloaded episode 84. File size: 36624256
Zero-padding episode 84 did not work, trying without...
Downloaded episode 85. File size: 76080457
Zero-padding episode 85 did not work, trying without...
Downloaded episode 82. File size: 41526260
Downloaded episode 83. File size: 83334130
Attempting to download Crate and Crowbar episode 86...
Attempting to download Crate and Crowbar episode 87...
Attempting to download Crate and Crowbar episode 88...
Attempting to download Crate and Crowbar episode 89...
AWS did not have episode 87, trying Tom F's web server...
AWS did not have episode 86, trying Tom F's web server...
AWS did not have episode 88, trying Tom F's web server...
Downloaded episode 88. File size: 47090618
Zero-padding episode 88 did not work, trying without...
Downloaded episode 87. File size: 45022660
Zero-padding episode 87 did not work, trying without...
Downloaded episode 86. File size: 133615064
Zero-padding episode 86 did not work, trying without...
Downloaded episode 89. File size: 55931276
******** All done! ********
```

## Organization/Explanation

There are three different functions that format a URI from which to download an episode:

- `format_aws_uri`
- `format_pentadact_uri_with_zero`
- `format_pentadact_uri_no_zero`

The earliest episodes are hosted on AWS.
Then, starting at episode 76, they are mostly hosted on [Tom Francis's website](https://www.pentadact.com).
At least two episodes (77 and 82) are linked to AWS on the crateandcrowbar.com (the "Download" link beneath the embedded media player).
At least two episodes are available on AWS but not linked to from the podcast website (107 and 108).

The URI looks like either:

"https://s3-eu-west-1.amazonaws.com/crateandcrowbar/episodes/CCEp001.mp3"

or:

"https://www.pentadact.com/podcast/CCEp079.mp3"

or:

"https://www.pentadact.com/podcast/CCEp78.mp3"

Unfortunately, there isn't a clear cut-off.
The zero prefix seems to be the standard.
But once ep 100 is hit, it stops being a concern.

Hence the use of multiple request attempts.

# Improvements

Currently, a fresh thread is created for each download.
Instead, I'd like to use a thread pool.
