const fs = require('fs');
const path = require('path');
const { Worker } = require('worker_threads');
const { Command } = require('commander');
const os = require('os');

const program = new Command();

program
    .requiredOption('--input-dir <dir>', 'input directory containing images')
    .requiredOption('--quality <number>', 'WEBP compression quality', parseInt)
    .requiredOption('--output-dir <dir>', 'output directory for compressed images');

program.parse(process.argv);

const { inputDir, quality, outputDir } = program.opts();

if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
}

const files = fs.readdirSync(inputDir).filter(file => file.endsWith('.png') || file.endsWith('.jpg') || file.endsWith('.jpeg'));

// Limit the number of concurrent workers to the number of CPU cores
const maxWorkers = os.cpus().length;
console.log(`Workers: ${maxWorkers}`);
const promises = [];
let currentWorkers = 0;

const startTime = Date.now();

function processFile(file) {
    const inputPath = path.join(inputDir, file);
    const outputPath = path.join(outputDir, `${path.basename(file, path.extname(file))}.webp`);
    const worker = new Worker(path.join(__dirname, 'worker.js'), {
        workerData: { inputPath, outputPath, quality }
    });

    currentWorkers++;
    worker.on('message', (msg) => {
        // console.log(msg);
        currentWorkers--;
        if (files.length > 0) {
            processFile(files.shift());
        }
    });

    worker.on('exit', () => {
        if (currentWorkers === 0 && files.length === 0) {
            const endTime = Date.now();
            console.log(`Total time taken: ${(endTime - startTime) / 1000} seconds`);
        }
    });
}

// Start initial batch of workers
for (let i = 0; i < Math.min(maxWorkers, files.length); i++) {
    processFile(files.shift());
}
