import type {Scene} from "babylonjs";

import type {AssetManager} from "../manager";

export abstract class BaseRenderer {
  
  protected sceneRef: Scene;

  protected managerRef: AssetManager;
  
  protected setupCompleted: boolean;

  constructor(sceneRef: Scene, managerRef: AssetManager) {
    this.sceneRef = sceneRef
    this.managerRef = managerRef;
    this.setupCompleted = false;
  }
 
  protected setup() {
    this.setupCompleted = true;
  }

  public render(...params: any) {
    if (this.setupCompleted === false) {
      return;
    }
    this.renderImpl(...params);
  }

  protected abstract renderImpl(...params: any): void;
}
