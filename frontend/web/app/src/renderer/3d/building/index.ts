import {Mesh, Vector3} from "babylonjs";

import type {RenderItemDto} from "../../../../pkg";

import {BaseRenderer} from "../base-renderer";
import {AssetManager} from "../manager";
import {meshAssetGuard} from "../utils";

export class BuildingRenderer extends BaseRenderer {
  
  private getMesh(glyph: String): Mesh {
    let name = "";
    
    switch (glyph) {
      default: 
        name = "building.babylon"; 
    }

    let asset = meshAssetGuard(this.managerRef, name); 
    return asset.data; 
  }

  public async setup(): Promise<void>{
    await this.managerRef.getMeshByFilenames(["building.babylon"]);
    super.setup();
  }

  protected renderImpl(id: string, item: RenderItemDto, _progress: number) {
    let match = this.sceneRef.getMeshByName(id);

    if (match == null) {
      let child = this.getMesh(item.glyph).createInstance(id); 
      child.position = new Vector3(item.x * AssetManager.defaultSize, 0, item.y * AssetManager.defaultSize);
      return;
    }
  }
}
