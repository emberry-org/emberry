import { insertString } from "@core/Utils";
import { setupHooks } from "./input/InputEventHandler";
import { MirageRenderer } from "./renderer/TextRenderer";
import { CustomSelection } from "./Selection";

/**
 * Custom input box implementation.
 */
export class InputBox {

  private renderer: MirageRenderer;
  display: HTMLDivElement;

  value: string = '';
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
    //setupHooks(this);
  }

  /**
   * Updates the current value of the input box.
   */
  setValue(value: string) {
    this.value = value;
    this.display.innerText = this.value;
  }

  /**
   * @returns The current value of the input box.
   */
  getValue(): string {
    return this.value.replaceAll('﻿', '');
  }

  /**
   * Setup the hooks for the textarea.
   */
  private initHooks() {
    // Setup the input event.
    this.display.addEventListener('beforeinput', (ev: InputEvent) => {
      ev.preventDefault();
      // Save the current selection before updating the content.
      this.selection.saveCurrentSelection();
      const position = getCaretIndex(this.display);
      console.log('pos: ', getCaretIndex(this.display));
      console.log(ev, this.selection.currentSelection.startOffset, this.selection.currentSelection.endOffset);

      if (ev.inputType == 'insertText') {
        this.value = insertString(this.value, ev.data, position);
        this.selection.currentOffset += 1;
        this.renderer.render(this.value);
      }

      if (ev.inputType == 'insertLineBreak') {
        this.value = insertString(this.value, '\n﻿', position);
        this.selection.currentOffset += 1;
        this.renderer.render(this.value);
      }

      // Update the input mirage.
      //console.log(JSON.stringify(this.display.innerText));
      //const oldValue = this.value;
      //this.value = parseChildNodesForValueAndLines('', true, this.display.childNodes);
      //if (this.value == oldValue) return;

      //console.log(JSON.stringify(this.value));
      //this.renderer.render(this.value);

      // Restore the lost selection after updating the mirage.
      this.selection.restoreSelection();
    });
  }
}

function getCaretIndex(element) {
  let position = 0;
  const isSupported = typeof window.getSelection !== "undefined";
  if (isSupported) {
    const selection = window.getSelection();
    if (selection.rangeCount !== 0) {
      const range = window.getSelection().getRangeAt(0);
      console.log(range);
      const preCaretRange = range.cloneRange();
      preCaretRange.selectNodeContents(element);
      preCaretRange.setEnd(range.endContainer, range.endOffset);
      position = preCaretRange.toString().length;
      //return { start: range.startOffset, end: range.endOffset };
    }
  }
  return position;
}

// Recursive function to navigate childNodes and build linebreaks with text
function parseChildNodesForValueAndLines(value: string, isOnFreshLine: boolean, childNodes: NodeListOf<ChildNode>): string {
  for (let i = 0; i < childNodes.length; i++) {
    const childNode = childNodes[i];

    if (childNode.nodeName === 'BR') {
      // BRs are always line breaks which means the next loop is on a fresh line
      value += '\n';
      isOnFreshLine = true;
      continue;
    }

    // We may or may not need to create a new line
    if (childNode.nodeName === 'DIV' && isOnFreshLine === false) {
      // Divs create new lines for themselves if they aren't already on one
      value += '\n';
    }

    // Whether we created a new line or not, we'll use it for this content so the next loop will not be on a fresh line:
    isOnFreshLine = false;

    // Add the text content if this is a text node:
    if (childNode.nodeType === 3 && childNode.textContent) {
      value += childNode.textContent;
    }

    // If this node has children, get into them as well:
    parseChildNodesForValueAndLines(value, isOnFreshLine, childNode.childNodes);
  }

  return value;
}