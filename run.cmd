Move-Item *.exe ./build/executables/ -Force
Move-Item *.o   ./build/objects/     -Force
Move-Item *.lin ./examples/basic/    -Force
Move-Item *.pdb ./build/debug/       -Force

cmd /C "dumpbin /EXPORTS lib\dotlin_runtime.lib || true"
