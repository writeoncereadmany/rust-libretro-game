mkdir -p package-pi

cp target/release/libpandamonium.so package-pi/pandamonium_libretro.so
cp pandamonium_libretro.info package-pi/
cp pandamonium.panda package-pi/
