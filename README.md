# net

A simple Rust reimplementation fundamental networking commands for learning purposes.

## Overview

`net` is a project designed to delve deep into the intricacies of network programming using Rust. As of now, it encapsulates the functionality of the `ping` command, sending ICMP echo requests to network hosts.

This project also involves calling C functions directly from Rust, which is interesting for learning purpose.

### Future Plans:
- Introduce the `traceroute` command.
- Incorporate functionalities resembling the `nmap` command.

## Installation

1. Clone the repository:
    ```bash
    git clone https://github.com/St0rmPetrel/net
    cd net
    ```

2. Build the project:
    ```bash
    cargo build --release
    ```

3. The executable will be available at `target/release/net`.

## Usage

To use `net`, run:

```bash
sudo ./target/release/net ping [HOSTNAME_OR_IP]
```

Replace `[HOSTNAME_OR_IP]` with the desired target, e.g., `google.com` or `8.8.8.8`.

## Example

```bash
sudo ./target/release/net ping google.com
```

```txt
PING google.com (74.125.130.102): 56 data bytes
64 bytes from 74.125.130.102: icmp_seq=0 time=69 ms
64 bytes from 74.125.130.102: icmp_seq=1 time=69 ms
64 bytes from 74.125.130.102: icmp_seq=2 time=70 ms
64 bytes from 74.125.130.102: icmp_seq=3 time=66 ms
64 bytes from 74.125.130.102: icmp_seq=4 time=69 ms
64 bytes from 74.125.130.102: icmp_seq=5 time=69 ms
^C
--- google.com ping statistics ---
6 packets transmitted, 6 packets received, 0.00 packet loss
round-trip min/avg/max/stddev = 66/68/70/1.41 ms
```

## Required Permissions

To run `net`, you will need elevated privileges (e.g., `sudo`) because it utilizes raw sockets. Raw sockets provide more control over network communication compared to higher-level socket types. Specifically, they allow manual crafting of packets, which is essential for ICMP echo requests used in the `ping` command.

### Why does the standard `ping` command not require `sudo`?

The standard `ping` command typically resides in a directory such as `/bin` or `/sbin` and is set with the setuid (Set User ID) bit. This means the `ping` command will always run with the file owner's permissions (usually root) regardless of who runs it. Thus, while the command can be executed by any user, it still has the necessary permissions to open raw sockets, negating the need to explicitly use `sudo`.


## Features

- `ping`: Sends ICMP echo requests akin to the standard `ping` command.
