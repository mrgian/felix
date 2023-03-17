FROM rust
COPY . /root/felix

RUN apt-get -y update
RUN apt-get -y install fdisk
RUN apt-get -y install mtools
RUN apt-get -y install dosfstools

WORKDIR /root/felix
CMD ["./build.sh"]