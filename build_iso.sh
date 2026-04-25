#!/usr/bin/env bash
set -euo pipefail

KERNEL_NAME="neuros"
TARGET="x86_64-unknown-none"
BUILD_DIR="target/${TARGET}/debug"
PROJECT_ROOT="$(pwd)"

echo "🔨 Building kernel..."
cargo build --target "$TARGET"

BIN_FILE="${BUILD_DIR}/${KERNEL_NAME}"
if [ ! -f "$BIN_FILE" ]; then
    echo "❌ Error: kernel binary not found at $BIN_FILE"
    exit 1
fi
echo "✅ Found: $BIN_FILE"

echo "📦 Creating bootable ISO (BIOS/Legacy)..."
rm -rf "${PROJECT_ROOT}/iso_build"
mkdir -p "${PROJECT_ROOT}/iso_build/boot/grub"

# Копируем ядро
cp "$BIN_FILE" "${PROJECT_ROOT}/iso_build/boot/kernel.bin"

# Создаём ISO — обратите внимание на "." в конце (содержимое iso_build)
xorriso -as mkisofs \
  -iso-level 3 \
  -rock \
  -volid "NEUROS" \
  -b boot/kernel.bin \
  -no-emul-boot \
  -boot-load-size 4 \
  -boot-info-table \
  -o "${PROJECT_ROOT}/neuros_bios.iso" \
  "${PROJECT_ROOT}/iso_build"

rm -rf "${PROJECT_ROOT}/iso_build"
echo "🎉 BIOS ISO ready: ${PROJECT_ROOT}/neuros_bios.iso"