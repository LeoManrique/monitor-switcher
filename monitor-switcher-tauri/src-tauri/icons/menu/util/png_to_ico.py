"""Convert PNG to ICO file.

Usage:
    python png_to_ico.py <input.png> [output.ico] [--size SIZE]

Examples:
    python png_to_ico.py delete.png
    python png_to_ico.py delete.png delete.ico
    python png_to_ico.py icon.png icon.ico --size 32
    python png_to_ico.py icon.png --size 16,24,32,48
"""

from PIL import Image
import os
import io
import struct
import argparse


def png_to_ico(png_path, ico_path, sizes):
    """Convert PNG to ICO with specified sizes."""
    base_img = Image.open(png_path).convert('RGBA')

    png_data_list = []
    for size in sizes:
        resized = base_img.resize((size, size), Image.Resampling.LANCZOS)
        buf = io.BytesIO()
        resized.save(buf, format='PNG')
        png_data_list.append((size, buf.getvalue()))

    # Build ICO file
    num_images = len(png_data_list)
    ico_header = struct.pack('<HHH', 0, 1, num_images)
    data_offset = 6 + (16 * num_images)

    directory_entries = []
    image_data = b''

    for size, png_bytes in png_data_list:
        # Width and height: 0 means 256 in ICO format
        w = 0 if size >= 256 else size
        h = 0 if size >= 256 else size

        entry = struct.pack('<BBBBHHII',
            w, h, 0, 0, 1, 32,
            len(png_bytes),
            data_offset + len(image_data)
        )
        directory_entries.append(entry)
        image_data += png_bytes

    with open(ico_path, 'wb') as f:
        f.write(ico_header)
        for entry in directory_entries:
            f.write(entry)
        f.write(image_data)

    return os.path.getsize(ico_path)


def main():
    parser = argparse.ArgumentParser(
        description='Convert PNG to ICO file',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
    python png_to_ico.py delete.png                    # Creates delete.ico with 16x16
    python png_to_ico.py delete.png --size 32          # Creates delete.ico with 32x32
    python png_to_ico.py icon.png out.ico --size 16,32,48  # Multiple sizes
        """
    )
    parser.add_argument('input', help='Input PNG file')
    parser.add_argument('output', nargs='?', help='Output ICO file (default: same name as input)')
    parser.add_argument('--size', '-s', default='16',
                        help='Icon size(s), comma-separated (default: 16)')

    args = parser.parse_args()

    # Parse sizes
    sizes = [int(s.strip()) for s in args.size.split(',')]

    # Determine output path
    if args.output:
        ico_path = args.output
    else:
        ico_path = os.path.splitext(args.input)[0] + '.ico'

    # Convert
    file_size = png_to_ico(args.input, ico_path, sizes)
    print(f"Created {ico_path} ({file_size} bytes) with sizes: {sizes}")


if __name__ == '__main__':
    main()
