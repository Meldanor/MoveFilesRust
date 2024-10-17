# Move Files with Rust

A simple program that move files from a collection of subfolders to the main folder and renames them.

## Usage

    move_files_rust <PATH>

## Example

Structure before the program ran:
```
ParentFolder
    -> 01 [THIS IS A CHILD FOLDER, BUT NAMED 01]
        -> a.tmp
        -> b.tmp
    -> 02 [THIS IS A CHILD FOLDER, BUT NAMED 02]
        -> c.tmp
        -> d.tmp
```
Execute the program:
    move_files_rust ParentFolder

Structure after:
```
ParentFolder
    01_a.tmp
    01_b.tmp
    02_c.tmp
    02_d.tmp
    -> 01 [Folder is not deleted]
    -> 02 [Folder is not deleted]
```

## Reason

I had some folder with the structure and wanted to fix it. I used python, javascript and bash, but wanted to have a single binary for my OS. So I converted the script to Rust.