import {BaseRenderer} from "../base-renderer";

import type {RenderItemDto} from "../../../../pkg";

export class SelectionRenderer extends BaseRenderer {
  
  public async setup(): Promise<void>{
    await this.managerRef.getMeshByFilename("untitled.babylon");
    super.setup();
  }
  
  protected renderImpl(item: RenderItemDto) {
    if(item == null) {
      return;
    }
  }
}
