export const defaultMapSize = (
  tileWidth: number = 1,
  tileHeight: number = 1,
): {
  width: number, height: number
}  => {
  const rows = 100;
  const columns = 100;

  return {
    width: columns * tileWidth,
    height: rows * tileHeight,
  }
}
