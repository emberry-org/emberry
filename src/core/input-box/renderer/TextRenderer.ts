import { createLine, LineRenderer } from "./LineRenderer";

/**
 * Input box display renderer.
 */
export class MirageRenderer {

  private display: HTMLDivElement;
  private lines: LineRenderer[];
  private value: string[];

  constructor(displayElement: HTMLDivElement) {
    this.display = displayElement;
    this.lines = [];
    this.value = [];
  }

  /**
   * Render a single line.
   * @param value The text to assign to the line.
   * @param index The index of the line.
   */
   private renderLine(value: string, index: number) {
    // if (this.lines.length > index && this.lines[index]) {
    //   // This line already exists so recycle the elements.
    //   this.lines[index].setValue(value);
    // } else {
    //   // This line doesn't exist yet so create a new one.
    //   const line = createLine(this.display, value);

    //   // Add the new line to the lines buffer.
    //   if (this.lines.length > index) this.lines[index] = line;
    //   else this.lines.push(line);
    // }

    const line = createLine(this.display, value);
    this.lines.push(line);
  }

  /**
   * Render all lines of this display.
   * @param value The text to display.
   */
  render(value: string) {
    this.value = value.split('\n');
    //this.display.innerText = value;

    let html = '';
    for (let i = 0; i < this.value.length; i++) {
      const line = this.value[i];
      if (i > 0) html += '<br>';
      html += `<span class="line">${line}</span>`;
    }
    console.log(html);
    this.display.innerHTML = html;

    //console.log(this.value);

    // Remove the existing lines.
    //while (this.display.lastElementChild) {
    //  this.display.removeChild(this.display.lastElementChild);
    //}
    //this.lines = [];

    // Loop over each line and render it.
    //for (let i = 0; i < this.value.length; i++) {
    //  const line = this.value[i];
    //  this.renderLine(line, i);
    //}
  }
}