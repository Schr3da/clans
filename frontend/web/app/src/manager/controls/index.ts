import {
  KeyboardEventDto,
  KeyboardEventType,
  MouseButtonType,
  MouseEventDto,
  MouseEventType
} from "../../../pkg";

import {ControlActionTypes} from "./action";

export class ControlsManager {
  
  private mouseEvent: MouseEventDto | null;

  private keyboardEvent: KeyboardEventDto | null;

  constructor() {
    window.mouse_event = this.handleMouseEvent;
    window.keyboard_event = this.handleKeyboardEvent;
    window.triggerControlAction = this.setConrolAction;
  }
  
  private handleMouseEvent = (): MouseEventDto => {
    if (this.mouseEvent == null) {
      return null;
    }

    let event = MouseEventDto.new(
      this.mouseEvent.event_type,
      this.mouseEvent.button_type,
      this.mouseEvent.x,
      this.mouseEvent.y
    );

    this.mouseEvent = null;
    return event;
  };

  private handleKeyboardEvent = () => {
    if (this.keyboardEvent == null) {
      return null;
    }

    let event = KeyboardEventDto.new(
      this.keyboardEvent.event_type,
      this.keyboardEvent.keycode
    );
  
    this.keyboardEvent = null;
    return event;
  }

  private setMouseEvent(eventType: MouseEventType, buttonType: MouseButtonType, x: number, y: number) {
    this.mouseEvent = MouseEventDto.new(eventType, buttonType, x, y);
  }
  
  private setMouseUpEvent(buttonType: MouseButtonType, x: number, y: number) {
    this.setMouseEvent(MouseEventType.ButtonUp, buttonType, x, y);
  }
 
  public setMouseMoveEvent(x: number, y: number) {
    this.setMouseEvent(MouseEventType.Move, MouseButtonType.None, x, y);
  }

  public setLeftMouseButtonUpEvent(x: number, y: number) {
    this.setMouseUpEvent(MouseButtonType.Left, x, y);
  }

  public setRightMouseButtonUpEvent(x: number, y: number) {
    this.setMouseUpEvent(MouseButtonType.Right, x, y);
  }

  public setConrolAction = (action: ControlActionTypes) => {
    // Control actions are simulated keyboard events which map UI based actions to keyboard events
    this.keyboardEvent = KeyboardEventDto.new(KeyboardEventType.KeyUp, action);
  }

  public setKeyboardEvent(eventType: KeyboardEventType, keycode: string) {
    this.keyboardEvent = KeyboardEventDto.new(eventType, keycode);
  }
  
  public dispose() {
    this.mouseEvent = null;
    this.keyboardEvent = null;
  }

}
