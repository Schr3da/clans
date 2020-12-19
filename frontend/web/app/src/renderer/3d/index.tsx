import * as React from "react";

import type {RenderItemDto} from "../../../pkg";
import type {ControlsManager} from "../../manager";
import type {IRenderer} from "../shared.interfaces";
import type {Renderer} from "./init";

import "./index.less";

interface IProps {
  controls: ControlsManager;
}

export class Renderer3D extends React.PureComponent<IProps, unknown> implements IRenderer {
  
  private canvasRef: HTMLCanvasElement;
  
  private renderer: Renderer;

  public componentDidMount() {
    import("./init").then((module) => {
      this.renderer = new module.Renderer(this.canvasRef, this.props.controls); 
      this.renderer.setup();
    });
  }

  public componentWillUnmount() {
    this.renderer.dispose();
  }

  public handleMapData(id: string, item: RenderItemDto, isVisible: boolean) {
    if (this.renderer == null) {
      return;
    }
    this.renderer.handleMapData(id, item, isVisible);
  }

  public handleBuildingData(id: string, item: RenderItemDto, progress: number) {
    if (this.renderer == null) {
      return;
    }
    this.renderer.handleBuildingData(id, item, progress);
  }

  public handleUnitData(id: string, item: RenderItemDto) {
    if (this.renderer == null) {
      return;
    }
    this.renderer.handleUnitData(id, item);
  }

  public handlePreviewData(item: RenderItemDto) {
    if (this.renderer == null) {
      return;
    }
    this.renderer.handlePreviewData(item);
  }

  public handleSelectionData(item: RenderItemDto) {
    if (this.renderer == null) {
      return;
    }
    this.renderer.handleSelectionData(item);
  }

  public handleRenderCycleCompleted() {
    if (this.renderer == null) {
      return;
    }
    this.renderer.handleRenderCycleCompleted();
  }

  public render(): JSX.Element {
    return <div className="renderer-3d-wrapper">
      <canvas
        id="renderer-canvas"
        className="canvas-component"
        ref={(r) => this.canvasRef = r}
      />
    </div>;
  }
}
