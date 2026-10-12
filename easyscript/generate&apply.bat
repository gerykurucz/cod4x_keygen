@echo off
setlocal EnableDelayedExpansion

set "REGPATH=HKCU\Software\Activision\Call of Duty 4"
set "REGVAL=codkey"

rem ============================================================
rem  STEP 1: Generate + apply codkey if not already present
rem ============================================================
reg query "%REGPATH%" /v "%REGVAL%" >nul 2>&1
if !errorlevel! neq 0 (
    set "CHARSET=0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    set "HEXDIGITS=0123456789ABCDEF"

    set "n=48"
    for %%c in (0 1 2 3 4 5 6 7 8 9) do (
        set "VAL_%%c=!n!"
        set /a "n+=1"
    )
    set "n=65"
    for %%c in (A B C D E F G H I J K L M N O P Q R S T U V W X Y Z) do (
        set "VAL_%%c=!n!"
        set /a "n+=1"
    )

    set "KEY="
    for /l %%i in (1,1,16) do (
        set /a "idx=!random! %% 36"
        for %%I in (!idx!) do set "KEY=!KEY!!CHARSET:~%%I,1!"
    )

    set /a "v2=0"
    for /l %%i in (0,1,15) do (
        set "c=!KEY:~%%i,1!"
        for %%x in (!c!) do set "cv=!VAL_%%x!"
        set /a "v2=v2^cv"
        for /l %%j in (1,1,8) do (
            set /a "bit=v2&1"
            if !bit! NEQ 0 set /a "v2=v2^0x14002"
            set /a "v2=v2>>1"
        )
        set /a "v2=v2&0xFFFF"
    )

    set "CS="
    set /a "tmp=v2"
    for /l %%i in (1,1,4) do (
        set /a "nib=tmp&15"
        for %%n in (!nib!) do set "CS=!CS!!HEXDIGITS:~%%n,1!"
        set /a "tmp=tmp>>4"
    )
    set "CS=!CS:~3,1!!CS:~2,1!!CS:~1,1!!CS:~0,1!"

    set "CDKEY=!KEY!!CS!"
    reg add "%REGPATH%" /v "%REGVAL%" /t REG_SZ /d "!CDKEY!" /f >nul 2>&1
)

endlocal
exit /b 0