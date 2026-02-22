mkdir -p package-pi

cp target/release/libpandamonium.so package-pi/pandamonium-libretro.so
cp pandamonium-libretro.info package-pi/
cp pandamonium.panda package-pi/
