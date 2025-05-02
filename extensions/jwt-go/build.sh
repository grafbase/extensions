#!/bin/bash
set -e

VERSION=$(grep 'version =' extension.toml | awk -F\" '{print $2}')
GOOS=js GOARCH=wasm go build -o jwt.wasm \
  -ldflags "-X main.grafbase_extension_version=$VERSION" \
  .

echo "Successfully built jwt extension version $VERSION"
