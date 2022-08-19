export async function resizeBase64Image(base64: string, size: [number, number], callback: (img: string) => void) {

  const img = document.createElement("img");
  img.onload = () => {
    // Create a canvas element
    const canvas = document.createElement("canvas");
    canvas.width = 128;
    canvas.height = 128;

    // Draw the image onto the canvas
    const ctx = canvas.getContext("2d");
    ctx.drawImage(img, 0, 0, 128, 128);

    // Get the base64 of the resized image
    const dataurl = canvas.toDataURL();

    // Callback the resized base64 string
    callback(dataurl.substring(22));
  };
  img.src = 'data:image/png;base64,' + base64;
}