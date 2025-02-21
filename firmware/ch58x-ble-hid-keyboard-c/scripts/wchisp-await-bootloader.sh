#!/usr/bin/env sh

which wchisp >/dev/null 2>&1 || {
    echo "ERROR: 'wchisp' not found on PATH."
    exit 1
}

wchisp_available () {
  wchisp info >/dev/null 2>&1
  return $?
}

if ! wchisp_available; then
  echo "Waiting for WCH ISP bootloader..."
  while ! wchisp_available
  do
    sleep 1
  done
fi
