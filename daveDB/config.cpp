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
		tag = "daveDB";
		class System {
			file = "\davedb";
			class init {preInit=1;};
			class test {};
		};
	};
};