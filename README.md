# Addmin DLL
Simple DLL that adds a new local administrator user, executes arbitrary commands, and logs output to a text file for validation.

Will write a file to the directory `c:\temp` named `pwned_{pid}.txt`.
- `{pid}` = pid of process injected.

File content example:
```
[*]          Pid: 10368
[*]      Process: "C:\\WINDOWS\\system32\\rundll32.exe"
[*]         Args: ["addmin.dll,DllMain"]
[*]         User: user
[*]       Domain: DESKTOP-2LBLGJR
[*] Created file: "c:\\pwned\\pwned_10368.txt"

[*] Whoami Output:
desktop-2lblgjr\user

[*] IP Config:
Windows IP Configuration


Ethernet adapter Ethernet0:

   Connection-specific DNS Suffix  . : localdomain
   Link-local IPv6 Address . . . . . : fe80::bb7e:e347:d059:897d%3
   IPv4 Address. . . . . . . . . . . : 192.168.65.135
   Subnet Mask . . . . . . . . . . . : 255.255.255.0
   Default Gateway . . . . . . . . . : 192.168.65.2

Ethernet adapter Bluetooth Network Connection:

   Media State . . . . . . . . . . . : Media disconnected
   Connection-specific DNS Suffix  . :

[*] Net User Add Output:
The command completed successfully.

[*] Net Group Add Output:
The command completed successfully.
```

## Compilation
**From WSL/Linux (Cross-compile for Windows):**
```
# Install cross-compilation toolchain (one-time setup)
sudo apt install mingw-w64
rustup target add x86_64-pc-windows-gnu

# Build
cargo build --release --target x86_64-pc-windows-gnu
```
Output: `target/x86_64-pc-windows-gnu/release/addmin.dll`

**From Windows (Native):**
```
cargo build --release
```
Output: `target\release\addmin.dll`

