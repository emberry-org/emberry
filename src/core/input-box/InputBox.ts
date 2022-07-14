import { insertString } from "@core/Utils";
import { MirageRenderer } from "./InputMirage";
import { CustomSelection } from "./Selection";

/**
 * Custom input box implementation.
 */
export class InputBox {

  private renderer: MirageRenderer;
  private display: HTMLDivElement;

  private value: string = '';
  private selection: CustomSelection;

  private onSubmit: () => void;

  constructor(displayElement: HTMLDivElement, onSubmit: () => void) {
    this.display = displayElement;
    this.display.setAttribute('contenteditable', 'true');

    this.selection = new CustomSelection(this.display);

    this.renderer = new MirageRenderer(this.display);
    this.renderer.render(this.value);

    this.onSubmit = onSubmit;

    /** HTML layout :
     * 
     *  ...
     *  <div class="text-line">
     *    <span> ... </span>
     *    <div class="selection" />
     *  </div>
     *  ...
     */

    this.initHooks();
  }

  /**
   * Updates the current value of the input box.
   */
  setValue(value: string) {
    this.value = value;
    this.renderer = new MirageRenderer(this.display);
    this.renderer.render(this.value);
  }

  /**
   * @returns The current value of the input box.
   */
  getValue(): string {
    return this.value;
  }

  /**
   * Setup the hooks for the textarea.
   */
  private initHooks() {
    // Setup the input event.
    this.display.addEventListener('input', (ev: InputEvent) => {
      if (this.value == this.display.innerText) return;
      // Save the current selection before updating the content.
      this.selection.saveCurrentSelection();

      // Update the input mirage.
      console.log(JSON.stringify(this.display.innerText));
      this.value = this.display.innerText;
      this.renderer.render(this.value);

      // Restore the lost selection after updating the mirage.
      this.selection.restoreSelection();
    });
  }
}