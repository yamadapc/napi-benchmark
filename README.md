# napi-benchmark

Tries to determine what is the overhead of calling into a native node.js
module, assuming nothing else is happening.

## C++

Install

* CMake
* C++ compiler
* Node.js

```
cd cpp
npm install && npm run build
node ./test.js
```

## Rust

Install

* Cargo
* Node.js

```
cd neon_benchmark
npm run build
node ./test.js
```
