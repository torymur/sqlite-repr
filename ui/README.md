## SQLite3 Visual Representation

Describes on-disk [database file format](https://www.sqlite.org/fileformat2.html) used by all releases of SQLite since version 3.0.0.

## Development

1. Install Rust

Go to https://rust-lang.org and install the Rust compiler (preferably using rustup).

Once installed, make sure you add the stable toolchain and any relevant toolchains (ie wasm32-unknown-unknown for web apps):

```bash
rustup toolchain install stable
rustup target add wasm32-unknown-unknown
```

2. Install the [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started/)

```bash
cargo install dioxus-cli
```

3. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm

4. Install the tailwind css cli: https://tailwindcss.com/docs/installation

5. Install necessary plugins

Like daisyUI and official Tailwind CSS Typography plugin:

```bash
npm i -D daisyui@latest @tailwindcss/typography@latest tailwindcss-bg-patterns@latest

```
6. Run the following command to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
```

7. Run the following command to start the Dioxus dev server:

```bash
dx serve
```

8. Open the browser at http://localhost:8080
