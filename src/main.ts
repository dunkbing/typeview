import { listen, Event } from "@tauri-apps/api/event";
import { KeyStoreEvent, keyStore } from "./key-history-store";

const createDOMElementForKey = (key: string) => `<span class="pressed-key">${key}</span>`

type KeyEvent = Event<string>

const addNormalizedKeyFromSystem = (event: KeyEvent) => {
  const { payload: key } = event

  keyStore.add(key)
}

const removeNormalizedKeyFromSystem = (event: KeyEvent) => {
  const { payload: key } = event

  keyStore.remove(key)
}

document.addEventListener("DOMContentLoaded", () => {
  document.addEventListener("contextmenu", event => {
    event.preventDefault()
  })

  listen("KeyPress", addNormalizedKeyFromSystem)
  listen("KeyRelease", removeNormalizedKeyFromSystem)

  keyStore.addEventListener(KeyStoreEvent.KEYS_UPDATED, ({ detail }) => {
    const keys = detail
    const content = document.getElementById('content') as HTMLDivElement;
    content.innerHTML = keys.map(createDOMElementForKey).join(" ")
  })
})
