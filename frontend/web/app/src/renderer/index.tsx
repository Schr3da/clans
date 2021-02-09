import * as React from "react";

import {Renderer3D} from "./3d";
import {Renderer2D} from "./2d";
import {IRenderer} from "./shared.interfaces";

import type {ControlsManager} from "../manager";
import type {RenderItemDto} from "../../pkg";

import "./index.less";

interface IState {
  controls: ControlsManager | null;
}

interface IProps {
  is3D: boolean;
}

export class Renderer extends React.PureComponent<IProps, IState> {

  private renderRef: IRenderer; 

  constructor(props: IProps) {
    super(props);
    this.state = {
      controls: null,
    };
  }

  private initCallbacks() {
    window.map_tile_renderer = this.handleMapData; 
    window.building_renderer = this.handleBuildingData; 
    window.unit_renderer = this.handleUnitData;
    window.preview_renderer = this.handlePreviewData;
    window.selection_renderer = this.handleSelectionData; 
    window.render_cycle_completed = this.handleRenderCycleCompleted;
    window.path_builder_renderer = this.handlePathBuilderData;
  }

  private async loadComponents() {
    let wasm = await import("../../pkg"); 
    
    let module = await import("../manager/index");
    let controls = new module.ControlsManager();
    
    this.setState((state) => 
      ({...state, controls}),
      wasm.init_new_game
    );
  }

  private handleMapData = (id: string, item: RenderItemDto, isVisible: boolean) => {
    this.renderRef.handleMapData(id, item, isVisible);   
  }

  private handleBuildingData = (id: string, item: RenderItemDto, progress: number) => {
    this.renderRef.handleBuildingData(id, item, progress);   
  }

  private handleUnitData = (id: string, item: RenderItemDto) => {
    this.renderRef.handleUnitData(id, item);   
  }

  private handlePreviewData = (item: RenderItemDto) => {
    this.renderRef.handlePreviewData(item);
  }

  private handleSelectionData = (item: RenderItemDto) => {
    this.renderRef.handleSelectionData(item); 
  }

  private handlePathBuilderData = (data: Array<number>) => {
    this.renderRef.handlePathBuilderData(data);
  }

  private handleRenderCycleCompleted = () => {
    this.renderRef.handleRenderCycleCompleted(); 
  }

  public componentWillUnmount() {
    if (this.state.controls == null) {
      return;
    }
    this.state.controls.dispose();
  }

  public componentDidMount() {
    this.initCallbacks();
    this.loadComponents(); 
  }

  public render() {
    let is3D = this.props.is3D;
    let controls = this.state.controls;

    return (is3D ? <>
        <h1>Babylonjs based 3d Renderer</h1>
        <Renderer3D ref={(r) => this.renderRef = r} controls={controls}/> 
      </> : <>
        <h1>React based 2d Renderer</h1>
        <Renderer2D ref={(r) => this.renderRef = r}controls={controls}/>
      </>
    );
  }
}
