/**
 * Input box line renderer.
 */
export class LineRenderer {

  private display: HTMLDivElement;
  private value: string;

  private line: HTMLDivElement;
  private content: HTMLSpanElement;

  constructor(displayElement: HTMLDivElement, value: string) {
    this.display = displayElement;
    this.value = value;

    // Create the line element.
    this.line = document.createElement('div');
    this.line.classList.add('text-line');
    this.display.insertAdjacentElement('beforeend', this.line);

    // Create the content spanner.
    this.content = document.createElement('span');
    this.setValue(value);
    this.line.insertAdjacentElement('beforeend', this.content);
  }

  /**
   * Set the value of this line.
   * @param value The value to set as innerText.
   */
  setValue(value: string) {
    this.value = value;
    console.log(this.value.length);
    this.content.innerText = this.value.length > 0 ? this.value : '-';
  }
}

/**
 * Create a new line renderer.
 * @param displayElement The display element to set as parent.
 * @param value The value of this line.
 * @returns The line renderer instance.
 */
export function createLine(displayElement: HTMLDivElement, value: string): LineRenderer {
  return new LineRenderer(displayElement, value);
}