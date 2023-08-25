test:
	cargo test -- --nocapture

merge-to-jba:
	git checkout JBA_brainstorm
	git merge dfebs_brainstorm


merge-to-dfebs:
	git checkout dfebs_brainstorm
	git merge JBA_brainstorm

merge-all-theirs:
	grep -lr '<<<<<<<' . | xargs -I {} git checkout --theirs {}

merge-all-ours:
	grep -lr '<<<<<<<' . | xargs -I {} git checkout --ours {}
