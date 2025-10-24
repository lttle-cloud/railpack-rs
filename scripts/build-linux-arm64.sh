#!/bin/bash
set -e

echo "Building Go library for linux-arm64..."

mkdir -p prebuilt/linux-arm64

cd src && GOOS=linux GOARCH=arm64 CGO_ENABLED=1 go build -buildmode=c-archive -o ../prebuilt/linux-arm64/librailpack.a railpack.go

echo "Successfully built prebuilt/linux-arm64/librailpack.a"
echo "Remember to commit this file to the repository"
