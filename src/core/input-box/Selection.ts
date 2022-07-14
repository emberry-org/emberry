type NodeWithOffset = { node: Node, currentOffset: number };

/**
 * Custom selection for the input box.
 */
export class CustomSelection {

  private display: HTMLDivElement;

  private currentSelection: Range;
  private startOffset: number;
  private currentOffset: number;

  constructor(display: HTMLDivElement) {
    this.display = display;
  }

  private getSelection(): Range {
    if (window.getSelection) {
      return window.getSelection().getRangeAt(0);
    }
  }

  private sumCurrentOffset(root: Node, node: Node, startOffset: number): number {
    for (let ele of Array.from(root.childNodes)) {
      if (node === ele) {
        break;
      }
      if (ele.contains(node)) {
        const result = this.sumCurrentOffset(ele, node, 0);
        startOffset += result;
        break;
      } else {
        startOffset += ele.textContent.length;
      }
    }
    return startOffset;
  }

  private findNodeForPosition(container: Node, currentOffset: number): NodeWithOffset {
    let node: Node;
    const res = this.findNode(
      container.childNodes,
      currentOffset
    );
    if (res) {
      node = res.node;
      currentOffset = res.currentOffset;
      if (node.childNodes.length === 0) {
        return { node, currentOffset };
      } else {
        return this.findNodeForPosition(node, currentOffset);
      }
    } else {
      return { node: container, currentOffset };
    }
  }

  private findNode(childNodes: NodeListOf<ChildNode>, currentOffset: number): NodeWithOffset {
    for (let node of Array.from(childNodes)) {
      if (currentOffset - node.textContent.length <= 0) {
        return { node, currentOffset };
      } else {
        currentOffset -= node.textContent.length;
      }
    }
  }

  /**
   * Save the current selection of the input box.
   */
  saveCurrentSelection() {
    this.currentSelection = this.getSelection();
    this.startOffset = this.currentSelection.startOffset;

    this.currentOffset = this.sumCurrentOffset(
      this.display,
      this.currentSelection.startContainer,
      this.startOffset
    );
  }

  /**
   * Restore the selection within the input box.
   */
  restoreSelection() {
    let node: Node;
    if (this.currentOffset === 0) return;

    const range = document.createRange();
    ({ node, currentOffset: this.currentOffset } = this.findNodeForPosition(
      this.display,
      this.currentOffset
    ));

    range.setStart(node, this.currentOffset);
    range.collapse(true);

    const sel = window.getSelection();
    sel.removeAllRanges();
    sel.addRange(range);
  }
}