import * as BABYLON from "babylonjs";

import type {RenderItemDto} from "../../../../pkg";

import {BaseRenderer} from "../base-renderer";

export class MapRenderer extends BaseRenderer{
  
  public async setup(): Promise<void>{
    
    const options = {
      width: 2048,
      height: 2048,
    };

    const mesh = BABYLON.MeshBuilder.CreatePlane("ground", options, this.sceneRef);
    mesh.rotateAround(new BABYLON.Vector3(0, 0, 0), new BABYLON.Vector3(1, 0, 0), BABYLON.Tools.ToRadians(0));

    const material = new BABYLON.StandardMaterial("myMaterial", this.sceneRef);
    material.diffuseColor = BABYLON.Color3.FromHexString("#F9F7EB");
    material.specularColor = BABYLON.Color3.FromHexString("#F9F7EB");
    material.emissiveColor = BABYLON.Color3.FromHexString("#F9F7EB");
    material.ambientColor = BABYLON.Color3.FromHexString("#F9F7EB");
    
    mesh.material = material;

    super.setup();
  }
  
  protected renderImpl(_id: string, _item: RenderItemDto, _isVisible: boolean) {

  }
}
