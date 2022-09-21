# shy🍂

> A command line remote controller for MusicBee!

requires the MusicBee web server plugin [mb_WWWserver](https://github.com/Cynosphere/mb_WWWserver) to be installed.

## Installation

```
cargo install --git https://github.com/x6r/shy
```

## Usage

```
$ shy --help
shy 0.3.6

USAGE:
    shy.exe [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    add           Add a track to queue [aliases: queue, a]
    help          Print this message or the help of the given subcommand(s)
    next          Play the next track in the queue [aliases: n]
    nowplaying    Print information about the current track [aliases: np]
    play          Play/pause the current track [aliases: pause, p]
    previous      Play the previous track in the queue [aliases: b]
    seek          Seek playback
    stop          Stop playback [aliases: s]
    volume        Modify player volume [aliases: vol, v]
```

Most subcommands have aliases which are the recommended way of usage.

> _`|` in the following examples represents or_

```sh
shy np # print now playing
shy v +10 | 80 | -40 # increase volume by 10 points | set volume to 80 | decrease volume by 40
shy seek 5 | 70% | -20% # seek 5 seconds | set position to 70% | go back 20%
shy add track.mp3 track.flac path/to/album # adds track.mp3, track.flac and every valid audio file in "album" to queue
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
- [ ] List playlist
- [x] Add to queue
- [ ] Album art to file

## License

[OSL-3.0](LICENSE)
