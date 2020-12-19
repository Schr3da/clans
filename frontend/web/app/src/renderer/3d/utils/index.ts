import * as BABYLON from "babylonjs";

import {AssetManager} from "../manager";
import {IAsset, isMeshAsset} from "../manager/assets";

export const meshAssetGuard = (ref: AssetManager, name: string): IAsset<BABYLON.Mesh>  => {
    let asset = ref.getCachedAsset(name);
    
  if (asset == null) {
      throw new Error("Asset does not exist " + name);
    }

    if (isMeshAsset(asset) === false) {
      throw new Error("Asset is not a mesh"); 
    }

    return asset as IAsset<BABYLON.Mesh>;
}
