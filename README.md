## About

Project is generated via:
```
cargo generate --git https://github.com/mishaszu/yew-wasm-pack-template.git
```

This repo is field of my experiments with [yew.rs](https://yew.rs/docs/) framework.
[This repo is forked from here](https://github.com/yewstack/yew-wasm-pack-template)

### 🛠️ Build
When building for the first time, ensure to install dependencies first.

```
yarn install
```

```
yarn run build
```

### 🔬 Serve locally

```
yarn run start:dev
```


## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
