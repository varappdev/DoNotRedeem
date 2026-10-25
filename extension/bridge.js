export function listenToExtensionMessages(callback) {
  chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
    callback(JSON.stringify(message));
  });
}

export function locationHash() {
  return location.hash;
}

export function postToPage(type) {
  const height = Math.ceil(document.body.getBoundingClientRect().height);
  window.parent.postMessage({ type, height }, "*");
}

export function watchSize() {
  new ResizeObserver(() => postToPage("resize")).observe(document.body);
}

export async function requestPage() {
  console.info("Requesting page...")
  const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
  const page = JSON.stringify(await chrome.tabs.sendMessage(tab.id, { type: "scan" }));
  console.log(page)
  return page;
}

export async function getElementCheck() {
  const { elementCheck = false } = await chrome.storage.local.get("elementCheck");
  return elementCheck;
}

export async function setElementCheck(enabled) {
  await chrome.storage.local.set({ elementCheck: enabled });
}
