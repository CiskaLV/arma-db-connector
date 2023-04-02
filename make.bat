cargo build --release

cd .\out

mkdir @daveDB
mkdir @daveDB\addons

D:\"PBO Manager"\pboc.exe pack ..\daveDB\ -o .\@daveDB\addons\

copy ..\target\release\davedb.dll .\@daveDB\davedb_x64.dll
