.PHONY: usage patches sources

repo_base = https://raw.githubusercontent.com/buttplugio/buttplug/refs/heads/master
ws_trans_path = $(repo_base)/buttplug/src/core/connector/transport/websocket

usage:
	@echo "Use \`make patches\` to update patches, \`make sources\` to update sources."

patches: patches/websocket_client.patch patches/websocket_server.patch
	@$(MAKE) GOAL=patches $?

sources: src/websocket_client.rs src/websocket_server.rs
	@$(MAKE) GOAL=sources $?

ifeq ($(GOAL),patches)

patches/websocket_%.patch: src/websocket_%.rs
	diff -u <(curl $(ws_trans_path)/$(<F)) $< > $@ ; true

else ifeq ($(GOAL),src)

src/websocket_%.rs: patches/websocket_%.patch
	curl $(ws_trans_path)/$(@F) > $@
	patch -u $@ -i $<

endif
