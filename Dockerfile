FROM ubuntu:jammy

ENV DEBIAN_FRONTEND=noninteractive
ENV ENV="/etc/profile"

COPY install.sh /root
COPY build.sh /root
COPY package.sh /root
COPY rust_shared_lib /root/sdr-kit-build
COPY rust_shared_lib /root

RUN chmod +x /root/install.sh && cd /root && ./install.sh