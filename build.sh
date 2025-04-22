#!/bin/bash

# build frontend
cd frontend
pnpm build
cd ..

# Build
cd Crosslic_App
cargo build --release
