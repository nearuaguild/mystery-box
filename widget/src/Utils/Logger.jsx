const LOGGING_ON = false;

function logInfo(arg1, arg2, arg3, arg4) {
    if(LOGGING_ON) {
        const nonEmptyArgs = [arg1, arg2, arg3, arg4].filter((argument) => argument != undefined);

        console.log(...nonEmptyArgs);
    }
}

function logError(arg1, arg2, arg3, arg4) {
    if(LOGGING_ON) {
        const nonEmptyArgs = [arg1, arg2, arg3, arg4].filter((argument) => !!argument);

        console.error(...nonEmptyArgs);
    }
}

return { logInfo };