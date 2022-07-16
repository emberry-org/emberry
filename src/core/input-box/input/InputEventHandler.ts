import { insertString, removeRange } from "@core/Utils";
import type { InputBox } from "../InputBox";
import { CustomSelection } from "../Selection";

export function setupHooks(inputBox: InputBox) {
  const selection: CustomSelection = new CustomSelection(inputBox.display);

  // inputBox.display.addEventListener('keypress', (ev: KeyboardEvent) => { 
  //   //ev.preventDefault(); 
  //   //console.log(ev);
  //   //if (ev.key.length == 1) inputBox.value += ev.key;
  //   selection.saveCurrentSelection();
  // });

  // Setup the input event hook.
  inputBox.display.addEventListener('input', (ev: InputEvent) => {
    const pos = getCaretPosition(inputBox.display);
    selection.saveCurrentSelection();

    // Reset the text.
    inputBox.display.innerText = inputBox.value;
    console.log(ev);

    if (ev.inputType == 'insertText') {
      // Add character
      inputBox.value = insertString(inputBox.value, ev.data, pos);
      inputBox.display.innerText = inputBox.value;
      selection.restoreSelection();
    } else if (ev.inputType == 'insertLineBreak') {
      // Newline
      inputBox.value = insertString(inputBox.value, '\n﻿', pos - 1);
      inputBox.display.innerText = inputBox.value;
      selection.restoreSelection();
    } else if (ev.inputType == 'deleteContentBackward') { // Weird stuff with these positions (USE THE SELECTION class INSTEAD)
      // Backspace
      inputBox.value = removeRange(inputBox.value, [pos + 1, pos + 2]);
      inputBox.display.innerText = inputBox.value;
      selection.restoreSelection();
    } else if (ev.inputType == 'deleteContentForward') {
      // Delete
      inputBox.value = removeRange(inputBox.value, [pos, pos + 1]);
      inputBox.display.innerText = inputBox.value;
      selection.restoreSelection();
    }
  });
}

function getCaretPosition(div: HTMLDivElement) {
  let caretOffset = 0;

  if (window.getSelection) {
    const range = window.getSelection().getRangeAt(0);
    const preCaretRange = range.cloneRange();
    preCaretRange.selectNodeContents(div);
    preCaretRange.setEnd(range.endContainer, range.endOffset);
    caretOffset = preCaretRange.toString().length;
  } 
  
  else if ((document as any).selection && (document as any).selection.type != "Control") {
    const textRange = (document as any).selection.createRange();
    const preCaretTextRange = (document as any).body.createTextRange();
    preCaretTextRange.moveToElementText(div);
    preCaretTextRange.setEndPoint("EndToEnd", textRange);
    caretOffset = preCaretTextRange.text.length;
  }

  return caretOffset;
}

function setCaretPosition(div: any, pos: number) {
  const range = document.createRange()
  const sel = window.getSelection()
  
  range.setStart(div, pos)
  range.collapse(true)
  
  sel.removeAllRanges()
  sel.addRange(range)
}

function createRange(node, chars, range) {
  if (!range) {
      range = document.createRange()
      range.selectNode(node);
      range.setStart(node, 0);
  }

  if (chars.count === 0) {
      range.setEnd(node, chars.count);
  } else if (node && chars.count >0) {
      if (node.nodeType === Node.TEXT_NODE) {
          if (node.textContent.length < chars.count) {
              chars.count -= node.textContent.length;
          } else {
              range.setEnd(node, chars.count);
              chars.count = 0;
          }
      } else {
         for (var lp = 0; lp < node.childNodes.length; lp++) {
              range = createRange(node.childNodes[lp], chars, range);

              if (chars.count === 0) {
                  break;
              }
          }
      }
  } 

  return range;
};