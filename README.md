# Minesweeper
A minesweeper clone written in Rust with the egor-graphics-engine

## Known issues
* The resizing after selecting a medium or hard game still doesn't work correct. 
  It seems to be a Wayland problem, when using XWayland the font scaling is
  correct (Write ´´´WAYLAND_DISPLAY="" cargo run´´´ on the command line).
