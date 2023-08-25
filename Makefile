test:
	cargo test -- --nocapture

merge-to-jba:
	git checkout JBA_brainstorm
	git merge dfebs_brainstorm


merge-to-dfebs:
	git checkout dfebs_brainstorm
	git merge JBA_brainstorm

# This stuff is for me because I always forget how to deal with merge conflicts
merge-all-theirs:
	grep -lr '<<<<<<<' . | xargs -I {} git checkout --theirs {}

merge-all-ours:
	grep -lr '<<<<<<<' . | xargs -I {} git checkout --ours {}
