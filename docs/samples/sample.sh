#! /bin/bash

DOTS=~/w/dotfiles
d() { rsync "$@" ; }
f() { rsync "$@" ; }

mkdir -p "$DOTS"/.config
d -a --delete-excluded --exclude='fish_variables' ~/.config/fish "$DOTS"/.config
d -a --delete-excluded --exclude='packer_compiled.lua' ~/.config/nvim "$DOTS"/.config
d -a --delete ~/.iterm "$DOTS"
d -a --delete-excluded --exclude='.netrwhist' --exclude='pack/' ~/.vim "$DOTS"

f ~/.bashrc "$DOTS"
f ~/.ghci "$DOTS"

f ~/.gitconfig "$DOTS"
git config --file="$DOTS"/.gitconfig user.name "John Doe"
git config --file="$DOTS"/.gitconfig user.email johndoe@example.com

f ~/.profile "$DOTS"
f ~/.tmux.conf "$DOTS"
f ~/.vimrc "$DOTS"
f ~/.zprofile "$DOTS"
f ~/.zshenv "$DOTS"
f ~/.zshrc "$DOTS"
