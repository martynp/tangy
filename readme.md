# Tangy 

## Description

Tangy-lib is an implementation of Tang server written in rust.

The Tang protocol allows clients to store secrets which can only be recovered when they have access to the Tang server. For example, the Clevis tools allows the automated decryption of LUKS partitions when the encrypted device is connected to the local network that Tang is accessible on.

See the original Tang project for a complete description: https://github.com/latchset/tang

Fraser Tweedale's 2020 Linux Conference Australia talk on "Clevis and Tang: securing your secrets at rest" is a great resource:

[![Clevis and Tang: securing your secrets at rest](https://img.youtube.com/vi/Dk6ZuydQt9I/0.jpg)](https://www.youtube.com/watch?v=Dk6ZuydQt9I)


## Installation / Usage

Installation via cargo:

``` bash
cargo install tangy
tangy --help
tangy --dir /path/to/keys --port 8000 --address 0.0.0.0 &
```

Or use the docker version:

``` bash
docker run -it --restart=unless-stopped -p 8000:8000 -v /path/to/keys:/keys martynp/tangy:latest
```

## To / From Tangd

Tang and Tangy use the same .jwk file format for storing keys, technically pointing tang at tangy's keys or tangy at tangs keys should work. Please remember to back up any data before attempting anything!

## Credits

The original authors of [Tang](https://github.com/latchset/tang) are [Latchset](https://github.com/latchset/). Tang is based on the protocol described by Nathaniel McCallum and Robert Relyea (https://marc.info/?m=144173814525805).

## License

Apache-2.0 or MIT - you decide!

## How to Contribute

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed (Apache-2.0 and MIT), without any additional terms or conditions.