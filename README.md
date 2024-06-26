# Kavita Import Tool

[![test](https://github.com/mackenly/kavita-import-tool/actions/workflows/test.yml/badge.svg)](https://github.com/mackenly/kavita-import-tool/actions/workflows/test.yml)

<img src="./app-icon.png" width="100" height="100">

## Description
Kavita Import Tool allows users to import a variety of eBook, manga, and image files and export them into the correct [folder structure for Kavita](https://wiki.kavitareader.com/guides/scanner/managefiles). When multiple files are imported with the same name, the program picks the best format or the format selected by the user.

The tool currently supports simple file name-based structures like the one below. If you have a different structure you would like to see supported, please [create an issue](https://gitub.com/mackenly/kavita-import-tool/issues/new) with the details.

Example folder structure:
```
kavita-import-tool-output-[epoch time]
  ┖── LearningAssemblyLanguage
        ┖── LearningAssemblyLanguage.epub 
      RustForTypeScriptDevelopers
          ┖── RustForTypeScriptDevelopers.pdf
      DesigningMultimediaPipelinesWithGStreamer
          ┖── DesigningMultimediaPipelinesWithGStreamer.cbz
```

Supports: `epub`, `pdf`, `cbz`, `zip`, `rar`, `cbr`, `tar.gz`, `7zip`, `7z`, `cb7`, `cbt`, `png`, `jpeg`, `jpg`, `webp`, `gif`, and `avif`

## Documentation / Installation
Read the docs at [kavitaimporttool.mackenly.com](https://kavitaimporttool.mackenly.com/)

[Read the Quickstart Guide](https://kavitaimporttool.mackenly.com/quickstart) for a step-by-step guide on how to install and use the program.

TL;DR:
- Download the latest release from the [releases page](https://github.com/mackenly/kavita-import-tool/releases) for your operating system
- Install the program (not currently signed, so you may need to allow the installation)

## Features / Roadmap
- [x] Import EPUB files
- [x] Import PDF files
- [x] Support for all file types supported by Kavita
- [x] Select file based on a ranked list of formats
- [x] Allow users to select the preferred format to use
- [x] Export directly into the selected output folder or into a container folder for manual import
- [x] Drag and drop or select files to import
- [x] Automatic updates
- [ ] Support for episodic and series-based content following the Kavita format
- [ ] Your feature idea here! [Create an issue](https://github.com/mackenly/kavita-import-tool/issues/new) to suggest a feature

## Development Setup
- Clone the repository
- Ensure Rust and cargo are installed and setup 
- Run `npm install` to install dependencies
- Install Rust dependencies by running `cargo build`
- Run `npm run tauri dev` to start the development server and open the program in a window


## Disclaimer
Not associated with Kavita. This is a tool for creating the correct file structure for Kavita. This project is licensed under [AGPL-3.0](./LICENSE) license. AGPL-3.0 is a copyleft license that requires modifications to be released under the same license. Additionally, there is no warranty, and the software is provided as is. Contributors are not responsible for any damages or losses arising from the use of the software. Copyright 2024 Tricities Media Group LLC.
