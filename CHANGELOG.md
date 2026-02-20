# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-02-20

### Added
* Added support for the Japanese release of DQMJ1. (#6)

### Changed
* Made various misc improvements to GitHub Actions builds. (#3, #5)
    * Cache Rust dependencies
    * Check Rust code formatting
    * Run clippy linter on Rust code
    * Run Biome formatter and linter on JS, CSS, JSON, and HTML
    * Add status badges to readme

## [0.2.0] - 2026-02-14

### Added
* Added support for editing skill sets. (#2)

## [0.1.1] - 2026-02-09

### Fixed
* Fixed bug that cause the program to close on startup. (#1)

## [0.1.0] - 2026-02-08

### Added
* Added support for editing encounters.
* Added support for patching ROM files.
* Added support for saving and loading mods.