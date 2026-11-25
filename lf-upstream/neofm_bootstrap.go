//go:build neofm

// NeoFM's native packaging adapter. Navigation and rendering are unchanged.
package main

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
)

var neofmVersion = "0.3.0"
var neofmBin, neofmState string

func neofmProgramName() string { return "NeoFM" }

func neofmMust(err error) {
	if err != nil {
		fmt.Fprintln(os.Stderr, "NeoFM startup:", err)
		os.Exit(1)
	}
}

func neofmSet(k, v string) { neofmMust(os.Setenv(k, v)) }

// Initialize before lf resolves configuration/state directories and PATHEXT.
func neofmInit() {
	exe, err := os.Executable()
	neofmMust(err)
	exe, err = filepath.EvalSymlinks(exe)
	neofmMust(err)
	neofmBin = filepath.Dir(exe)
	home, err := os.UserHomeDir()
	neofmMust(err)
	sep := string(os.PathListSeparator)
	path := neofmBin + sep + os.Getenv("PATH")
	if runtime.GOOS == "windows" {
		sys := os.Getenv("SystemRoot")
		if sys == "" {
			sys = "C:\\Windows"
			neofmSet("SystemRoot", sys)
		}
		cmd := filepath.Join(sys, "System32", "cmd.exe")
		_, err := os.Stat(cmd)
		neofmMust(err)
		neofmSet("ComSpec", cmd)
		path = filepath.Join(sys, "System32") + sep + sys + sep + path
		neofmSet("PATHEXT", ".COM;.EXE;.BAT;.CMD;"+os.Getenv("PATHEXT"))
		local := os.Getenv("LOCALAPPDATA")
		if local == "" {
			neofmMust(fmt.Errorf("LOCALAPPDATA is empty"))
		}
		neofmState = filepath.Join(local, "LFpreview") // retain deployed state and logs
		neofmSet("LFP_HOME", neofmBin+string(os.PathSeparator))
		if os.Getenv("LFP_QUICKLOOK_EXE") == "" {
			neofmSet("LFP_QUICKLOOK_EXE", filepath.Join(local, "Programs", "QuickLook", "QuickLook.exe"))
		}
	} else {
		stateName := "lfpreview-linux"
		if runtime.GOOS == "darwin" {
			stateName = "lfmac"
		}
		neofmState = filepath.Join(home, ".local", "state", stateName)
		for _, k := range []string{"LFP_QL_DRY_RUN", "LFP_QL_TRACE", "LFP_POC_TRACE"} {
			neofmMust(os.Unsetenv(k))
		}
	}
	neofmSet("PATH", path)
	envPath = path
	for _, sub := range []string{"config", "data", "data/lf"} {
		neofmMust(os.MkdirAll(filepath.Join(neofmState, sub), 0700))
	}
	neofmSet("LF_CONFIG_HOME", filepath.Join(neofmState, "config"))
	neofmSet("LF_DATA_HOME", filepath.Join(neofmState, "data"))
	gVersion = "NeoFM v" + neofmVersion + " (" + runtime.GOOS + "/" + runtime.GOARCH + ")"
}

const neofmKeys = "nmap y\nnmap d\nnmap yy copy\nnmap p paste\nnmap x cut\nnmap dd delete\ncmd open :toggle\nnmap <enter> open\nnmap <c-space> toggle\n"

func neofmPrepare() func() {
	args := []string{os.Args[0], "-single"}
	if runtime.GOOS == "windows" {
		args = append(args, "-config", filepath.Join(neofmBin, "lfrc"), "-log", filepath.Join(neofmState, "data", "lfwin.log"))
		os.Args = append(args, os.Args[1:]...)
		return func() {}
	}
	f, err := os.CreateTemp(filepath.Join(neofmState, "config"), "neofm-lfrc.")
	neofmMust(err)
	path := f.Name()
	config := fmt.Sprintf("set shell sh\nset preview\nset hidden\nset nopreload\nset ratios 1:2:4\nset previewer %q\n", filepath.Join(neofmBin, "neofm-preview")) + neofmKeys
	config += fmt.Sprintf("cmd delete \x24{{ %q }}\n", filepath.Join(neofmBin, "neofm-trash"))
	if runtime.GOOS == "darwin" {
		config += "nmap o &{{ /usr/bin/open \"$f\" }}\nnmap <space> \x24{{ /usr/bin/qlmanage -p \"$f\" }}\n"
	} else {
		config += "nmap o &{{ xdg-open \"$f\" }}\n"
		_, err := exec.LookPath("dzurkel")
		if (os.Getenv("DISPLAY") != "" || os.Getenv("WAYLAND_DISPLAY") != "") && strings.Join([]string{os.Getenv("SSH_CONNECTION"), os.Getenv("SSH_TTY"), os.Getenv("SSH_CLIENT")}, "") == "" && err == nil {
			config += "nmap <space> &{{ dzurkel --show \"$f\" }}\n"
		} else {
			config += fmt.Sprintf("nmap <space> \x24{{ %q quicklook \"$f\" }}\n", filepath.Join(neofmBin, "neofm-preview"))
		}
	}
	_, err = f.WriteString(config)
	neofmMust(err)
	neofmMust(f.Close())
	args = append(args, "-config", path)
	os.Args = append(args, os.Args[1:]...)
	return func() { _ = os.Remove(path) }
}
