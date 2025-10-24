#!/bin/bash
set -e

echo "Building Go library for linux-amd64..."

mkdir -p prebuilt/linux-amd64

cd src && GOOS=linux GOARCH=amd64 CGO_ENABLED=1 go build -buildmode=c-archive -o ../prebuilt/linux-amd64/librailpack.a railpack.go

echo "Successfully built prebuilt/linux-amd64/librailpack.a"
echo "Remember to commit this file to the repository"
