import * as React from "react";

import "./index.less";

interface IProps {
  id: string;
  x: number;
  y: number;
  glyph: string;
  isVisible: boolean
  onClick: (x: number, y: number) => void
}

export class Column2d extends React.PureComponent<IProps, unknown> {

  static padding = 6;
  static width = 10;
  static height = 10;

  constructor(props: IProps) {
    super(props);
  }

  private paramsToStyles() {
    return {
      "width": Column2d.width,
      "height": Column2d.height,
      "padding": Column2d.padding
    };
  }

  private handleClick = () => {
    const {x, y} = this.props;
    this.props.onClick(x, y);
  }

  public render() {
    const {glyph, isVisible} = this.props;
    return (
      <div 
        className={`grid-column-2d ${isVisible ? "is-visible" : ""}`} 
        style={this.paramsToStyles()}
        onClick={this.handleClick}
      >
        {glyph}
      </div>
    );
  }

}
