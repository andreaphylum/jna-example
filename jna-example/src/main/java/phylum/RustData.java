package phylum;

import com.sun.jna.*;
import java.util.List;
import java.util.Arrays;

public interface RustData extends Library {
  String JNA_LIBRARY_NAME = "jna_example";
  NativeLibrary JNA_NATIVE_LIB = NativeLibrary.getInstance(JNA_LIBRARY_NAME);

  RustData INSTANCE = (RustData) Native.loadLibrary(JNA_LIBRARY_NAME, RustData.class);

  Pointer rust_data_new();
  String rust_data_package_id(Pointer self);
  String rust_data_var_names_next(Pointer self);
  void rust_data_drop(Pointer self);
}
