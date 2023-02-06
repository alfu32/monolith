#!/bin/bash

# see https://github.com/denoland/rusty_v8/releases

REL=$1
ARCHVNDSYS=$2
if [ -n "$ARCHVNDSYS" ]; then
  echo "ARCHVNDSYS is -n $ARCHVNDSYS"
else
  ARCHVNDSYS=x86_64-unknown-linux-gnu
  echo "ARCHVNDSYS is not -n $ARCHVNDSYS"
fi

#for REL in v0.60.1 v0.61.0 v0.62.2; do
  mkdir -p $RUSTY_V8_MIRROR/$REL
  for FILE in \
    librusty_v8_debug_$ARCHVNDSYS.a \
    librusty_v8_release_$ARCHVNDSYS.a \
  ; do
    if [ ! -f $RUSTY_V8_MIRROR/$REL/$FILE ]; then
      wget -O $RUSTY_V8_MIRROR/$REL/$FILE \
        https://github.com/denoland/rusty_v8/releases/download/$REL/$FILE
    fi
  done
#done

