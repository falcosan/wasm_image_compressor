# WASM Image Compressor

A powerful WebAssembly-based image compression library for efficient client-side image processing with flexible input options.

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

The `convertImage` function supports two input types: **URL** and **Uint8Array**

### Compress Image from URL

```javascript
const compressedImage = await convertImage(
  "https://example.com/image.jpg", // Image URL
  "image/jpeg", // Source image type
  "image/webp", // Target image type
  0.75, // Compression strength (0-1)
  (progress, message) => {
    console.log(`${progress}% - ${message}`);
  }
);
```

### Compress Image from Uint8Array

```javascript
const uint8ArrayInput = new Uint8Array([
  /* image binary data */
]);
const compressedImage = await convertImage(
  uint8ArrayInput, // Image as Uint8Array
  "image/jpeg", // Source image type
  "image/webp", // Target image type
  0.75, // Compression strength (0-1)
  (progress, message) => {
    console.log(`${progress}% - ${message}`);
  }
);
```

## API Reference

#### Parameters

| Parameter     | Type              | Description                                                        |
| ------------- | ----------------- | ------------------------------------------------------------------ |
| `input`       | URL or Uint8Array | Input image as a URL or Uint8Array                                 |
| `srcType`     | string            | Type of the source image (e.g., "image/jpeg", "image/png")         |
| `targetType`  | string            | Desired type for the output image (e.g., "image/webp")             |
| `compression` | number            | Compression level (0-1), lower values result in higher compression |
| `callback`    | function          | Progress update callback function                                  |

#### Callback Function

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
