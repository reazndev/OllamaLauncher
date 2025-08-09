import { listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";

listen("show-launcher", () => {
  appWindow.show();
  appWindow.setFocus();
});
