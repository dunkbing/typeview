import { listen } from "@tauri-apps/api/event";
import {
  appWindow,
  LogicalPosition,
  LogicalSize,
  currentMonitor,
} from "@tauri-apps/api/window";

const content = document.getElementById("content") as HTMLDivElement;

document.addEventListener("mousedown", (e) => {
  if (e.button === 0) {
    appWindow.startDragging();
  }
});

async function adjustWindowSize(extraWidth: number = 0) {
  const keystrokeDivs = document.querySelectorAll(
    ".keystroke",
  ) as NodeListOf<HTMLDivElement>;

  let totalWidth = content.offsetWidth + extraWidth;

  keystrokeDivs.forEach((keystrokeDiv) => {
    totalWidth = Math.max(totalWidth, keystrokeDiv.offsetWidth + extraWidth);
  });

  const width = totalWidth;
  const height = content.offsetHeight;

  const currentSize = await appWindow.innerSize();
  if (currentSize.width !== width || currentSize.height !== height) {
    await appWindow.setSize(new LogicalSize(width, height));
    await centerWindow();
  }
}

async function centerWindow() {
  const screenSize = await currentMonitor();
  const windowSize = await appWindow.innerSize();

  const x = (screenSize!.size.width - windowSize.width) / 2;
  const y = (screenSize!.size.height - windowSize.height) / 2;

  await appWindow.setPosition(new LogicalPosition(x, y));
}

let resizeTimeout: number;
window.addEventListener("load", () => {
  clearTimeout(resizeTimeout);
  resizeTimeout = setTimeout(() => adjustWindowSize(), 100);
});
window.addEventListener("resize", () => {
  clearTimeout(resizeTimeout);
  resizeTimeout = setTimeout(() => adjustWindowSize(), 100);
});

let lastKeystrokeTime = 0;
let keystrokeTimeout: number;
let updateTimeout: number | null = null;

function updateKeystrokeText(key: string) {
  const currentTime = Date.now();

  if (currentTime - lastKeystrokeTime > 1000) {
    const keystrokeDiv = document.createElement("div");
    keystrokeDiv.classList.add("keystroke");
    keystrokeDiv.textContent = key;
    content.appendChild(keystrokeDiv);
  } else {
    const lastKeystrokeDiv = content.lastElementChild as HTMLDivElement;
    lastKeystrokeDiv.textContent += key;
  }

  lastKeystrokeTime = currentTime;
}

listen("update-content", async function (e) {
  const key = e.payload as string;

  updateKeystrokeText(key);
  adjustWindowSize();

  if (updateTimeout) {
    clearTimeout(updateTimeout);
  }

  clearTimeout(keystrokeTimeout);
  keystrokeTimeout = window.setTimeout(clearKeystrokes, 2000);
});

function clearKeystrokes() {
  const keystrokeDivs = document.querySelectorAll(".keystroke");
  let delay = 0;

  keystrokeDivs.forEach((keystrokeDiv) => {
    window.setTimeout(() => {
      keystrokeDiv.classList.add("hide");
      window.setTimeout(() => {
        keystrokeDiv.remove();
        adjustWindowSize();
      }, 500);
    }, delay);

    delay += 500;
  });
}
