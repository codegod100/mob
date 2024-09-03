FROM ubuntu:24.04

WORKDIR /home/

COPY . .

RUN bash ./setup.sh

ENV PATH="/root/.cargo/bin:$PATH"

RUN cargo install tauri-cli@2.0.0-rc.9
