## 打包为wasm

双击 `build/build_release.bat`将本库打包为`release`版本的wasm，输出文件`pkg/wasm_engine.wasm.ts`和`pkg/wasm_engine.wasm`可直接拷贝到`pi_utils/wasm/`目录下使用

双击 `build/build_profiling.bat`将本库打包为`profiling`版本的wasm，其性能与release版本差距不大，但是保留了rust函数的名称，在调试中发挥一定作用。输出文件`pkg/wasm_engine.wasm.ts`和`pkg/wasm_engine.wasm`可直接拷贝到`pi_utils/wasm/`目录下使用

build/build_debug.bat将本库编译为debug版本的wasm，但debug版本会对函数接口的输入参数作类型检查，一些gui提供的接口不能正确运行（如，ios的小游戏中的wasm对bool类型的参数支持存在bug，gui将这类型的参数在js层强行改为了数字类型，与rust层要求的参数类型不符合，debug版本调用这类接口会出错）
