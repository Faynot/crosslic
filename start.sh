#!/bin/bash

# build frontend
cd frontend
pnpm install
pnpm build
cd ..

# Run
cd Crosslic_App
cargo watch -x run
