FROM rust
COPY . /root/felix

RUN apt-get -y update
RUN apt-get -y install fdisk mtools dosfstools

WORKDIR /root/felix
CMD ["./build.sh"]