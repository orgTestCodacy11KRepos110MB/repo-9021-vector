FROM ghcr.io/cross-rs/armv7-unknown-linux-musleabihf:0.2.4

COPY scripts/cross/bootstrap-ubuntu.sh scripts/environment/install-protoc.sh /
RUN /bootstrap-ubuntu.sh && bash /install-protoc.sh

# Stick `libstdc++` somewhere it can be found other than it's normal location, otherwise we end up using the wrong version
# of _other_ libraries, which ultimately just breaks linking. We'll set `/lib/native-libs` as a search path in `.cargo/config.toml`.
RUN mkdir -p /lib/native-libs && cp /usr/local/arm-linux-musleabihf/lib/libstdc++.a /lib/native-libs/
