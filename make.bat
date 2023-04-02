cargo build --release

cd .\out

mkdir @daveDB
mkdir @daveDB\addon

D:\"PBO Manager"\pboc.exe pack ..\daveDB\ -o .\@daveDB\addon\

copy ..\target\release\davedb.dll .\@daveDB\davedb_x64.dll
