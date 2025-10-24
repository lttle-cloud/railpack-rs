#!/bin/bash

set -e

ZIP_URL=$1

echo "Downloading and unpacking prebuilts..."

wget $ZIP_URL -O prebuilts.zip

unzip prebuilts.zip
rm prebuilts.zip

rm -rf prebuilt

tar -xzf prebuilt-libraries.tar.gz
rm prebuilt-libraries.tar.gz