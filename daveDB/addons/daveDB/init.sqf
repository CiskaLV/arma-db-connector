"ext" callExtension ["uuid", []];

addMissionEventHandler ["ExtensionCallback", {
    params ["_name", "_component", "_data"];
    if ((tolower _name) != "ext_log") exitWith {};
    (parseSimpleArray _data) params ["_level", "_message"];
    diag_log text format ["[Dave DB] (%1) %2: %3", _component, _level, _message];
}];