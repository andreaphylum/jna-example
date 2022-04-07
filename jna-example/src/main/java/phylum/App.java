package phylum;

import com.sun.jna.Pointer;

public class App {
  public static void main( String[] args ) {
    System.setProperty("jna.library.path", "../target/release");
    Pointer ptr = RustData.INSTANCE.rust_data_new();
    System.out.println(RustData.INSTANCE.rust_data_package_id(ptr));
    System.out.println(RustData.INSTANCE.rust_data_var_names_next(ptr));
    System.out.println(RustData.INSTANCE.rust_data_var_names_next(ptr));
    System.out.println(RustData.INSTANCE.rust_data_var_names_next(ptr));
    System.out.println(RustData.INSTANCE.rust_data_var_names_next(ptr));
    System.out.println(RustData.INSTANCE.rust_data_var_names_next(ptr));
    System.out.println(RustData.INSTANCE.rust_data_var_names_next(ptr));
    System.out.println(RustData.INSTANCE.rust_data_var_names_next(ptr));
    RustData.INSTANCE.rust_data_drop(ptr);
  }
}
