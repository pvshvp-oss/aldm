# ALDM

A driver manager for Arch Linux.

[![Discord Server](https://dcbadge.vercel.app/api/server/cU5s6MPpQH?style=flat)](https://discord.gg/cU5s6MPpQH)
[![License: MPL v2.0](https://img.shields.io/badge/License-MPL--2.0-blue.svg)](https://www.mozilla.org/en-US/MPL/2.0/)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/shiva-patt-oss/aldm)
[![Release](https://github.com/shiva-patt-oss/aldm/actions/workflows/release.yml/badge.svg)](https://github.com/shiva-patt-oss/aldm/actions/workflows/release.yml)
[![Pre-Release (Git)](https://github.com/shiva-patt-oss/aldm/actions/workflows/pre_release.yml/badge.svg)](https://github.com/shiva-patt-oss/aldm/actions/workflows/pre_release.yml)

> **Note**: This project is not meant to track any specific database or configuration except for the application packaging files (`PKGBUILD`, build scripts), icons, and launch scripts. Use the [aldm-db](https://github.com/shiva-patt-oss/aldm-db) project for specific database or configuration.

## Cloning

To download the source code to your local computer for testing, or for development, you can clone from the remote repository using either SSH, or HTTPS. Below are instructions on how to do so using GitHub hosted code as remote.

### HTTPS

```bash
git clone https://github.com/shiva-patt-oss/aldm.git 
```

OR

### SSH

```bash
git clone git@github.com:shiva-patt-oss/aldm.git
```

## Packaging

Change to the project directory (`cd aldm`) and run any of the below scripts:
- `sh packaging/setup.sh <MODE>`: Builds and installs a package
- `sh packaging/build-package.sh <MODE>`: Just builds a package without installing it locally
- `sh packaging/install-package.sh <MODE>`: Just installs a package locally, except if no built package is detected, a package is built.

- where `<MODE>` can be one of the below
     1. `local`: Selects *aldm-local* from the local project that you have cloned already.
     2. `git`: Selects *aldm-git* from the latest git commit.
     3. `stable`: Selects *aldm* from the git tag corresponding to the [`pkgver` specified in the PKGBUILD](https://github.com/shiva-patt-oss/aldm/blob/main/packaging/aldm/PKGBUILD#L5). If `pkgver=0.0.1`, then the git tag `v0.0.1` is used for packaging. 
     
> **Note**: Any additional parameters passed to the above scripts are automatically sent to `makepkg` or `pacman` (whichever is applicable).

## Cloning

To download the source code to your local computer for testing, or for development, you can clone from the remote repository using either SSH, or HTTPS. Below are instructions on how to do so using GitHub hosted code as remote.

### HTTPS

```bash
git clone https://github.com/shiva-patt-oss/aldm.git 
```

OR

### SSH

```bash
git clone git@github.com:shiva-patt-oss/aldm.git
```

## Packaging

Change to the project directory (`cd aldm`) and run any of the below scripts:
- `sh packaging/setup.sh <MODE>`: Builds and installs a package
- `sh packaging/build-package.sh <MODE>`: Just builds a package without installing it locally
- `sh packaging/install-package.sh <MODE>`: Just installs a package locally, except if no built package is detected, a package is built.

- where `<MODE>` can be one of the below
     1. `local`: Selects *aldm-local* from the local project that you have cloned already.
     2. `git`: Selects *aldm-git* from the latest git commit.
     3. `stable`: Selects *aldm* from the git tag corresponding to the [`pkgver` specified in the PKGBUILD](https://github.com/shiva-patt-oss/aldm/blob/main/packaging/aldm/PKGBUILD#L5). If `pkgver=0.0.1`, then the git tag `v0.0.1` is used for packaging. 
     
> **Note**: Any additional parameters passed to the above scripts are automatically sent to `makepkg` or `pacman` (whichever is applicable).
