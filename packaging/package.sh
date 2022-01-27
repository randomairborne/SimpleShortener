#!/bin/bash
SQLX_OFFLINE=true
cargo build --release --target x86_64-unknown-linux-gnu
cross build --release --target aarch64-unknown-linux-gnu
mv ../target/x86_64-unknown-linux-gnu/release/simple_shortener ./simpleshortener_*_amd64/usr/sbin/
mv ../target/aarch64-unknown-linux-gnu/release/simple_shortener ./simpleshortener_*_arm64/usr/sbin/
dpkg-deb --build --root-owner-group ./simpleshortener_*_amd64
dpkg-deb --build --root-owner-group ./simpleshortener_*_arm64
