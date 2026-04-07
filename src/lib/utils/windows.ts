import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { getCurrentWindow } from "@tauri-apps/api/window";

async function switchWindow() {
  const newWindow = new WebviewWindow("editor", {
    url: "/editor",
    title: "initiation",
    fullscreen: true,
  });
  console.log("all work and no play makes jack a dull boy");

  newWindow.once("tauri://created", async () => {
    console.log("here we are");
    const current = getCurrentWindow();
    await current.close();
  });

  newWindow.once("tauri://error", (e) => {
    console.error("Ошибка создания окна:", e);
  });
}
export default switchWindow;
