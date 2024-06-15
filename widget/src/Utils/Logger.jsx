const LOGGING_ON = true;

function logInfo(arg1, arg2, arg3, arg4) {
    if(LOGGING_ON) {
        const nonEmptyArgs = [arg1, arg2, arg3, arg4].filter((argument) => !!argument);

        console.log(...nonEmptyArgs);
    }
}

return { logInfo };