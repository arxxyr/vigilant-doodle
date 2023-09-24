#!/usr/bin/env sh

rm -rf AppIcon.iconset/*
mkdir -p AppIcon.iconset
convert  -resize 16x16      ../icon_1024x1024.png    AppIcon.iconset/icon_16x16.png     
convert  -resize 32x32      ../icon_1024x1024.png    AppIcon.iconset/icon_16x16@2x.png  
convert  -resize 32x32      ../icon_1024x1024.png    AppIcon.iconset/icon_32x32.png     
convert  -resize 64x64      ../icon_1024x1024.png    AppIcon.iconset/icon_32x32@2x.png  
convert  -resize 128x128    ../icon_1024x1024.png    AppIcon.iconset/icon_128x128.png   
convert  -resize 256x256    ../icon_1024x1024.png    AppIcon.iconset/icon_128x128@2x.png
convert  -resize 256x256    ../icon_1024x1024.png    AppIcon.iconset/icon_256x256.png   
convert  -resize 512x512    ../icon_1024x1024.png    AppIcon.iconset/icon_256x256@2x.png
convert  -resize 512x512    ../icon_1024x1024.png    AppIcon.iconset/icon_512x512.png   
convert  -resize 1024x1024  ../icon_1024x1024.png    AppIcon.iconset/icon_512x512@2x.png
# cp ../icon_1024x1024.png AppIcon.iconset/icon_512x512@2x.png
png2icns AppIcon.icns ./AppIcon.iconset/*
mkdir -p src/Game.app/Contents/Resources
mv AppIcon.icns src/Game.app/Contents/Resources/
