require("colorbuddy").setup()

local Color = require('colorbuddy.init').Color
local colors = require('colorbuddy.init').colors
-- get none
local none = require('colorbuddy.color').colors.none
local Group = require('colorbuddy.init').Group
local groups = require('colorbuddy.init').groups
local styles = require('colorbuddy.init').styles

Color.new('white',     '#f2e5bc')
Color.new('black',     '#000000')
Color.new('red',       '#cc6666')
Color.new('pink',      '#fef601')
Color.new('green',     '#99cc99')
Color.new('yellow',    '#f8fe7a')
Color.new('blue',      '#81a2be')
Color.new('aqua',      '#8ec07c')
Color.new('cyan',      '#8abeb7')
Color.new('purple',    '#8e6fbd')
Color.new('violet',    '#b294bb')
Color.new('orange',    '#de935f')
Color.new('brown',     '#a3685a')
Color.new('selection',     '#0000aa')

Color.new('seagreen',  '#698b69')
Color.new('turquoise', '#698b69')

Group.new("Normal", colors.white, colors.black)
Group.new("Cursor", groups.normal.bg, groups.normal.fg)
-- Group.new("CursorLine", colors.red, colors.blue)
Group.new("Visual", colors.black, colors.selection)
-- defaults
Color.new("CursorLineNrColor", "#f8fe7a")
Group.new("SignColumn", groups.SignColumn.fg, none)
Group.new("EndOfBuffer", groups.EndOfBuffer.fg, none)
Group.new("ColorColumn", groups.SignColumn.fg, none)
Group.new("CusorLineNr", colors.CursorLineNrColor, none, styles.bold)
Group.new("LineNr", colors.grey, none)
