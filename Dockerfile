FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /substrate
COPY . /substrate
RUN rm -rf /usr/local/rustup /usr/local/cargo && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain none && \
    rustup show && \
    cargo build --release --locked

FROM docker.io/library/ubuntu:22.04
LABEL description="Docker image for Skill Tracker Node Software" \
  image.source="https://github.com/ndkazu/democracy_temp" \
  image.authors="Kazunobu Ndong" \
  image.documentation="https://github.com/ndkazu/democracy_temp" \
  image.vendor="Skill Tracker"

COPY --from=builder /substrate/target/release/node-template /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /node-dev node-dev && \
	mkdir -p /data /node-dev/.local/share && \
	chown -R node-dev:node-dev /data && \
	ln -s /data /node-dev/.local/share/node-template && \
	rm -rf /usr/bin /usr/sbin \
# check if executable works in this container
	/usr/local/bin/node-template --version

USER node-dev
EXPOSE 30333 9933 9944 9615
VOLUME ["/chain-data"]

CMD ["/usr/local/bin/node-template"]