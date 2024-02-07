mkdir -Force -p icons
cd icons

# $smol_icon = "16.png"
# $medium_icon = "32.png"
# $icon_sizes = 40,48,64,96,128

# $extra_sizes = @()
# Foreach($r in $icon_sizes) {
#     magick convert -resize "${r}x${r}" -depth 8 -alpha off $medium_icon "${r}.png"
#     echo $r
#     $extra_sizes += "${r}.png"
# }

# # Combine all PNG image files into an icon.ico file
# echo $smol_icon $medium_icon $extra_sizes

# magick convert $smol_icon $medium_icon $extra_sizes "icon.ico"

# # Remove extra icons
# Foreach($extra in $extra_sizes) {
#     Remove-Item $extra
# }

magick.exe convert "16.png" "32.png" "32.png" "64.png" "96.png" "128.png" "256.png" "../icon.ico"

cd ..