import * as React from "react";

import {ControlActionTypes, triggerControlAction} from "../../manager/controls/action";

import "./index.less";

export class ActionMenu extends React.Component<unknown, unknown>{
 
  constructor(props: unknown) {
    super(props);
  }

  public componentDidMount() {
    window.resources_renderer = this.resource_update; 
  }

  private resource_update = (food: number, materials: number) => {
    console.log("food: " + food +  " materials:" + materials);
  }

  private handlePreview = (action: ControlActionTypes) => 
    triggerControlAction(action);

  public render() {
    return(
      <div className="game-action-menu">
        <button onClick={() => this.handlePreview(ControlActionTypes.ON_SHOW_PREVIEW_FACTORY)}>Factory</button> 
        <button onClick={() => this.handlePreview(ControlActionTypes.ON_SHOW_PREVIEW_MINE)}>Mine</button> 
        <button onClick={() => this.handlePreview(ControlActionTypes.ON_SHOW_PREVIEW_FARM)}>Farm</button> 
        <button onClick={() => this.handlePreview(ControlActionTypes.ON_INIT_PATH)}>Path</button> 
      </div>
    );
  }

}
