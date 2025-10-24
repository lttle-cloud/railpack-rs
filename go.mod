module github.com/lttle-cloud/railway-rs

go 1.25.3

require github.com/railwayapp/railpack v0.9.2

require (
	github.com/BurntSushi/toml v1.4.0 // indirect
	github.com/Masterminds/semver/v3 v3.4.0 // indirect
	github.com/alexflint/go-filemutex v1.3.0 // indirect
	github.com/aymanbagabas/go-osc52/v2 v2.0.1 // indirect
	github.com/bahlo/generic-list-go v0.2.0 // indirect
	github.com/bmatcuk/doublestar/v4 v4.8.0 // indirect
	github.com/buger/jsonparser v1.1.1 // indirect
	github.com/charmbracelet/lipgloss v1.0.0 // indirect
	github.com/charmbracelet/log v0.4.0 // indirect
	github.com/charmbracelet/x/ansi v0.4.2 // indirect
	github.com/go-logfmt/logfmt v0.6.0 // indirect
	github.com/invopop/jsonschema v0.13.0 // indirect
	github.com/lucasb-eyer/go-colorful v1.2.0 // indirect
	github.com/mailru/easyjson v0.7.7 // indirect
	github.com/mattn/go-isatty v0.0.20 // indirect
	github.com/mattn/go-runewidth v0.0.15 // indirect
	github.com/moby/patternmatcher v0.6.0 // indirect
	github.com/muesli/termenv v0.15.2 // indirect
	github.com/rivo/uniseg v0.4.7 // indirect
	github.com/stretchr/objx v0.5.2 // indirect
	github.com/tailscale/hujson v0.0.0-20241010212012-29efb4a0184b // indirect
	github.com/wk8/go-ordered-map/v2 v2.1.8 // indirect
	golang.org/x/exp v0.0.0-20240909161429-701f63a606c0 // indirect
	golang.org/x/sys v0.28.0 // indirect
	gopkg.in/yaml.v2 v2.4.0 // indirect
	gopkg.in/yaml.v3 v3.0.1 // indirect
)

// replace github.com/railwayapp/railpack => ../railpack
replace github.com/railwayapp/railpack => github.com/lttle-cloud/railpack v0.0.0-20251024012635-b5666f37246d
