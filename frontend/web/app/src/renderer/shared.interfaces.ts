import type {
  KeyboardEventDto,
  MouseEventDto,
  RenderItemDto
} from "../../pkg";

import type {ControlActionTypes} from "../manager/controls/action";

declare global {
    interface Window { 
      map_tile_renderer: (id: string, item: RenderItemDto, isVisible: boolean, index?: number, total?: number) => void; 
      building_renderer: (id: string, item: RenderItemDto, progress: number, index?: number, total?: number) => void; 
      unit_renderer: (id: string, item: RenderItemDto, index?: number, total?: number) => void;
      preview_renderer: (item: RenderItemDto | null) => void;
      selection_renderer: (item: RenderItemDto | null) => void;
      resources_renderer: (food: number, materials: number) => void;
      render_cycle_completed: () => void;
      mouse_event: () => MouseEventDto| null;
      keyboard_event: () => KeyboardEventDto| null;
      triggerControlAction: (action: ControlActionTypes) => void; 
    }
}

export interface IRenderer {
  handleMapData: (id: string, item: RenderItemDto, isVisible: boolean) => void;
  handleBuildingData: (id: string, item: RenderItemDto, progress: number) => void;
  handleUnitData: (id: string, item: RenderItemDto) => void;
  handlePreviewData: (item: RenderItemDto) => void;
  handleSelectionData: (item: RenderItemDto) => void;
  handleRenderCycleCompleted: () => void;
}
