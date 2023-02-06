((globalThis) => {
    const { core } = Deno;
    const { ops } = core;
    // Note: Do not call this when snapshotting, it should be called
    // at runtime. This example does not use V8 snapshots.
    core.initializeAsyncOps();

    function argsToMessage(...args) {
        return args.map((arg) => JSON.stringify(arg)).join(" ");
    }

    globalThis.console = {
        log: (...args) => {
            core.print(`[out]: ${argsToMessage(...args)}\n`, false);
        },
        error: (...args) => {
            core.print(`[err]: ${argsToMessage(...args)}\n`, true);
        },
    };

    globalThis.runjs = {
        readFile: (path) => {
            return ops.op_read_file(path);
        },
        writeFile: (path, contents) => {
            return ops.op_write_file(path, contents);
        },
        removeFile: (path) => {
            return ops.op_remove_file(path);
        },
    };
    globalThis.monolith = {
        read(dbname,data){
            return ops.op_db_read(dbname,data)
        },
        delete(dbname,data){
            return ops.op_db_delete(dbname,data)
        },
        create(dbname,data){
            return ops.op_db_create(dbname,data)
        },
        write(dbname,data){
            return ops.op_db_write(dbname,data)
        },
        all(dbname,data){
            return ops.op_db_read_all(dbname,data)
        },
        fromFile(dbname,data){
            return ops.op_db_write_from_file(dbname,data)
        },
    };
})(globalThis);