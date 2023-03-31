class CfgPatches {
	class daveDB {
		projectName="daveDB";
		author="Ciska";
		version="0.0.1";
		requiredAddons[] = {};
		units[] = {};
	};
};

class CfgFunctions {
	class daveDB {
		file = "daveDB";
        class init {
            preInit = 1;
        };
	};
};