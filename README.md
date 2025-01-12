# WASM Image Compressor

A powerful WebAssembly-based image compression library for efficient client-side image processing.

## Installation

Install the WASM Image Compressor package using npm:

```bash
npm install wasm_image_compressor
```

## Setup

To use the WASM Image Compressor in your project:

1. Import and initialize the WASM module:

```javascript
import initWasm from "wasm_image_compressor";
import module_or_path from "wasm_image_compressor/lib/index_bg.wasm?url";

await initWasm({ module_or_path });
```

2. Import the `convertImage` function:

```javascript
import { convertImage } from "wasm_image_compressor";
```

## Usage

After initialization, compress images using the `convertImage` function:

```javascript
const compressedImage = await convertImage(
  inputFile,
  inputType,
  outputType,
  compressionStrength,
  progressCallback
);
```

## API Reference

#### Parameters

| Parameter     | Type       | Description                                                        |
| ------------- | ---------- | ------------------------------------------------------------------ |
| `file`        | Uint8Array | Input image file as a Uint8Array                                   |
| `srcType`     | string     | type of the source image (e.g., "jpeg", "png")                     |
| `targetType`  | string     | Desired MIME type for the output image (e.g., "webp", "jpeg")      |
| `compression` | number     | Compression level (0-1), lower values result in higher compression |
| `callback`    | function   | Progress update callback function                                  |

The `callback` function receives two arguments:

- `progress` (number): Conversion progress (0-100)
- `message` (string): Description of the current conversion stage

#### Supported Image Formats

- JPEG
- PNG
- WebP
- GIF
- TIFF
- BMP
- ICO

**Note**: Actual format support may vary based on the underlying image processing library and WebAssembly capabilities.

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
npm run build:wasm
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

We welcome contributions! Please review our Contributing Guidelines for more information on how to get started.

## License

This project is licensed under the MIT License.
