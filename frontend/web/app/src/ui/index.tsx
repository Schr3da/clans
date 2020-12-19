import * as React from "react";

import {ActionMenu} from "./game";

interface IProps {
  onToggleRenderer: () => void;
}

export const UI = (props: IProps) => <>
  <ActionMenu/>

  <div style={{
    position: "fixed",
    bottom: 10,
    zIndex: 999,
    left: 0,
    right: 0,
    textAlign: "center",
  }}>
    <button onClick={props.onToggleRenderer}>
      Toggle Renderer
    </button>
  </div>
</>;
