@echo off
REM Test script for Aegis Sentinel clipboard monitoring (Windows)
REM This script demonstrates how the application detects sensitive patterns

echo.
echo 🛡️  Aegis Sentinel - Clipboard Monitor Test
echo ============================================
echo.
echo This script will test the clipboard monitoring patterns.
echo Please run 'npm run tauri dev' in another terminal first.
echo.
pause

echo.
echo Test 1: Generic API Key Pattern
echo --------------------------------
echo Copying: api_key: sk_test_1234567890abcdefghijklmnop
echo api_key: sk_test_1234567890abcdefghijklmnop | clip
timeout /t 2 /nobreak > nul

echo.
echo Test 2: Password Pattern
echo ------------------------
echo Copying: password: MySecurePassword123
echo password: MySecurePassword123 | clip
timeout /t 2 /nobreak > nul

echo.
echo Test 3: AWS Credentials
echo -----------------------
echo Copying: AKIA1234567890ABCDEF
echo AKIA1234567890ABCDEF | clip
timeout /t 2 /nobreak > nul

echo.
echo Test 4: GitHub Token
echo --------------------
echo Copying: ghp_123456789012345678901234567890abcdef
echo ghp_123456789012345678901234567890abcdef | clip
timeout /t 2 /nobreak > nul

echo.
echo Test 5: Safe Text (No Alert Expected)
echo --------------------------------------
echo Copying: This is just normal text that should not trigger any alerts
echo This is just normal text that should not trigger any alerts | clip
timeout /t 2 /nobreak > nul

echo.
echo ✅ Tests complete!
echo.
echo Check the Aegis Sentinel application window to see the detected events.
echo You should see 4 security warnings and the risk meter should be elevated.
pause
