#!/bin/bash
set -e

echo "🔨 Building croner-wasm for all targets..."

# Build for all targets
echo "📦 Building for web..."
wasm-pack build --target web --out-dir pkg-web

echo "📦 Building for Node.js..."
wasm-pack build --target nodejs --out-dir pkg-node

echo "📦 Building for bundlers..."
wasm-pack build --target bundler --out-dir pkg-bundler

# Create main package directory
echo "📦 Creating unified package..."
mkdir -p dist

# Copy web build to dist root (default)
cp pkg-web/* dist/

# Copy Node.js build
mkdir -p dist/node
cp pkg-node/* dist/node/

# Copy bundler build
mkdir -p dist/bundler
cp pkg-bundler/* dist/bundler/

echo "✅ Build complete!"
echo "📦 Package ready in ./dist"
