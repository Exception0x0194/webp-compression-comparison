package main

import (
	"flag"
	"image"
	_ "image/jpeg"
	_ "image/png"
	"os"
	"path/filepath"
	"runtime"
	"sync"
	"time"

	"github.com/kolesa-team/go-webp/encoder"
	"github.com/kolesa-team/go-webp/webp"
)

func compressToWebp(inputPath, outputPath string, quality float32) error {
	// Open input file
	file, err := os.Open(inputPath)
	if err != nil {
		return err
	}
	defer file.Close()

	// Decode image
	img, _, err := image.Decode(file)
	if err != nil {
		return err
	}

	// Create output file
	output, err := os.Create(outputPath)
	if err != nil {
		return err
	}
	defer output.Close()

	// Set encoding options
	options, _ := encoder.NewLossyEncoderOptions(encoder.PresetDefault, quality)

	// Encode image to WEBP
	return webp.Encode(output, img, options)
}

func main() {
	var inputDir, outputDir string
	var quality int

	flag.StringVar(&inputDir, "input-dir", "", "Path to the input directory containing images")
	flag.StringVar(&outputDir, "output-dir", "", "Path to the output directory for compressed images")
	flag.IntVar(&quality, "quality", 90, "WEBP compression quality")
	flag.Parse()

	if inputDir == "" || outputDir == "" {
		flag.PrintDefaults()
		os.Exit(1)
	}

	files, err := filepath.Glob(filepath.Join(inputDir, "*"))
	if err != nil {
		panic(err)
	}

	var wg sync.WaitGroup
	sem := make(chan struct{}, runtime.NumCPU()) // semaphore to limit the number of concurrent goroutines

	startTime := time.Now() // Start timing

	for _, file := range files {
		wg.Add(1)
		go func(file string) {
			defer wg.Done()
			sem <- struct{}{} // acquire a token
			outputPath := filepath.Join(outputDir, filepath.Base(file)+".webp")
			err := compressToWebp(file, outputPath, float32(quality))
			if err != nil {
				println("Failed to compress", file, ":", err.Error())
			} else {
				// println("Compressed", file, "to", outputPath)
			}
			<-sem // release the token
		}(file)
	}

	wg.Wait()
	endTime := time.Now() // End timing
	duration := endTime.Sub(startTime)
	println("Total time taken:", duration)
}
