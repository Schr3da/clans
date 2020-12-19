import {Mesh, Vector3} from "babylonjs";

import type {RenderItemDto} from "../../../../pkg";

import {BaseRenderer} from "../base-renderer";
import {AssetManager} from "../manager";
import {meshAssetGuard} from "../utils";

export class MapRenderer extends BaseRenderer{
  
  private getMesh(glyph: String): Mesh {
    let name = "";
    
    switch (glyph) {
      case "#":
        name = "tree.babylon";
        break;
      case ".":
        name = "ground_plain.babylon";
        break;
      default: 
        name = "ground_plain.babylon"; 
    }

    let asset = meshAssetGuard(this.managerRef, name); 
    return asset.data; 
  }

  public async setup(): Promise<void>{
    await this.managerRef.getMeshByFilenames(["untitled.babylon", "tree.babylon", "ground_plain.babylon"]);
    super.setup();
  }

  protected renderImpl(id: string, item: RenderItemDto, _isVisible: boolean) {
    let match = this.sceneRef.getMeshByName(id);

    if (match == null) {
      let child = this.getMesh(item.glyph).createInstance(id); 
      child.position = new Vector3(item.x * AssetManager.defaultSize, 0, item.y * AssetManager.defaultSize);
      return;
    }
  
    match.isVisible = true;
  }
}
