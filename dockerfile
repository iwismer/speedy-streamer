FROM ubuntu:trusty

COPY toolchain/ toolchain/
RUN dpkg --add-architecture i386
RUN apt-get update
RUN apt-get install -y g++-4.8 gcc-4.8-multilib g++-4.8-multilib
RUN apt-get install -y lib32z1 libc6:i386 libc6-dev-i386 libssl-dev:i386 curl
RUN apt-get install -y build-essential
RUN rm /usr/bin/g++
RUN rm /usr/bin/x86_64-linux-gnu-g++
RUN ln -s /usr/bin/g++-4.8 /usr/bin/g++
RUN ln -s /usr/bin/g++-4.8 /usr/bin/x86_64-linux-gnu-g++
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN rustup target add armv5te-unknown-linux-gnueabi

ENV PATH=$PATH:/toolchain/octane_etk-6.0.0.240:/toolchain/octane_etk-6.0.0.240/arm-toolchain/bin:/toolchain/octane_etk_sample-6.0.0.240:/root/.cargo/bin
