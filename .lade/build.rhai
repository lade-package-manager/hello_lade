let current = path("./main.rs");
print(current.read_file());

system("rustc", ["main.rs", "-o", "hello_lade"])
