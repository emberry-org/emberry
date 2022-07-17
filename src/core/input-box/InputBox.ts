export class InputBox extends HTMLElement {

  textarea: HTMLTextAreaElement;
  mirage: HTMLDivElement;

  constructor() {
    super();
  }

  /**
   * Called everytime the textarea is scrolled or the value has changed.
   */
  update(value: string) {
    if(this.value != value) this.value = value; // Change value attribute if necessary.
    if(this.textarea.value != value) this.textarea.value = value; 
    this.textarea.style.height = '';
    this.textarea.style.height = this.textarea.scrollHeight - 16 + 'px';

    // Handle final newlines
    if (value[value.length - 1] == "\n") {
      value += " ";
    }

    // Update mirage (visible text)
    const html = this.render(value);
    this.mirage.innerHTML = html;
    this.textarea.innerHTML = html;
  }

  /**
   * Converts the text from the textarea into rich text.
   */
  render(text: string): string {

    // Render the rich mirage here ...

    let html = '';
    const lines = text.split('\n');

    for (let i = 0; i < lines.length; i++) {
      let line = lines[i];
      if (i > 0) html += '<br>';

      const bold = /\*\*(.*?)\*\*/gm;
      line = line.replace(bold, '<strong>**$1**</strong>');    
      html += `<span class="line">${line}</span>`;
    }

    return this.sanitize(html);
  }

  /**
   * Cleans the html of any harmful statements.
   */
  sanitize(html: string): string {

    // Sanitize the html here ...

    return html;
  }

  sync_scroll() {
    // Sync the horizontal and vertical scroll on the mirage and the textarea.
    this.mirage.scrollTop = this.textarea.scrollTop;
    this.mirage.scrollLeft = this.textarea.scrollLeft;
  }

  /**
   * Setup the elements.
   */
  setup() {
    // Get the initial value.
    let value = this.value || this.innerHTML || "";

    // Get the placeholder attribute.
    let placeholder = this.getAttribute("placeholder") || "";

    // Clear all html inside the element.
    this.innerHTML = "";

    // Initialize the textarea element.
    this.textarea = document.createElement("textarea");
    this.textarea.placeholder = placeholder;
    this.textarea.value = value;
    //this.textarea.setAttribute("spellcheck", "false");

    // For form compatibility.
    if (this.getAttribute("name")) {
        this.textarea.setAttribute("name", this.getAttribute("name"));
        this.removeAttribute("name");
    }

    // Setup the events oninput and onscroll.
    this.textarea.setAttribute("oninput", "this.parentElement.update(this.value); this.parentElement.sync_scroll();");
    this.textarea.setAttribute("onscroll", "this.parentElement.sync_scroll();");

    // Add the textarea as child.
    this.append(this.textarea);

    // Create the mirage div element.
    this.mirage = document.createElement('div');
    this.mirage.classList.add('mirage');
    this.append(this.mirage);
    
    // Add event listeners, bound so `this` can be referenced
    this.transfer_event("change", this.textarea, null, this.onchange);
    this.transfer_event("selectionchange", this.textarea, null, this.onselectionchange);

    this.update(value);
  }

  last_events: any = {};

  /** Transfer an event by name from this to an inner element. */
  transfer_event(evt_name: string, transfer_to: HTMLElement, oldValue: any, newValue: any) {
    // Doesn't exist
    if (oldValue) transfer_to.removeEventListener(evt_name, this.last_events[evt_name]);
    if (newValue) {
      this.last_events[evt_name] = this.onchange.bind(this);
      transfer_to.addEventListener(evt_name, this.last_events[evt_name]);
      this[`on${evt_name}`] = undefined; // Prevent duplicate
    }
  }

  /** Start the setup once we are connected :D */
  connectedCallback() {
    this.setup();
  }

  /** Clear the input box. */
  clear() { this.update(''); }

  /* Value attribute */
  get value() { return this.getAttribute("value"); }
  set value(val) { this.setAttribute("value", val); }
  
  /* Placeholder attribute */
  get placeholder() { return this.getAttribute("placeholder"); }
  set placeholder(val) { this.setAttribute("placeholder", val); }
}

if (!customElements.get('input-box')) customElements.define('input-box', InputBox);