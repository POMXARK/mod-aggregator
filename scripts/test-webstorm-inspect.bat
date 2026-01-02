@echo off
REM Test WebStorm inspect.bat

echo Running WebStorm inspection...
echo Project: %~dp0..
echo Profile: %~dp0..\.idea\inspectionProfiles\Project_Default.xml
echo Output: %~dp0..\inspection-results\webstorm-test.xml

"C:\Program Files\JetBrains\WebStorm 2025.2.3\bin\inspect.bat" "%~dp0.." "%~dp0..\.idea\inspectionProfiles\Project_Default.xml" "%~dp0..\inspection-results\webstorm-test.xml" -v2

echo Inspection completed with exit code %ERRORLEVEL%
pause

