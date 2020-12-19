import {Mesh, Vector3} from "babylonjs";

import type {RenderItemDto} from "../../../../pkg";

import {BaseRenderer} from "../base-renderer";
import {AssetManager} from "../manager";
import {meshAssetGuard} from "../utils";

export class UnitRenderer extends BaseRenderer {
  
  private getMesh(glyph: String): Mesh {
    let name = "";
    
    switch (glyph) {
      default: 
        name = "unit.babylon"; 
    }

    let asset = meshAssetGuard(this.managerRef, name);
    return asset.data;
  }

  public async setup(): Promise<void>{
    await this.managerRef.getMeshByFilenames(["unit.babylon"]);
    this.setupCompleted = true;
  }

  public renderImpl(id: string, item: RenderItemDto) {
    let match = this.sceneRef.getMeshByName(id);

    if (match == null) {
      let child = this.getMesh(item.glyph).createInstance(id); 
      child.position = new Vector3(item.x * AssetManager.defaultSize, 0, item.y * AssetManager.defaultSize);
      return;
    }
    
    match.position = new Vector3(item.x * AssetManager.defaultSize, 0, item.y * AssetManager.defaultSize);
  }
}
