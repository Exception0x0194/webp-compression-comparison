const { workerData, parentPort } = require('worker_threads');
const sharp = require('sharp');

const { inputPath, outputPath, quality } = workerData;

sharp(inputPath)
    .toFormat('webp', { quality: quality })
    .toFile(outputPath)
    .then(() => parentPort.postMessage(`Compressed: ${outputPath}`))
    .catch(err => parentPort.postMessage(`Error: ${err.message}`));
