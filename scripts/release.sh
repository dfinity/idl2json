#!/usr/bin/env bash

set -e

die () {
    echo >&2 "$@"
    exit 1
}

announce() {
    term_green
    echo
    echo "===================================================================="
    echo "= $1"
    echo "===================================================================="
    echo
    term_reset
}

term_green() {
  tput setaf 2
}

term_reset() {
  tput sgr0
}

get_parameters() {
    [ "$#" -eq 1 ] || die "Usage: $0 <n.n.n>"
    [[ "$1" =~ ^([0-9]+\.[0-9]+\.[0-9]+)(-([A-Za-z]+)\.[0-9]+)?$ ]] || \
        die "'$1' is not a valid semantic version"

    export FINAL_RELEASE_BRANCH="release-${BASH_REMATCH[1]}"
    export NEW_VERSION=$1
    export BRANCH=$USER/release-$NEW_VERSION

    if [[ "$DRY_RUN" == '' ]]; then
        export DRY_RUN_ECHO=''
        dry_run_explain=''
    else
        export DRY_RUN_ECHO='echo DRY RUN: '
        dry_run_explain=' (dry run)'
    fi

    announce "Building release $NEW_VERSION as branch $BRANCH ($FINAL_RELEASE_BRANCH) $dry_run_explain"
}

pre_release_check() {
    announce "Checking that git is clean"
    if git status --untracked-files=no --porcelain | grep . ; then
        echo "Refusing to publsh with unclean git repo."
        exit 1
    fi
}

#
# build the release candidate and prepend the target directory to the PATH.
#
build_release_candidate() {
    announce "Building release candidate."
    cargo clean --release
    cargo build --release --locked
}

wait_for_response() {
    expected="$1"
    while true; do
        echo
        echo "All good?  Type '$expected' to continue."
        read -r answer
        if [ "$answer" == "$expected" ]; then
            break
        fi
    done
}

update_version() {
    for path in src/*/Cargo.toml ; do
        echo "... updating version in $path"
        # update first version in $path to be NEW_VERSION
        awk 'NR==1,/^version = ".*"/{sub(/^version = ".*"/, "version = \"'"$NEW_VERSION"'\"")} 1' <"$path" | sponge "$path"
	awk -v version="$NEW_VERSION" '/^idl2json *=/{sub(/^idl2json *= *"[0-9.]+"/, "idl2json = \"" version "\"")}{print $0}' <"$path" | sponge "$path"
    done
}

build_release_branch() {

    announce "Building branch $BRANCH for release $NEW_VERSION"

    echo "Cleaning up cargo build files..."
    $DRY_RUN_ECHO cargo clean --release

    echo "Switching to branch: $BRANCH"
    $DRY_RUN_ECHO git switch -c "$BRANCH"

    echo "Updating version in src/*/Cargo.toml to $NEW_VERSION"
    update_version

    echo "Building with cargo."
    # not --locked, because Cargo.lock needs to be updated with the new version
    # we already checked that it builds with --locked, when building the release candidate.
    cargo build --release

    echo "Appending version to public/manifest.json"
    # Append the new version to `public/manifest.json` by appending it to the `versions` list.
    jq --indent 4 '.versions += ["'"$NEW_VERSION"'"]' public/manifest.json | sponge public/manifest.json

    echo "Creating release branch: $BRANCH"
    $DRY_RUN_ECHO git add -u
    $DRY_RUN_ECHO git commit --signoff --message "chore: Release $NEW_VERSION"
    $DRY_RUN_ECHO git push origin "$BRANCH"

    echo "Please open a pull request to the $FINAL_RELEASE_BRANCH branch, review and approve it, then merge it manually."
    echo "  (The automerge-squash label will not work because the PR is not to the master branch)"

    wait_for_response 'PR merged'
}

tag_release_commit() {
    announce 'Tagging release commit'

    echo "Switching to the release branch."
    $DRY_RUN_ECHO git switch "$FINAL_RELEASE_BRANCH"

    $DRY_RUN_ECHO git branch --set-upstream-to=origin/"$FINAL_RELEASE_BRANCH" "$FINAL_RELEASE_BRANCH"

    echo "Pulling the remote branch"
    $DRY_RUN_ECHO git pull

    echo "Creating a new tag $NEW_VERSION"
    $DRY_RUN_ECHO git tag --annotate "$NEW_VERSION" --message "Release: $NEW_VERSION"

    echo "Displaying tags"
    git log -1
    git describe --always

    echo "Pushing tag $NEW_VERSION"
    $DRY_RUN_ECHO git push origin "$NEW_VERSION"
}

{
    get_parameters "$@"
    pre_release_check
    build_release_candidate
    build_release_branch
    tag_release_commit

    echo "All done!"
    exit
}
