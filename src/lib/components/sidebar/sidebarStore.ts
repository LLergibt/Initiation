import type { Pane } from "paneforge";
import { get, writable } from "svelte/store";
import type { PaneAPI } from "paneforge";
// this store now is used generally for rewriting some adaptive logic of paneforge library and prevent sidebar to resizing

export const sidebarStore = writable<PaneAPI | null>();

export const preventGrow = async (containerWidth: number) => {
  const newPercent = (sidebarWidthPx / containerWidth) * 100;
  sidebarStore.update((pane) => {
    if (pane) {
      pane.resize(newPercent);
      return pane;
    }
    return pane;
  });
};

let sidebarWidthPx = 224;
export const handleResize = async (
  sizeInPercent: number,
  containerWidth: number,
) => {
  console.log("resize func");
  if (containerWidth) {
    sidebarWidthPx = (sizeInPercent / 100) * containerWidth;
  }
};
