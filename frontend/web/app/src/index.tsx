import * as React from "react";
import * as ReactDom from "react-dom";

import {Renderer} from "./renderer";
import {UI} from "./ui";

interface IState {
  is3D: boolean;
}

class Entry extends React.PureComponent<unknown, IState> {
  
  constructor(props: unknown) {
    super(props);

    this.state = {
      is3D: false,
    };
  }

  private toggleRenderer = () => {
    this.setState((state) => ({
      is3D: !state.is3D,
    }));
  }

  public render() {
    return (
      <div>
        <Renderer is3D={this.state.is3D}/>
        <UI onToggleRenderer={this.toggleRenderer}/> 
      </div>
    );
  }
}

let container = document.createElement("div");
document.body.append(container);

ReactDom.render(<Entry/>, container);
