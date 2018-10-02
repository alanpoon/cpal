ndk-build

cp -r $GSTREAMER_ROOT_ANDROID/arm/lib/pkgconfig gst-build-armeabi/
mkdir gstream_out

for D in gst-build-armeabi
do
  echo 'Processing '$D
  cd $D
  sed -i -e 's?libdir=.*?libdir='`pwd`'?g' pkgconfig/*
  sed -i -e 's?.* -L${.*?Libs: -L${libdir} -lgstreamer_android?g' pkgconfig/*
  sed -i -e 's?Libs:.*?Libs: -L${libdir} -lgstreamer_android?g' pkgconfig/*
  sed -i -e 's?Libs.private.*?Libs.private: -lgstreamer_android?g' pkgconfig/*
done