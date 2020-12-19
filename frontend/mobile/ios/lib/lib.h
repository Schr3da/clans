#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Data Data;

typedef struct {
  wchar_t glyph;
  int32_t x;
  int32_t y;
  int32_t width;
  int32_t height;
} RenderItemDto;

void free_game_state(Data *data);

Data *init_game_state(void (*map_tile_renderer)(RenderItemDto, bool),
                      void (*building_renderer)(RenderItemDto, int64_t),
                      void (*unit_renderer)(RenderItemDto),
                      void (*preview_renderer)(RenderItemDto),
                      void (*selection_renderer)(RenderItemDto));

void on_key_up(void);

void on_touch_down(Data *data, bool is_left_click, int32_t x, int32_t y);

void on_touch_move(Data *data, int32_t x, int32_t y);

void render_state(Data *data);

void update_state(Data *data);
