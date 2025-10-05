.PHONY: install enable disable status logs clean

install:
	cargo build --release
	mkdir -p ~/.config/systemd/user
	mkdir -p ~/.local/bin
	mkdir -p ~/.config/anyrun/plugins
	cp target/release/libraindrop.so ~/.config/anyrun/plugins/
	cp target/release/liblogseq.so ~/.config/anyrun/plugins/
	cp target/release/update-raindrop-cache ~/.local/bin/
	cp res/download_raindrop_favicons.sh ~/.config/anyrun/
	chmod +x ~/.config/anyrun/download_raindrop_favicons.sh
	cp res/update-logseq-cache.sh ~/.local/bin/update-logseq-cache
	chmod +x ~/.local/bin/update-logseq-cache
	cp res/update-raindrop-cache.service res/update-raindrop-cache.timer ~/.config/systemd/user/
	cp res/download-raindrop-favicons.service res/download-raindrop-favicons.timer ~/.config/systemd/user/
	cp res/update-logseq-cache.service res/update-logseq-cache.timer ~/.config/systemd/user/
	systemctl --user daemon-reload

enable: install
	systemctl --user enable --now update-raindrop-cache.timer
	systemctl --user enable --now download-raindrop-favicons.timer
	systemctl --user enable --now update-logseq-cache.timer

disable:
	systemctl --user disable --now update-raindrop-cache.timer
	systemctl --user disable --now download-raindrop-favicons.timer
	systemctl --user disable --now update-logseq-cache.timer

status:
	systemctl --user status update-raindrop-cache.timer
	systemctl --user status download-raindrop-favicons.timer
	systemctl --user status update-logseq-cache.timer
	systemctl --user list-timers update-raindrop-cache.timer download-raindrop-favicons.timer update-logseq-cache.timer

logs:
	journalctl --user -u update-raindrop-cache.service -f

clean:
	systemctl --user disable --now update-raindrop-cache.timer || true
	systemctl --user disable --now download-raindrop-favicons.timer || true
	systemctl --user disable --now update-logseq-cache.timer || true
	systemctl --user daemon-reload
	rm -f ~/.config/systemd/user/update-raindrop-cache.service
	rm -f ~/.config/systemd/user/update-raindrop-cache.timer
	rm -f ~/.local/bin/update-raindrop-cache
	rm -f ~/.config/systemd/user/download-raindrop-favicons.service
	rm -f ~/.config/systemd/user/download-raindrop-favicons.timer
	rm -f ~/.config/anyrun/download_raindrop_favicons.sh
	rm -f ~/.config/systemd/user/update-logseq-cache.service
	rm -f ~/.config/systemd/user/update-logseq-cache.timer
	rm -f ~/.local/bin/update-logseq-cache
