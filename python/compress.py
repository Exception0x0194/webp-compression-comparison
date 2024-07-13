import argparse
import os
import time
from concurrent.futures import ProcessPoolExecutor
from PIL import Image


def compress_image(input_path, output_path, quality):
    img = Image.open(input_path)
    output_file_path = os.path.join(output_path, os.path.basename(input_path))
    img.save(output_file_path, "WEBP", quality=quality)


def process_images(input_dir, output_dir, quality):
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    images = [
        os.path.join(input_dir, f)
        for f in os.listdir(input_dir)
        if f.endswith((".png", ".jpg", ".jpeg"))
    ]
    with ProcessPoolExecutor() as executor:
        for image_path in images:
            executor.submit(compress_image, image_path, output_dir, quality)


def main():
    parser = argparse.ArgumentParser(description="Image compression tool.")
    parser.add_argument(
        "--input-dir",
        required=True,
        help="Path to the input directory containing images.",
    )
    parser.add_argument(
        "--quality", type=int, required=True, help="WEBP compression quality."
    )
    parser.add_argument(
        "--output-dir",
        required=True,
        help="Path to the output directory for compressed images.",
    )

    args = parser.parse_args()

    start_time = time.time()
    process_images(args.input_dir, args.output_dir, args.quality)
    end_time = time.time()

    print(f"Total time taken: {end_time - start_time:.2f} seconds")


if __name__ == "__main__":
    main()
