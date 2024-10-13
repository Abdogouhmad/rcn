# RCN - React Component Generator

RCN is a command-line tool designed to generate React components with the necessary files. It supports both JavaScript and TypeScript components.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Commands](#commands)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

## Installation

To use RCN, you need to have Rust installed on your system. You can download and install Rust from [rust-lang.org](https://www.rust-lang.org/).

Once Rust is installed, you can clone this repository and build the tool using Cargo:

```sh
cargo install --git https://github.com/Abdogouhmad/rcn.git
```
oor 

```sh
bash <(curl -sSL https://raw.githubusercontent.com/Abdogouhmad/rcn/main/setup.sh)
```
## Usage

```
rcn <Command> [options]
```

## Commands
### modeles

Generates a React component with the specified name and language (JavaScript or TypeScript).

```bash
rcn modeles <name> <language>

    <name>: The name of the component.
    <language>: The language of the component (js for JavaScript, ts for TypeScript).
```

````
└── searchbar # component
    ├── searchbar.component.tsx
    ├── searchbar.service.ts
    └── searchbar.types.ts
````
### help

Displays the help message with information about the available commands and their usage.
```bash
rcn help
# or 
rcn -h
```

### Examples
Generate a JavaScript or TS React Component

```
rcn modeles MyComponent js/ts
```
This command will create a directory components/MyComponent with the following files:

````
- MyComponent.component.jsx/tsx
- MyComponent.service.js/ts
- MyComponent.types.ts # js won't generate the file types.js 
````

