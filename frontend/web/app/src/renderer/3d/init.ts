import {AssetManager} from "./manager";
import {GameScene} from "./scene";
import {BuildingRenderer} from "./building";
import {MapRenderer} from "./map";
import {PreviewRenderer} from "./preview";
import {SelectionRenderer} from "./selection";
import {UnitRenderer} from "./unit";

import type {IRenderer} from "../shared.interfaces";
import type {ControlsManager} from "../../manager";
import type {RenderItemDto} from "../../../pkg";

export class Renderer implements IRenderer{

  private scene: GameScene;

  private assets: AssetManager;
  
  private map: MapRenderer;
  
  private building: BuildingRenderer;
  
  private unit: UnitRenderer;
  
  private preview: PreviewRenderer;
  
  private selection: SelectionRenderer;

  constructor(canvas: HTMLCanvasElement, controls: ControlsManager) {
    this.scene = new GameScene(canvas, controls);
    this.assets = new AssetManager(this.isLoadingAsset); 
    this.map = new MapRenderer(this.scene.self, this.assets);
    this.building = new BuildingRenderer(this.scene.self, this.assets);
    this.unit = new UnitRenderer(this.scene.self, this.assets);
    this.preview = new PreviewRenderer(this.scene.self, this.assets);
    this.selection = new SelectionRenderer(this.scene.self, this.assets);
  }

  private isLoadingAsset = (_isLoaded: boolean) => {}

  public async setup() {  
    await this.map.setup();
    await this.building.setup();
    await this.unit.setup();
    await this.preview.setup();
    await this.selection.setup();
  }

  public handleMapData(id: string, item: RenderItemDto, isVisible: boolean) {
    if (this.map == null) {
      return;
    }
    this.map.render(id, item, isVisible);
  }

  public handleBuildingData(id: string, item: RenderItemDto, progress: number) {
    if (this.building == null) {
      return;
    }
    this.building.render(id, item, progress); 
  }

  public handleUnitData(id: string, item: RenderItemDto) {
    if (this.unit == null) {
      return;
    }
    this.unit.render(id, item); 
  }

  public handlePreviewData(item: RenderItemDto) {
    if (this.preview == null) {
      return;
    }
    this.preview.render(item); 
  }

  public handleSelectionData(item: RenderItemDto) {
    if (this.selection == null) {
      return;
    }
    this.selection.render(item); 
  }

  public handlePathBuilderData(id: string | null, item: RenderItemDto | null) {}

  public handleRenderCycleCompleted() {}

  public dispose() {
    this.scene.dispose();
    this.scene = null;

    this.assets.dispose(); 
    this.assets = null;
  }
}
