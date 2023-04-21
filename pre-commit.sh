#!/bin/sh
# requires gum
#
# This script runs a few checks that should be run before commiting

git_newline() {
	for f in $(git grep --cached -Il '')
	do
		tail --bytes=1 $f | read -r _ || (echo "Missing newline: '$f'" && return 1) || FAIL=true
	done

	if [ -z "$FAIL" ]; then
		return 0
	else
		return 1
	fi
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
	$cmd
	if [ $? -eq 0 ]; then
		success "$cmd"
	else
		fail "$cmd"
		exit 1
	fi

}

run "git_newline"

run "pnpm audit"

cd src-tauri

run "cargo audit"
run "cargo fmt --check"
run "cargo clippy --all-targets --all-features"
run "cargo test"

gum style \
		--foreground '#5F5' --border-foreground '#5F5' --border double \
		--align center --width 50 --margin "1 2" --padding "2 4" \
		$'all pre-commit checks cleared'

