#!/bin/bash
ARCH=arm64  # Set to 'amd64' for x86_64 systems

GOARCH=$ARCH GOOS=darwin go build -ldflags="-s -w" -o day17