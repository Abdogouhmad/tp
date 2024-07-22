# TP
tp or TP is a command line tool that intends to parse and validate the configuration file 
for you text editor. The supported text editors are:
- [Diwan](https://github.com/Abdogouhmad/Diwan)
- [Helix](https://github.com/helix-editor/helix/)

## Tasks
- [ ] Create CLI (clap / idk)
	- [x] init the cli
	- [x] define 1st option command `-c / --check` for checking config file in specific path
	- [x] define 2nd op cmd `-g / generate` for generating the absolute config
		- [ ] http request that will download a template from my repo and place it in `~/.config/helix`
- [ ] get the absolute configuration 1st
- [x] handle the error displaying
	- [x] identify where is the error and print it to be colorized.
