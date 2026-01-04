.PHONY:: watch-doc kak-tool.tmplr misc test watch-test
watch-doc:
	fd -e rs | entr -r cargo doc

TMPLR_TMP := $(shell mktemp -p /tmp -d tmplr.XXXX)

kak-tool.tmplr:
	fd -e rs -e toml | tar cvvf - -T - | tar xvvf - -C $(TMPLR_TMP)
	cd $(TMPLR_TMP); tmplr create kak-tool
	cp -f $(TMPLR_TMP)/kak-tool.tmplr ./


test:
	cargo test -- --nocapture

watch-test:
	fd -e rs | entr -r make test

misc: LICENSE

LICENSE: cue/LICENSE.cue
	cue export $< --out text -o LICENSE -e license



