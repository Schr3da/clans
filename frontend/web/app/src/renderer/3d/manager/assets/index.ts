import 'babylonjs-loaders';

import {Mesh, SceneLoader} from "babylonjs";

export type isLoadingCb = (isComplete: boolean) => void;

enum AssetType {
  mesh,
}

export interface IAsset<T>{
  data: T,
  type: AssetType;
}

type SupportedTypes = Mesh;

export const isMeshAsset = (data: IAsset<any>): data is IAsset<Mesh> =>
  data.type === AssetType.mesh;


export class AssetManager {

  private rootUrl: string;

  private path: string;

  private cache: Map<string, IAsset<SupportedTypes>>; 

  private isLoading: isLoadingCb 

  public static defaultSize = 5;

  constructor(isLoading: isLoadingCb) {
    this.rootUrl = "./";
    this.path = "assets/";
    this.cache = new Map<string, IAsset<SupportedTypes>>();
    this.isLoading = isLoading;
  }

  private async loadMesh(filename: string): Promise<void> {
    let path = this.path + "models/" + filename; 
    
    let result = await SceneLoader.ImportMeshAsync(null, this.rootUrl, path); 

    let mesh = <Mesh>result.meshes[0]; 
    mesh.doNotSyncBoundingInfo = true;
    mesh.isVisible = false;

    this.cache.set(filename, { 
      data: mesh,
      type: AssetType.mesh, 
    });
  }

  public getCachedAsset(filename: string): IAsset<SupportedTypes>| undefined {
    return this.cache.get(filename);
  }
  
  public async getMeshByFilename(filename: string): Promise<IAsset<Mesh>>{
    let asset = await this.getMeshByFilenames([filename]);
    return asset[0];
  }

  public async getMeshByFilenames(filenames: string[]): Promise<IAsset<Mesh>[]> {
    let refs: IAsset<Mesh>[] = [];

    this.isLoading(true);

    for (let i = 0; i<filenames.length; i++) {
      let filename = filenames[i];
      let asset = this.getCachedAsset(filename);
      
      if (asset == null) {
        await this.loadMesh(filename); 
      }

      asset = this.getCachedAsset(filename);
      if (asset == null) {
        throw new Error(`asset not found for ${filenames}`);
      }

      if (isMeshAsset(asset) == false) {
        throw new Error(`not a mesh asset request for mesh ${filenames}`);
      }

      refs.push(asset as IAsset<Mesh>); 
    }
    
    this.isLoading(false);
    
    return refs;
  }

  public dispose() {
    this.cache.forEach((asset, key) => { 
      if (isMeshAsset(asset)) {
        asset.data.dispose(true); 
        return;
      }

      this.cache.delete(key);
    });
  }

}
