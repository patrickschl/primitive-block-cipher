Version 0.1.0

# primitive-block-cipher

`**Disclaimer:**`
- `**THIS IS NOT SECURE!**`
- `**DO NOT USE IN PRODUCTION!**`
- `**NEVER WRITE YOUR OWN CRYPTO!!!!1!!**`

## How to

This script has been written in Rust. In Order to compile it
yourself make sure rust is installed.

Install for Linux & MacOS:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

to verfiy if rust is properly install run these commands:

```sh
cargo –version
rustc –version
```

To install Rust on Windows follow the instructions from this
[Tutorial](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup?tabs=winget).

When Rust is installed clone the Repository, cd into the project
and run it with `cargo run`.

```sh
git clone https://github.com/patrickschl/primitive-block-cipher.git

cd primitive-block-cipher

cargo run
```

This project also provides executable releases for Windows and Linux.


## Theory and Motivation

This cipher was implemented for a university class in order
to demonstrate the basic functions of symmetric cryptography:

- division into blocks
- substitution
- transposition
- key integration

Modern crypto algorithms have the ambition to adhere to two 
principles defined by Claude Shannon in the 1940s:

### Confusion
Confusion has the objective of masking the statistical relationship
between the ciphertext and key. This can be achieved with the usage
of a non linear substitution function. This algorithm utilizes a
4-Bit S-Box (taken from PRESENT).

### Diffusion
Diffusion has the goal of dispersing the statistical information
of the plaintext across the whole cipher. Think about Hashing. 
Although hashing not an encryption algorithm, a single bit changes of a
file produces a completely different Filehash (e.g SHA256). "Real"
algorithms (e.g AES) achieve this by complex tanspositions. 

## Workflow

This algorithms satisfies the following requirements:

- The algorithm must split the plaintext into blocks
- The algorithm must an invertable transposition
- The algorithm must use a non-linear, invertable Substitution function
- The algorithm must utilize a key
- The algorithm must be able to decrypt an encrypted ciphertext

For simplicity, this algorithm performs the encryption one time for each
block. Real algorithms perform multiple rounds of permutation, 
substitution and key integration. This repitition provides the necessary
confusion and diffusion to make it secure.

This picture illustrates the workflow of the algorithm:

![Workflow](images/workflow.jpg)

In a nutshell the Plaintext is chunked into 4 byte sized blocks
and each block gets substituted, bytewise XOR'ed with a keybit and
inverted. The produced cipher block is then appended to the ciphertext.

The decryption performs the encrpytions steps in reverse order.

The main function provides a small menu for the user to chose whether to
encrypt or decrypt a text.

The given plaintext- and key-length are arbitrary.

---

## References

```
[1]   Bogdanov, Andrey; Knudsen, Lars R.; Leander, Gregor; Paar, Christof; Poschmann, Axel; Robshaw, Matthew J. B.; Seurin, Yannick; Vikkelsoe, Charlotte (2007). "PRESENT: An Ultra-Lightweight Block Cipher". Cryptographic Hardware and Embedded Systems - CHES 2007. Lecture Notes in Computer Science. Vol. 4727. pp. 450–466. doi:10.1007/978-3-540-74735-2_31. ISBN 978-3-540-74734-5.
[2] W. Stallings, L. Brown, Computer Security – Principles and Practice, fourth edition
[3] https://www.geeksforgeeks.org/computer-networks/what-is-s-box-substitution/
```