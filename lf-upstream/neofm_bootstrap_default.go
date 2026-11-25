//go:build !neofm

package main

func neofmInit()               {}
func neofmPrepare() func()     { return func() {} }
func neofmProgramName() string { return "lf" }
