![Crosslic](./img/logo.png)

# Introduction

Crosslic is a very minimalistic framework for creating maximally cross-platform desktop and mobile applications. It has only what is minimally necessary, nothing extra.

Crosslic runs in two independent processes, which allows you to create a highly loaded backend with an instant response from the frontend.


# Getting Started

To get started with Crosslic, follow these simple steps:

1. install CLI
```
npm i -g crosslic
```

2. init the project
```
crosslic init
```

3. go to project and run him
```
cd project-name
crosslic dev
```
or run for wayland
```
crosslic dev --wayland
```

4. build the project
```
crosslic build
```
Build in **project-name/Crosslic_App/target/release**



# Todo

- [x] Create CLI
- [x] npm, yarn support
- [x] react native, vue, angular support
- [ ] Completely end development mode
