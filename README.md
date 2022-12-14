# shy🍂

> A command line remote controller for MusicBee!

<!-- temporairly linked to my fork of a fork until Cynosphere/mb_WWWserver#1 gets merged (if ever) -->

requires the MusicBee web server plugin [mb_WWWserver](https://github.com/fawni/mb_WWWserver) to be installed.

## Installation

```
cargo install --git https://github.com/fawni/shy
```

## Usage

```
Usage: shy [COMMAND]

Commands:
  add         Add a track to queue [aliases: a]
  clear       Clear current queue [aliases: c]
  play        Play/pause the current track [aliases: pause, p]
  stop        Stop playback [aliases: s]
  next        Play the next track in the queue [aliases: n]
  previous    Play the previous track in the queue [aliases: prev, b]
  nowplaying  Print information about the current track [aliases: np]
  queue       List queued tracks [aliases: q]
  volume      Modify player volume [aliases: vol, v]
  seek        Seek playback
  shuffle     Change shuffle status
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

Most subcommands have aliases which are the recommended way of usage.

> ###### In the following examples, | represents or.

```sh
shy np # print now playing
shy add track.mp3 track.flac path/to/album # adds track.mp3 track.flac and valid audio files in album directory to queue
shy v +10 | 80 | -40 # increase volume by 10 points | set volume to 80 | decrease volume by 40
shy seek 5 | 70% | -20% # seek 5 seconds | set position to 70% | go back 20%
```

## Implemented

- [ ] Player commands
  - [x] Play/pause
  - [x] Next
  - [x] Previous
  - [x] Stop
  - [x] Seek
  - [x] Volume
  - [x] Shuffle
  - [ ] Repeat
  - [ ] Scrobble
- [x] Now playing
- [x] List queue
- [x] Add to queue
- [x] Clear queue
- [ ] Album art to file

## License

[OSL-3.0](LICENSE)
