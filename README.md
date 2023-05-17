# RustyPwnedPasswordsDownloader

This is a Rust implementation of this project: https://github.com/HaveIBeenPwned/PwnedPasswordsDownloader

## Improvements
The original project has a few issues that this project solves:

- dotnet requirment: I appreciate that dotnet now runs on Linux. This has been noted in multiple Github issues, however many of us do not wish to install a Microsoft Framework just to install a small app that downloads a file. This app is a standalone executable.
- The output is sorted properly. If you are a user for example, of (DSInternals)[https://github.com/MichaelGrafnetter/DSInternals/blob/master/Documentation/PowerShell/Test-PasswordQuality.md] and specifically the parameter `-WeakPasswordHashesSortedFile` the behaviour of the dotnet application may be a surprise to you.

## New Features
If you would like to run a "has this password been seen before" type service, you require the way the dotnet downloader works. However, if you would like to meet modern password requirements for "block commonly used passwords", you do not need a constantly updated, massive text file of passwords that have been seen once (which are not "common"). This downloader allows output to be filtered based on the number of times a password was see (defaults to 3). Set this parameter to 1 to get the old behaviour.

## Usage

Usage: rustypwneddownloader.exe [OPTIONS] --filename <FILENAME>

Options:
  -f, --filename <FILENAME>  Name of output file
  -m, --minimum <MINIMUM>    Filter to passwords breached this many times [default: 3]
  -h, --help                 Print help
  -V, --version              Print version

## References
This issue describes a shell script version on which this app was based.
https://github.com/HaveIBeenPwned/PwnedPasswordsDownloader/issues/30

 