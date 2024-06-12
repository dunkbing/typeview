import { invoke } from "@tauri-apps/api/tauri";

type Config = {
  fontSize: number;
  padding: number;
  sound: string;
};

const soundSelect = document.getElementById(
  "sound-select",
) as HTMLSelectElement;
const fontSizeRange = document.getElementById(
  "font-size-range",
) as HTMLInputElement;
const paddingRange = document.getElementById(
  "padding-range",
) as HTMLInputElement;
const applyBtn = document.getElementById("apply-button") as HTMLButtonElement;

let config: Config = {
  fontSize: 24,
  padding: 10,
  sound: "key_1.mp3",
};

function updateSettings() {
  const sound = soundSelect.value;
  const fontSize = parseInt(fontSizeRange?.value || "0");
  const padding = parseInt(paddingRange?.value || "0");
  config = { sound, fontSize: fontSize, padding };
  invoke("update_settings", {
    sound,
    fontSize,
    padding,
  });
}

applyBtn.addEventListener("click", updateSettings);

soundSelect.addEventListener("change", updateSettings);

invoke("get_state").then((selectedSound) => {
  config = selectedSound as Config;
  console.log(config);
  soundSelect.value = config.sound;
  fontSizeRange.value = String(config.fontSize);
  paddingRange.value = String(config.padding);
});
