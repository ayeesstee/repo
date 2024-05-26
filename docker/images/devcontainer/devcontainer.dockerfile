# Copyright Ian Stewart 2024, All Rights Reserved.
FROM fedora:40

# Dependency Versions.
ENV VERSION_RUST=1.78.0
ENV VERSION_FLATBUFFERS=24.3.25

# Copy over our .bashrc file.
COPY docker/images/devcontainer/home/.bashrc /root/.bashrc

# Install VSCode extension dependencies, development tools, and generic system tools.
RUN dnf -qy upgrade \
    && dnf -qy install \
    clang-tools-extra \
    gcc-c++ mingw64-gcc-c++ \
    git \
    lcov \
    lldb \
    libicu \
    mingw64-llvm-tools \
    mingw64-binutils \
    mingw64-winpthreads \
    procps \
    rsync \
    time \
    which \
    zip unzip \
    && dnf clean all

# Install Rust.
#   Add Windows target for cross-compilation.
#   Install LLVM Tools Preview for code coverage support.
#   Install grcov for code coverage support.
RUN curl -sSf https://sh.rustup.rs > /tmp/rustup.sh \
    && sh /tmp/rustup.sh -y \
    && /root/.cargo/bin/rustup toolchain install ${VERSION_RUST} \
    && /root/.cargo/bin/rustup target add x86_64-pc-windows-gnu \
    && /root/.cargo/bin/rustup component add llvm-tools-preview \
    && /root/.cargo/bin/cargo install grcov \
    && rm -f /tmp/rustup.sh

# Install .NET.
RUN cd /tmp \
    && curl -sSf https://download.visualstudio.microsoft.com/download/pr/4a252cd9-d7b7-41bf-a7f0-b2b10b45c068/1aff08f401d0e3980ac29ccba44efb29/dotnet-sdk-8.0.300-linux-x64.tar.gz > dotnet-sdk-8.0.300-linux-x86_64.tar.gz \
    && mkdir -p dotnet/linux-x86_64 \
    && tar -xvf dotnet-sdk-8.0.300-linux-x86_64.tar.gz -C dotnet/linux-x86_64 \
    && rm -f dotnet-sdk-8.0.300-linux-x86_64.tar.gz \
    && curl -sSf https://download.visualstudio.microsoft.com/download/pr/6dd60d95-f5ae-414e-8259-b2a115e51714/c56f08471133d789dee9ffa52ddf5c1e/dotnet-sdk-8.0.300-win-x64.zip > dotnet-sdk-8.0.300-win-x86_64.zip \
    && mkdir -p dotnet/windows-x86_64 \
    && unzip dotnet-sdk-8.0.300-win-x86_64.zip -d dotnet/windows-x86_64 \
    && rm -f dotnet-sdk-8.0.300-win-x86_64.zip \
    && mv dotnet /usr/local

ENV PATH=/usr/local/dotnet/linux-x86_64:$PATH

# Install Fantomas.
RUN dotnet tool install fantomas --global

# Install FlatBuffers.
RUN cd /tmp \
    && curl -sSfL https://github.com/google/flatbuffers/releases/download/v${VERSION_FLATBUFFERS}/Linux.flatc.binary.g++-13.zip > Linux.flatc.binary.g++-13.zip \
    && mkdir -p flatbuffers \
    && unzip Linux.flatc.binary.g++-13.zip -d flatbuffers \
    && rm -f Linux.flatc.binary.g++-13.zip \
    && mv flatbuffers/flatc /usr/local/bin \
    && rm -rf flatbuffers