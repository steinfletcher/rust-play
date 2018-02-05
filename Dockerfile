# Static builder for Rust
FROM ekidd/rust-musl-builder AS builder
COPY . /home/rust/src
RUN sudo /home/rust/.cargo/bin/cargo test
RUN sudo /home/rust/.cargo/bin/cargo build --release

# New image from scratch (minimal sized)
FROM scratch
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/play /play
EXPOSE 3000
ENTRYPOINT ["/play"]
