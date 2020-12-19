export enum ControlActionTypes {
  ON_SHOW_PREVIEW_MINE = "m",
  ON_SHOW_PREVIEW_FARM = "h",
  ON_SHOW_PREVIEW_FACTORY = "f",
  ON_INIT_PATH = "p",
  ON_CANCEL = "Escape",
}

export const triggerControlAction = (action: ControlActionTypes) => {
  if (window.triggerControlAction == null) {
    throw new Error("window.triggerControlAction is undefined. Ensure you have initialised it");
  }
  window.triggerControlAction(action);
}
