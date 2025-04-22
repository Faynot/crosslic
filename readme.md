![Crosslic](./img/logo.png)

# Introduction

Crosslic is a very minimalistic framework for creating maximally cross-platform desktop and mobile applications. It has only what is minimally necessary, nothing extra.

Crosslic runs in two independent processes, which allows you to create a highly loaded backend with an instant response from the frontend.


# Getting Started

To get started with Crosslic, follow these simple steps:

1. Clone the repository:
   ```
   git clone https://github.com/Faynot/crosslic.git
   ```
2. Install dependencies:
   ```
   cd crosslic/frontend
   pnpm install
   ```
3. Run the project:
   ```
   cd ../
   chmod +x start.sh
   ./start.sh
   ```

   or run for wayland:
   ```
   ./start_wayland.sh
   ```

4. Build the project:
   ```
   cd ../
   chmod +x build.sh
   ./build.sh
   ```



# Todo

- [ ] Create CLI
- [ ] npm, yarn support
- [ ] react native, vue, angular support
- [ ] Completely end development mode
