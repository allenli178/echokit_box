default:
    just -l

# build the project
build:
    cargo build --release

# build the project and create a flashable image
build-image: build
    espflash save-image --chip esp32s3 --merge --flash-size 16mb target/xtensa-esp32s3-espidf/release/echokit echokit.bin