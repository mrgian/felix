FROM rust
COPY . /root/felix

RUN apt-get -y update
RUN apt-get -y install build-essentials fdisk mtools dosfstools

WORKDIR /root/felix
CMD ["make all"]