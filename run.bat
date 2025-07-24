@echo off
setlocal enabledelayedexpansion

REM Configurare directoare
set "ROOT_DIR=%~dp0"
set "SOURCE_DIR=%ROOT_DIR%sourceCode"
set "CLIENT_DIR=%SOURCE_DIR%\Trap_the_mouse_client"
set "SERVER_DIR=%SOURCE_DIR%\Trap_the_mouse_server"

REM Creare directoare devTool
set "DEV_TOOL_DIR=%ROOT_DIR%devTool"
set "BUILD_DIR=%DEV_TOOL_DIR%\build"
set "CLIENT_BUILD_DIR=%BUILD_DIR%\clientBuild"
set "SERVER_BUILD_DIR=%BUILD_DIR%\serverBuild"

REM Creare structura directoare daca nu exista
if not exist "%DEV_TOOL_DIR%" mkdir "%DEV_TOOL_DIR%"
if not exist "%BUILD_DIR%" mkdir "%BUILD_DIR%"
if not exist "%CLIENT_BUILD_DIR%" mkdir "%CLIENT_BUILD_DIR%"
if not exist "%SERVER_BUILD_DIR%" mkdir "%SERVER_BUILD_DIR%"

REM Definirea cailor catre executabile 
set "SERVER_EXE=%SERVER_BUILD_DIR%\release\trap_the_mouse_server.exe"
set "CLIENT_EXE=%CLIENT_BUILD_DIR%\release\trap_the_mouse_client.exe"

set "BUILD_NEEDED=0"

if not exist "%SERVER_EXE%" (
    echo [Info] Serverul nu este compilat. Se va construi.
    set "BUILD_NEEDED=1"
)

if not exist "%CLIENT_EXE%" (
    echo [Info] Clientul nu este compilat. Se va construi.
    set "BUILD_NEEDED=1"
)

REM Se face BUILD, daca nu s-a facut pana acum
if "%BUILD_NEEDED%"=="1" (
    echo [Info] Compilare server...
    cd /d "%SERVER_DIR%"
    set "CARGO_TARGET_DIR=%SERVER_BUILD_DIR%"
    cargo build --release
    if errorlevel 1 (
        echo [Eroare] Compilarea serverului a esuat!
        exit /b 1
    )
    
    echo [Info] Compilare client...
    cd /d "%CLIENT_DIR%"
    set "CARGO_TARGET_DIR=%CLIENT_BUILD_DIR%"
    cargo build --release
    if errorlevel 1 (
        echo [Eroare] Compilarea clientului a esuat!
        exit /b 1
    )
)

REM Verificare existenta executabile
if not exist "%SERVER_EXE%" (
    echo [Eroare] Executabilul server nu a fost gasit la: %SERVER_EXE%
    echo Verificati numele corect in directorul: %SERVER_BUILD_DIR%\release\
    dir "%SERVER_BUILD_DIR%\release\*.exe"
    exit /b 1
)

if not exist "%CLIENT_EXE%" (
    echo [Eroare] Executabilul client nu a fost gasit la: %CLIENT_EXE%
    echo Verificati numele corect in directorul: %CLIENT_BUILD_DIR%\release\
    dir "%CLIENT_BUILD_DIR%\release\*.exe"
    exit /b 1
)

REM Copiere directorul files necesar clientului
if exist "%CLIENT_DIR%\files" (
    echo [Info] Copiez directorul files pentru client...
    if not exist "%BUILD_DIR%\files" mkdir "%BUILD_DIR%\files"
    xcopy /E /Y /I "%CLIENT_DIR%\files" "%BUILD_DIR%\files"
)

REM Verificam daca este deja deschis serverul(il deschidem o singura data)
tasklist /FI "IMAGENAME eq trap_the_mouse_server.exe" | find "trap_the_mouse_server.exe" > nul
if errorlevel 1 (
    echo [Info] Pornire server...
    start "" "%SERVER_EXE%"
    
    REM Delay pentru initializarea serverului
    timeout /t 2 /nobreak > nul
) else (
    echo [Info] Serverul este deja pornit.
)

REM Pornire client cu directorul de lucru setat la directorul build
echo [Info] Pornire client...
cd /d "%BUILD_DIR%"
start "" "%CLIENT_EXE%"

echo [Succes] Aplicatia a fost pornita cu succes!