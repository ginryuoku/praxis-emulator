Generation 1:

black-and-white, 1bpp, 4 page framebuffer

memory map:
0x400000 page 0
0x410000 page 1
0x420000 page 2
0x430000 page 3

Each page is chopped into 4096-byte segments. The control segment is at 0x4nF000-0x4nFFFF.

Control segment:
- u16: horizontal res
- u16: vertical res
- u8: text columns
- u8: text rows
- u8: control bits 0
    bit 0: landscape 0, portrait 1
    bit 1+2+3: page type 
        text:          000
        graphics:      001
        text buffer 0: 010
        text buffer 1: 011 
        8x8 atlas:     100
        16x16 atlas:   101
        32x32 atlas:   110 
        64x64 atlas:   111
    bit 4: text cursor     - off 0 on 1
    bit 5: graphics cursor - off 0 on 1
    bit 6: reserved
    bit 7: reserved
- u8: control bits 1
    bit 0: reserved
    bit 1: reserved
    bit 2: reserved
    bit 3: reserved
    bit 4: reserved
    bit 5: reserved
    bit 6: reserved
    bit 7: reserved
- u16: text cursor x
- u16: text cursor y
- u16: graphics cursor x
- u16: graphics cursor y
- u16: cursor glyph

command:
drawpoint
drawline
drawrect
drawroundrect
drawcircle
clear(point/line/rect/roundrect/circle)
clearpage
clearpagecontents
updatetextcursor
updategraphicscursor
changepage
