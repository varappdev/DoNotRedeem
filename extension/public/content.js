const id = Array.from(crypto.getRandomValues(new Uint8Array(16)), (b) => b.toString(16).padStart(2, "0")).join("");
const origin = new URL(chrome.runtime.getURL("")).origin;

const highlight = document.createElement("div");
highlight.style.cssText =
  "position:absolute;pointer-events:none;z-index:2147483646;box-sizing:border-box;" +
  "border:2px solid #000080;background:rgba(0,0,128,0.15);display:none;";

const tooltip = document.createElement("iframe");
tooltip.style.cssText =
  "position:absolute;z-index:2147483647;border:0;width:300px;height:0;" +
  "background:transparent;display:none;color-scheme:normal;";
tooltip.setAttribute("allowtransparency", "true");
tooltip.setAttribute("scrolling", "no");

const INLINE = new Set(["inline", "inline-block", "inline-flex", "contents"]);

let leaf = null;
let hovered = null;
let trail = [];
let locked = false;
let enabled = false;

function enable() {
  enabled = true;
  tooltip.src = chrome.runtime.getURL(`index.html#tooltip:${id}`);
  document.documentElement.append(highlight, tooltip);
}

function disable() {
  enabled = false;
  locked = false;
  hide();
  highlight.remove();
  tooltip.remove();
}

chrome.storage.local.get("elementCheck").then(({ elementCheck }) => elementCheck && enable());

chrome.storage.onChanged.addListener((changes, area) => {
  if (area !== "local" || !changes.elementCheck) return;
  changes.elementCheck.newValue ? enable() : disable();
});

function selectable(el) {
  return el instanceof Element && el !== document.body && el !== document.documentElement;
}

function block(el) {
  while (selectable(el.parentElement) && INLINE.has(getComputedStyle(el).display)) {
    el = el.parentElement;
  }
  return el;
}

function select(el) {
  hovered = el;
  place();
}

function place() {
  const rect = hovered.getBoundingClientRect();
  Object.assign(highlight.style, {
    display: "block",
    top: `${rect.top + scrollY}px`,
    left: `${rect.left + scrollX}px`,
    width: `${rect.width}px`,
    height: `${rect.height}px`,
  });

  const height = tooltip.offsetHeight;
  const top = Math.min(Math.max(rect.bottom, 0), innerHeight - height);
  const left = Math.min(Math.max(rect.left, 0), innerWidth - 300);
  Object.assign(tooltip.style, {
    display: "block",
    top: `${Math.max(top, 0) + scrollY}px`,
    left: `${Math.max(left, 0) + scrollX}px`,
  });
}

function hide() {
  leaf = null;
  hovered = null;
  trail = [];
  highlight.style.display = "none";
  tooltip.style.display = "none";
}

document.addEventListener("mouseover", (event) => {
  const el = event.target;
  if (!enabled || locked || el === tooltip || el === leaf || !selectable(el)) return;
  leaf = el;
  trail = [];
  select(block(el));
});

document.addEventListener("mouseout", (event) => {
  if (enabled && !locked && !event.relatedTarget) hide();
});

addEventListener("scroll", () => hovered && place(), { passive: true });

addEventListener("message", (event) => {
  if (!enabled || event.source !== tooltip.contentWindow || event.origin !== origin) return;
  const { type, height } = event.data;

  // do not break this, this highlight code is so odd
  if (type === "resize") {
    tooltip.style.height = `${height}px`;
    if (hovered) place();
  } else if (type === "parent" && hovered && selectable(hovered.parentElement)) {
    trail.push(hovered);
    select(hovered.parentElement);
  } else if (type === "child" && trail.length) {
    select(trail.pop());
  } else if (type === "lock" && hovered) {
    locked = true;
    chrome.runtime.sendMessage({
      type: "element",
      id,
      html: hovered.outerHTML,
      text: hovered.innerText || hovered.textContent || "",
      url: location.href,
    });
  } else if (type === "unlock") {
    locked = false;
    hide();
  }
});

chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  if (message.type !== "scan") return;
  sendResponse({
    html: document.documentElement.outerHTML,
    text: document.body?.innerText || "",
    url: location.href,
  });
});
