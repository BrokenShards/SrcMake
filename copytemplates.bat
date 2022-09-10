
:: Copy Assets
echo off

set "copy_dir=%solution_dir%templates\\"
set "target_dir=%build_dir%target\debug\templates\\"
call :XCopyDirectory

set "target_dir=%build_dir%target\release\templates\\"
call :XCopyDirectory

exit /B 0

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
