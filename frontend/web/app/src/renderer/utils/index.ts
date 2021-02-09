const rows = 20;
const columns = 20;

export const defaultMapSize = (
  tileWidth: number = 1,
  tileHeight: number = 1,
): {
  width: number, height: number
}  => {
  return {
    width: columns * tileWidth,
    height: rows * tileHeight,
  }
};

export const mapIndexToCoordinates = (index: number) => {
  let x = index % columns;
  let y = Math.floor(index / columns);
  return { x, y };
}