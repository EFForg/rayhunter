#!/usr/bin/env bash
#
# Use a privileged docker container to install Rayhunter.  The --privilege
# flag is necessary for USB port access.
#

docker run --privileged ubuntu /bin/bash -l -c '
  set -e

  apt-get update
  apt-get -y install curl adb
  mkdir /tmp/rayhunter
  cd /tmp/rayhunter
  curl -LOs "https://github.com/EFForg/rayhunter/releases/latest/download/release.tar"
  curl -LOs "https://github.com/EFForg/rayhunter/releases/latest/download/release.tar.sha256"
  if ! sha256sum -c --quiet release.tar.sha256; then
      echo "Download corrupted! (╯°□°)╯︵ ┻━┻"
      exit 1
  fi

  tar -xf release.tar
  ./install.sh
'
