#include <vxWorks.h>
#include <taskLib.h>
#include <errnoLib.h>
#include <semLib.h>

int taskIdSelfWrapper() {
    return taskIdSelf();
}


int taskSpawnWrapper(
    char *  name,      /* name of new task (stored at pStackBase) */
    int     priority,  /* priority of new task */
    int     options,   /* task option word */
    int     stackSize, /* size (bytes) of stack needed plus name */
    FUNCPTR entryPt,   /* entry point of new task */
    int     arg1,      /* 1st of 10 req'd task args to pass to func */
    int     arg2,
    int     arg3,
    int     arg4,
    int     arg5,
    int     arg6,
    int     arg7,
    int     arg8,
    int     arg9,
    int     arg10
) {
    return taskSpawn(name, priority, options, stackSize, entryPt, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10);
}

int taskDelayWrapper(int tick) {
    return taskDelay(tick);
}

int taskPrioritySetWrapper(int tid, int priority) {
    return taskPrioritySet(tid, priority);
}
int taskPriorityGetWrapper(int tid, int* priority) {
    return taskPriorityGet(tid, priority);
}

int errnoGetWrapper() {
    return errnoGet();
}

SEM_ID semBCreateWrapper(int option, int initial) {
    return semBCreate(option, initial);
}

int semGiveWrapper(SEM_ID sid) {
    return semGive(sid);
}

int semTakeWrapper(SEM_ID sid, int timeout) {
    return semTake(sid, timeout);
}