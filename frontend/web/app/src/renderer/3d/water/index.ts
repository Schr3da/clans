import * as BABYLON from "babylonjs";

import {defaultMapSize} from "../../utils";

import {AssetManager} from "../manager";

export class WaterRenderer {
  
  private base: BABYLON.Mesh;
  
  private material: BABYLON.StandardMaterial;

  constructor(scene: BABYLON.Scene) {
    let defaultSize = defaultMapSize(AssetManager.defaultSize, AssetManager.defaultSize);

    this.base = BABYLON.Mesh.CreateGround("water_mesh", defaultSize.width, defaultSize.height, 1, scene);
    this.base.position = new BABYLON.Vector3(defaultSize.width * 0.5, -1, defaultSize.height * 0.5);
    
    this.material = new BABYLON.StandardMaterial("water_bump_map", scene);
    this.material.bumpTexture = new BABYLON.Texture("./assets/images/png/water_bump_map_1.png", scene); 
    this.material.invertNormalMapX = true;
    this.material.invertNormalMapY = true
    this.base.material = this.material;
  }

  public dispose() {
    this.base.dispose();
    this.base = null;

    this.material.dispose()
    this.material = null;
  }

}
