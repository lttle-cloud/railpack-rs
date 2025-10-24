#!/bin/bash
set -e

echo "Building Go library for darwin-arm64..."

mkdir -p prebuilt/darwin-arm64

cd src && GOOS=darwin GOARCH=arm64 CGO_ENABLED=1 go build -buildmode=c-archive -o ../prebuilt/darwin-arm64/librailpack.a railpack.go

echo "Successfully built prebuilt/darwin-arm64/librailpack.a"
echo "Remember to commit this file to the repository"
