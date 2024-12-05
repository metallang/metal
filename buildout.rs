       Fresh unicode-ident v1.0.13
       Fresh lazy_static v1.5.0
warning: elided lifetime has a name
  --> D:\Scoop\persist\rustup\.cargo\registry\src\index.crates.io-6f17d22bba15001f\lazy_static-1.5.0\src\inline_lazy.rs:26:43
   |
26 |     pub fn get<F>(&'static self, f: F) -> &T
   |                                           ^ this elided lifetime gets resolved as `'static`
   |
   = note: `#[warn(elided_named_lifetimes)]` on by default
help: consider specifying it explicitly
   |
26 |     pub fn get<F>(&'static self, f: F) -> &'static T
   |                                            +++++++

       Fresh beef v0.5.2
warning: `lazy_static` (lib) generated 1 warning
       Fresh regex-syntax v0.8.5
       Fresh fnv v1.0.7
       Fresh proc-macro2 v1.0.89
       Fresh countme v3.0.1
       Fresh quote v1.0.37
       Fresh leb128 v0.2.5
       Fresh peekable v0.1.0 (C:\Users\lena\Development\Projects\Rust\metal\crates\peekable)
       Fresh text-size v1.1.1
       Fresh ungrammar v1.16.1
       Fresh rustc-hash v1.1.0
       Fresh hashbrown v0.14.5
       Fresh proc-macro-error-attr2 v2.0.0
       Fresh syn v2.0.87
       Fresh wasm-encoder v0.220.0
       Fresh rowan v0.15.16
       Fresh heck v0.5.0
       Fresh tapcli v0.1.0 (https://github.com/elenakrittik/gdtk/?rev=699bffcd52ae85bd7297dbf552eb010ae9bc1f6e#699bffcd)
       Fresh thiserror-impl v2.0.3
       Fresh logos-codegen v0.14.2 (https://github.com/elenakrittik/logos?rev=533228db3e8bbc9ac27e790628844b7d423e10cb#533228db)
       Fresh extend v1.2.0
       Fresh enum-as-inner v0.6.1
       Fresh proc-macro-error2 v2.0.1
       Fresh prettyplease v0.2.25
       Fresh logos-derive v0.14.2 (https://github.com/elenakrittik/logos?rev=533228db3e8bbc9ac27e790628844b7d423e10cb#533228db)
       Fresh thiserror v2.0.3
       Fresh metal-proc-macros v0.1.0 (C:\Users\lena\Development\Projects\Rust\metal\crates\metal-proc-macros)
       Fresh metal-linter v0.1.0 (C:\Users\lena\Development\Projects\Rust\metal\crates\metal-linter)
       Fresh metal-formatter v0.1.0 (C:\Users\lena\Development\Projects\Rust\metal\crates\metal-formatter)
       Fresh metal-mir v0.1.0 (C:\Users\lena\Development\Projects\Rust\metal\crates\metal-mir)
       Fresh logos v0.14.2 (https://github.com/elenakrittik/logos?rev=533228db3e8bbc9ac27e790628844b7d423e10cb#533228db)
       Dirty metal-ast-ungram v0.1.0 (C:\Users\lena\Development\Projects\Rust\metal\crates\metal-ast-ungram): the file `crates\metal-ast-ungram\src\engram.rs` has changed (13377807183.197278000s, 1m 7s after last build at 13377807116.369706900s)
   Compiling metal-ast-ungram v0.1.0 (C:\Users\lena\Development\Projects\Rust\metal\crates\metal-ast-ungram)
       Fresh metallic v0.1.0 (C:\Users\lena\Development\Projects\Rust\metal\crates\metallic)
       Fresh metal-lsp v0.1.0 (C:\Users\lena\Development\Projects\Rust\metal\crates\metal-lsp)
       Fresh metal-lexer v0.1.0 (C:\Users\lena\Development\Projects\Rust\metal\crates\metal-lexer)
       Fresh metal-ast v0.1.0 (C:\Users\lena\Development\Projects\Rust\metal\crates\metal-ast)
       Fresh metal-parser v0.1.0 (C:\Users\lena\Development\Projects\Rust\metal\crates\metal-parser)
       Fresh metal-codegen-wasm v0.1.0 (C:\Users\lena\Development\Projects\Rust\metal\crates\metal-codegen-wasm)
     Running `set CARGO=\\?\D:\Scoop\persist\rustup\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\cargo.exe&& set CARGO_CRATE_NAME=metal_ast_ungram&& set CARGO_MANIFEST_DIR=C:\Users\lena\Development\Projects\Rust\metal\crates\metal-ast-ungram&& set CARGO_MANIFEST_PATH=C:\Users\lena\Development\Projects\Rust\metal\crates\metal-ast-ungram\Cargo.toml&& set CARGO_PKG_AUTHORS=""&& set CARGO_PKG_DESCRIPTION=""&& set CARGO_PKG_HOMEPAGE=""&& set CARGO_PKG_LICENSE=""&& set CARGO_PKG_LICENSE_FILE=""&& set CARGO_PKG_NAME=metal-ast-ungram&& set CARGO_PKG_README=""&& set CARGO_PKG_REPOSITORY=""&& set CARGO_PKG_RUST_VERSION=""&& set CARGO_PKG_VERSION=0.1.0&& set CARGO_PKG_VERSION_MAJOR=0&& set CARGO_PKG_VERSION_MINOR=1&& set CARGO_PKG_VERSION_PATCH=0&& set CARGO_PKG_VERSION_PRE=""&& set CARGO_PRIMARY_PACKAGE=1&& set CARGO_RUSTC_CURRENT_DIR=C:\Users\lena\Development\Projects\Rust\metal&& set PATH="C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps;D:\Scoop\persist\rustup\.cargo\bin;C:\Program Files\PowerShell\7;C:\Program Files (x86)\Common Files\Oracle\Java\javapath;C:\Users\lena\scoop\apps\openssh\current;C:\WINDOWS\system32;C:\WINDOWS;C:\WINDOWS\System32\Wbem;C:\WINDOWS\System32\WindowsPowerShell\v1.0\;C:\WINDOWS\System32\OpenSSH\;C:\Program Files (x86)\NVIDIA Corporation\PhysX\Common;C:\Program Files\Git\cmd;C:\Program Files\dotnet\;C:\Program Files (x86)\Gpg4win\..\GnuPG\bin;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit\;C:\Program Files\Cloudflare\Cloudflare WARP\;D:\Programs\AMDuProf\bin;D:\Programs\Performance;C:\Program Files\NVIDIA Corporation\NVIDIA app\NvDLISR;C:\Program Files\PowerShell\7\;D:\Scoop\apps\vscode\current\bin;D:\Scoop\apps\temurin17-jdk\current\bin;D:\Scoop\apps\mingw\current\bin;D:\Scoop\apps\llvm\current\bin;D:\Scoop\apps\volta\current\appdata\bin;D:\Scoop\apps\temurin11-jdk\current\bin;D:\Scoop\apps\rustup\current\.cargo\bin;D:\Scoop\apps\gcc\current\bin;D:\Scoop\apps\python\current\Scripts;D:\Scoop\apps\python\current;D:\Scoop\shims;C:\Users\lena\AppData\Local\pnpm;C:\Users\lena\AppData\Roaming\edgedb\bin;C:\Users\lena\AppData\Local\Microsoft\WindowsApps;c:\users\lena\appdata\roaming\python\python311\scripts;c:\users\lena\.local\bin;C:\Users\lena\.dotnet\tools;C:\Users\lena\.dotnet\tools;C:\Users\lena\.dotnet\tools;C:\Users\lena\.dotnet\tools;C:\Users\lena\.dotnet\tools;C:\Users\lena\AppData\Local\JetBrains\Toolbox\scripts;C:\Users\lena\Downloads\bup\dm\bin;D:\Programs\ghcup\bin;C:\Users\lena\AppData\Roaming\local\bin;C:\Users\lena\.dotnet\tools;C:\Users\lena\AppData\Local\Programs\Ollama;C:\Users\lena\.dotnet\tools;D:\Programs\Airshipper;C:\Users\lena\.deno\bin;"&& D:\Scoop\persist\rustup\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\rustc.exe --crate-name metal_ast_ungram --edition=2021 crates\metal-ast-ungram\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 --warn=clippy::wildcard_enum_match_arm --warn=rustdoc::broken_intra_doc_links --check-cfg cfg(docsrs) --check-cfg "cfg(feature, values())" -C metadata=7aa2c5fc6d958a97 -C extra-filename=-7aa2c5fc6d958a97 --out-dir C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps -C incremental=C:\Users\lena\Development\Projects\Rust\metal\target\debug\incremental -L dependency=C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps --extern extend=C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps\extend-b002e8f973b8ee30.dll --extern heck=C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps\libheck-9d4c2162299e3aa7.rmeta --extern prettyplease=C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps\libprettyplease-09b85808bd7a77e1.rmeta --extern proc_macro2=C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps\libproc_macro2-f9925568991b4630.rmeta --extern quote=C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps\libquote-ea3ac1e3a9530e57.rmeta --extern syn=C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps\libsyn-032ed1f854f9a3f0.rmeta --extern thiserror=C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps\libthiserror-30ae39977246014c.rmeta --extern ungrammar=C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps\libungrammar-7c34767ebe0d7c06.rmeta`
       Fresh metal v0.1.0 (C:\Users\lena\Development\Projects\Rust\metal\crates\metal)
     Running `set CARGO=\\?\D:\Scoop\persist\rustup\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\cargo.exe&& set CARGO_CRATE_NAME=metal_ast_ungram&& set CARGO_MANIFEST_DIR=C:\Users\lena\Development\Projects\Rust\metal\crates\metal-ast-ungram&& set CARGO_MANIFEST_PATH=C:\Users\lena\Development\Projects\Rust\metal\crates\metal-ast-ungram\Cargo.toml&& set CARGO_PKG_AUTHORS=""&& set CARGO_PKG_DESCRIPTION=""&& set CARGO_PKG_HOMEPAGE=""&& set CARGO_PKG_LICENSE=""&& set CARGO_PKG_LICENSE_FILE=""&& set CARGO_PKG_NAME=metal-ast-ungram&& set CARGO_PKG_README=""&& set CARGO_PKG_REPOSITORY=""&& set CARGO_PKG_RUST_VERSION=""&& set CARGO_PKG_VERSION=0.1.0&& set CARGO_PKG_VERSION_MAJOR=0&& set CARGO_PKG_VERSION_MINOR=1&& set CARGO_PKG_VERSION_PATCH=0&& set CARGO_PKG_VERSION_PRE=""&& set CARGO_PRIMARY_PACKAGE=1&& set CARGO_RUSTC_CURRENT_DIR=C:\Users\lena\Development\Projects\Rust\metal&& set PATH="C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps;D:\Scoop\persist\rustup\.cargo\bin;C:\Program Files\PowerShell\7;C:\Program Files (x86)\Common Files\Oracle\Java\javapath;C:\Users\lena\scoop\apps\openssh\current;C:\WINDOWS\system32;C:\WINDOWS;C:\WINDOWS\System32\Wbem;C:\WINDOWS\System32\WindowsPowerShell\v1.0\;C:\WINDOWS\System32\OpenSSH\;C:\Program Files (x86)\NVIDIA Corporation\PhysX\Common;C:\Program Files\Git\cmd;C:\Program Files\dotnet\;C:\Program Files (x86)\Gpg4win\..\GnuPG\bin;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit\;C:\Program Files\Cloudflare\Cloudflare WARP\;D:\Programs\AMDuProf\bin;D:\Programs\Performance;C:\Program Files\NVIDIA Corporation\NVIDIA app\NvDLISR;C:\Program Files\PowerShell\7\;D:\Scoop\apps\vscode\current\bin;D:\Scoop\apps\temurin17-jdk\current\bin;D:\Scoop\apps\mingw\current\bin;D:\Scoop\apps\llvm\current\bin;D:\Scoop\apps\volta\current\appdata\bin;D:\Scoop\apps\temurin11-jdk\current\bin;D:\Scoop\apps\rustup\current\.cargo\bin;D:\Scoop\apps\gcc\current\bin;D:\Scoop\apps\python\current\Scripts;D:\Scoop\apps\python\current;D:\Scoop\shims;C:\Users\lena\AppData\Local\pnpm;C:\Users\lena\AppData\Roaming\edgedb\bin;C:\Users\lena\AppData\Local\Microsoft\WindowsApps;c:\users\lena\appdata\roaming\python\python311\scripts;c:\users\lena\.local\bin;C:\Users\lena\.dotnet\tools;C:\Users\lena\.dotnet\tools;C:\Users\lena\.dotnet\tools;C:\Users\lena\.dotnet\tools;C:\Users\lena\.dotnet\tools;C:\Users\lena\AppData\Local\JetBrains\Toolbox\scripts;C:\Users\lena\Downloads\bup\dm\bin;D:\Programs\ghcup\bin;C:\Users\lena\AppData\Roaming\local\bin;C:\Users\lena\.dotnet\tools;C:\Users\lena\AppData\Local\Programs\Ollama;C:\Users\lena\.dotnet\tools;D:\Programs\Airshipper;C:\Users\lena\.deno\bin;"&& D:\Scoop\persist\rustup\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\rustc.exe --crate-name metal_ast_ungram --edition=2021 crates\metal-ast-ungram\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no --warn=clippy::wildcard_enum_match_arm --warn=rustdoc::broken_intra_doc_links --check-cfg cfg(docsrs) --check-cfg "cfg(feature, values())" -C metadata=bd4e83e7d6a4a831 -C extra-filename=-bd4e83e7d6a4a831 --out-dir C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps -C incremental=C:\Users\lena\Development\Projects\Rust\metal\target\debug\incremental -L dependency=C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps --extern extend=C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps\extend-b002e8f973b8ee30.dll --extern heck=C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps\libheck-69cdd3aa2a103f41.rmeta --extern prettyplease=C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps\libprettyplease-dbe46fe18a27945b.rmeta --extern proc_macro2=C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps\libproc_macro2-f99cf3e7d97edd64.rmeta --extern quote=C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps\libquote-5795843e7c1488b2.rmeta --extern syn=C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps\libsyn-d945fac95682f792.rmeta --extern thiserror=C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps\libthiserror-75be73d414632c0c.rmeta --extern ungrammar=C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps\libungrammar-0bd186ce293cea37.rmeta`
warning: unused macro definition: `dbg_`
  --> crates\metal-ast-ungram\src\generate\nodes.rs:93:14
   |
93 | macro_rules! dbg_ {
   |              ^^^^
   |
   = note: `#[warn(unused_macros)]` on by default

warning: unreachable expression
  --> crates\metal-ast-ungram\src\lib.rs:38:5
   |
36 |     panic!();
   |     -------- any code following this expression is unreachable
37 |
38 |     Ok(())
   |     ^^^^^^ unreachable expression
   |
   = note: `#[warn(unreachable_code)]` on by default

warning: `metal-ast-ungram` (lib) generated 2 warnings
warning: `metal-ast-ungram` (lib) generated 2 warnings (2 duplicates)
       Dirty metal-ast-ng v0.1.0 (C:\Users\lena\Development\Projects\Rust\metal\crates\metal-ast-ng): the dependency metal_ast_ungram was rebuilt
   Compiling metal-ast-ng v0.1.0 (C:\Users\lena\Development\Projects\Rust\metal\crates\metal-ast-ng)
     Running `set CARGO=\\?\D:\Scoop\persist\rustup\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\cargo.exe&& set CARGO_CRATE_NAME=build_script_build&& set CARGO_MANIFEST_DIR=C:\Users\lena\Development\Projects\Rust\metal\crates\metal-ast-ng&& set CARGO_MANIFEST_PATH=C:\Users\lena\Development\Projects\Rust\metal\crates\metal-ast-ng\Cargo.toml&& set CARGO_PKG_AUTHORS=""&& set CARGO_PKG_DESCRIPTION=""&& set CARGO_PKG_HOMEPAGE=""&& set CARGO_PKG_LICENSE=""&& set CARGO_PKG_LICENSE_FILE=""&& set CARGO_PKG_NAME=metal-ast-ng&& set CARGO_PKG_README=""&& set CARGO_PKG_REPOSITORY=""&& set CARGO_PKG_RUST_VERSION=""&& set CARGO_PKG_VERSION=0.1.0&& set CARGO_PKG_VERSION_MAJOR=0&& set CARGO_PKG_VERSION_MINOR=1&& set CARGO_PKG_VERSION_PATCH=0&& set CARGO_PKG_VERSION_PRE=""&& set CARGO_PRIMARY_PACKAGE=1&& set CARGO_RUSTC_CURRENT_DIR=C:\Users\lena\Development\Projects\Rust\metal&& set PATH="C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps;D:\Scoop\persist\rustup\.cargo\bin;C:\Program Files\PowerShell\7;C:\Program Files (x86)\Common Files\Oracle\Java\javapath;C:\Users\lena\scoop\apps\openssh\current;C:\WINDOWS\system32;C:\WINDOWS;C:\WINDOWS\System32\Wbem;C:\WINDOWS\System32\WindowsPowerShell\v1.0\;C:\WINDOWS\System32\OpenSSH\;C:\Program Files (x86)\NVIDIA Corporation\PhysX\Common;C:\Program Files\Git\cmd;C:\Program Files\dotnet\;C:\Program Files (x86)\Gpg4win\..\GnuPG\bin;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit\;C:\Program Files\Cloudflare\Cloudflare WARP\;D:\Programs\AMDuProf\bin;D:\Programs\Performance;C:\Program Files\NVIDIA Corporation\NVIDIA app\NvDLISR;C:\Program Files\PowerShell\7\;D:\Scoop\apps\vscode\current\bin;D:\Scoop\apps\temurin17-jdk\current\bin;D:\Scoop\apps\mingw\current\bin;D:\Scoop\apps\llvm\current\bin;D:\Scoop\apps\volta\current\appdata\bin;D:\Scoop\apps\temurin11-jdk\current\bin;D:\Scoop\apps\rustup\current\.cargo\bin;D:\Scoop\apps\gcc\current\bin;D:\Scoop\apps\python\current\Scripts;D:\Scoop\apps\python\current;D:\Scoop\shims;C:\Users\lena\AppData\Local\pnpm;C:\Users\lena\AppData\Roaming\edgedb\bin;C:\Users\lena\AppData\Local\Microsoft\WindowsApps;c:\users\lena\appdata\roaming\python\python311\scripts;c:\users\lena\.local\bin;C:\Users\lena\.dotnet\tools;C:\Users\lena\.dotnet\tools;C:\Users\lena\.dotnet\tools;C:\Users\lena\.dotnet\tools;C:\Users\lena\.dotnet\tools;C:\Users\lena\AppData\Local\JetBrains\Toolbox\scripts;C:\Users\lena\Downloads\bup\dm\bin;D:\Programs\ghcup\bin;C:\Users\lena\AppData\Roaming\local\bin;C:\Users\lena\.dotnet\tools;C:\Users\lena\AppData\Local\Programs\Ollama;C:\Users\lena\.dotnet\tools;D:\Programs\Airshipper;C:\Users\lena\.deno\bin;"&& D:\Scoop\persist\rustup\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\rustc.exe --crate-name build_script_build --edition=2021 crates\metal-ast-ng\build.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no --warn=clippy::wildcard_enum_match_arm --warn=rustdoc::broken_intra_doc_links --check-cfg cfg(docsrs) --check-cfg "cfg(feature, values())" -C metadata=a30c9476952bb12c -C extra-filename=-a30c9476952bb12c --out-dir C:\Users\lena\Development\Projects\Rust\metal\target\debug\build\metal-ast-ng-a30c9476952bb12c -C incremental=C:\Users\lena\Development\Projects\Rust\metal\target\debug\incremental -L dependency=C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps --extern metal_ast_ungram=C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps\libmetal_ast_ungram-bd4e83e7d6a4a831.rlib`
     Running `set CARGO=\\?\D:\Scoop\persist\rustup\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\cargo.exe&& set CARGO_CFG_FMT_DEBUG=full&& set CARGO_CFG_OVERFLOW_CHECKS=""&& set CARGO_CFG_PANIC=unwind&& set CARGO_CFG_RELOCATION_MODEL=pic&& set CARGO_CFG_TARGET_ABI=""&& set CARGO_CFG_TARGET_ARCH=x86_64&& set CARGO_CFG_TARGET_ENDIAN=little&& set CARGO_CFG_TARGET_ENV=msvc&& set CARGO_CFG_TARGET_FAMILY=windows&& set CARGO_CFG_TARGET_FEATURE=cmpxchg16b,fxsr,lahfsahf,sse,sse2,sse3&& set CARGO_CFG_TARGET_HAS_ATOMIC=128,16,32,64,8,ptr&& set CARGO_CFG_TARGET_HAS_ATOMIC_EQUAL_ALIGNMENT=128,16,32,64,8,ptr&& set CARGO_CFG_TARGET_HAS_ATOMIC_LOAD_STORE=128,16,32,64,8,ptr&& set CARGO_CFG_TARGET_OS=windows&& set CARGO_CFG_TARGET_POINTER_WIDTH=64&& set CARGO_CFG_TARGET_THREAD_LOCAL=""&& set CARGO_CFG_TARGET_VENDOR=pc&& set CARGO_CFG_UB_CHECKS=""&& set CARGO_CFG_WINDOWS=""&& set CARGO_ENCODED_RUSTFLAGS=""&& set CARGO_MANIFEST_DIR=C:\Users\lena\Development\Projects\Rust\metal\crates\metal-ast-ng&& set CARGO_MANIFEST_PATH=C:\Users\lena\Development\Projects\Rust\metal\crates\metal-ast-ng\Cargo.toml&& set CARGO_PKG_AUTHORS=""&& set CARGO_PKG_DESCRIPTION=""&& set CARGO_PKG_HOMEPAGE=""&& set CARGO_PKG_LICENSE=""&& set CARGO_PKG_LICENSE_FILE=""&& set CARGO_PKG_NAME=metal-ast-ng&& set CARGO_PKG_README=""&& set CARGO_PKG_REPOSITORY=""&& set CARGO_PKG_RUST_VERSION=""&& set CARGO_PKG_VERSION=0.1.0&& set CARGO_PKG_VERSION_MAJOR=0&& set CARGO_PKG_VERSION_MINOR=1&& set CARGO_PKG_VERSION_PATCH=0&& set CARGO_PKG_VERSION_PRE=""&& set DEBUG=true&& set HOST=x86_64-pc-windows-msvc&& set NUM_JOBS=8&& set OPT_LEVEL=0&& set OUT_DIR=C:\Users\lena\Development\Projects\Rust\metal\target\debug\build\metal-ast-ng-0415d808c3f882a3\out&& set PATH="C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps;C:\Users\lena\Development\Projects\Rust\metal\target\debug;D:\Scoop\persist\rustup\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib;D:\Scoop\persist\rustup\.cargo\bin;C:\Program Files\PowerShell\7;C:\Program Files (x86)\Common Files\Oracle\Java\javapath;C:\Users\lena\scoop\apps\openssh\current;C:\WINDOWS\system32;C:\WINDOWS;C:\WINDOWS\System32\Wbem;C:\WINDOWS\System32\WindowsPowerShell\v1.0\;C:\WINDOWS\System32\OpenSSH\;C:\Program Files (x86)\NVIDIA Corporation\PhysX\Common;C:\Program Files\Git\cmd;C:\Program Files\dotnet\;C:\Program Files (x86)\Gpg4win\..\GnuPG\bin;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit\;C:\Program Files\Cloudflare\Cloudflare WARP\;D:\Programs\AMDuProf\bin;D:\Programs\Performance;C:\Program Files\NVIDIA Corporation\NVIDIA app\NvDLISR;C:\Program Files\PowerShell\7\;D:\Scoop\apps\vscode\current\bin;D:\Scoop\apps\temurin17-jdk\current\bin;D:\Scoop\apps\mingw\current\bin;D:\Scoop\apps\llvm\current\bin;D:\Scoop\apps\volta\current\appdata\bin;D:\Scoop\apps\temurin11-jdk\current\bin;D:\Scoop\apps\rustup\current\.cargo\bin;D:\Scoop\apps\gcc\current\bin;D:\Scoop\apps\python\current\Scripts;D:\Scoop\apps\python\current;D:\Scoop\shims;C:\Users\lena\AppData\Local\pnpm;C:\Users\lena\AppData\Roaming\edgedb\bin;C:\Users\lena\AppData\Local\Microsoft\WindowsApps;c:\users\lena\appdata\roaming\python\python311\scripts;c:\users\lena\.local\bin;C:\Users\lena\.dotnet\tools;C:\Users\lena\.dotnet\tools;C:\Users\lena\.dotnet\tools;C:\Users\lena\.dotnet\tools;C:\Users\lena\.dotnet\tools;C:\Users\lena\AppData\Local\JetBrains\Toolbox\scripts;C:\Users\lena\Downloads\bup\dm\bin;D:\Programs\ghcup\bin;C:\Users\lena\AppData\Roaming\local\bin;C:\Users\lena\.dotnet\tools;C:\Users\lena\AppData\Local\Programs\Ollama;C:\Users\lena\.dotnet\tools;D:\Programs\Airshipper;C:\Users\lena\.deno\bin;"&& set PROFILE=debug&& set RUSTC=D:\Scoop\persist\rustup\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\rustc.exe&& set RUSTDOC=D:\Scoop\persist\rustup\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\rustdoc.exe&& set TARGET=x86_64-pc-windows-msvc&& C:\Users\lena\Development\Projects\Rust\metal\target\debug\build\metal-ast-ng-a30c9476952bb12c\build-script-build`
[metal-ast-ng 0.1.0] cargo::rerun-if-changed=./metal.ungram
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Token(
[metal-ast-ng 0.1.0]     Token(
[metal-ast-ng 0.1.0]         0,
[metal-ast-ng 0.1.0]     ),
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = Some(
[metal-ast-ng 0.1.0]     Token(
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             0,
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ),
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Opt(
[metal-ast-ng 0.1.0]     Token(
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             1,
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ),
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Token(
[metal-ast-ng 0.1.0]     Token(
[metal-ast-ng 0.1.0]         1,
[metal-ast-ng 0.1.0]     ),
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = Some(
[metal-ast-ng 0.1.0]     Token(
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             1,
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ),
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = Some(
[metal-ast-ng 0.1.0]     Token(
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             1,
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ),
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Opt(
[metal-ast-ng 0.1.0]     Token(
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             2,
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ),
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Token(
[metal-ast-ng 0.1.0]     Token(
[metal-ast-ng 0.1.0]         2,
[metal-ast-ng 0.1.0]     ),
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = Some(
[metal-ast-ng 0.1.0]     Token(
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             2,
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ),
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = Some(
[metal-ast-ng 0.1.0]     Token(
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             2,
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ),
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 3,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 4,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Alt(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 0,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 27,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 28,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 29,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 30,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 4,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 6,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Node(
[metal-ast-ng 0.1.0]     Node(
[metal-ast-ng 0.1.0]         0,
[metal-ast-ng 0.1.0]     ),
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = Some(
[metal-ast-ng 0.1.0]     Node(
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             0,
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ),
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 5,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 8,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 6,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Alt(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 9,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 10,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 4,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 11,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 12,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 13,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "vis",
[metal-ast-ng 0.1.0]             rule: Node(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     1,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 7,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 0,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "ty",
[metal-ast-ng 0.1.0]             rule: Opt(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     Node(
[metal-ast-ng 0.1.0]                         5,
[metal-ast-ng 0.1.0]                     ),
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "value",
[metal-ast-ng 0.1.0]             rule: Node(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     3,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "vis",
[metal-ast-ng 0.1.0]             rule: Node(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     1,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 8,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 0,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 5,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "body",
[metal-ast-ng 0.1.0]             rule: Opt(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     Node(
[metal-ast-ng 0.1.0]                         14,
[metal-ast-ng 0.1.0]                     ),
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 6,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "vis",
[metal-ast-ng 0.1.0]             rule: Node(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     1,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 12,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 0,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "sig",
[metal-ast-ng 0.1.0]             rule: Node(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     18,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "body",
[metal-ast-ng 0.1.0]             rule: Node(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     7,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "vis",
[metal-ast-ng 0.1.0]             rule: Node(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     1,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 13,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "tree",
[metal-ast-ng 0.1.0]             rule: Node(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     20,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 14,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "vis",
[metal-ast-ng 0.1.0]             rule: Node(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     1,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 16,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 0,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 5,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "body",
[metal-ast-ng 0.1.0]             rule: Node(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     24,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 6,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 15,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Rep(
[metal-ast-ng 0.1.0]             Seq(
[metal-ast-ng 0.1.0]                 [
[metal-ast-ng 0.1.0]                     Token(
[metal-ast-ng 0.1.0]                         Token(
[metal-ast-ng 0.1.0]                             9,
[metal-ast-ng 0.1.0]                         ),
[metal-ast-ng 0.1.0]                     ),
[metal-ast-ng 0.1.0]                     Node(
[metal-ast-ng 0.1.0]                         Node(
[metal-ast-ng 0.1.0]                             15,
[metal-ast-ng 0.1.0]                         ),
[metal-ast-ng 0.1.0]                     ),
[metal-ast-ng 0.1.0]                 ],
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Opt(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 Token(
[metal-ast-ng 0.1.0]                     9,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Alt(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 16,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 11,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 0,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "data_ty",
[metal-ast-ng 0.1.0]             rule: Opt(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     Node(
[metal-ast-ng 0.1.0]                         17,
[metal-ast-ng 0.1.0]                     ),
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 10,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 6,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 11,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 10,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "inputs",
[metal-ast-ng 0.1.0]             rule: Opt(
[metal-ast-ng 0.1.0]                 Seq(
[metal-ast-ng 0.1.0]                     [
[metal-ast-ng 0.1.0]                         Node(
[metal-ast-ng 0.1.0]                             Node(
[metal-ast-ng 0.1.0]                                 19,
[metal-ast-ng 0.1.0]                             ),
[metal-ast-ng 0.1.0]                         ),
[metal-ast-ng 0.1.0]                         Rep(
[metal-ast-ng 0.1.0]                             Seq(
[metal-ast-ng 0.1.0]                                 [
[metal-ast-ng 0.1.0]                                     Token(
[metal-ast-ng 0.1.0]                                         Token(
[metal-ast-ng 0.1.0]                                             9,
[metal-ast-ng 0.1.0]                                         ),
[metal-ast-ng 0.1.0]                                     ),
[metal-ast-ng 0.1.0]                                     Node(
[metal-ast-ng 0.1.0]                                         Node(
[metal-ast-ng 0.1.0]                                             19,
[metal-ast-ng 0.1.0]                                         ),
[metal-ast-ng 0.1.0]                                     ),
[metal-ast-ng 0.1.0]                                 ],
[metal-ast-ng 0.1.0]                             ),
[metal-ast-ng 0.1.0]                         ),
[metal-ast-ng 0.1.0]                         Opt(
[metal-ast-ng 0.1.0]                             Token(
[metal-ast-ng 0.1.0]                                 Token(
[metal-ast-ng 0.1.0]                                     9,
[metal-ast-ng 0.1.0]                                 ),
[metal-ast-ng 0.1.0]                             ),
[metal-ast-ng 0.1.0]                         ),
[metal-ast-ng 0.1.0]                     ],
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 11,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "return_ty",
[metal-ast-ng 0.1.0]             rule: Opt(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     Node(
[metal-ast-ng 0.1.0]                         5,
[metal-ast-ng 0.1.0]                     ),
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "mutness",
[metal-ast-ng 0.1.0]             rule: Node(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     2,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 0,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "ty",
[metal-ast-ng 0.1.0]             rule: Opt(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     Node(
[metal-ast-ng 0.1.0]                         5,
[metal-ast-ng 0.1.0]                     ),
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "default",
[metal-ast-ng 0.1.0]             rule: Opt(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     Node(
[metal-ast-ng 0.1.0]                         3,
[metal-ast-ng 0.1.0]                     ),
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Alt(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 21,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 22,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 23,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Node(
[metal-ast-ng 0.1.0]     Node(
[metal-ast-ng 0.1.0]         0,
[metal-ast-ng 0.1.0]     ),
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = Some(
[metal-ast-ng 0.1.0]     Node(
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             0,
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ),
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "segment",
[metal-ast-ng 0.1.0]             rule: Node(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     0,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 15,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "rest",
[metal-ast-ng 0.1.0]             rule: Node(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     20,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 5,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "subtrees",
[metal-ast-ng 0.1.0]             rule: Opt(
[metal-ast-ng 0.1.0]                 Seq(
[metal-ast-ng 0.1.0]                     [
[metal-ast-ng 0.1.0]                         Node(
[metal-ast-ng 0.1.0]                             Node(
[metal-ast-ng 0.1.0]                                 20,
[metal-ast-ng 0.1.0]                             ),
[metal-ast-ng 0.1.0]                         ),
[metal-ast-ng 0.1.0]                         Rep(
[metal-ast-ng 0.1.0]                             Seq(
[metal-ast-ng 0.1.0]                                 [
[metal-ast-ng 0.1.0]                                     Token(
[metal-ast-ng 0.1.0]                                         Token(
[metal-ast-ng 0.1.0]                                             9,
[metal-ast-ng 0.1.0]                                         ),
[metal-ast-ng 0.1.0]                                     ),
[metal-ast-ng 0.1.0]                                     Node(
[metal-ast-ng 0.1.0]                                         Node(
[metal-ast-ng 0.1.0]                                             20,
[metal-ast-ng 0.1.0]                                         ),
[metal-ast-ng 0.1.0]                                     ),
[metal-ast-ng 0.1.0]                                 ],
[metal-ast-ng 0.1.0]                             ),
[metal-ast-ng 0.1.0]                         ),
[metal-ast-ng 0.1.0]                         Opt(
[metal-ast-ng 0.1.0]                             Token(
[metal-ast-ng 0.1.0]                                 Token(
[metal-ast-ng 0.1.0]                                     9,
[metal-ast-ng 0.1.0]                                 ),
[metal-ast-ng 0.1.0]                             ),
[metal-ast-ng 0.1.0]                         ),
[metal-ast-ng 0.1.0]                     ],
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 6,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 25,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Rep(
[metal-ast-ng 0.1.0]             Seq(
[metal-ast-ng 0.1.0]                 [
[metal-ast-ng 0.1.0]                     Token(
[metal-ast-ng 0.1.0]                         Token(
[metal-ast-ng 0.1.0]                             9,
[metal-ast-ng 0.1.0]                         ),
[metal-ast-ng 0.1.0]                     ),
[metal-ast-ng 0.1.0]                     Node(
[metal-ast-ng 0.1.0]                         Node(
[metal-ast-ng 0.1.0]                             25,
[metal-ast-ng 0.1.0]                         ),
[metal-ast-ng 0.1.0]                     ),
[metal-ast-ng 0.1.0]                 ],
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Opt(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 Token(
[metal-ast-ng 0.1.0]                     9,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Alt(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 26,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 11,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "vis",
[metal-ast-ng 0.1.0]             rule: Node(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     1,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 0,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "ty",
[metal-ast-ng 0.1.0]             rule: Node(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     5,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "op",
[metal-ast-ng 0.1.0]             rule: Node(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     31,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 4,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "lhs",
[metal-ast-ng 0.1.0]             rule: Node(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     4,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "op",
[metal-ast-ng 0.1.0]             rule: Node(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     32,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]         Labeled {
[metal-ast-ng 0.1.0]             label: "rhs",
[metal-ast-ng 0.1.0]             rule: Node(
[metal-ast-ng 0.1.0]                 Node(
[metal-ast-ng 0.1.0]                     4,
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         },
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 4,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 10,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Rep(
[metal-ast-ng 0.1.0]             Seq(
[metal-ast-ng 0.1.0]                 [
[metal-ast-ng 0.1.0]                     Node(
[metal-ast-ng 0.1.0]                         Node(
[metal-ast-ng 0.1.0]                             4,
[metal-ast-ng 0.1.0]                         ),
[metal-ast-ng 0.1.0]                     ),
[metal-ast-ng 0.1.0]                     Opt(
[metal-ast-ng 0.1.0]                         Token(
[metal-ast-ng 0.1.0]                             Token(
[metal-ast-ng 0.1.0]                                 9,
[metal-ast-ng 0.1.0]                             ),
[metal-ast-ng 0.1.0]                         ),
[metal-ast-ng 0.1.0]                     ),
[metal-ast-ng 0.1.0]                 ],
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 11,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Alt(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 33,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Node(
[metal-ast-ng 0.1.0]             Node(
[metal-ast-ng 0.1.0]                 34,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Alt(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 17,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 18,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 19,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 20,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Alt(
[metal-ast-ng 0.1.0]     [
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 3,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 21,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 22,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 23,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 24,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 25,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 26,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 27,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 28,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 29,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 30,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 31,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 17,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 18,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 32,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 33,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 34,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 35,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 36,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 37,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 38,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 39,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 40,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 41,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 42,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 43,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 44,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 45,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 46,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 47,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             Token(
[metal-ast-ng 0.1.0]                 48,
[metal-ast-ng 0.1.0]             ),
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]         Seq(
[metal-ast-ng 0.1.0]             [
[metal-ast-ng 0.1.0]                 Token(
[metal-ast-ng 0.1.0]                     Token(
[metal-ast-ng 0.1.0]                         49,
[metal-ast-ng 0.1.0]                     ),
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]                 Token(
[metal-ast-ng 0.1.0]                     Token(
[metal-ast-ng 0.1.0]                         15,
[metal-ast-ng 0.1.0]                     ),
[metal-ast-ng 0.1.0]                 ),
[metal-ast-ng 0.1.0]             ],
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ],
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = None
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Token(
[metal-ast-ng 0.1.0]     Token(
[metal-ast-ng 0.1.0]         50,
[metal-ast-ng 0.1.0]     ),
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = Some(
[metal-ast-ng 0.1.0]     Token(
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             50,
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ),
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:20] self = Token(
[metal-ast-ng 0.1.0]     Token(
[metal-ast-ng 0.1.0]         51,
[metal-ast-ng 0.1.0]     ),
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
[metal-ast-ng 0.1.0] {
[metal-ast-ng 0.1.0]     rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
[metal-ast-ng 0.1.0]     rule.as_ref().unwrap_simple(), _ => None,
[metal-ast-ng 0.1.0] } = Some(
[metal-ast-ng 0.1.0]     Token(
[metal-ast-ng 0.1.0]         Token(
[metal-ast-ng 0.1.0]             51,
[metal-ast-ng 0.1.0]         ),
[metal-ast-ng 0.1.0]     ),
[metal-ast-ng 0.1.0] )
[metal-ast-ng 0.1.0] thread 'main' panicked at crates\metal-ast-ungram\src\lib.rs:36:5:
[metal-ast-ng 0.1.0] explicit panic
[metal-ast-ng 0.1.0] note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: failed to run custom build command for `metal-ast-ng v0.1.0 (C:\Users\lena\Development\Projects\Rust\metal\crates\metal-ast-ng)`

Caused by:
  process didn't exit successfully: `set CARGO=\\?\D:\Scoop\persist\rustup\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\cargo.exe&& set CARGO_CFG_FMT_DEBUG=full&& set CARGO_CFG_OVERFLOW_CHECKS=""&& set CARGO_CFG_PANIC=unwind&& set CARGO_CFG_RELOCATION_MODEL=pic&& set CARGO_CFG_TARGET_ABI=""&& set CARGO_CFG_TARGET_ARCH=x86_64&& set CARGO_CFG_TARGET_ENDIAN=little&& set CARGO_CFG_TARGET_ENV=msvc&& set CARGO_CFG_TARGET_FAMILY=windows&& set CARGO_CFG_TARGET_FEATURE=cmpxchg16b,fxsr,lahfsahf,sse,sse2,sse3&& set CARGO_CFG_TARGET_HAS_ATOMIC=128,16,32,64,8,ptr&& set CARGO_CFG_TARGET_HAS_ATOMIC_EQUAL_ALIGNMENT=128,16,32,64,8,ptr&& set CARGO_CFG_TARGET_HAS_ATOMIC_LOAD_STORE=128,16,32,64,8,ptr&& set CARGO_CFG_TARGET_OS=windows&& set CARGO_CFG_TARGET_POINTER_WIDTH=64&& set CARGO_CFG_TARGET_THREAD_LOCAL=""&& set CARGO_CFG_TARGET_VENDOR=pc&& set CARGO_CFG_UB_CHECKS=""&& set CARGO_CFG_WINDOWS=""&& set CARGO_ENCODED_RUSTFLAGS=""&& set CARGO_MANIFEST_DIR=C:\Users\lena\Development\Projects\Rust\metal\crates\metal-ast-ng&& set CARGO_MANIFEST_PATH=C:\Users\lena\Development\Projects\Rust\metal\crates\metal-ast-ng\Cargo.toml&& set CARGO_PKG_AUTHORS=""&& set CARGO_PKG_DESCRIPTION=""&& set CARGO_PKG_HOMEPAGE=""&& set CARGO_PKG_LICENSE=""&& set CARGO_PKG_LICENSE_FILE=""&& set CARGO_PKG_NAME=metal-ast-ng&& set CARGO_PKG_README=""&& set CARGO_PKG_REPOSITORY=""&& set CARGO_PKG_RUST_VERSION=""&& set CARGO_PKG_VERSION=0.1.0&& set CARGO_PKG_VERSION_MAJOR=0&& set CARGO_PKG_VERSION_MINOR=1&& set CARGO_PKG_VERSION_PATCH=0&& set CARGO_PKG_VERSION_PRE=""&& set DEBUG=true&& set HOST=x86_64-pc-windows-msvc&& set NUM_JOBS=8&& set OPT_LEVEL=0&& set OUT_DIR=C:\Users\lena\Development\Projects\Rust\metal\target\debug\build\metal-ast-ng-0415d808c3f882a3\out&& set PATH="C:\Users\lena\Development\Projects\Rust\metal\target\debug\deps;C:\Users\lena\Development\Projects\Rust\metal\target\debug;D:\Scoop\persist\rustup\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib;D:\Scoop\persist\rustup\.cargo\bin;C:\Program Files\PowerShell\7;C:\Program Files (x86)\Common Files\Oracle\Java\javapath;C:\Users\lena\scoop\apps\openssh\current;C:\WINDOWS\system32;C:\WINDOWS;C:\WINDOWS\System32\Wbem;C:\WINDOWS\System32\WindowsPowerShell\v1.0\;C:\WINDOWS\System32\OpenSSH\;C:\Program Files (x86)\NVIDIA Corporation\PhysX\Common;C:\Program Files\Git\cmd;C:\Program Files\dotnet\;C:\Program Files (x86)\Gpg4win\..\GnuPG\bin;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit\;C:\Program Files\Cloudflare\Cloudflare WARP\;D:\Programs\AMDuProf\bin;D:\Programs\Performance;C:\Program Files\NVIDIA Corporation\NVIDIA app\NvDLISR;C:\Program Files\PowerShell\7\;D:\Scoop\apps\vscode\current\bin;D:\Scoop\apps\temurin17-jdk\current\bin;D:\Scoop\apps\mingw\current\bin;D:\Scoop\apps\llvm\current\bin;D:\Scoop\apps\volta\current\appdata\bin;D:\Scoop\apps\temurin11-jdk\current\bin;D:\Scoop\apps\rustup\current\.cargo\bin;D:\Scoop\apps\gcc\current\bin;D:\Scoop\apps\python\current\Scripts;D:\Scoop\apps\python\current;D:\Scoop\shims;C:\Users\lena\AppData\Local\pnpm;C:\Users\lena\AppData\Roaming\edgedb\bin;C:\Users\lena\AppData\Local\Microsoft\WindowsApps;c:\users\lena\appdata\roaming\python\python311\scripts;c:\users\lena\.local\bin;C:\Users\lena\.dotnet\tools;C:\Users\lena\.dotnet\tools;C:\Users\lena\.dotnet\tools;C:\Users\lena\.dotnet\tools;C:\Users\lena\.dotnet\tools;C:\Users\lena\AppData\Local\JetBrains\Toolbox\scripts;C:\Users\lena\Downloads\bup\dm\bin;D:\Programs\ghcup\bin;C:\Users\lena\AppData\Roaming\local\bin;C:\Users\lena\.dotnet\tools;C:\Users\lena\AppData\Local\Programs\Ollama;C:\Users\lena\.dotnet\tools;D:\Programs\Airshipper;C:\Users\lena\.deno\bin;"&& set PROFILE=debug&& set RUSTC=D:\Scoop\persist\rustup\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\rustc.exe&& set RUSTDOC=D:\Scoop\persist\rustup\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\rustdoc.exe&& set TARGET=x86_64-pc-windows-msvc&& C:\Users\lena\Development\Projects\Rust\metal\target\debug\build\metal-ast-ng-a30c9476952bb12c\build-script-build` (exit code: 101)
  --- stdout
  cargo::rerun-if-changed=./metal.ungram

  --- stderr
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Token(
      Token(
          0,
      ),
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = Some(
      Token(
          Token(
              0,
          ),
      ),
  )
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Opt(
      Token(
          Token(
              1,
          ),
      ),
  )
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Token(
      Token(
          1,
      ),
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = Some(
      Token(
          Token(
              1,
          ),
      ),
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = Some(
      Token(
          Token(
              1,
          ),
      ),
  )
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Opt(
      Token(
          Token(
              2,
          ),
      ),
  )
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Token(
      Token(
          2,
      ),
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = Some(
      Token(
          Token(
              2,
          ),
      ),
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = Some(
      Token(
          Token(
              2,
          ),
      ),
  )
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
      [
          Token(
              Token(
                  3,
              ),
          ),
          Node(
              Node(
                  4,
              ),
          ),
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Alt(
      [
          Node(
              Node(
                  0,
              ),
          ),
          Node(
              Node(
                  27,
              ),
          ),
          Node(
              Node(
                  28,
              ),
          ),
          Node(
              Node(
                  29,
              ),
          ),
          Node(
              Node(
                  30,
              ),
          ),
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
      [
          Token(
              Token(
                  4,
              ),
          ),
          Node(
              Node(
                  6,
              ),
          ),
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Node(
      Node(
          0,
      ),
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = Some(
      Node(
          Node(
              0,
          ),
      ),
  )
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
      [
          Token(
              Token(
                  5,
              ),
          ),
          Node(
              Node(
                  8,
              ),
          ),
          Token(
              Token(
                  6,
              ),
          ),
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Alt(
      [
          Node(
              Node(
                  9,
              ),
          ),
          Node(
              Node(
                  10,
              ),
          ),
          Node(
              Node(
                  4,
              ),
          ),
          Node(
              Node(
                  11,
              ),
          ),
          Node(
              Node(
                  12,
              ),
          ),
          Node(
              Node(
                  13,
              ),
          ),
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
      [
          Labeled {
              label: "vis",
              rule: Node(
                  Node(
                      1,
                  ),
              ),
          },
          Token(
              Token(
                  7,
              ),
          ),
          Node(
              Node(
                  0,
              ),
          ),
          Labeled {
              label: "ty",
              rule: Opt(
                  Node(
                      Node(
                          5,
                      ),
                  ),
              ),
          },
          Labeled {
              label: "value",
              rule: Node(
                  Node(
                      3,
                  ),
              ),
          },
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
      [
          Labeled {
              label: "vis",
              rule: Node(
                  Node(
                      1,
                  ),
              ),
          },
          Token(
              Token(
                  8,
              ),
          ),
          Node(
              Node(
                  0,
              ),
          ),
          Token(
              Token(
                  5,
              ),
          ),
          Labeled {
              label: "body",
              rule: Opt(
                  Node(
                      Node(
                          14,
                      ),
                  ),
              ),
          },
          Token(
              Token(
                  6,
              ),
          ),
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
      [
          Labeled {
              label: "vis",
              rule: Node(
                  Node(
                      1,
                  ),
              ),
          },
          Token(
              Token(
                  12,
              ),
          ),
          Node(
              Node(
                  0,
              ),
          ),
          Labeled {
              label: "sig",
              rule: Node(
                  Node(
                      18,
                  ),
              ),
          },
          Labeled {
              label: "body",
              rule: Node(
                  Node(
                      7,
                  ),
              ),
          },
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
      [
          Labeled {
              label: "vis",
              rule: Node(
                  Node(
                      1,
                  ),
              ),
          },
          Token(
              Token(
                  13,
              ),
          ),
          Labeled {
              label: "tree",
              rule: Node(
                  Node(
                      20,
                  ),
              ),
          },
          Token(
              Token(
                  14,
              ),
          ),
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
      [
          Labeled {
              label: "vis",
              rule: Node(
                  Node(
                      1,
                  ),
              ),
          },
          Token(
              Token(
                  16,
              ),
          ),
          Node(
              Node(
                  0,
              ),
          ),
          Token(
              Token(
                  5,
              ),
          ),
          Labeled {
              label: "body",
              rule: Node(
                  Node(
                      24,
                  ),
              ),
          },
          Token(
              Token(
                  6,
              ),
          ),
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
      [
          Node(
              Node(
                  15,
              ),
          ),
          Rep(
              Seq(
                  [
                      Token(
                          Token(
                              9,
                          ),
                      ),
                      Node(
                          Node(
                              15,
                          ),
                      ),
                  ],
              ),
          ),
          Opt(
              Token(
                  Token(
                      9,
                  ),
              ),
          ),
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Alt(
      [
          Node(
              Node(
                  16,
              ),
          ),
          Node(
              Node(
                  11,
              ),
          ),
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
      [
          Node(
              Node(
                  0,
              ),
          ),
          Labeled {
              label: "data_ty",
              rule: Opt(
                  Node(
                      Node(
                          17,
                      ),
                  ),
              ),
          },
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
      [
          Token(
              Token(
                  10,
              ),
          ),
          Node(
              Node(
                  6,
              ),
          ),
          Token(
              Token(
                  11,
              ),
          ),
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
      [
          Token(
              Token(
                  10,
              ),
          ),
          Labeled {
              label: "inputs",
              rule: Opt(
                  Seq(
                      [
                          Node(
                              Node(
                                  19,
                              ),
                          ),
                          Rep(
                              Seq(
                                  [
                                      Token(
                                          Token(
                                              9,
                                          ),
                                      ),
                                      Node(
                                          Node(
                                              19,
                                          ),
                                      ),
                                  ],
                              ),
                          ),
                          Opt(
                              Token(
                                  Token(
                                      9,
                                  ),
                              ),
                          ),
                      ],
                  ),
              ),
          },
          Token(
              Token(
                  11,
              ),
          ),
          Labeled {
              label: "return_ty",
              rule: Opt(
                  Node(
                      Node(
                          5,
                      ),
                  ),
              ),
          },
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
      [
          Labeled {
              label: "mutness",
              rule: Node(
                  Node(
                      2,
                  ),
              ),
          },
          Node(
              Node(
                  0,
              ),
          ),
          Labeled {
              label: "ty",
              rule: Opt(
                  Node(
                      Node(
                          5,
                      ),
                  ),
              ),
          },
          Labeled {
              label: "default",
              rule: Opt(
                  Node(
                      Node(
                          3,
                      ),
                  ),
              ),
          },
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Alt(
      [
          Node(
              Node(
                  21,
              ),
          ),
          Node(
              Node(
                  22,
              ),
          ),
          Node(
              Node(
                  23,
              ),
          ),
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Node(
      Node(
          0,
      ),
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = Some(
      Node(
          Node(
              0,
          ),
      ),
  )
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
      [
          Labeled {
              label: "segment",
              rule: Node(
                  Node(
                      0,
                  ),
              ),
          },
          Token(
              Token(
                  15,
              ),
          ),
          Labeled {
              label: "rest",
              rule: Node(
                  Node(
                      20,
                  ),
              ),
          },
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
      [
          Token(
              Token(
                  5,
              ),
          ),
          Labeled {
              label: "subtrees",
              rule: Opt(
                  Seq(
                      [
                          Node(
                              Node(
                                  20,
                              ),
                          ),
                          Rep(
                              Seq(
                                  [
                                      Token(
                                          Token(
                                              9,
                                          ),
                                      ),
                                      Node(
                                          Node(
                                              20,
                                          ),
                                      ),
                                  ],
                              ),
                          ),
                          Opt(
                              Token(
                                  Token(
                                      9,
                                  ),
                              ),
                          ),
                      ],
                  ),
              ),
          },
          Token(
              Token(
                  6,
              ),
          ),
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
      [
          Node(
              Node(
                  25,
              ),
          ),
          Rep(
              Seq(
                  [
                      Token(
                          Token(
                              9,
                          ),
                      ),
                      Node(
                          Node(
                              25,
                          ),
                      ),
                  ],
              ),
          ),
          Opt(
              Token(
                  Token(
                      9,
                  ),
              ),
          ),
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Alt(
      [
          Node(
              Node(
                  26,
              ),
          ),
          Node(
              Node(
                  11,
              ),
          ),
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
      [
          Labeled {
              label: "vis",
              rule: Node(
                  Node(
                      1,
                  ),
              ),
          },
          Node(
              Node(
                  0,
              ),
          ),
          Labeled {
              label: "ty",
              rule: Node(
                  Node(
                      5,
                  ),
              ),
          },
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
      [
          Labeled {
              label: "op",
              rule: Node(
                  Node(
                      31,
                  ),
              ),
          },
          Node(
              Node(
                  4,
              ),
          ),
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
      [
          Labeled {
              label: "lhs",
              rule: Node(
                  Node(
                      4,
                  ),
              ),
          },
          Labeled {
              label: "op",
              rule: Node(
                  Node(
                      32,
                  ),
              ),
          },
          Labeled {
              label: "rhs",
              rule: Node(
                  Node(
                      4,
                  ),
              ),
          },
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Seq(
      [
          Node(
              Node(
                  4,
              ),
          ),
          Token(
              Token(
                  10,
              ),
          ),
          Rep(
              Seq(
                  [
                      Node(
                          Node(
                              4,
                          ),
                      ),
                      Opt(
                          Token(
                              Token(
                                  9,
                              ),
                          ),
                      ),
                  ],
              ),
          ),
          Token(
              Token(
                  11,
              ),
          ),
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Alt(
      [
          Node(
              Node(
                  33,
              ),
          ),
          Node(
              Node(
                  34,
              ),
          ),
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Alt(
      [
          Token(
              Token(
                  17,
              ),
          ),
          Token(
              Token(
                  18,
              ),
          ),
          Token(
              Token(
                  19,
              ),
          ),
          Token(
              Token(
                  20,
              ),
          ),
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Alt(
      [
          Token(
              Token(
                  3,
              ),
          ),
          Token(
              Token(
                  21,
              ),
          ),
          Token(
              Token(
                  22,
              ),
          ),
          Token(
              Token(
                  23,
              ),
          ),
          Token(
              Token(
                  24,
              ),
          ),
          Token(
              Token(
                  25,
              ),
          ),
          Token(
              Token(
                  26,
              ),
          ),
          Token(
              Token(
                  27,
              ),
          ),
          Token(
              Token(
                  28,
              ),
          ),
          Token(
              Token(
                  29,
              ),
          ),
          Token(
              Token(
                  30,
              ),
          ),
          Token(
              Token(
                  31,
              ),
          ),
          Token(
              Token(
                  17,
              ),
          ),
          Token(
              Token(
                  18,
              ),
          ),
          Token(
              Token(
                  32,
              ),
          ),
          Token(
              Token(
                  33,
              ),
          ),
          Token(
              Token(
                  34,
              ),
          ),
          Token(
              Token(
                  35,
              ),
          ),
          Token(
              Token(
                  36,
              ),
          ),
          Token(
              Token(
                  37,
              ),
          ),
          Token(
              Token(
                  38,
              ),
          ),
          Token(
              Token(
                  39,
              ),
          ),
          Token(
              Token(
                  40,
              ),
          ),
          Token(
              Token(
                  41,
              ),
          ),
          Token(
              Token(
                  42,
              ),
          ),
          Token(
              Token(
                  43,
              ),
          ),
          Token(
              Token(
                  44,
              ),
          ),
          Token(
              Token(
                  45,
              ),
          ),
          Token(
              Token(
                  46,
              ),
          ),
          Token(
              Token(
                  47,
              ),
          ),
          Token(
              Token(
                  48,
              ),
          ),
          Seq(
              [
                  Token(
                      Token(
                          49,
                      ),
                  ),
                  Token(
                      Token(
                          15,
                      ),
                  ),
              ],
          ),
      ],
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = None
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Token(
      Token(
          50,
      ),
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = Some(
      Token(
          Token(
              50,
          ),
      ),
  )
  [crates\metal-ast-ungram\src\engram.rs:51:20] self = Token(
      Token(
          51,
      ),
  )
  [crates\metal-ast-ungram\src\engram.rs:51:9] match dbg!(self)
  {
      rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule), Rule::Opt(rule) =>
      rule.as_ref().unwrap_simple(), Rule::Labeled { rule, .. } =>
      rule.as_ref().unwrap_simple(), _ => None,
  } = Some(
      Token(
          Token(
              51,
          ),
      ),
  )
  thread 'main' panicked at crates\metal-ast-ungram\src\lib.rs:36:5:
  explicit panic
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
