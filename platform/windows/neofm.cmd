@echo off
setlocal
set "LFP_HOME=%~dp0"
if not defined LFP_QUICKLOOK_EXE set "LFP_QUICKLOOK_EXE=%LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe"
if not defined SystemRoot set "SystemRoot=C:\Windows"
set "ComSpec=%SystemRoot%\System32\cmd.exe"
if not exist "%ComSpec%" (
  echo ERROR: Windows cmd.exe not found at "%ComSpec%"
  exit /b 1
)
set "PATH=%SystemRoot%\System32;%SystemRoot%;%~dp0;%PATH%"
set "PATHEXT=.COM;.EXE;.BAT;.CMD;%PATHEXT%"
set "LF_CONFIG_HOME=%LOCALAPPDATA%\LFpreview\config"
set "LF_DATA_HOME=%LOCALAPPDATA%\LFpreview\data"
if not exist "%LF_CONFIG_HOME%" mkdir "%LF_CONFIG_HOME%"
if not exist "%LF_DATA_HOME%\lf" mkdir "%LF_DATA_HOME%\lf"
"%~dp0lf.exe" -single -config "%~dp0lfrc" -log "%LF_DATA_HOME%\lfwin.log" %*
exit /b %ERRORLEVEL%
