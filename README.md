# WASM Image Compressor

A WebAssembly-based image compression library for efficient client-side image processing.

## Installation

To use the WASM Image Compressor in your project, follow these steps:

1. Install the package:

```bash
npm install wasm_image_compressor
```

2. Import and initialize the WASM module:

```javascript
import initWasm from "wasm_image_compressor";
import module_or_path from "wasm_image_compressor/lib/index_bg.wasm?url";

await initWasm({ module_or_path });
```

3. Use the `convertImage` function in your code:

```javascript
import { convertImage } from "wasm_image_compressor";

// Use convertImage function here
```

## Usage

After initialization, you can use the `convertImage` function to compress images. Here's a basic example:

```javascript
const compressedImage = await convertImage(inputFile, inputType, outputType, compressionStrength  progressCallback);
```

For detailed usage instructions and API documentation, please refer to the [API Documentation](#) section.

## Development

### Prerequisites

Ensure you have the following tools installed:

- Node.js: 20.12.2 or later
- Rust: 1.83.0 or later
- Cargo: 1.83.0 or later
- wasm-pack: 0.13.1 or later

### Building the Project

1. Build the Rust component:

```bash
cd wasm && cargo build
```

2. Install dependencies:

```bash
npm install
```

3. Start the development server:

```bash
npm run dev
```

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](#) for more information on how to get started.

## License

This project is licensed under the [MIT License](#).
