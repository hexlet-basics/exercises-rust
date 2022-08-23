FROM hexletbasics/base-image:latest

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

WORKDIR /exercises-rust

COPY . .

ENV PATH /exercises-rust/bin:/root/.cargo/bin/:$PATH
