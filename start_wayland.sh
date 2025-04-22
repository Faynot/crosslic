#!/bin/bash

# build frontend
cd frontend
pnpm install
pnpm build
cd ..

# Run on x11
cd Crosslic_App
WEBKIT_DISABLE_COMPOSITING_MODE=1 GDK_BACKEND=x11 cargo watch -x run
