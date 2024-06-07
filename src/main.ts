import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { appWindow, LogicalPosition, LogicalSize, currentMonitor } from "@tauri-apps/api/window";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});

document.addEventListener("mousedown", (e) => {
  if (e.button === 0) {
    appWindow.startDragging();
  }
});

async function adjustWindowSize() {
  const content = document.querySelector("#content") as HTMLDivElement;
  const keystrokeDivs = document.querySelectorAll(".keystroke") as NodeListOf<HTMLDivElement>;

  let totalWidth = content.offsetWidth;
  let totalHeight = content.offsetHeight;

  keystrokeDivs.forEach((keystrokeDiv) => {
    totalWidth = Math.max(totalWidth, keystrokeDiv.offsetWidth);
    totalHeight += keystrokeDiv.offsetHeight;
  });

  const width = totalWidth;
  const height = totalHeight;

  const { type } = await appWindow.innerSize();
  console.log("size type", type, width, height);

  const currentSize = await appWindow.innerSize();
  if (currentSize.width !== width || currentSize.height !== height) {
    await appWindow.setSize(new LogicalSize(width, height));
    centerWindow();
  }
}

async function centerWindow() {
  const screenSize = await currentMonitor();
  const windowSize = await appWindow.innerSize();

  const x = (screenSize!.size.width - windowSize.width) / 2;
  const y = (screenSize!.size.height - windowSize.height) / 2;

  appWindow.setPosition(new LogicalPosition(x, y));
}

window.addEventListener("load", adjustWindowSize);
window.addEventListener("resize", adjustWindowSize);

let lastKeystrokeTime = 0;
let keystrokeTimeout: number;

listen("update-content", function (e) {
  const currentTime = Date.now();
  const key = e.payload as string;

  if (currentTime - lastKeystrokeTime > 2000) {
    const keystrokeDiv = document.createElement("div");
    keystrokeDiv.classList.add("keystroke");
    keystrokeDiv.textContent = key;
    document.body.appendChild(keystrokeDiv);
  } else {
    const lastKeystrokeDiv = document.body.lastElementChild!;
    lastKeystrokeDiv.textContent += key;
  }

  lastKeystrokeTime = currentTime;
  adjustWindowSize();

  clearTimeout(keystrokeTimeout);
  keystrokeTimeout = setTimeout(() => {
    const keystrokeDivs = document.querySelectorAll(".keystroke");
    keystrokeDivs.forEach((keystrokeDiv) => {
      keystrokeDiv.classList.add("hide");
      setTimeout(() => {
        keystrokeDiv.remove();
        adjustWindowSize();
      }, 500);
    });
  }, 3000);
});
