#!/usr/bin/env bash

PROJECT_NAME="basisregisters-auth-lambda"
BUILD_SERVER_PLATFORM="linux/amd64" # set this to either linux/arm64 depending on your machine CPU. 
RUST_TARGET="aarch64-unknown-linux-gnu" # set this to aarch64 or x86_64 -unknown-linux-gnu for ARM or x86 functions.
OUPUT_ZIP_NAME=${PROJECT_NAME}_$(date +'%Y-%m-%d%-H%M%S')'.zip'
DOCKER_IMAGE="ghcr.io/cargo-lambda/cargo-lambda"
RUST_VERSION="latest"
CARGO_CACHE='' #speeds up the process
# CARGO_CACHE="-v ${HOME}/.cargo/registry:/usr/local/cargo/registry" 

docker run \
  --platform ${BUILD_SERVER_PLATFORM} \
  --rm \
  --user "$(id -u)":"$(id -g)" \
  -v "${PWD}/src/${PROJECT_NAME}":/app \
  ${CARGO_CACHE} \
  -w /app \
  ${DOCKER_IMAGE}:${RUST_VERSION} \
	cargo lambda build \
  --release \
  --target ${RUST_TARGET} &&\
cp ./src/${PROJECT_NAME}/target/lambda/${PROJECT_NAME}/bootstrap ./bootstrap &&\
zip dist/${PROJECT_NAME}/${OUPUT_ZIP_NAME} ./bootstrap &&\
rm bootstrap