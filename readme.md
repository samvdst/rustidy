# rustidy

rustidy forces you to find a home for your downloaded files. if you don't they are deleted after 14 days.

## installation macOS

1. build with `cargo build --release`
2. move the binary somewhere permanent `mv ~/dev/rustidy/target/release/rustidy ~/.cargo/bin/rustidy`
3. create a launch agent file `~/Library/LaunchAgents/com.username.rustidy.plist` and replace with your `username`

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple Computer//DTD PLIST 1.0//EN"
    "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>com.username.rustidy</string>
  <key>ServiceDescription</key>
  <string>Runs the ~/.cargo/bin/rustidy program</string>
  <key>ProgramArguments</key>
  <array>
    <string>/Users/username/.cargo/bin/rustidy</string>
  </array>
  <key>UserName</key>
  <string>username</string>
  <key>RunAtLoad</key>
  <true/>
  <key>StartInterval</key>
  <integer>3600</integer>
</dict>
</plist>
```

4. start the launch agent `launchctl load ~/Library/LaunchAgents/com.username.rustidy.plist`
