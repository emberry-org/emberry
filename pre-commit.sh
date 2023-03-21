#!/bin/sh
# requires gum
#
# This script runs a few checks that should be run before commiting
#
# This script will automatically add itself to the git-hooks if there
# is no other pre-commit hook present

if [ ! -f .git/hooks/pre-commit ]; then
	ln -s pre-commit.sh .git/hooks/pre-commit
fi

git_newline() {
	for f in $(git grep --cached -Il '')
	do
		tail --bytes=1 $f | read -r _ || return 1
	done
}

fail() {
	cmd=$1
	gum style \
	--foreground '#F55' --border-foreground '#F55' --border double \
	--align center --width 50 --margin "1 2" --padding "2 4" \
	$'pre-commit checks failed\n' "cmd: '$cmd'"
}

success() {
	cmd=$1
	gum style \
	--foreground '#5F5' --border-foreground '#5F5' --border double \
	--align center --width 50 --margin "1 2" --padding "2 4" \
	$'pre-commit checks cleared\n' "cmd: '$cmd'"
}

run() {
	cmd=$1
	$cmd > /dev/null 2> /dev/null
	if [ $? -eq 0 ]; then
		success "$cmd"
	else
		fail "$cmd"
		exit 1
	fi

}

run "git_newline"

cd src-tauri

run "cargo fmt --check"
run "cargo clippy --all-targets --all-features"
run "cargo test"

gum style \
		--foreground '#5F5' --border-foreground '#5F5' --border double \
		--align center --width 50 --margin "1 2" --padding "2 4" \
		$'all pre-commit checks cleared'

