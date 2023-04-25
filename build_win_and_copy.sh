#!/bin/bash

cargo build --target x86_64-pc-windows-gnu && cp -v target/x86_64-pc-windows-gnu/debug/photosort.exe /mnt/c/Users/Bohne/Documents/photosort.exe