use env;

env.set_var("path", "/home/antes/path");
println(env.get_var("path"));

env.unset("path");
println(env.get_var("path")); # null

println(env.var("HOME"));

println(env.cwd());
println(env.home());

println(env.arch);
println(env.os);
println(env.family);
println(env.dll);
println(env.exe);
