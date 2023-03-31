
diag_log("Inside of test");

private _uuid = "davedb" callExtension ["db:init", ["mysql"]];
diag_log(format["UUID: %1", _uuid]);

private _query = "davedb" callExtension ["db:query", [_uuid,"SELECT 1;"]];

diag_log(format["Query: %1", _query]);

