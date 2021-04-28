# Speedy Streamer

This is a software utility to get tag IDs from an Impinj Speedway RFID reader to timing software.

This was developed and tested on the Speedway R420.

**This is still very much in beta, do not use it during real events!**

# Building

Start by running an installation of Ubuntu 14.04. I used a VM downloaded from here: <https://www.osboxes.org/ubuntu/>.

Download the Octane embedded development tools 6.0.0: <https://support.impinj.com/hc/en-us/articles/202755288-Speedway-Revolution-Embedded-Development-Tools-and-Sample-Application-ETK->.
Follow the instructions on the page for setting up Ubuntu with the cross compilation toolchain.
Compile the sample application to ensure it is all working.

Install cargo and build this application with the command `cargo build --target=armv5te-unknown-linux-gnueabi`.
If there is an error, it is likely that you have installed the toolchain incorrectly.

Next, copy the `lib` and `include` directories from the sample application into this folder.
It should now be possible to build the `rfid_reader` application using the `make arm` command.

Next you can generate the CAP file using `make cap`

# Running

Log into your reader's interface and upload the generated CAP file.
Reboot the reader and the application will start automatically.

Connect to the reader on port 10000 to start receiving reads.

# License

All files are licensed under the ISC license unless otherwise specified (like rfid_reader.cpp).
The full license text is available in `LICENSE.txt`.
