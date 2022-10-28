export function getDominantColor(data: Uint8ClampedArray): Rgb {
  const rgb = buildRgb(data);

  const colors = quantization(rgb, 0);

  return mostSaturated(colors);
}

type Rgb = { r: number, g: number, b: number };

const buildRgb = (bytes: Uint8ClampedArray): Rgb[] => {
  const rgbValues = [];
  for (let i = 0; i < bytes.length; i += 4) {
    const rgb = {
      r: bytes[i],
      g: bytes[i + 1],
      b: bytes[i + 2],
    };
    rgbValues.push(rgb);
  }
  return rgbValues;
};

const findBiggestColorRange = (rgb: Rgb[]): "r" | "g" | "b" => {
  let rMin = Number.MAX_VALUE;
  let gMin = Number.MAX_VALUE;
  let bMin = Number.MAX_VALUE;

  let rMax = Number.MIN_VALUE;
  let gMax = Number.MIN_VALUE;
  let bMax = Number.MIN_VALUE;

  rgb.forEach((pixel) => {
    rMin = Math.min(rMin, pixel.r);
    gMin = Math.min(gMin, pixel.g);
    bMin = Math.min(bMin, pixel.b);

    rMax = Math.max(rMax, pixel.r);
    gMax = Math.max(gMax, pixel.g);
    bMax = Math.max(bMax, pixel.b);
  });

  const rRange = rMax - rMin;
  const gRange = gMax - gMin;
  const bRange = bMax - bMin;

  const biggestRange = Math.max(rRange, gRange, bRange);
  if (biggestRange === rRange) {
    return "r";
  } else if (biggestRange === gRange) {
    return "g";
  } else {
    return "b";
  }
};

/**
 * Find the most unqiue colors within the color space.
 */
const quantization = (rgb: Rgb[], depth: number): Rgb[] => {

  const MAX_DEPTH = 4;
  if (depth === MAX_DEPTH || rgb.length === 0) {
    const color = rgb.reduce(
      (prev, curr) => {
        prev.r += curr.r;
        prev.g += curr.g;
        prev.b += curr.b;

        return prev;
      },
      {
        r: 0,
        g: 0,
        b: 0,
      }
    );

    color.r = Math.round(color.r / rgb.length);
    color.g = Math.round(color.g / rgb.length);
    color.b = Math.round(color.b / rgb.length);
    return [color];
  }

  const componentToSortBy = findBiggestColorRange(rgb);
  rgb.sort((p1, p2) => {
    return p1[componentToSortBy] - p2[componentToSortBy];
  });

  const mid = rgb.length / 2;
  return [
    ...quantization(rgb.slice(0, mid), depth + 1),
    ...quantization(rgb.slice(mid + 1), depth + 1),
  ];
};

/**
 * Get the most saturated color. 
 */
const mostSaturated = (colors: Rgb[]): Rgb => {
  console.log(colors);
  const color = colors.reduce((prev, curr) => {
    let currentScore = saturation(curr);
    let previousScore = saturation(prev);
    return previousScore < currentScore ? curr : prev;
  });
  return color;
};

/**
 * Get the saturation of a color.
 */
const saturation = (color: Rgb): number => {
  const min = Math.min(color.r, color.g, color.b);
  const max = Math.max(color.r, color.g, color.b);

  const delta = max - min;
  if (max != 0)
    return delta / max;
  else {
    return 0;
  }
}