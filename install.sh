#!/bin/bash
echo "Hi! I'll help you to automate installing len from sources"
echo "Simply, I will just build it from source and will install it into your system"
echo
echo "Building len (running cargo build --release)..."
cargo build --release

if [ $? -eq 0 ]; then
  echo "Attempting to install len (requires system authentication)"
  sudo cp target/release/len /usr/bin/len
  sudo chmod 777 /usr/bin/len
  echo "Great!!, len v0.1.0 is successfully installed."
  echo "Remember, to give the repo a star on https://github.com/vulpheonix/len, cause this encourages me to do more for the opensource community :)"
else
  echo "Cargo build failed!"
  echo "Correct any error, if you can."
  echo "OR Open up an issue on https://github.com/vulpheonix/len along with the current terminal output, to get it instantly fixed and get your installation of len up and running ;)"
fi


