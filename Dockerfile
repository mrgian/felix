FROM archlinux
COPY . /root/felix

RUN pacman --noconfirm -Syu mtools dosfstools base-devel rustup

WORKDIR /root/felix
CMD ["./build.sh"]