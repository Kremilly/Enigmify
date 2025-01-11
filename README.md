# Enigma Simulator in Rust

This project is a simple implementation of the Enigma machine in Rust. It allows users to input a message, configure the rotor and reflector settings, and encrypt or decrypt the message.

## Installation

Ensure you have Rust installed on your system. If not, install it using [rustup](https://rustup.rs/):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installing Rust, install the crate:

```sh
cargo install enigmify
```

## Usage

Run the program:

```sh
enigmify
```

The program will prompt you to enter:

1. The message to encrypt.
2. The wiring configuration for rotor 1.
3. The wiring configuration for rotor 2.
4. The wiring configuration for the reflector.

Example input:
```
Enter the message to encrypt:
HELLO
Enter rotor 1 wiring:
EKMFLGDQVZNTOWYHXUSPAIBRCJ
Enter rotor 2 wiring:
AJDKSIRUXBLHWTMCQGZNPYFVOE
Enter reflector wiring:
YRUHQSLDPXNGOKMIEBFZCWVJAT
```

Example output:
```
Encrypted: ZEBBW
Decrypted: HELLO
```

## How It Works

- The **rotors** shift characters based on predefined wiring and rotate with each encryption step.
- The **reflector** substitutes letters in a symmetric manner.
- The encryption process is reversible, meaning decrypting an encrypted message with the same settings will return the original message.

## License

This project is licensed under the MIT License.

