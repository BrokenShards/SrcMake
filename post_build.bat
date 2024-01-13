:: Copy Assets
@echo off

set "copy_dir=templates\\"
set "target_dir=target\debug\templates\\"
call :XCopyDirectory

set "target_dir=target\release\templates\\"
call :XCopyDirectory

set "copy_dir=languages\\"
set "target_dir=target\debug\languages\\"
call :XCopyDirectory

set "target_dir=target\release\languages\\"
call :XCopyDirectory

exit /B 0

:: Functions
:XCopyDirectory
xcopy "%copy_dir%" "%target_dir%" /Q /I /E /K /R /H /Y
call :PrintResult
exit /B %ERRORLEVEL%

:PrintResult
if %ERRORLEVEL% NEQ 0 ( 
   echo Failed copying from %copy_dir% to %target_dir% && exit /B %ERRORLEVEL%
) else (
	echo Coppied %copy_dir% to %target_dir%
)
exit /B 0
