@echo off
setlocal enabledelayedexpansion

set TOTAL=0
set PASSED=0
set FAILED=0

echo Running all tests...
echo.

for %%f in (test*.txt) do (
    set /a TOTAL+=1
    echo Testing %%f...
    ..\target\debug\econocode.exe %%f --run >nul 2>&1
    if !errorlevel! equ 0 (
        echo   PASS
        set /a PASSED+=1
    ) else (
        echo   FAIL
        set /a FAILED+=1
    )
    echo.
)

echo Test Summary:
echo Total: %TOTAL%
echo Passed: %PASSED%
echo Failed: %FAILED%
