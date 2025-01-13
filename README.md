# WASM Image Compressor

A powerful WebAssembly-based image compression library for efficient client-side image processing with flexible input options.  
You can choose between returning a **blob URL** or a raw **Uint8Array** of the compressed image.

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

2. Import the functions you need:

```javascript
import {
  convertImage, // Returns a blob URL of the compressed image
  convertImageAsUint8Array, // Returns a Uint8Array of the compressed image
} from "wasm_image_compressor";
```

## Usage

Both `convertImage` and `convertImageAsUint8Array` support two input types: **URL** and **Uint8Array**.

### 1. Returning a Blob URL

Use `convertImage` if you want to receive a blob URL of the compressed image, which can be directly used as an `<img src="...">` value.

#### Compress Image from URL

```javascript
const compressedBlobUrl = await convertImage(
  "https://example.com/image.jpg", // Image URL
  "image/jpeg", // Source image type
  "image/webp", // Target image type
  0.75 // Compression strength (0-1)
);

// Now you can directly use the blob URL in an <img> tag:
// document.getElementById("myImage").src = compressedBlobUrl;
```

#### Compress Image from Uint8Array

```javascript
const uint8ArrayInput = new Uint8Array([
  /* image binary data */
]);
const compressedBlobUrl = await convertImage(
  uint8ArrayInput,
  "image/jpeg",
  "image/webp",
  0.75
);

// Use the blob URL (e.g., assign it to an <img> or store it for later use).
```

### 2. Returning a Uint8Array

Use `convertImageAsUint8Array` if you want the raw compressed bytes for more customized handling.

#### Compress Image from URL (Uint8Array result)

```javascript
const compressedBytes = await convertImageAsUint8Array(
  "https://example.com/image.png",
  "image/png",
  "image/jpeg",
  0.8
);

// compressedBytes is now a Uint8Array with your compressed image data.
```

#### Compress Image from Uint8Array (Uint8Array result)

```javascript
const uint8ArrayInput = new Uint8Array([
  /* image binary data */
]);
const compressedBytes = await convertImageAsUint8Array(
  uint8ArrayInput,
  "image/png",
  "image/webp",
  0.8
);

// You can then handle the Uint8Array as needed, e.g., convert it to a Blob or store it.
```

## API Reference

Each function shares the same parameters, but returns different data types:

| Function Name                | Return Type       | Description                                 |
| ---------------------------- | ----------------- | ------------------------------------------- |
| **convertImage**             | string (Blob URL) | Converts an image and returns a blob URL    |
| **convertImageAsUint8Array** | Uint8Array        | Converts an image and returns the raw bytes |

#### Parameters

| Parameter     | Type              | Description                                                        |
| ------------- | ----------------- | ------------------------------------------------------------------ |
| `input`       | URL or Uint8Array | Input image as a URL or Uint8Array                                 |
| `srcType`     | string            | Type of the source image (e.g., "image/jpeg", "image/png")         |
| `targetType`  | string            | Desired type for the output image (e.g., "image/webp")             |
| `compression` | number            | Compression level (0-1), lower values result in higher compression |

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
