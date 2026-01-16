#!/usr/bin/bash

git status 
git add . 
read -p "Your Commit?: " commit_name
git commit -m "${commit_name}"
