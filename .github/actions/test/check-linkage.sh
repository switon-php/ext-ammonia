#!/usr/bin/env bash
# The extension must be one file. System libraries may show up.
# A second project library must not.
set -euo pipefail

file=${1:?library path}
os=$(uname -s)

if [[ "$os" == Darwin ]]; then
  deps=$(otool -L "$file" | awk 'NR>2 { print $1 }')
  allowed='^(libc|libm|libgcc_s|libpthread|libdl|ld-linux|linux-vdso|libiconv|libSystem)([.]|-|$)'
elif [[ "$os" == Linux ]]; then
  deps=$(ldd "$file" | awk '
    /=>/ { print $1; next }
    $1 ~ /^\// { print $1; next }
  ')
  allowed='^(libc|libm|libgcc_s|libpthread|libdl|ld-linux|linux-vdso|libiconv|libSystem)([.]|-|$)'
elif [[ "$os" == MINGW* || "$os" == MSYS* || "$os" == CYGWIN* ]]; then
  vswhere="/c/Program Files (x86)/Microsoft Visual Studio/Installer/vswhere.exe"
  if [[ ! -f "$vswhere" ]]; then
    echo "vswhere not found; cannot list DLL imports of $file" >&2
    exit 1
  fi
  # sed reads the whole list. head would close the pipe early, and
  # Git Bash then reports vswhere's SIGPIPE as exit 157 under pipefail.
  dumpbin=$(
    "$vswhere" -latest -products '*' \
      -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 \
      -find 'VC\Tools\MSVC\**\bin\Hostx64\x64\dumpbin.exe' \
      | sed -n '1p'
  )
  if [[ -z "$dumpbin" || ! -f "$dumpbin" ]]; then
    echo "dumpbin not found; cannot list DLL imports of $file" >&2
    exit 1
  fi
  deps=$("$dumpbin" /nologo /dependents "$file" | sed -n 's/^[[:space:]]*\(.*\.dll\)[[:space:]]*$/\1/p')
  allowed='^(php8|php8ts|kernel32|kernelbase|ntdll|bcrypt|bcryptprimitives|advapi32|ws2_32|userenv|secur32|crypt32|ncrypt|vcruntime140|vcruntime140_[0-9]+|msvcp140|msvcp140_[0-9]+|concrt140|ucrtbase|api-ms-win-.+)$'
else
  echo "unsupported OS $os" >&2
  exit 1
fi

shopt -s nocasematch
bad=0
while IFS= read -r dep; do
  [[ -z "$dep" ]] && continue
  base=$(basename "$dep")
  base=${base%.dll}
  base=${base%.DLL}
  if [[ "$base" =~ $allowed ]]; then
    continue
  fi
  echo "unexpected library dependency: $dep" >&2
  bad=1
done <<<"$deps"
shopt -u nocasematch

if [[ "$bad" -ne 0 ]]; then
  echo "linkage of $file:" >&2
  if [[ "$os" == Darwin ]]; then
    otool -L "$file" >&2
  elif [[ "$os" == Linux ]]; then
    ldd "$file" >&2
  else
    "$dumpbin" /nologo /dependents "$file" >&2
  fi
  exit 1
fi

echo "linkage ok: $file"
