# moon-browser
A DApp browser that aims to be a superior alternative to all the solutions currently on the market.

### Description
In the same way Operating Systems changed forever the way we deal with computers and web browsers made a huge impact on how we interact with the internet, we believe an upcoming era of crypto-browsers will transform the way we interact with blockchains. Moon is our endeavour to build a crypto-browser that makes justice to everything Ethereum has to offer. It contains the precise set of functionalities necessary to provide the best user-experience possible. Among them, we can highlight:

**Performance:** Moon features the first general-purpose language that runs in a massively parallel architecture, the GPU, as well as the first high-level programming language that is beta optimal and isn’t garbage collected, which, in simple words, means it won’t eat GBs of your memory and won’t put your coolers in panic mode.

**Forks:** imagine being able to download YouTube in a single click? Not the front-end, not a video, the entire company and all of its back-end logic, in a single click? On Moon, users can copy, edit and publish DApps in moments, allowing people to develop software as easy and fast as ever.

**Proof:** with money now being stored in computers, security has never been so important. On Moon, it is possible to build mathematical models and proofs about the expected behavior of software. Those proofs are checked locally and displayed in an user-friendly panel with English descriptions of their meanings. That allows users to trust the apps they use even if they don’t trust the developers: after all, isn’t the whole point of Ethereum to eliminate trust barriers?

### Implemented Functionality

  * GTK gui
  * IPFS upload/download

### How to run Moon browser
1. Download this repository to get Moon source code
2. Download [this](https://github.com/leonardostsouza/rust-ipfs-api) repository to get the IPFS Api source code
3. In the file Cargo.toml from the Moon repository, change the line
```
ipfsapi = { path = "../rust-ipfs-api" }
```
so the path is pointing to the IPFS Api root folder

4. Cargo run

### Changelog
v0.1.2
   * Browser now displays an OpenGL context instead of only text
   * Render formality-document files

v0.1.1
   * Changes in GUI
   * IPFS Download and Upload
   * Moon now uses IPFS server from Infura instead of a local instance

v0.1.0:
   * Basic GUI
   * IPFS file download in local server
