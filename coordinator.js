const yaml = require("js-yaml");
const got = require("got");
const argparse = require("argparse");
const fs = require("fs");
const SAMPLES_PER_WORKER = 5;
const LINES_PER_WORKER = 5;
const { PNG } = require("pngjs");
const Bluebird = require("bluebird");

async function renderSample(args, scene, current_top) {
  let image = scene.image;
  let body = {
    ...scene,
    image: {
      ...image,
      samples: SAMPLES_PER_WORKER,
      slice: {
        height: LINES_PER_WORKER,
        top: current_top
      }
    }
  };
  try {
    let response = await got.post(args.url, {
      body,
      json: true
    });
    return response.body;
  } catch (err) {
    console.log(err);
    throw err;
  }
}

async function renderLine(args, scene, current_top) {
  let promises = [];
  for (let i = 0; i < SAMPLES_PER_WORKER; ++i) {
    promises.push(renderSample(args, scene, current_top));
  }
  let results = await Promise.all(promises);

  let final_pixels = [];

  for (let i = 0; i < results[0].pixels.length; ++i) {
    let pixel = [0, 0, 0, 255];
    for (let result of results) {
      pixel[0] += result.pixels[i][0];
      pixel[1] += result.pixels[i][1];
      pixel[2] += result.pixels[i][2];
    }
    pixel[0] /= SAMPLES_PER_WORKER;
    pixel[1] /= SAMPLES_PER_WORKER;
    pixel[2] /= SAMPLES_PER_WORKER;
    final_pixels.push(pixel);
  }
  return final_pixels;
}
async function renderImage(args, scene) {
  let image = scene.image;
  let lineTops = [];
  for (
    let current_top = 0;
    current_top < image.height;
    current_top += LINES_PER_WORKER
  ) {
    lineTops.push(current_top);
  }
  let lineChunks = await Bluebird.map(
    lineTops,
    async currentTop => {
      let result = await renderLine(args, scene, currentTop);
      console.log("done with one segment ", currentTop);
      return result;
    },
    {
      concurrency: 10
    }
  );
  let png = new PNG({
    width: image.width,
    height: image.height
  });
  let i = 0;
  for (let chunk of lineChunks) {
    for (let pixel of chunk) {
      for (let channel of pixel) {
        png.data[i++] = channel;
      }
    }
  }

  png.pack().pipe(fs.createWriteStream(args.output));
}

async function main() {
  let parser = new argparse.ArgumentParser({
    version: "0.1.0",
    addHelp: true,
    description: "Tracer coordinator"
  });

  parser.addArgument(["-o", "--output"], {
    defaultValue: "image.png"
  });

  parser.addArgument(["-i", "--input", "--scene"], {
    defaultValue: "scene.yml"
  });
  parser.addArgument(["--url"], {
    required: true
  });

  let args = parser.parseArgs();

  let scene = fs.readFileSync(args.input, {
    encoding: "utf8"
  });
  scene = yaml.load(scene);
  let image = scene.image;
  let numWorkers =
    (image.height * image.samples) / (SAMPLES_PER_WORKER * LINES_PER_WORKER);

  console.log("=> Workers: ", numWorkers);
  await renderImage(args, scene);
}

main();
