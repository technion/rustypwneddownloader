# RustyPwnedPasswordsDownloader

This is a Rust implementation of this project: https://github.com/HaveIBeenPwned/PwnedPasswordsDownloader

## Installation

Grab the single executable download from here: https://github.com/technion/rustypwneddownloader/releases/latest/download/rustypwneddownloader.exe
This app is expected to run on any platform that Rust compiles for, but presently you'll need to fetch the source and build it.

## Improvements
The major issue that this project solves is the dotnet requirement: I appreciate that dotnet now runs on Linux. This has been noted in multiple Github issues, however many of us do not wish to install a Microsoft Framework just to then install a small app that downloads a file. This app is a standalone executable.

## New Features
If you would like to run a "has this password been seen before" type service, you require the way the dotnet downloader works. However, if you would like to meet modern password requirements for "block commonly used passwords", you do not need a constantly updated, massive text file of passwords that have been seen once (which are not "common"). This downloader allows output to be filtered based on the number of times a password was see (defaults to 3). Set this parameter to 1 to get the old behaviour.

## Usage
```
Usage: rustypwneddownloader.exe [OPTIONS] --filename <FILENAME>

Options:
  -f, --filename <FILENAME>  Name of output file
  -m, --minimum <MINIMUM>    Filter to passwords breached this many times [default: 3]
  -h, --help                 Print help
  -V, --version              Print version
```

## Verifying Builds
All releases are Authenticode signed. You can verify this as below:
```
PS > Get-AuthenticodeSignature .\rustypwneddownloader.exe


    Directory: Downloads


SignerCertificate                         Status                                 Path
-----------------                         ------                                 ----
9C74A96F12A82D4AE8E23E7214D21033D81705A2  Valid                                  rustypwneddownloader.exe
```

In addition, for complete supply chain transparency all releases are signed with [SigStore](https://sigstore.dev/). If you have downloaded the cosign executable, you can verify a download with:
```
.\cosign-windows-amd64.exe verify-blob .\rustypwneddownloader.exe --bundle .\cosign<version>.bundle --certificate-oidc-issuer=https://github.com/login/oauth --certificate-identity=technion@lolware.net
```

## References
This issue describes a shell script version on which this app was based.
https://github.com/HaveIBeenPwned/PwnedPasswordsDownloader/issues/30

 