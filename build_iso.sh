#!/usr/bin/env bash
set -euo pipefail

echo "🔨 Building kernel..."
cargo build --target x86_64-unknown-none

echo "🔍 Searching for UEFI bootloader..."
# Ищем .efi файл в target
EFI_FILE=$(find target/x86_64-unknown-none/debug -name "*.efi" -type f | head -n1)

if [ -z "$EFI_FILE" ]; then
    echo "❌ Error: .efi file not found. Run 'cargo clean' and rebuild."
    exit 1
fi
echo "✅ Found: $EFI_FILE"

echo "📦 Preparing ISO structure..."
rm -rf iso_build
mkdir -p iso_build/EFI/BOOT
cp "$EFI_FILE" iso_build/EFI/BOOT/BOOTX64.EFI

echo "💿 Creating UEFI ISO..."
xorriso -as mkisofs \
  -iso-level 3 \
  -full-iso9660-filenames \
  -volid "MYOS" \
  -e EFI/BOOT/BOOTX64.EFI \
  -no-emul-boot \
  -boot-load-size 4 \
  -boot-info-table \
  -o myos.iso \
  iso_build/

rm -rf iso_build
echo "🎉 ISO ready: ./myos.iso"
echo "🖥️  Test: qemu-system-x86_64 -cdrom myos.iso -serial stdio -nographic"