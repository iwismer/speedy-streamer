# Speedy Streamer

This is a software utility to get tag IDs from an Impinj Speedway RFID reader to timing software.

This was developed and tested on the Speedway R420.

**This is still very much in beta, do not use it during real events!**

# Building with Docker (recommended)

Download the Octane embedded development tools 6.0.0: <https://support.impinj.com/hc/en-us/articles/202755288-Speedway-Revolution-Embedded-Development-Tools-and-Sample-Application-ETK->.

Create a folder `toolchain` in the root of this repo. Extract the archives so that the file structure looks like the following (the following illustration excludes the files):

```
toolchain
├── octane_etk-6.0.0.240
│   ├── arm-toolchain
│   └── include
└── octane_etk_sample-6.0.0.240
    ├── cap
    ├── include
    └── lib
```

Next, copy the `lib` and `include` directories from the sample application into the root of the repo. The following directories should now exist:

```
speedy-streamer
├── cap
├── include
├── lib
├── src
└── toolchain
```

Build the docker image with the following command: `docker build -i speedy-streamer-build .`.

Run the build container with the following command: `docker run -it --rm -v ${PWD}:/speed-streamer/ rfid`, you can replace `${PWD}` with the current directory (speedy-streamer's absolute path) if that command doesn't work correctly. Then `cd` into `/speedy-streamer` and run `make cap` to build the cap file.

# Building with a VM

Start by running an installation of Ubuntu 14.04. I used a VM downloaded from here: <https://www.osboxes.org/ubuntu/>.

Download the Octane embedded development tools 6.0.0: <https://support.impinj.com/hc/en-us/articles/202755288-Speedway-Revolution-Embedded-Development-Tools-and-Sample-Application-ETK->.
Follow the instructions on the page for setting up Ubuntu with the cross compilation toolchain.
Compile the sample application to ensure it is all working.

The commands required to be run to install the toolchain can be found in the dockerfile.

Install cargo and build this application with the command `cargo build --target=armv5te-unknown-linux-gnueabi --bin streamer`.
If there is an error, it is likely that you have installed the toolchain incorrectly.

Next, copy the `lib` and `include` directories from the sample application into this folder.
It should now be possible to build the `rfid_reader` application using the `make arm` command.

Next you can generate the CAP file using `make cap`

# Running

Log into your reader's interface and upload the generated CAP file.
Reboot the reader and the application will start automatically.

Connect to the reader on port 10000 to start receiving reads.

# Clients

There are 2 clients included. The python client can be easily modified to do any number of things, right now it graphs the first 250 reads it receives.

The Rust client just writes everything it receives over the network to a file. It is run with the arguments: `receiver [reader-ip] [reader-port] [output-file]`

# License

All files are licensed under the ISC license unless otherwise specified (like rfid_reader.cpp).
The full license text is available in `LICENSE.txt`.
