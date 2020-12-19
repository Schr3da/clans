import * as BABYLON from "babylonjs";

import type {ControlsManager} from "../../../manager";

import {AssetManager} from "../manager";

export class GameScene {
  
  public self: BABYLON.Scene;
  
  private canvasRef: HTMLCanvasElement;
  
  private controlsRef: ControlsManager;

  private engine: BABYLON.Engine;

  private camera: BABYLON.ArcRotateCamera;

  private didMouseMove: boolean;

  constructor(canvasRef: HTMLCanvasElement, controlsRef: ControlsManager) {
    this.canvasRef = canvasRef;
    this.controlsRef = controlsRef;
    
    this.engine = new BABYLON.Engine(this.canvasRef, true); 
    
    this.self = new BABYLON.Scene(this.engine);
    this.self.shadowsEnabled = false;
    this.self.clearColor = new BABYLON.Color4(0.45, 0.77, 0.99, 1); 

    this.camera = new BABYLON.ArcRotateCamera("camera", -Math.PI / 2, Math.PI / 2.5, 3, new BABYLON.Vector3(40, 0, 40), this.self);
    this.camera.attachControl(canvasRef, true);

    new BABYLON.HemisphericLight("default_light", new BABYLON.Vector3(1, 1, 0), this.self);
  
    this.self.onPointerObservable.add(this.handlePointer);
    this.engine.runRenderLoop(() => this.self.render());
  }

  private handlePointer = (event: BABYLON.PointerInfo) => {
    if(event.type === BABYLON.PointerEventTypes.POINTERDOWN) {
      return this.pointerDown(event.pickInfo);
    }

    if(event.type === BABYLON.PointerEventTypes.POINTERMOVE) {
      return this.pointerMove(event.pickInfo);
    }
    
    if(event.type === BABYLON.PointerEventTypes.POINTERUP) {
      return this.pointerUp(event.pickInfo);
    }
  }

  private pointerDown(_info: BABYLON.PickingInfo) {
    this.didMouseMove = false; 
  }

  private pointerMove(_info: BABYLON.PickingInfo) {
    this.didMouseMove = true;
  }

  private pointerUp(info: BABYLON.PickingInfo) {
    if (this.didMouseMove === true) {
      this.didMouseMove = false;
      return;
    }

    if (info == null) {
      console.log("pick info is null pointerDown");
      return;
    }
   
    let position = info.pickedMesh.position; 
    let x = Math.floor(position.x / AssetManager.defaultSize);
    let y = Math.floor(position.z / AssetManager.defaultSize);
    this.controlsRef.setLeftMouseButtonUpEvent(x, y);
  }

  public dispose() {
    this.camera.dispose();
    this.camera = null;

    this.self.dispose();
    this.self = null;

    this.engine.stopRenderLoop();
    this.engine.dispose();
    this.engine = null;
  
    this.controlsRef = null;
    this.canvasRef = null;
  }

}
