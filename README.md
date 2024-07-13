# WebP Compression Efficiency Comparison

[简体中文](/README_zh-CN.md)

I previously developed a [WebP image compression software based on Tauri](https://github.com/Exception0x0194/webp-compressor) implemented in Rust, which is much faster than many current image compression softwares. But how much does Rust actually contribute to the efficiency of the software? To unravel this question, I wrote command line tools in several different languages to parallel compress images to WebP and save them, testing the efficiency of these languages in this task.

The code for building the command line tools is shown in the [project Repo](https://github.com/Exception0x0194/webp-compression-comparison), using languages including `Python`, `JavaScript`, `Go`, and `Rust`. These command line tools all accept three parameters:

- `--input-dir <dir>`: The folder where the input images are located.
- `--quality <number>`: WebP compression quality.
- `--output-dir <dir>`: The folder where the output images are stored.

## Test Environment

- **System Parameters**
  - **CPU**: AMD R5-5600
  - **Windows10**: 22H2 19045.4651
  - **WSL**: Ubuntu 24.04 LTS
- **Test Input**
  - **Input Images**: 3000 PNG images, resolution 1280x1856
  - **Compression Quality**: 90

## Test Results

| Language | Time/s - Windows 10 | Time/s - WSL | Notes                                                        |
| -------- | ------------------- | ------------ | ------------------------------------------------------------ |
| Python   | 101.48              | 155.60       | Python==3.11.7(Win)/3.12.3(WSL), Pillow==9.5.0               |
| Node.js  | 182.90              | 388.56       | Node==20.15.0(Win)/18.19.1(WSL), Sharp==0.33.4               |
| Rust     | 87.63               | 99.50        | rustc==1.79.0, webp==0.3.0, image==0.25.1, rayon==1.10.0     |
| Go       |                     | 160.93       | go==1.22.2, [go-webp](github.com/kolesa-team/go-webp)==1.0.4 |

> During testing, except for Node.js, other languages fully loaded the CPU, so disk IO and other factors affecting efficiency can likely be ignored.

> After checking the output file content, it was found that the data were exactly the same, seems that the same methods are used for lossy compression of images (probably from libwebp).

## Discussion and Conclusion

- Although Python is an interpreted language, the libraries it calls internally are highly efficient, with overall efficiency close to compiled languages (in fact, it has surpassed Go).
- Node.js's Workers are not very convenient to use and seem ill-suited for high IO and compute-load tasks, being the only one that did not fully load the CPU during testing.
- Rust, while having a strict syntax, is not difficult to develop with the help of various packages, and it also has the highest efficiency. In the WSL environment, the performance loss is also smaller than other languages.
- The official Go webp library can only decode images, while the community-provided go-webp library requires libwebp during compilation, making it difficult to configure on Windows, so it was only compiled and tested on WSL. However, it was unexpectedly slow (even with `-ldflags "-s -w"`), which may be a problem with library optimization.

It appears that using Rust libraries was a very correct decision. Through this test, it was found that Rust exhibits outstanding performance in handling high IO and CPU intensive tasks, such as image compression. Rust not only performs well on Windows systems but also shows smaller performance loss in WSL environments compared to other languages, demonstrating its stability and efficiency.

Python, as an interpreted language, is not typically considered the best choice for performance, especially in scenarios requiring extensive computation and data processing. However, by utilizing well-optimized third-party libraries, Python can also demonstrate surprisingly high performance in handling heavy-load tasks, with some libraries even competing with compiled languages like C++ and Rust. Considering Python's additional advantage in development efficiency, writing Python scripts for data processing tasks in some fast-paced development contexts is also a good choice.