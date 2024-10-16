

enter:
	docker run -e "NEOVIM_UID=1000" -e "NEOVIM_GID=1000" -e "NEOVIM_MNT_DIR=$$(pwd)" -v $$pwd:/mnt/project kolserdav/coc-neovim-rust:latest 

